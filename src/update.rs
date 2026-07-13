use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') if key_event.modifiers == KeyModifiers::CONTROL => {
            app.quit()
        }
        // KeyCode::Right | KeyCode::Char('j') => app.increment_counter(),
        // KeyCode::Left | KeyCode::Char('k') => app.decrement_counter(),
        KeyCode::Char('j') | KeyCode::Down => app.list_state.select_next(),
        KeyCode::Char('k') | KeyCode::Up => app.list_state.select_previous(),
        _ => {}
    };
}
