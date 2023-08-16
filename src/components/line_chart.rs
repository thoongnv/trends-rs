use crate::{app::AppState, components::Component};
use uuid::Uuid;

use crossterm::event::{Event, KeyCode};

#[derive(Debug, PartialEq)]
pub struct LineChart {
    id: Uuid,
    focused: bool,
    hidden: bool,
}

impl LineChart {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            focused: false,
            hidden: false,
        }
    }
}

impl Component for LineChart {
    fn id(&self) -> Uuid {
        self.id
    }

    fn handle_events(&mut self, event: Event, state: &mut AppState) {}

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
