use crossterm::event::{self, KeyCode};

#[derive(PartialEq)]
pub enum Message {
    Increment,
    Decrement,
    Reset,
    Quit,
}

pub fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('j') => Some(Message::Increment),
        KeyCode::Char('k') => Some(Message::Decrement),
        KeyCode::Char('q') => Some(Message::Quit),
        _ => None,
    }
}

