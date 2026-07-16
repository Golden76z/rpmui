#[derive(Debug)]
pub enum Element {
    Label(String),
    Champ { label: String, valeur: String },
    Zone { label: String, valeur: String },
    // Bouton { label: String, action: Action },
    Bascule { label: String, actif: bool },
}

#[derive(Default, Debug)]
pub struct Section {
    pub title: String,
    pub items: Vec<Element>,
}

impl Section {
    pub fn new(title: String, items: Vec<Element>) -> Self {
        Section { title, items }
    }
}
