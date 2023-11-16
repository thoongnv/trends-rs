use crate::{
    app::{AppState, FacetIndex},
    components::{Component, KeySymbols},
    widgets::list::MultiListState,
};
use uuid::Uuid;

use crossterm::event::{Event, KeyCode};
// use ratatui::widgets::ListState;

// https://github.com/ratatui-org/ratatui/blob/v0.22.0/examples/list.rs
// #[derive(Debug)]
// pub struct StatefulList<T> {
//     pub state: ListState,
//     pub items: Vec<T>,
//     pub focused: bool,
// }

// impl<T> StatefulList<T> {
//     pub fn new() -> StatefulList<T> {
//         StatefulList {
//             state: ListState::default(),
//             items: vec![],
//             focused: false,
//         }
//     }

//     pub fn set_items(&mut self, items: Vec<T>) {
//         self.items = items;
//     }

//     fn next(&mut self) {
//         let i = match self.state.selected() {
//             Some(i) => {
//                 if i >= self.items.len() - 1 {
//                     0
//                 } else {
//                     i + 1
//                 }
//             }
//             None => 0,
//         };
//         self.state.select(Some(i));
//     }

//     fn previous(&mut self) {
//         let i = match self.state.selected() {
//             Some(i) => {
//                 if i == 0 {
//                     self.items.len() - 1
//                 } else {
//                     i - 1
//                 }
//             }
//             None => 0,
//         };
//         self.state.select(Some(i));
//     }

//     fn unselect(&mut self) {
//         self.state.select(None);
//     }
// }

// impl<T> Component for StatefulList<T> {
//     fn handle_events(&mut self, event: Event, state: &mut AppState) {
//         match event {
//             Event::Key(key_event) => match key_event.code {
//                 KeyCode::Left => self.unselect(),
//                 KeyCode::Down => self.next(),
//                 KeyCode::Up => self.previous(),
//                 KeyCode::Esc => {
//                     state.focused = false;
//                     self.focused = false;
//                 }
//                 _ => {}
//             },
//             _ => {}
//         }
//     }

//     fn focused(&self) -> bool {
//         self.focused
//     }

//     fn set_focus(&mut self, focus: bool) {
//         self.focused = focus;
//     }
// }

// Custom stateful list support multi select items
#[derive(Debug)]
pub struct MultiStatefulList<T> {
    id: Uuid,
    pub state: MultiListState,
    pub state_key: Option<String>,
    pub items: Vec<T>,
    pub focused: bool,
    pub hidden: bool,
}

impl<T> Default for MultiStatefulList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> MultiStatefulList<T> {
    pub fn new() -> MultiStatefulList<T> {
        MultiStatefulList {
            id: Uuid::new_v4(),
            state: MultiListState::default(),
            state_key: None, // Saved selected indexes with the key if exists
            items: vec![],
            focused: false,
            hidden: false,
        }
    }

    pub fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
    }

    pub fn set_state_key(&mut self, state_key: Option<String>) {
        self.state_key = state_key;
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
    fn id(&self) -> Uuid {
        self.id
    }

    fn handle_events(&mut self, event: Event, state: &mut AppState) {
        if !self.items.is_empty() {
            if let Event::Key(key_event) = event {
                match key_event.code {
                    KeyCode::Left => {
                        self.state.with_selected_indexes(vec![]);
                        self.unselect();
                    }
                    KeyCode::Right => {
                        self.state
                            .with_selected_indexes(Vec::from_iter(0..self.items.len()));

                        if self.state.selected().is_none() {
                            self.state.select(Some(self.items.len() - 1));
                        }
                    }
                    KeyCode::Down => self.next(),
                    KeyCode::Up => self.previous(),
                    KeyCode::Enter => self.toggle(),
                    _ => {}
                }
            }

            // Save selected indexes of each facet values
            if let Some(state_key) = &self.state_key {
                state.facet_indexes.insert(
                    state_key.to_owned(),
                    FacetIndex {
                        selected: self.state.selected(),
                        selected_indexes: self.state.selected_indexes().to_owned(),
                    },
                );
            }
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

    fn help_keys(&self) -> Vec<String> {
        vec![
            format!("Up/ Down [{}{}]", KeySymbols::UP, KeySymbols::DOWN),
            format!("Toggle [{}]", KeySymbols::ENTER),
            format!(
                "Select/ Unselect All [{}{}]",
                KeySymbols::RIGHT,
                KeySymbols::LEFT
            ),
        ]
    }
}
