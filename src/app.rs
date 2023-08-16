use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::{collections::HashMap, vec};

use crate::components::Component;
use serde_json::json;
use uuid::Uuid;

use crate::components::line_chart::LineChart;
use crate::components::stateful_list::MultiStatefulList;
use crate::components::user_input::UserInput;
use crate::util;
use human_repr::HumanCount;
use serde_json::{self, Value};
use std::collections::BTreeMap;
use ureq;
use url::form_urlencoded;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const API_ENDPOINT: &str = "https://trends.shodan.io";
const API_TIMEOUT: u64 = 90; // in seconds

// Trends API data already in right format so we just need a bit mapping, otherwise use create chrono for datetime parsing
const MONTH_ABBR: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];
pub const EXIT_ERROR_CODE: i32 = 1;
pub const EXIT_SUCCESS_CODE: i32 = 0;

#[derive(Debug, Clone)]
pub struct Points {
    pub label: String,
    pub total: i64,
    pub data: Vec<(f64, f64)>,
}

#[derive(Debug, Clone, Default)]
pub struct Chart {
    pub datasets: Vec<Points>,
    pub x_bounds: Vec<f64>,
    pub y_bounds: Vec<f64>,
    pub x_ticks: Vec<String>,
    pub y_ticks: Vec<String>,
    pub x_labels: Vec<String>,
    pub facets: Option<Box<Chart>>,
}

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub blocking: usize, // Show block when making API request, 0 for unblock, 1 -> Searching. 2 -> Searching..
    pub blocking_char: String,
    api_key: String,
    pub no_results: bool,
    pub queries: Vec<String>, // Hold success queries (exclude no results or errored out query)
    pub last_query: String,   // Last submitted query
    pub prev_query: String,

    pub charts: BTreeMap<String, Chart>,
    pub api_error: String,

    pub search_input: UserInput,
    pub facets_input: UserInput,
    pub line_chart: LineChart,
    pub saved_queries: MultiStatefulList<String>,
    pub facet_values: MultiStatefulList<String>,
    pub widget_index: usize,

    pub receiver: mpsc::Receiver<Result<ureq::Response, ureq::Error>>,
}

#[derive(Debug)]
pub struct FacetIndex {
    pub selected: Option<usize>,
    pub selected_indexes: Vec<usize>,
}

// Shared state between UI and widgets because we can't pass &mut App when calling below, it raised borrow errors
// src/handler.rs: widgets[widget_index].handle_events(event.clone(), state);
#[derive(Debug)]
pub struct AppState {
    pub unfocused: bool, // If there is no focused panel
    pub submitted: bool,
    pub first_render: bool,
    pub facet_indexes: HashMap<String, FacetIndex>, // Saved <query.facet_values, selected_indexes>
    pub app_log: String,                            // Application log show at the bottom
    pub sender: mpsc::Sender<Result<ureq::Response, ureq::Error>>,
}

impl App {
    pub fn new(
        query: String,
        facets: String,
        receiver: mpsc::Receiver<Result<ureq::Response, ureq::Error>>,
    ) -> Self {
        let api_key = match util::get_api_key() {
            Ok(key) => key,
            Err(_) => {
                println!(
                    "Error: {}",
                    "Missing API key, please run \"strend init <API key>\""
                );
                std::process::exit(EXIT_ERROR_CODE);
            }
        };

        let mut app = Self {
            running: true,
            blocking: 0,
            blocking_char: String::from("."),
            api_key,

            queries: vec![],
            last_query: String::new(),
            prev_query: String::new(),
            charts: BTreeMap::new(),
            api_error: String::new(),
            no_results: false,

            saved_queries: MultiStatefulList::new(),
            facet_values: MultiStatefulList::new(),
            line_chart: LineChart::new(),
            search_input: UserInput::new(query),
            facets_input: UserInput::new(facets),
            widget_index: 0,

            receiver,
        };

        // Default hide some widgets
        app.saved_queries.set_hide(true);
        app.facet_values.set_hide(true);
        app.line_chart.set_hide(true);

        // Point to last panel so first Tab will focus on searchbox
        app.widget_index = app.get_widgets().len() - 1;
        app
    }

