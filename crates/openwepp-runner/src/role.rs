#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryRole {
    Watershed,
    Hillslope,
    Replay,
}

impl BinaryRole {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Watershed => "watershed",
            Self::Hillslope => "hillslope",
            Self::Replay => "replay",
        }
    }

    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "watershed" => Some(Self::Watershed),
            "hillslope" => Some(Self::Hillslope),
            "replay" => Some(Self::Replay),
            _ => None,
        }
    }
}
