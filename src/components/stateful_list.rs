use crate::{app::AppState, components::Component, widgets::list::MultiListState};

use crossterm::event::{Event, KeyCode};
use ratatui::widgets::ListState;

// https://github.com/ratatui-org/ratatui/blob/v0.22.0/examples/list.rs
#[derive(Debug)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
    pub focused: bool,
}

impl<T> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: vec![],
            focused: false,
        }
    }

    pub fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

impl<T> Component for StatefulList<T> {
    fn handle_events(&mut self, event: Event, state: &mut AppState) {
        match event {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Left => self.unselect(),
                KeyCode::Down => self.next(),
                KeyCode::Up => self.previous(),
                KeyCode::Esc => {
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

    fn set_focus(&mut self, focus: bool) {
        self.focused = focus;
    }
}

// Custom stateful list support multi select items
#[derive(Debug)]
pub struct MultiStatefulList<T> {
    pub state: MultiListState,
    pub items: Vec<T>,
    pub focused: bool,
}

impl<T> MultiStatefulList<T> {
    pub fn new() -> MultiStatefulList<T> {
        MultiStatefulList {
            state: MultiListState::default(),
            items: vec![],
            focused: false,
        }
    }

    pub fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }

    fn toggle(&mut self) {
        self.state.toggle();
    }
}

impl<T> Component for MultiStatefulList<T> {
    fn handle_events(&mut self, event: Event, state: &mut AppState) {
        match event {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Left => self.unselect(),
                KeyCode::Down => self.next(),
                KeyCode::Up => self.previous(),
                KeyCode::Enter => self.toggle(),
                KeyCode::Esc => {
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

    fn set_focus(&mut self, focus: bool) {
        self.focused = focus;
    }
}
