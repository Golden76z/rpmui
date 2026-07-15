use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier},
    widgets::List,
};

use crate::{app::App, ui::panel};

pub fn render_main_block(app: &mut App, frame: &mut Frame, main_content: Rect) {
    // Main content items
    let main_items = ["a", "b", "c", "d"];
    let main_list = List::new(main_items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    // Main content render
    frame.render_stateful_widget(
        main_list.block(panel("")),
        main_content,
        &mut app.main_state,
    );
}
