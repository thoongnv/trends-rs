use crate::{app::AppState, components::Component};

use crossterm::event::{Event, KeyCode};

#[derive(Debug)]
pub struct LineChart {
    focused: bool,
    hidden: bool,
}

impl LineChart {
    pub fn new() -> Self {
        Self {
            focused: false,
            hidden: false,
        }
    }
}

impl Component for LineChart {
    fn handle_events(&mut self, event: Event, state: &mut AppState) {
        match event {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Esc => {
                    // Unfocus the widget and allow to exist app
                    state.unfocused = true;
                    self.focused = false;
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn focused(&self) -> bool {
        self.focused
    }

    fn hidden(&self) -> bool {
        self.hidden
    }

    fn set_focus(&mut self, focused: bool) {
        self.focused = focused;
    }

    fn set_hide(&mut self, hidden: bool) {
        self.hidden = hidden;
    }
}
