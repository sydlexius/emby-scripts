use super::app::*;
use super::keymap::*;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

#[test]
fn sidebar_down_navigates() {
    let action = map_key(
        Mode::Normal,
        Pane::Sidebar,
        Section::Servers,
        key(KeyCode::Down),
    );
    assert_eq!(action, Some(Action::NextSection));
}

#[test]
fn sidebar_j_navigates() {
    let action = map_key(
        Mode::Normal,
        Pane::Sidebar,
        Section::Servers,
        key(KeyCode::Char('j')),
    );
    assert_eq!(action, Some(Action::NextSection));
}

#[test]
fn sidebar_tab_focuses_content() {
    let action = map_key(
        Mode::Normal,
        Pane::Sidebar,
        Section::Servers,
        key(KeyCode::Tab),
    );
    assert_eq!(action, Some(Action::TogglePane));
}

#[test]
fn sidebar_enter_focuses_content() {
    let action = map_key(
        Mode::Normal,
        Pane::Sidebar,
        Section::Servers,
        key(KeyCode::Enter),
    );
    assert_eq!(action, Some(Action::TogglePane));
}

#[test]
fn content_tab_focuses_sidebar() {
    let action = map_key(
        Mode::Normal,
        Pane::Content,
        Section::Servers,
        key(KeyCode::Tab),
    );
    assert_eq!(action, Some(Action::TogglePane));
}

#[test]
fn content_esc_focuses_sidebar() {
    let action = map_key(
        Mode::Normal,
        Pane::Content,
        Section::Servers,
        key(KeyCode::Esc),
    );
    assert_eq!(action, Some(Action::TogglePane));
}

#[test]
fn content_s_saves() {
    let action = map_key(
        Mode::Normal,
        Pane::Content,
        Section::Servers,
        key(KeyCode::Char('s')),
    );
    assert_eq!(action, Some(Action::Save));
}

#[test]
fn content_q_quits() {
    let action = map_key(
        Mode::Normal,
        Pane::Content,
        Section::Servers,
        key(KeyCode::Char('q')),
    );
    assert_eq!(action, Some(Action::Quit));
}

#[test]
fn content_enter_edits() {
    let action = map_key(
        Mode::Normal,
        Pane::Content,
        Section::Servers,
        key(KeyCode::Enter),
    );
    assert_eq!(action, Some(Action::Edit));
}

#[test]
fn content_a_adds_server() {
    let action = map_key(
        Mode::Normal,
        Pane::Content,
        Section::Servers,
        key(KeyCode::Char('a')),
    );
    assert_eq!(action, Some(Action::Add));
}

#[test]
fn content_d_deletes_server() {
    let action = map_key(
        Mode::Normal,
        Pane::Content,
        Section::Servers,
        key(KeyCode::Char('d')),
    );
    assert_eq!(action, Some(Action::Delete));
}

#[test]
fn fullscreen_space_toggles() {
    let action = map_key(
        Mode::FullScreen,
        Pane::Content,
        Section::Genres,
        key(KeyCode::Char(' ')),
    );
    assert_eq!(action, Some(Action::Toggle));
}

#[test]
fn fullscreen_esc_cancels() {
    let action = map_key(
        Mode::FullScreen,
        Pane::Content,
        Section::Genres,
        key(KeyCode::Esc),
    );
    assert_eq!(action, Some(Action::Cancel));
}

#[test]
fn fullscreen_slash_filters() {
    let action = map_key(
        Mode::FullScreen,
        Pane::Content,
        Section::Genres,
        key(KeyCode::Char('/')),
    );
    assert_eq!(action, Some(Action::StartFilter));
}

#[test]
fn editing_enter_confirms() {
    let action = map_key(
        Mode::Editing,
        Pane::Content,
        Section::Servers,
        key(KeyCode::Enter),
    );
    assert_eq!(action, Some(Action::Confirm));
}

#[test]
fn editing_esc_cancels() {
    let action = map_key(
        Mode::Editing,
        Pane::Content,
        Section::Servers,
        key(KeyCode::Esc),
    );
    assert_eq!(action, Some(Action::Cancel));
}
