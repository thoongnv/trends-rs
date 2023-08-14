use crossterm::event::Event;

use crate::app::AppState;

pub mod line_chart;
pub mod stateful_list;
pub mod user_input;

pub enum KeySymbols {
    ENTER,
    LEFT,
    RIGHT,
    UP,
    DOWN,
    BACKSPACE,
    HOME,
    END,
    PAGE_UP,
    PAGE_DOWN,
    TAB,
    BACK_TAB,
    DELETE,
    INSERT,
    ESC,
    CONTROL,
    SHIFT,
    ALT,
}

impl KeySymbols {
    pub fn to_string(&self) -> String {
        // https://en.wikipedia.org/wiki/List_of_Unicode_characters#Arrows
        match self {
            KeySymbols::ENTER => "\u{23ce}".into(),     //⏎
            KeySymbols::LEFT => "\u{2190}".into(),      //←
            KeySymbols::RIGHT => "\u{2192}".into(),     //→
            KeySymbols::UP => "\u{2191}".into(),        //↑
            KeySymbols::DOWN => "\u{2193}".into(),      //↓
            KeySymbols::BACKSPACE => "\u{232b}".into(), //⌫
            KeySymbols::HOME => "\u{2912}".into(),      //⤒
            KeySymbols::END => "\u{2913}".into(),       //⤓
            KeySymbols::PAGE_UP => "\u{21de}".into(),   //⇞
            KeySymbols::PAGE_DOWN => "\u{21df}".into(), //⇟
            KeySymbols::TAB => "\u{21e5}".into(),       //⇥
            KeySymbols::BACK_TAB => "\u{21e4}".into(),  //⇤
            KeySymbols::DELETE => "\u{2326}".into(),    //⌦
            KeySymbols::INSERT => "\u{2380}".into(),    //⎀
            KeySymbols::ESC => "\u{238b}".into(),       //⎋
            KeySymbols::CONTROL => "^".into(),
            KeySymbols::SHIFT => "\u{21e7}".into(), //⇧
            KeySymbols::ALT => "\u{2325}".into(),   //⌥
        }
    }
}

pub trait Component {
    fn handle_events(&mut self, event: Event, state: &mut AppState) {}

    fn focused(&self) -> bool;

    fn set_focus(&mut self, focus: bool) {}

    fn allow_enter(&self) -> bool {
        false
    }

    fn help_keys(&self) -> Vec<String> {
        vec![]
    }
}
