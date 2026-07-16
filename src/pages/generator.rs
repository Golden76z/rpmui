use crate::{
    app::App,
    pages::types::{Element, Section},
};
use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

#[derive(Default, Debug)]
pub struct Generator {
    pub sections: Vec<Section>,
    pub cursor: usize,
    pub opened: Option<usize>,
}

impl Generator {
    pub fn new() -> Self {
        Generator {
            sections: vec![
                Section::new(
                    "Create".to_string(),
                    vec![
                        Element::Champ {
                            label: "longueur".to_string(),
                            valeur: "20".to_string(),
                        },
                        Element::Bascule {
                            label: "majuscules".to_string(),
                            actif: true,
                        },
                    ],
                ),
                Section::new(
                    "Diceware".to_string(),
                    vec![
                        Element::Champ {
                            label: "longueur".to_string(),
                            valeur: "20".to_string(),
                        },
                        Element::Bascule {
                            label: "majuscules".to_string(),
                            actif: true,
                        },
                    ],
                ),
                Section::new(
                    "Alphanumeric".to_string(),
                    vec![
                        Element::Champ {
                            label: "longueur".to_string(),
                            valeur: "20".to_string(),
                        },
                        Element::Bascule {
                            label: "majuscules".to_string(),
                            actif: true,
                        },
                    ],
                ),
            ],
            cursor: 0,
            opened: None,
        }
    }

    // Open/Close the section the cursor is on
    pub fn toggle(&mut self) {
        self.opened = if self.opened == Some(self.cursor) {
            None
        } else {
            Some(self.cursor)
        };
    }

    // Checking if a section is opened or not
    pub fn is_open(&self, i: usize) -> bool {
        self.opened == Some(i)
    }

    // Incrementing the cursor
    pub fn increment_cursor(&mut self) {
        if self.cursor + 1 < self.sections.len() {
            self.cursor += 1;
        }
    }

    // Decrementing the cursor
    pub fn decrement_cursor(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    // Handle the keys while this page has focus
    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => self.increment_cursor(),
            KeyCode::Char('k') | KeyCode::Up => self.decrement_cursor(),
            KeyCode::Enter => self.toggle(),
            _ => {}
        }
    }
}

pub fn render_generator_page(app: &mut App, frame: &mut Frame, main_content: Rect) {
    let g = &app.generator;

    let g = &app.generator;

    // 1. le grand cadre
    let outer = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Generator");

    // 2. l'espace À L'INTÉRIEUR du grand cadre
    let inner = outer.inner(main_content);

    // 3. on dessine le grand cadre
    frame.render_widget(outer, main_content);

    // one vertical slot per section — taller when open
    let constraints: Vec<Constraint> = g
        .sections
        .iter()
        .enumerate()
        .map(|(i, s)| {
            if g.is_open(i) {
                Constraint::Length(s.items.len() as u16 + 2) // items + top/bottom border
            } else {
                Constraint::Length(3) // just the header bar
            }
        })
        .collect();

    let slots = Layout::vertical(constraints).split(inner);

    for (i, section) in g.sections.iter().enumerate() {
        // the section's box; highlight it when the cursor is on it
        let mut block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(section.title.clone());
        if g.cursor == i {
            block = block.border_style(Style::default().fg(Color::Magenta));
        }

        let inner = block.inner(slots[i]);
        frame.render_widget(block, slots[i]);

        // show the items only when the section is open
        if g.is_open(i) {
            let lines: Vec<Line> = section
                .items
                .iter()
                .map(|el| {
                    let text = match el {
                        Element::Label(t) => t.clone(),
                        Element::Champ { label, valeur } => format!("{label}: {valeur}"),
                        Element::Zone { label, valeur } => format!("{label}: {valeur}"),
                        Element::Bascule { label, actif } => {
                            format!("{label}: {}", if *actif { "on" } else { "off" })
                        }
                    };
                    Line::from(text)
                })
                .collect();
            frame.render_widget(Paragraph::new(lines), inner);
        }
    }
}
