use crossterm::event::Event;

use crate::app::AppState;

pub mod line_chart;
pub mod list;
pub mod user_input;

pub trait Component {
    fn handle_events(&mut self, event: Event, state: &mut AppState) {}

    fn focused(&self) -> bool;

    fn set_focus(&mut self, focus: bool) {}

    fn allow_enter(&self) -> bool {
        false
    }
}
