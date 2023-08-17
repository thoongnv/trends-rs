use std::fmt;
use uuid::Uuid;

use crate::app::AppState;
use crossterm::event::Event;

pub mod line_chart;
pub mod stateful_list;
pub mod user_input;

#[allow(non_camel_case_types)]
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

impl fmt::Display for KeySymbols {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // https://en.wikipedia.org/wiki/List_of_Unicode_characters#Arrows
        match self {
            KeySymbols::ENTER => write!(f, "\u{23ce}"),     //⏎
            KeySymbols::LEFT => write!(f, "\u{2190}"),      //←
            KeySymbols::RIGHT => write!(f, "\u{2192}"),     //→
            KeySymbols::UP => write!(f, "\u{2191}"),        //↑
            KeySymbols::DOWN => write!(f, "\u{2193}"),      //↓
            KeySymbols::BACKSPACE => write!(f, "\u{232b}"), //⌫
            KeySymbols::HOME => write!(f, "\u{2912}"),      //⤒
            KeySymbols::END => write!(f, "\u{2913}"),       //⤓
            KeySymbols::PAGE_UP => write!(f, "\u{21de}"),   //⇞
            KeySymbols::PAGE_DOWN => write!(f, "\u{21df}"), //⇟
            KeySymbols::TAB => write!(f, "\u{21e5}"),       //⇥
            KeySymbols::BACK_TAB => write!(f, "\u{21e4}"),  //⇤
            KeySymbols::DELETE => write!(f, "\u{2326}"),    //⌦
            KeySymbols::INSERT => write!(f, "\u{2380}"),    //⎀
            KeySymbols::ESC => write!(f, "\u{238b}"),       //⎋
            KeySymbols::CONTROL => write!(f, "^"),
            KeySymbols::SHIFT => write!(f, "\u{21e7}"), //⇧
            KeySymbols::ALT => write!(f, "\u{2325}"),   //⌥
        }
    }
}

#[allow(unused)]
pub trait Component {
    // Unique Id use to compare struct
    fn id(&self) -> Uuid;

    fn handle_events(&mut self, event: Event, state: &mut AppState) {}

    fn focused(&self) -> bool;

    fn hidden(&self) -> bool;

    fn set_focus(&mut self, focus: bool) {}

    fn set_hide(&mut self, hide: bool) {}

    fn allow_enter(&self) -> bool {
        false
    }

    fn help_keys(&self) -> Vec<String> {
        vec![]
    }
}
