use std::cmp;
use std::collections::HashMap;
use std::vec;

use crate::components::KeySymbols;
use crate::widgets::list::{List as MultiList, ListItem as MultiListItem};
use human_repr::HumanCount;
use ratatui::prelude::*;
use ratatui::widgets::*;
use ratatui::{
    backend::Backend,
    layout::Alignment,
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use url::form_urlencoded;

use crate::app::App;
use crate::app::AppState;
use crate::components::Component;

// Pre parsed Trends Rgb colors from hex with https://github.com/emgyrz/colorsys.rs
const LINE_COLORS: [Color; 30] = [
    Color::Rgb(213, 5, 39),
    Color::Rgb(21, 137, 64),
    Color::Rgb(248, 152, 253),
    Color::Rgb(36, 201, 215),
    Color::Rgb(203, 155, 100),
    Color::Rgb(134, 104, 136),
    Color::Rgb(34, 230, 122),
    Color::Rgb(229, 9, 174),
    Color::Rgb(157, 171, 250),
    Color::Rgb(67, 126, 138),
    Color::Rgb(178, 27, 255),
    Color::Rgb(255, 123, 145),
    Color::Rgb(148, 170, 5),
    Color::Rgb(172, 89, 6),
    Color::Rgb(130, 166, 141),
    Color::Rgb(254, 102, 22),
    Color::Rgb(122, 115, 82),
    Color::Rgb(249, 188, 15),
    Color::Rgb(182, 93, 102),
    Color::Rgb(7, 162, 230),
    Color::Rgb(192, 145, 174),
    Color::Rgb(134, 116, 210),
    Color::Rgb(138, 145, 167),
    Color::Rgb(136, 252, 7),
    Color::Rgb(234, 66, 254),
    Color::Rgb(158, 128, 16),
    Color::Rgb(16, 180, 55),
    Color::Rgb(194, 129, 254),
    Color::Rgb(249, 43, 117),
    Color::Rgb(7, 201, 157),
];
const MAX_SAVED_QUERIES: usize = 5;
const SELECTED_FACET_LINES: usize = 5;

/// Renders the user interface widgets.
// - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
// - https://github.com/tui-rs-revival/ratatui/tree/master/examples
pub fn render<B: Backend>(app: &mut App, state: &mut AppState, frame: &mut Frame<'_, B>) {
    // There are 3 possible layouts when users run command:
    // - Launched without search query
    // - Search query with success results
    // - Search query returns no results or errored out

    // Truncate data before rendering
    while app.charts.len() > MAX_SAVED_QUERIES {
        app.charts.remove_entry(&app.queries[0]);
        app.queries.remove(0);
    }

    // Only display application log in few seconds
    if app.ticks > 40 {
        state.app_log = String::new();
        app.ticks = 0;
    }

    // Run search if first launch with --query --facets, skip if has error
    if state.first_render && !app.search_input.get_input().is_empty() {
        // Use _ to ignore Err() if current function don't have return type
        let _ = app.search(state.sender.clone());
    }

    let layouts = Layout::default()
        .constraints(
            [
                Constraint::Length(4),
                Constraint::Min(0),
                Constraint::Length(2),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let mut help_keys = vec![];
    let mut default_keys = vec![
        format!("Switch panels [{}]", KeySymbols::TAB),
        format!("Exit [{}C]", KeySymbols::CONTROL),
    ];

    if !app.search_input.focused() && !app.facets_input.focused() && !app.line_chart.data.is_empty()
    {
        default_keys.insert(
            default_keys.len() - 1,
            format!("Export [{}E]", KeySymbols::CONTROL),
        );
    }

    // Get focused widget keys
    for (_, widget) in app.get_widgets().into_iter().enumerate() {
        if widget.focused() && !widget.hidden() {
            help_keys = widget.help_keys().to_owned();
            help_keys.push(format!("Unfocused [{}]", KeySymbols::ESC));
            break;
        }
    }

    // Append default keys
    help_keys.append(&mut default_keys.to_owned());
    let footer_padding = Padding {
        left: 0,
        right: 0,
        top: 1,
        bottom: 0,
    };
    let help_commands = Paragraph::new(help_keys.join("  "))
        .block(Block::default().padding(footer_padding.clone()));

    // Show application log if any, e.g. Export chart to ./data.csv
    if !state.app_log.is_empty() {
        let footer_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
            .split(layouts[2]);
        frame.render_widget(help_commands, footer_layout[0]);

        let footer_msg = Paragraph::new(state.app_log.to_string())
            .block(Block::default().padding(footer_padding))
            .alignment(Alignment::Right);
        frame.render_widget(footer_msg, footer_layout[1]);
    } else {
        frame.render_widget(help_commands, layouts[2]);
    }

    // Some reuse widgets
    let error_block = Block::default()
        .title("Error")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red))
        .style(Style::default().fg(Color::Red));
    let error_widget = Paragraph::new(format!(
        "{}\n\
        API documents: https://developer.shodan.io/api
    ",
        app.api_error.to_owned()
    ))
    .style(Style::default().fg(Color::Red))
    .alignment(Alignment::Center);
    let info_block = Block::default().title("Info").borders(Borders::ALL);
    let no_results_widget: Paragraph<'_> =
        Paragraph::new("No results found").alignment(Alignment::Center);
    // A wrapper block used to render info/ no results/ error widget which align center vertically and horizontally
    let center_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
            ]
            .as_ref(),
        )
        .split(layouts[1]);

    let colors_len = LINE_COLORS.len();
    let focused_style = Style::default().fg(Color::Yellow);
    let search_box_style = match app.search_input.focused() || app.facets_input.focused() {
        true => focused_style,
        false => Style::default(),
    };

    // Group 4 blocks into 1 search box block and have custom borders around it
    let search_box_layouts = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(layouts[0]);

    let search_prefix_text = "Query:";
    let search_layouts = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(search_prefix_text.len() as u16 + 3), // Padding some spaces
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(search_box_layouts[0]);

    let facet_prefix_text = "Facets (optional):";
    let facet_layouts = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(facet_prefix_text.len() as u16 + 3),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(search_box_layouts[1]);

    let search_prefix: Paragraph<'_> = Paragraph::new(vec![Line::from(Span::styled(
        search_prefix_text,
        match app.search_input.focused() {
            true => Style::default().bold(),
            _ => Style::default(),
        },
    ))])
    .block(
        Block::default()
            .borders(Borders::LEFT | Borders::TOP)
            .border_style(search_box_style)
            .padding(Padding::new(1, 0, 0, 0)),
    );
    frame.render_widget(search_prefix, search_layouts[0]);

    let search_query = Paragraph::new(app.search_input.get_input()).block(
        Block::default()
            .borders(Borders::TOP | Borders::RIGHT)
            .border_style(search_box_style),
    );
    frame.render_widget(search_query, search_layouts[1]);

    let facet_prefix = Paragraph::new(vec![Line::from(Span::styled(
        facet_prefix_text,
        match app.facets_input.focused() {
            true => Style::default().bold(),
            _ => Style::default(),
        },
    ))])
    .block(
        Block::default()
            .borders(Borders::LEFT | Borders::BOTTOM)
            .border_style(search_box_style)
            .padding(Padding::new(1, 0, 0, 0)),
    );
    frame.render_widget(facet_prefix, facet_layouts[0]);

    let search_facets = Paragraph::new(app.facets_input.get_input()).block(
        Block::default()
            .borders(Borders::RIGHT | Borders::BOTTOM)
            .border_style(search_box_style),
    );
    frame.render_widget(search_facets, facet_layouts[1]);

    // Show cursor if focused in UserInput
    if app.search_input.focused() {
        frame.set_cursor(
            // Draw the cursor at the current position in the input field.
            // This position is can be controlled via the left and right arrow key
            search_layouts[1].x + app.search_input.cursor_position as u16,
            // Move one line down, from the border to the input line
            search_layouts[1].y + 1,
        );
    } else if app.facets_input.focused() {
        frame.set_cursor(
            facet_layouts[1].x + app.facets_input.cursor_position as u16,
            facet_layouts[1].y,
        );
    }

    // API request in the background
    if app.blocking > 0 {
        let mut dots = vec![app.blocking_char.to_owned(); app.blocking];
        while dots.len() <= 3 {
            // Pad some spaces so Alignment::Center block layout not moving
            dots.push(String::from(" "));
        }

        let wrapper_block =
            Block::default()
                .borders(Borders::ALL)
                .border_style(match app.line_chart.focused() {
                    true => focused_style,
                    _ => Style::default(),
                });
        let loading =
            Paragraph::new(format!("Searching{}\n", dots.join(""))).alignment(Alignment::Center);
        frame.render_widget(wrapper_block, layouts[1]);
        frame.render_widget(loading, center_layout[1]);

        app.blocking += 1;
        // Only show 3 dots
        if app.blocking > 3 {
            app.blocking = 1;
        }
    } else if app.queries.is_empty() {
        // First query errored out or has no results
        if !app.api_error.is_empty() {
            frame.render_widget(error_block, layouts[1]);
            frame.render_widget(error_widget, center_layout[1]);
        } else if app.no_results {
            frame.render_widget(info_block, layouts[1]);
            frame.render_widget(no_results_widget, center_layout[1]);
        } else {
            // Launched without search query
            frame.render_widget(info_block, layouts[1]);

            let welcome = Paragraph::new(
                "Make search by `Enter` a query in search box.\n\
                        Press `Ctrl-C` to stop running, switch between panels by `Tab`",
            )
            .alignment(Alignment::Center);
            frame.render_widget(welcome, center_layout[1]);
        }
    } else {
        // We have saved queries then we should show it, errors on the right side if any
        let main_layouts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(layouts[1]);
        let sidebar_layouts = Layout::default()
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(main_layouts[0]);

        // Build saved queries block
        let mut query_lines: Vec<String> = vec![];
        let mut query_items: Vec<MultiListItem> = vec![];
        // Color mapping used to draw chart later
        let mut query_colors: HashMap<String, Color> = HashMap::new();

        for (index, query) in app.queries.iter().rev().enumerate() {
            let label_color = LINE_COLORS[index];
            query_colors.insert(query.to_owned(), label_color);

            let lines = vec![Line::from(vec![query.to_owned().into()])];
            query_items.push(MultiListItem::new(lines).style(Style::default().fg(label_color)));
            query_lines.push(query.to_owned());
        }

        // Auto highlight on first initialize if have only one saved query and there is no error
        if app.charts.len() == 1
            && app.saved_queries.items.is_empty()
            && app.api_error.is_empty()
            && !app.no_results
        {
            app.saved_queries.state.with_selected_indexes(vec![0]);
            app.saved_queries.state.select(Some(0));
        }
        app.saved_queries.set_items(query_lines.clone());

        let saved_queries = MultiList::new(query_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Saved queries")
                    .border_style(match app.saved_queries.focused() {
                        true => focused_style,
                        _ => Style::default(),
                    }),
            )
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(" [x] ")
            .unselect_symbol(" [ ] ");
        frame.render_stateful_widget(
            saved_queries,
            sidebar_layouts[0],
            &mut app.saved_queries.state,
        );

        // Reserve empty facet values block, update later if user requested facets in their query
        let facet_values = Block::default()
            .title("Facet values")
            .borders(Borders::ALL)
            .border_style(match app.facet_values.focused() {
                true => focused_style,
                _ => Style::default(),
            });
        frame.render_widget(facet_values, sidebar_layouts[1]);

        // Get last submitted query
        let mut selected_query = &app.last_query.to_owned();

        // On submit new query
        if state.submitted {
            for (index, query) in query_lines.iter().enumerate() {
                if selected_query == query {
                    app.saved_queries.state.with_selected_indexes(vec![index]);
                    app.saved_queries.state.select(Some(index));
                    break;
                }
            }
        } else {
            // Only handle users interactive event in MultiStatefulList, the above `app.saved_queries.state.select` won't go there
            if let Some(index) = app.saved_queries.state.selected() {
                for (i, query) in query_lines.iter().enumerate() {
                    if i == index {
                        selected_query = query;
                        break;
                    }
                }

                // Load correct query/ facets in search box if select differently with previous
                if selected_query != &app.prev_query {
                    let decoded_query = form_urlencoded::parse(selected_query.as_bytes());
                    for pair in decoded_query.into_iter() {
                        if pair.0 == "query" {
                            app.search_input.set_input(&pair.1);
                        } else if pair.0 == "facets" {
                            app.facets_input.set_input(&pair.1);
                        }
                    }
                }
            }
        }

        // Default draw total chart if unfocused facet values block
        if !app.facet_values.focused() {
            // Construct export data in CSV format which easy load to Excel for generate charts
            // Month    | query=nginx&facets=asn%3A10 | query=apache&facets=org
            // Jun 2017 | 19799459,27382961           |
            // Jul 2017 | 21077099,29138371           |
            // ...
            // Jul 2023 | 37054878,20837852           |
            let mut chart_data: Vec<Vec<String>> = vec![vec!["Month".to_string()]];
            let mut datasets = vec![];

            // Just get one X Axis as it's same for all charts
            let mut x_bounds = vec![];
            let mut x_ticks = vec![];
            if let Some(entry) = app.charts.last_entry() {
                x_bounds = entry.get().x_bounds.clone();
                x_ticks = entry.get().x_ticks.clone();
                for month in &entry.get().x_labels {
                    chart_data.push(vec![month.to_owned()]);
                }
            }

            // Have to rebuild Y Axis data from selected charts
            let mut max_y_axis = 0.0;

            for index in app.saved_queries.state.selected_indexes() {
                let query = &app.queries[*index];
                if let Some(chart) = app.charts.get(query) {
                    datasets.push(
                        Dataset::default()
                            // .name(query.to_owned())
                            .marker(symbols::Marker::Braille)
                            .graph_type(GraphType::Line)
                            .style(Style::default().fg(query_colors[query]))
                            .data(&chart.datasets[0].data),
                    );

                    let chart_y_axis = chart.y_bounds[chart.y_bounds.len() - 1];
                    if chart_y_axis > max_y_axis {
                        max_y_axis = chart_y_axis;
                    }

                    // Build chart data
                    chart_data[0].push(query.to_string());
                    for (i, point) in chart.datasets[0].data.iter().enumerate() {
                        chart_data[i + 1].push(point.1.to_string());
                    }
                }
            }

            let y_bounds = vec![0.0, max_y_axis];
            // Convert float to human-readable format
            let y_ticks = vec![
                String::from("0"),
                ((max_y_axis / 2.0) as i64).human_count_bare().to_string(),
                (max_y_axis as i64).human_count_bare().to_string(),
            ];

            if !datasets.is_empty() {
                let query_chart = Chart::new(datasets)
                    .block(
                        Block::default()
                            .borders(Borders::NONE)
                            .padding(Padding::new(1, 0, 1, 0)),
                    )
                    .x_axis(
                        Axis::default()
                            .style(match app.line_chart.focused() {
                                true => focused_style,
                                false => Style::default().fg(Color::Gray),
                            })
                            .bounds([x_bounds[0], x_bounds[x_bounds.len() - 1]])
                            .labels(x_ticks.iter().cloned().map(Span::from).collect()),
                    )
                    .y_axis(
                        Axis::default()
                            // https://github.com/ratatui-org/ratatui/issues/379
                            // .title(Span::styled("num of banners", Style::default()))
                            .style(match app.line_chart.focused() {
                                true => focused_style,
                                false => Style::default().fg(Color::Gray),
                            })
                            .bounds([y_bounds[0], y_bounds[y_bounds.len() - 1]])
                            .labels(y_ticks.iter().cloned().map(Span::from).collect())
                            .labels_alignment(Alignment::Center),
                    );

                frame.render_widget(query_chart, main_layouts[1]);
            }

            if app.line_chart.data != chart_data {
                // Save current chart data for exporting purpose
                app.line_chart.data = chart_data;
            }
        }

        // Build facets blocks corresponding to selected query
        match app.charts.get(selected_query) {
            Some(chart) => {
                let mut facet_colors: HashMap<String, Color> = HashMap::new();
                let mut facet_lines: Vec<String> = vec![];

                // Load facet values if any
                match &chart.facets {
                    Some(chart) => {
                        let mut facet_items: Vec<MultiListItem> = vec![];

                        for (mut index, point) in chart.datasets.iter().enumerate() {
                            // Just a bit catch, but 30 defined colors are too many, and we shouldn't overflow widget
                            while index >= colors_len {
                                index -= colors_len;
                            }
                            let label_color = LINE_COLORS[index];
                            facet_colors.insert(point.label.to_owned(), label_color);

                            let lines = vec![Line::from(vec![point.label.to_owned().into()])];
                            facet_items.push(
                                MultiListItem::new(lines).style(Style::default().fg(label_color)),
                            );
                            facet_lines.push(point.label.to_owned());
                        }

                        // Highlight facet lines
                        if !facet_lines.is_empty() {
                            match state.facet_indexes.get(selected_query) {
                                Some(facet_index) => {
                                    // Load previous selected indexes
                                    app.facet_values.state.with_selected_indexes(
                                        facet_index.selected_indexes.to_owned(),
                                    );

                                    if selected_query != &app.prev_query {
                                        app.facet_values.state.select(facet_index.selected);
                                    }
                                }
                                None => {
                                    // On first initialize
                                    app.facet_values.state.with_selected_indexes(Vec::from_iter(
                                        0..(cmp::min(SELECTED_FACET_LINES, facet_lines.len())),
                                    ));
                                    app.facet_values.state.select(Some(
                                        app.facet_values.state.selected_indexes().len(),
                                    ));
                                }
                            }
                            // Attach state key for saving indexes later on Enter
                            app.facet_values
                                .set_state_key(Some(selected_query.to_owned()));
                        }
                        app.facet_values.set_items(facet_lines.clone());

                        let facet_values: MultiList<'_> = MultiList::new(facet_items)
                            .block(
                                Block::default()
                                    .borders(Borders::ALL)
                                    .title("Facet values")
                                    .border_style(match app.facet_values.focused() {
                                        true => focused_style,
                                        _ => Style::default(),
                                    }),
                            )
                            .highlight_style(
                                Style::default()
                                    .fg(Color::Black)
                                    .bg(Color::LightYellow)
                                    .add_modifier(Modifier::BOLD),
                            );
                        frame.render_stateful_widget(
                            facet_values,
                            sidebar_layouts[1],
                            &mut app.facet_values.state,
                        );

                        // Load facets chart
                        if app.facet_values.focused() {
                            let selected_facets = app.facet_values.state.selected_indexes();
                            let mut chart_data: Vec<Vec<String>> = vec![vec!["Month".to_string()]];
                            let mut datasets = vec![];

                            for month in &chart.x_labels {
                                chart_data.push(vec![month.to_owned()]);
                            }

                            for (_, point) in chart
                                .datasets
                                .iter()
                                .enumerate()
                                .filter(|(index, _)| selected_facets.contains(index))
                            {
                                datasets.push(
                                    Dataset::default()
                                        // Disable chart legend as we already show color in facet values block,
                                        // current legend won't display if facet line too long.
                                        // .name(point.label.to_owned())
                                        .marker(symbols::Marker::Braille)
                                        .graph_type(GraphType::Line)
                                        .style(
                                            Style::default()
                                                .fg(facet_colors[&point.label.to_owned()]),
                                        )
                                        .data(&point.data),
                                );

                                // Build saved data
                                chart_data[0].push(point.label.to_owned());
                                for (i, point) in point.data.iter().enumerate() {
                                    chart_data[i + 1].push(point.1.to_string());
                                }
                            }

                            // Don't want to override if data is the same
                            if app.line_chart.data != chart_data {
                                app.line_chart.data = chart_data;
                            }

                            let facet_chart = Chart::new(datasets)
                                .block(
                                    Block::default()
                                        .borders(Borders::NONE)
                                        .padding(Padding::new(1, 0, 1, 0)),
                                )
                                .x_axis(
                                    Axis::default()
                                        .style(match app.line_chart.focused() {
                                            true => focused_style,
                                            false => Style::default().fg(Color::Gray),
                                        })
                                        .bounds([
                                            chart.x_bounds[0],
                                            chart.x_bounds[chart.x_bounds.len() - 1],
                                        ])
                                        .labels(
                                            chart.x_ticks.iter().cloned().map(Span::from).collect(),
                                        ),
                                )
                                .y_axis(
                                    Axis::default()
                                        .style(match app.line_chart.focused() {
                                            true => focused_style,
                                            false => Style::default().fg(Color::Gray),
                                        })
                                        .bounds([
                                            chart.y_bounds[0],
                                            chart.y_bounds[chart.y_bounds.len() - 1],
                                        ])
                                        .labels(
                                            chart.y_ticks.iter().cloned().map(Span::from).collect(),
                                        )
                                        .labels_alignment(Alignment::Center),
                                );

                            frame.render_widget(facet_chart, main_layouts[1]);
                        }
                    }
                    None => {}
                }
            }
            None => {
                // Error or no results query don't save to app.charts
                if !app.api_error.is_empty() {
                    frame.render_widget(error_block, layouts[1]);
                    frame.render_widget(error_widget, center_layout[1]);
                } else if app.no_results {
                    frame.render_widget(info_block, layouts[1]);
                    frame.render_widget(no_results_widget, center_layout[1]);
                }

                // Unselect all lines
                app.saved_queries.state.with_selected_indexes(vec![]);
                app.facet_values.state.with_selected_indexes(vec![]);
                app.saved_queries.state.select(None);
                app.facet_values.state.select(None);
            }
        }

        // Used to load different total chart
        app.prev_query = selected_query.to_owned();
    }
}
