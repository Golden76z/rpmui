use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, Paragraph},
};

use crate::{
    app::App,
    constants::{APP_TITLE_TEXT, FOOTER_TEXT, SIDEBAR_LEFT_TEXT, SIDEBAR_RIGHT_TEXT},
};

fn panel(title: &str) -> Block<'static> {
    Block::default()
        .title(title.to_string())
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan))
}

// Function called by tui.rs to render the layout
pub fn render(app: &mut App, frame: &mut Frame) {
    let area = frame.area();

    // Header layout declaration
    let [header, body, footer] = Layout::vertical([
        Constraint::Length(3), // header: 3 lines tall
        Constraint::Fill(1),   // body: grows to fill the middle
        Constraint::Length(3), // footer: 3 lines tall
    ])
    .areas(area);

    // Geometry only: decide which regions exist. The detail panel is optional,
    // so the right column comes back as `Option<Rect>` — `None` when hidden.
    let (sidebar_left, main_content, sidebar_right) = if app.show_detail_panel {
        let [l, m, r] = Layout::horizontal([
            Constraint::Length(30),
            Constraint::Fill(1),
            Constraint::Length(70),
        ])
        .areas(body);
        (l, m, Some(r))
    } else {
        let [l, m] =
            Layout::horizontal([Constraint::Length(30), Constraint::Fill(1)]).areas(body);
        (l, m, None)
    };
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
    let items = ["Main menu", "Generator", "Storage", "Settings"];
    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");
    // Sidebar left render
    frame.render_stateful_widget(
        list.block(panel(SIDEBAR_LEFT_TEXT)),
        sidebar_left,
        &mut app.list_state,
    );

    // Main content render
    frame.render_widget(Paragraph::new("").block(panel("")), main_content);

    // Detail panel render — only when the layout produced a right column.
    if let Some(right) = sidebar_right {
        frame.render_widget(
            Paragraph::new("").block(panel(SIDEBAR_RIGHT_TEXT)),
            right,
        );
    }
}
