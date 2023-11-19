use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectMode {
    #[default]
    Gray,
    BitwiseNot,
    HueRotate,
    Blur,
    ToAscii,
    Rotate,
}

impl SelectMode {
    pub const ALL: [SelectMode;6] = [SelectMode::BitwiseNot, SelectMode::Gray,SelectMode::HueRotate,SelectMode::Blur,SelectMode::ToAscii,SelectMode::Rotate];
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
                SelectMode::Rotate => "Rotate",
            }
        )
    }
}
