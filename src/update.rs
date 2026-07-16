use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    widgets::ListState,
};

use crate::{
    app::{App, Focus},
    pages::layout::MenuItems,
};

pub fn update(app: &mut App, key_event: KeyEvent) {
    // 1. Global keys — active whatever the focus is
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
            return;
        }
        KeyCode::Char('c') | KeyCode::Char('C')
            if key_event.modifiers == KeyModifiers::CONTROL =>
        {
            app.quit();
            return;
        }

        // Toggle the detail panel
        KeyCode::Char('d') => {
            app.show_detail_panel = !app.show_detail_panel;
            if let Focus::Detail = app.focus {
                app.focus = Focus::Main;
                app.detail_state = ListState::default();
            }
            return;
        }

        // Move panel focus to the left
        KeyCode::Char('h') | KeyCode::Left => {
            match app.focus {
                Focus::Menu => {}
                Focus::Main => {
                    app.focus = Focus::Menu;
                    app.main_state = ListState::default();
                }
                Focus::Detail => {
                    app.focus = Focus::Main;
                    app.detail_state = ListState::default();
                }
            }
            return;
        }

        // Move panel focus to the right
        KeyCode::Char('l') | KeyCode::Right => {
            match app.focus {
                Focus::Menu => {
                    app.focus = Focus::Main;
                    app.main_state = ListState::default().with_selected(Some(0));
                }
                Focus::Main => {
                    if app.show_detail_panel {
                        app.focus = Focus::Detail;
                        app.detail_state = ListState::default().with_selected(Some(0));
                    }
                }
                Focus::Detail => {}
            }
            return;
        }

        _ => {}
    }

    // 2. Everything else → forward to whatever has focus
    match app.focus {
        Focus::Menu => menu_keys(app, key_event),
        Focus::Main => match app.page_focus {
            MenuItems::Generator => app.generator.handle_key(key_event),
            _ => match key_event.code {
                KeyCode::Char('j') | KeyCode::Down => app.main_state.select_next(),
                KeyCode::Char('k') | KeyCode::Up => app.main_state.select_previous(),
                _ => {}
            },
        },
        Focus::Detail => match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => app.detail_state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => app.detail_state.select_previous(),
            _ => {}
        },
    }
}

// Navigating the left menu — the ONLY place page_focus changes.
fn menu_keys(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('j') | KeyCode::Down => {
            app.menu_state.select_next();
            app.page_focus = match app.page_focus {
                MenuItems::MainMenu => MenuItems::Generator,
                MenuItems::Generator => MenuItems::Storage,
                MenuItems::Storage => MenuItems::Settings,
                MenuItems::Settings => MenuItems::Settings,
            };
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.menu_state.select_previous();
            app.page_focus = match app.page_focus {
                MenuItems::Settings => MenuItems::Storage,
                MenuItems::Storage => MenuItems::Generator,
                MenuItems::Generator => MenuItems::MainMenu,
                MenuItems::MainMenu => MenuItems::MainMenu,
            };
        }
        _ => {}
    }
}
