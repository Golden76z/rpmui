use ratatui::widgets::ListState;

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

    pub show_detail_panel: bool,

    pub focus: Focus,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            should_quit: false,
            counter: 0,
            menu_state: ListState::default().with_selected(Some(0)),
            main_state: ListState::default().with_selected(Some(0)),
            detail_state: ListState::default().with_selected(Some(0)),
            show_detail_panel: false,
            focus: Focus::Menu,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
