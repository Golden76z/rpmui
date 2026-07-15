use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier},
    widgets::List,
};

use crate::{app::App, constants::SIDEBAR_RIGHT_TEXT, ui::panel};

pub fn render_details(app: &mut App, frame: &mut Frame, sidebar_right: Option<Rect>) {
    // Detail content items
    let detail_items = ["a", "b", "c", "d"];
    let detail_list = List::new(detail_items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    // Detail panel render — only when the layout produced a right column.
    if let Some(right) = sidebar_right {
        frame.render_stateful_widget(
            detail_list.block(panel(SIDEBAR_RIGHT_TEXT)),
            right,
            &mut app.detail_state,
        );
    }
}
