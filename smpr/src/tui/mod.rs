// Configure wizard TUI (ratatui)

pub mod app;
pub mod event;
pub mod io;
pub mod keymap;
pub mod widgets;

#[cfg(test)]
mod app_tests;
#[cfg(test)]
mod io_tests;
#[cfg(test)]
mod keymap_tests;
