use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectMode {
    #[default]
    Gray,
    BitwiseNot,
    HueRotate,
    Blur,
    ToAscii,
}

impl SelectMode {
    pub const ALL: [SelectMode;5] = [SelectMode::BitwiseNot, SelectMode::Gray,SelectMode::HueRotate,SelectMode::Blur,SelectMode::ToAscii];
}

impl Display for SelectMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SelectMode::Gray => "Gray",
                SelectMode::BitwiseNot => "Invert",
                SelectMode::HueRotate => "HueRotate",
                SelectMode::Blur=> "Blur",
                SelectMode::ToAscii => "ToAscii",
            }
        )
    }
}