    pub fn get_widgets(&mut self) -> Vec<&mut dyn Component> {
        vec![
            &mut self.search_input,
            &mut self.facets_input,
            &mut self.saved_queries,
            &mut self.facet_values,
            &mut self.line_chart,
        ]
    }

    pub fn get_widget_index(&mut self, widget_id: Uuid) -> usize {
        let widget_index = self
            .get_widgets()
            .iter()
            .position(|widget| widget.id() == widget_id);
        widget_index.unwrap_or(0)
    }

    pub fn select_widget(&mut self, widget_index: usize) {
        for (index, widget) in self.get_widgets().into_iter().enumerate() {
            if widget_index == index && !widget.hidden() {
                widget.set_focus(true);
            } else {
                widget.set_focus(false);
            }
        }

        self.widget_index = widget_index;
    }

    pub fn switch_widgets(&mut self, state: &mut AppState, reverse: bool) -> AppResult<()> {
        let widgets_len = self.get_widgets().len();
        let mut success = false;

        while !success {
            let new_widget: usize = if reverse {
                self.widget_index
                    .wrapping_sub(1)
                    .min(widgets_len.saturating_sub(1))
            } else {
                self.widget_index.saturating_add(1) % widgets_len
            };

            self.select_widget(new_widget);
            state.unfocused = false;

            // Switch to next until find visible widget
            if !self.get_widgets()[new_widget].hidden() {
                success = true;
            }
        }

        Ok(())
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) -> AppResult<()> {
        // TODO Should we move the process out of tick event, maybe custom update event?
        match self.receiver.try_recv() {
            Ok(resp) => {
                let query = self.search_input.get_input().to_owned();
                let facets = self.facets_input.get_input().trim().to_owned();
                let encoded_query: String = form_urlencoded::Serializer::new(String::new())
                    .append_pair("query", &query)
                    .append_pair("facets", &facets)
                    .finish();
                // Save last submitted query
                self.last_query = encoded_query.to_owned();

                match resp {
                    Ok(response) => {
                        // As resp_json["facets"]["key"] key is dynamic based on user request,
                        // I din't find a proper way to define JSON response mapping struct for it so parse manually
                        let resp_str = response.into_string()?;
                        let resp_json: Result<Value, serde_json::Error> =
                            serde_json::from_str(&resp_str);

                        match resp_json {
                            Ok(resp_json) => {
                                let total = resp_json["total"].as_i64().unwrap();
                                // No results found
                                if total == 0 {
                                    self.no_results = true;
                                    // Make sure to release lock before return
                                    self.blocking = 0;
                                } else {
                                    let mut x_axis: f64 = 0.0;
                                    let mut x_axis_labels: Vec<String> = vec![];
                                    let mut max_y_axis = 0.0;
                                    let mut data: Vec<(f64, f64)> = vec![];

                                    for item in resp_json["matches"].as_array().unwrap() {
                                        let count = item["count"].as_i64().unwrap() as f64;
                                        if count > max_y_axis {
                                            max_y_axis = count;
                                        }

                                        data.push((x_axis, count));
                                        x_axis += 1.0; // Represent each YYYY-MM as float point data

                                        let month_str = item["month"].as_str().unwrap();
                                        let parts: Vec<&str> = month_str.split("-").collect();
                                        x_axis_labels.push(format!(
                                            "{} {}",
                                            MONTH_ABBR[parts[1].parse::<usize>().unwrap() - 1], // Index start from 0
                                            parts[0]
                                        ));
                                    }

                                    // Other chart data
                                    x_axis -= 1.0;
                                    let x_bounds = vec![0.0, x_axis];
                                    let y_bounds = vec![0.0, max_y_axis];

                                    // Just use three labels as current line chart looks weird on too many ticks
                                    // https://github.com/ratatui-org/ratatui/issues/334#issuecomment-1641459034
                                    let x_axis_len = x_axis_labels.len();
                                    let x_ticks = vec![
                                        x_axis_labels[0].to_owned(),
                                        x_axis_labels[x_axis_len / 2].to_owned(),
                                        x_axis_labels[x_axis_len - 1].to_owned(),
                                    ];
                                    // Convert float to human-readable format
                                    let y_ticks = vec![
                                        String::from("0"),
                                        ((max_y_axis / 2.0) as i64).human_count_bare().to_string(),
                                        (max_y_axis as i64).human_count_bare().to_string(),
                                    ];

                                    // If users requested facets then generate data for build facets line chart later
                                    let facets_data: Option<Box<Chart>> = match !facets.is_empty() {
                                        true => {
                                            // TODO Currently, we built chart for only first facet, also the API limit to 1 facet.
                                            let first_facet = facets
                                                .split(",")
                                                .nth(0)
                                                .unwrap()
                                                .split(":")
                                                .nth(0)
                                                .unwrap();

                                            let mut x_axis: f64 = 0.0;
                                            let mut x_axis_labels: Vec<String> = vec![];
                                            let mut facet_values: HashMap<String, i64> =
                                                HashMap::new();
                                            let mut month_value_maps: Vec<HashMap<String, f64>> =
                                                vec![];
                                            let mut max_y_axis = 0.0;
                                            let mut datasets = vec![];

                                            // Get mappings facet value -> count of each month
                                            for item in
                                                resp_json["facets"][first_facet].as_array().unwrap()
                                            {
                                                let mut tmp_values: HashMap<String, f64> =
                                                    HashMap::new();

                                                for bucket in item["values"].as_array().unwrap() {
                                                    let value = match bucket["value"].as_str() {
                                                        Some(value) => value.to_owned(),
                                                        // Some facet is number, e.g. port, http.html_hash
                                                        None => bucket["value"]
                                                            .as_i64()
                                                            .unwrap()
                                                            .to_string(),
                                                    };
                                                    let count =
                                                        bucket["count"].as_i64().unwrap() as f64;

                                                    if count > max_y_axis {
                                                        max_y_axis = count;
                                                    }

                                                    *facet_values
                                                        .entry(value.clone())
                                                        .or_insert(0) += count as i64;
                                                    tmp_values.insert(value, count);
                                                }

                                                month_value_maps.push(tmp_values);
                                                x_axis += 1.0; // Represent each YYYY-MM as float point data

                                                let month_str = item["key"].as_str().unwrap();
                                                let parts: Vec<&str> =
                                                    month_str.split("-").collect();
                                                x_axis_labels.push(format!(
                                                    "{} {}",
                                                    MONTH_ABBR
                                                        [parts[1].parse::<usize>().unwrap() - 1], // Index start from 0
                                                    parts[0]
                                                ));
                                            }

                                            // Construct line chart Points for each facet value
                                            for (name, total) in facet_values.iter() {
                                                let mut data: Vec<(f64, f64)> = vec![];
                                                for (month, maps) in
                                                    month_value_maps.iter().enumerate()
                                                {
                                                    data.push((
                                                        month as f64,
                                                        maps.get(name).cloned().unwrap_or(0.0),
                                                    ));
                                                }

                                                datasets.push(Points {
                                                    label: name.to_owned(),
                                                    total: *total,
                                                    data,
                                                });
                                            }

                                            x_axis -= 1.0;
                                            let x_bounds = vec![0.0, x_axis];
                                            let y_bounds = vec![0.0, max_y_axis];

                                            // Just use three labels as current line chart looks weird on too many ticks
                                            let x_axis_len = x_axis_labels.len();
                                            let x_ticks = vec![
                                                x_axis_labels[0].to_owned(),
                                                x_axis_labels[x_axis_len / 2].to_owned(),
                                                x_axis_labels[x_axis_len - 1].to_owned(),
                                            ];
                                            let y_ticks = vec![
                                                String::from("0"),
                                                ((max_y_axis / 2.0) as i64)
                                                    .human_count_bare()
                                                    .to_string(),
                                                (max_y_axis as i64).human_count_bare().to_string(),
                                            ];

                                            // A bit sorting facet value has most records first
                                            datasets.sort_by(|a, b| b.total.cmp(&a.total));

                                            Some(Box::new(Chart {
                                                datasets,
                                                x_bounds,
                                                y_bounds,
                                                x_ticks,
                                                y_ticks,
                                                x_labels: x_axis_labels,
                                                ..Default::default()
                                            }))
                                        }
                                        false => None,
                                    };

                                    // Save data to display chart
                                    self.charts.insert(
                                        encoded_query.to_owned(),
                                        Chart {
                                            datasets: vec![Points {
                                                label: encoded_query.to_owned(),
                                                total,
                                                data,
                                            }],
                                            x_bounds,
                                            y_bounds,
                                            x_ticks,
                                            y_ticks,
                                            x_labels: x_axis_labels,
                                            facets: facets_data,
                                            ..Default::default()
                                        },
                                    );

                                    // Saved queries to display in sidebar
                                    if !self.queries.contains(&encoded_query) {
                                        self.queries.push(encoded_query);
                                    }

                                    self.no_results = false;
                                }

                                // Clear error message
                                self.api_error = "".to_string();
                            }
                            Err(_) => {
                                self.api_error = format!("{}", "Failed to parse API response.");
                            }
                        }
                    }
                    Err(ureq::Error::Status(_, response)) => {
                        let resp_str = response.into_string()?;
                        let error: Value = serde_json::from_str(&resp_str).unwrap_or(json!({
                            // Failed to parse, e.g. 503 Service Unavailable
                            "error": "Search failed, please try again later.",
                        }));

                        // API return defined error response
                        self.api_error = format!("{}", error["error"].as_str().unwrap());
                    }
                    Err(err) => {
                        // Some kind of io/transport error
                        if err.to_string().contains("timed out") {
                            self.api_error = format!("{}", "Timed out, please try again later.");
                        } else {
                            self.api_error =
                                format!("{}", "Search failed, please try again later.");
                        }
                    }
                };

                // Release lock
                self.blocking = 0;
            }
            Err(_) => {}
        }

        // Some widgets need to hide and unfocused (not handle events)
        if self.queries.is_empty() {
            if !self.api_error.is_empty() || self.no_results {
                self.saved_queries.set_hide(true);
                self.facet_values.set_hide(true);
                self.line_chart.set_hide(true);
            }
        } else {
            if !self.api_error.is_empty() || self.no_results {
                self.line_chart.set_hide(true);
            } else {
                self.saved_queries.set_hide(false);
                self.facet_values.set_hide(false);
                self.line_chart.set_hide(false);
            }
        }

        Ok(())
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn search(
        &mut self,
        sender: mpsc::Sender<Result<ureq::Response, ureq::Error>>,
    ) -> AppResult<()> {
        let query = self.search_input.get_input().to_owned();
        let facets = self.facets_input.get_input().trim().to_owned();

        // Pre validate to skip API call
        if query.is_empty() {
            // TODO Save allowed facets then do local check?
            self.api_error = format!("{}", "Invalid search query");
        } else {
            // Lock application, delay terminal events
            self.blocking = 1;
            let api_key = self.api_key.to_owned();

            // Make API request in the background
            thread::spawn(move || {
                let resp: Result<ureq::Response, ureq::Error> =
                    ureq::get(&format!("{}/api/v1/search", API_ENDPOINT))
                        .query("query", &query)
                        .query("facets", &facets)
                        .query("key", &api_key)
                        .timeout(Duration::from_secs(API_TIMEOUT))
                        .call();
                // Let self.tick (unblocking function) process API response
                sender.send(resp).unwrap();
            });
        }

        Ok(())
    }
}
