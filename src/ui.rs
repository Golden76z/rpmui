use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, List, Paragraph, Widget},
};

use crate::{
    app::App,
    constants::{APP_TITLE, FOOTER_TEXT},
};

/// A `cols` x `rows` grid of bordered cells.
///
/// A widget is just "some data" + "instructions for drawing that data".
/// So `Grid` carries everything it needs to render: its shape, plus the
/// `counter` value we want to display inside it.
pub struct Grid {
    cols: usize,
    rows: usize,
    counter: u8,
    detail: bool,
}

impl Grid {
    pub fn new(cols: usize, rows: usize, counter: u8) -> Self {
        Self {
            cols,
            rows,
            counter,
            detail: false,
        }
    }
}

impl Widget for Grid {
    /// This is the `Widget` trait's *fixed* signature — you don't get to
    /// change it. You consume `self`, and draw into `buf` within the
    /// rectangle `area`. Note: there is NO `Frame` here. Inside a widget you
    /// write cells straight into the buffer; `Frame` lives one level up.
    fn render(self, area: Rect, buf: &mut Buffer) {
        // 1. Split `area` vertically into `rows` equal-height bands.

        let span = Span::styled(
            "This is text that will be yellow",
            Style::default().fg(Color::Yellow),
        );
        span.render(area, buf);

        let row_areas =
            Layout::vertical((0..self.rows).map(|_| Constraint::Ratio(1, self.rows as u32)))
                .split(area);

        for (r, &row_area) in row_areas.iter().enumerate() {
            // 2. Split each band horizontally into `cols` equal-width cells.
            let cell_areas =
                Layout::horizontal((0..self.cols).map(|_| Constraint::Ratio(1, self.cols as u32)))
                    .split(row_area);

            for (c, &cell_area) in cell_areas.iter().enumerate() {
                // 3. Build a bordered block for this cell.
                let block = Block::default()
                    .title(format!(" {r},{c} "))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .style(Style::default().fg(Color::White));

                // The rectangle *inside* the borders, where content goes.
                // Grab it before we move `block` on the next line.
                let inner = block.inner(cell_area);

                // 4. Render children into the same buffer. `Block`, `Paragraph`,
                //    etc. all implement `Widget`, so they expose the very same
                //    `.render(area, buf)` method we're inside right now.
                block.render(cell_area, buf);

                // Show the counter in the top-left cell — this is how app state
                // reaches a widget: you hand it the data via the constructor.
                if r == 0 && c == 0 {
                    Paragraph::new(format!("Counter: {}", self.counter))
                        .alignment(Alignment::Center)
                        .render(inner, buf);
                }
            }
        }
    }
}

fn panel(title: &str) -> Block<'static> {
    Block::default()
        .title(title.to_string())
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan))
}

/// Free function called from `tui.rs` once per frame.
///
/// This builds an *irregular* dashboard by NESTING layouts: we split the
/// screen into independent regions whose sizes are unrelated to each other.
/// The uniform `Grid` widget then lives inside just one of those regions.
pub fn render(app: &mut App, frame: &mut Frame) {
    let area = frame.area();

    // 1. Split the whole screen into a fixed-width sidebar + a main area.
    //    `.areas()` destructures straight into an array — the pattern's length
    //    must match the number of constraints.
    let [sidebar, main] =
        Layout::horizontal([Constraint::Length(30), Constraint::Fill(1)]).areas(area);

    // 2. Split ONLY the main area into three stacked rows. These heights are
    //    independent of the sidebar — that's exactly what nesting buys you.
    let [header, body, footer] = Layout::vertical([
        Constraint::Length(3), // header: 3 lines tall
        Constraint::Fill(1),   // body: grows to fill the middle
        Constraint::Length(3), // footer: 3 lines tall
    ])
    .areas(main);

    // 3. Draw each region. The sidebar is one tall cell spanning the full
    //    height, while header/footer are short — cells of different sizes that
    //    a uniform `Grid` could never produce.
    let items = ["Item 1", "Item 2", "Item 3", "Item 4"];

    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    frame.render_stateful_widget(list.block(panel(" Menu ")), sidebar, &mut app.list_state);

    frame.render_widget(
        Paragraph::new(APP_TITLE)
            .alignment(Alignment::Center)
            .block(panel("")),
        header,
    );
    frame.render_widget(
        Paragraph::new(FOOTER_TEXT)
            .alignment(Alignment::Center)
            .block(panel("")),
        footer,
    );

    // 4. Reuse the *uniform* Grid inside the irregular layout: a regular 2x2
    //    table nested within the non-uniform dashboard.
    frame.render_widget(Grid::new(1, 1, app.counter), body);
}
