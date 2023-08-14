use crate::app::{App, AppResult, AppState};
use crossterm::event::{Event, KeyCode, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_events(event: Event, app: &mut App, state: &mut AppState) -> AppResult<()> {
    // Use to prevent loop MultiStatefulList.state.select(Some(index))
    state.submitted = false;

    // On unfocused any panels
    if state.unfocused {
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

            // Currently, only handle Enter key event if focused searchbox
            if widgets[widget_index].allow_enter() {
                match event {
                    Event::Key(key_event) => match key_event.code {
                        KeyCode::Enter => {
                            state.submitted = true;
                            app.search(state.sender.clone())?;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }

    match event {
        Event::Key(key_event) => match key_event.code {
            KeyCode::Tab => {
                app.switch_widgets(state, false)?;
            }
            KeyCode::BackTab => {
                app.switch_widgets(state, true)?;
            }
            // Exit application on `Ctrl-C`
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                }
            }
            _ => {}
        },
        _ => {}
    }

    Ok(())
}
