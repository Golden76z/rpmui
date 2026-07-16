use ratatui::widgets::ListState;

use crate::pages::{generator::Generator, layout::MenuItems};

#[derive(Debug, Default)]
pub enum Focus {
    #[default]
    Menu,
    Main,
    Detail,
}

// /// Application.
#[derive(Debug, Default)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    /// counter
    pub counter: u8,

    // List states
    pub menu_state: ListState,
    pub main_state: ListState,
    pub detail_state: ListState,

    pub page_focus: MenuItems,
    pub show_detail_panel: bool,
    pub focus: Focus,

    // Page content structs
    pub generator: Generator,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            should_quit: false,
            counter: 0,
            menu_state: ListState::default().with_selected(Some(0)),
            main_state: ListState::default(),
            detail_state: ListState::default(),
            show_detail_panel: false,
            page_focus: MenuItems::MainMenu,
            focus: Focus::Menu,
            generator: Generator::new(),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
