use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, PartialOrd, Serialize, TS)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[ts(export)]
pub enum MangaStatus {
    /// Manga is still going on.
    Ongoing,
    /// Manga is completed.
    Completed,
    /// Manga is paused from publishing new chapters.
    Hiatus,
    /// Manga has been cancelled.
    Cancelled,
}

impl Default for MangaStatus {
    fn default() -> Self {
        Self::Ongoing
    }
}

impl std::fmt::Display for MangaStatus {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
            Self::Ongoing => "Ongoing",
            Self::Completed => "Completed",
            Self::Hiatus => "Hiatus",
            Self::Cancelled => "Cancelled",
        };
        fmt.write_str(name)
    }
}
