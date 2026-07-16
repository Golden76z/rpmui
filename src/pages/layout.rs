use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier},
    widgets::{List, Paragraph},
};

use crate::{
    app::App,
    constants::{APP_TITLE_TEXT, FOOTER_TEXT, SIDEBAR_LEFT_TEXT},
    ui::panel,
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItems {
    #[default]
    MainMenu,
    Generator,
    Storage,
    Settings,
}

impl MenuItems {
    pub const ALL: [MenuItems; 4] = [
        MenuItems::MainMenu,
        MenuItems::Generator,
        MenuItems::Storage,
        MenuItems::Settings,
    ];

    pub fn title(self) -> &'static str {
        match self {
            // <- exhaustif : le compilo t'oblige
            MenuItems::MainMenu => "Main menu",
            MenuItems::Generator => "Generator",
            MenuItems::Storage => "Storage",
            MenuItems::Settings => "Settings",
        }
    }
}

pub fn render_layout(
    app: &mut App,
    frame: &mut Frame,
    header: Rect,
    footer: Rect,
    sidebar_left: Rect,
) {
    // Header render
    frame.render_widget(
        Paragraph::new(APP_TITLE_TEXT)
            .alignment(Alignment::Center)
            .block(panel("")),
        header,
    );

    // Footer render
    frame.render_widget(
        Paragraph::new(FOOTER_TEXT)
            .alignment(Alignment::Center)
            .block(panel("")),
        footer,
    );

    // Sidebar left items
    let menu_items = MenuItems::ALL.map(|p| p.title());
    let menu_list = List::new(menu_items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");
    // Sidebar left render
    frame.render_stateful_widget(
        menu_list.block(panel(SIDEBAR_LEFT_TEXT)),
        sidebar_left,
        &mut app.menu_state,
    );
}
