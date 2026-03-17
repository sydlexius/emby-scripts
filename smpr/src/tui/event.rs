#![allow(dead_code)]

use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use std::time::Duration;

pub enum Event {
    Key(KeyEvent),
    Resize(u16, u16),
    Tick,
}

pub fn poll_event(timeout: Duration) -> std::io::Result<Event> {
    if event::poll(timeout)? {
        match event::read()? {
            CrosstermEvent::Key(key) => Ok(Event::Key(key)),
            CrosstermEvent::Resize(w, h) => Ok(Event::Resize(w, h)),
            _ => Ok(Event::Tick),
        }
    } else {
        Ok(Event::Tick)
    }
}
