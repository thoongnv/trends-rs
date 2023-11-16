use std::{fs::File, io::Write};

use crate::{
    app::{App, AppResult, AppState},
    components::Component,
};
use crossterm::event::{Event, KeyCode, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_events(event: Event, app: &mut App, state: &mut AppState) -> AppResult<()> {
    // Use to prevent loop MultiStatefulList.state.select(Some(index))
    state.submitted = false;

    // On unfocused any panels
    if !state.focused {
        // match event {
        //     Event::Key(key_event) => {
        //         match key_event.code {
        //             // Exit application on `ESC`
        //             KeyCode::Esc => {
        //                 app.quit();
        //             }
        //             // Unhandled key events
        //             _ => {}
        //         }
        //     }
        //     // Unhandled events
        //     _ => {}
        // }
    } else {
        // Let each widget handle events
        let widget_index = app.widget_index;
        let mut widgets = app.get_widgets();

        // Only focused & visible widget can handle events
        if widgets[widget_index].focused() && !widgets[widget_index].hidden() {
            widgets[widget_index].handle_events(event.clone(), state);

            // Unfocus current widget

            if let Event::Key(key_event) = event {
                if key_event.code == KeyCode::Esc {
                    state.focused = false;
                    widgets[widget_index].set_focus(false);
                }
            }

            // Special handler for focused searchbox
            if widgets[widget_index].allow_enter() {
                if let Event::Key(key_event) = event {
                    match key_event.code {
                        // Switch between searchbox lines
                        KeyCode::Up => {
                            if !app.search_input.focused() {
                                let index = app.get_widget_index(app.search_input.id());
                                app.select_widget(index);
                            }
                        }
                        KeyCode::Down => {
                            if !app.facets_input.focused() {
                                let index = app.get_widget_index(app.facets_input.id());
                                app.select_widget(index);
                            }
                        }
                        KeyCode::Enter => {
                            state.submitted = true;
                            app.search(state.sender.clone())?;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    if let Event::Key(key_event) = event {
        match key_event.code {
            KeyCode::Tab => {
                app.switch_widgets(state, false)?;
            }
            KeyCode::BackTab => {
                app.switch_widgets(state, true)?;
            }
            // Export selected chart data to CSV file
            KeyCode::Char('e') | KeyCode::Char('E') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    if app.line_chart.data.is_empty() || app.line_chart.data[0].len() == 1 {
                        state.app_log = "No chart data to export".to_string();
                    } else {
                        let mut has_error = false;
                        let outfile = "./data.csv";

                        match File::create(outfile) {
                            Ok(mut file) => {
                                state.app_log = format!("Exported chart to {}", outfile);
                                for row in &app.line_chart.data {
                                    let line = row.join(",") + "\n";
                                    if file.write_all(line.as_bytes()).is_err() {
                                        has_error = true;
                                    }
                                }
                            }
                            Err(_) => {
                                has_error = true;
                            }
                        };

                        if has_error {
                            state.app_log = "Failed to export chart data".to_string();
                        }
                    }

                    // Reset ticks
                    app.ticks = 0;
                }
            }
            // Exit application on `Ctrl-C`
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                }
            }
            _ => {}
        }
    }

    Ok(())
}
