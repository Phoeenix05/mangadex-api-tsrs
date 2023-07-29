use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Target demographic for manga.
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, PartialOrd, Serialize, TS)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[ts(export)]
pub enum Demographic {
    Shounen,
    Shoujo,
    Seinen,
    Josei,
    None,
}

impl From<String> for Demographic {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "shounen" => Self::Shounen,
            "shoujo" => Self::Shoujo,
            "josei" => Self::Josei,
            "seinen" => Self::Seinen,
            _ => Self::None,
        }
    }
}

impl std::fmt::Display for Demographic {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
            Self::Shounen => "shounen",
            Self::Shoujo => "shoujo",
            Self::Seinen => "seinen",
            Self::Josei => "josei",
            Self::None => "none",
        };
        fmt.write_str(name)
    }
}
