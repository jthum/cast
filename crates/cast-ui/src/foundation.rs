#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Intent {
    #[default]
    Neutral,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Variant {
    #[default]
    Solid,
    Subtle,
    Outline,
    Ghost,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Size {
    Small,
    #[default]
    Medium,
    Large,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Placement {
    #[default]
    Top,
    Right,
    Bottom,
    Left,
}
