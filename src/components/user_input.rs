use crate::{
    app::AppState,
    components::{Component, KeySymbols},
};
use uuid::Uuid;

use crossterm::event::{Event, KeyCode};

#[allow(unused)]
// https://github.com/ratatui-org/ratatui/blob/v0.22.0/examples/user_input.rs
#[derive(Debug)]
pub struct UserInput {
    id: Uuid,
    /// Current value of the input box
    input: String,
    /// Position of cursor in the editor area.
    pub cursor_position: usize,
    /// History of recorded messages
    messages: Vec<String>,
    /// If panel on focused by `Tab`
    focused: bool,
    /// The widget is not rendered in the UI
    hidden: bool,
}

#[allow(unused)]
impl UserInput {
    pub fn new(input: String) -> Self {
        let cursor_position = input.len();
        Self {
            id: Uuid::new_v4(),
            input,
            cursor_position,
            messages: Vec::new(),
            focused: false,
            hidden: false,
        }
    }

    pub fn get_input(&self) -> &str {
        self.input.as_str()
    }

    pub fn set_input(&mut self, input: &str) {
        self.input = input.to_owned();
        self.cursor_position = self.input.len();
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_position, new_char);

        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    fn submit_message(&mut self) {
        // TODO By saving messages, we could load next/ previous messages with Arrow keys
        self.messages.push(self.input.clone());
        self.input.clear();
        self.reset_cursor();
    }
}

#[allow(unused)]
impl Component for UserInput {
    fn id(&self) -> Uuid {
        self.id
    }

    fn handle_events(&mut self, event: Event, state: &mut AppState) {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Char(to_insert) => {
                    self.enter_char(to_insert);
                }
                KeyCode::Backspace => {
                    self.delete_char();
                }
                KeyCode::Left => {
                    self.move_cursor_left();
                }
                KeyCode::Right => {
                    self.move_cursor_right();
                }
                // TODO Handle more key events: Tab to insert space, Ctrl+W to delete previous word?
                _ => {}
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

    fn allow_enter(&self) -> bool {
        true
    }

    fn help_keys(&self) -> Vec<String> {
        // TODO Find a better way to map key events in fn handle_events to this?
        vec![
            format!("Search [{}]", KeySymbols::ENTER),
            format!("Move cursor [{}{}]", KeySymbols::LEFT, KeySymbols::RIGHT),
            format!("Delete Char [{}]", KeySymbols::BACKSPACE),
            format!("Up/ Down [{}{}]", KeySymbols::UP, KeySymbols::DOWN),
        ]
    }
}
