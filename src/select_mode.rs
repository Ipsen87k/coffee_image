use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectMode {
    #[default]
    Gray,
    BitwiseNot,
}

impl SelectMode {
    pub const ALL: [SelectMode; 2] = [SelectMode::BitwiseNot, SelectMode::Gray,];
}

impl Display for SelectMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SelectMode::Gray => "Gray",
                SelectMode::BitwiseNot => "Invert",
            }
        )
    }
}
