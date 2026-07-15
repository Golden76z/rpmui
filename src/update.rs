use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, Focus};

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') if key_event.modifiers == KeyModifiers::CONTROL => {
            app.quit()
        }

        // Detail right panel
        KeyCode::Char('d') => {
            app.show_detail_panel = !app.show_detail_panel;
            match app.focus {
                Focus::Detail => app.focus = Focus::Main,
                _ => {}
            }
        }

        // Changing tab focus
        KeyCode::Char('h') | KeyCode::Left => match app.focus {
            Focus::Menu => {}
            Focus::Main => app.focus = Focus::Menu,
            Focus::Detail => app.focus = Focus::Main,
        },
        KeyCode::Char('l') | KeyCode::Right => match app.focus {
            Focus::Menu => app.focus = Focus::Main,
            Focus::Main => {
                if app.show_detail_panel {
                    app.focus = Focus::Detail
                }
            }
            Focus::Detail => {}
        },

        // Up & Down keys depending on the current window focus
        _ => match app.focus {
            Focus::Menu => match key_event.code {
                KeyCode::Char('j') | KeyCode::Down => app.menu_state.select_next(),
                KeyCode::Char('k') | KeyCode::Up => app.menu_state.select_previous(),
                _ => {}
            },
            Focus::Main => match key_event.code {
                KeyCode::Char('j') | KeyCode::Down => app.main_state.select_next(),
                KeyCode::Char('k') | KeyCode::Up => app.main_state.select_previous(),
                _ => {}
            },
            Focus::Detail => match key_event.code {
                KeyCode::Char('j') | KeyCode::Down => app.detail_state.select_next(),
                KeyCode::Char('k') | KeyCode::Up => app.detail_state.select_previous(),
                _ => {}
            },
        },
    };
}
