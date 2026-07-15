use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
};

use crate::{
    app::App,
    pages::{details::render_details, layout::render_layout, main_block::render_main_block},
};

pub fn panel(title: &str) -> Block<'static> {
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
        let [l, m] = Layout::horizontal([Constraint::Length(30), Constraint::Fill(1)]).areas(body);
        (l, m, None)
    };

    // Rendering the layout (Header - Sidebar Left)
    render_layout(app, frame, header, footer, sidebar_left);

    // Rendering the middle main block
    render_main_block(app, frame, main_content);

    // Rendering the details panel
    render_details(app, frame, sidebar_right);
}
