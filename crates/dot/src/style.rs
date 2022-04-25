#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Style {
    None,
    Invisible,
    Solid,
    Dashed,
    Dotted,
    Bold,
    Rounded,
    Diagonals,
    Filled,
    Striped,
    Wedged,
}

impl Style {
    pub fn as_slice(self) -> &'static str {
        match self {
            Style::None => "",
            Style::Invisible => "invis",
            Style::Solid => "solid",
            Style::Dashed => "dashed",
            Style::Dotted => "dotted",
            Style::Bold => "bold",
            Style::Rounded => "rounded",
            Style::Diagonals => "diagonals",
            Style::Filled => "filled",
            Style::Striped => "striped",
            Style::Wedged => "wedged",
        }
    }
}