use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectMode {
    #[default]
    Gray,
    BitwiseNot,
    Rotate,
}

impl SelectMode {
    pub const ALL: [SelectMode; 3] = [SelectMode::BitwiseNot, SelectMode::Gray,SelectMode::Rotate];
}

impl Display for SelectMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SelectMode::Gray => "Gray",
                SelectMode::BitwiseNot => "Invert",
                SelectMode::Rotate => "Rotate",
            }
        )
    }
}
