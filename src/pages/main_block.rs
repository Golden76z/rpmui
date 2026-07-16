use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier},
    widgets::List,
};

use crate::{
    app::App,
    pages::{generator::render_generator_page, layout::MenuItems},
    ui::panel,
};

pub fn render_main_block(app: &mut App, frame: &mut Frame, main_content: Rect) {
    match app.page_focus {
        MenuItems::MainMenu => render_main_menu(app, frame, main_content),
        MenuItems::Generator => render_generator_page(app, frame, main_content),
        MenuItems::Storage => render_storage_page(app, frame, main_content),
        MenuItems::Settings => render_settings_page(app, frame, main_content),
    }
}

fn render_main_menu(app: &mut App, frame: &mut Frame, main_content: Rect) {
    // Main content items
    let main_items = [" a", " b", " c", " d"];
    let main_list = List::new(main_items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol(">");

    // Main content render
    frame.render_stateful_widget(
        main_list.block(panel("")),
        main_content,
        &mut app.main_state,
    );
}

fn render_storage_page(app: &mut App, frame: &mut Frame, main_content: Rect) {}

fn render_settings_page(app: &mut App, frame: &mut Frame, main_content: Rect) {}
