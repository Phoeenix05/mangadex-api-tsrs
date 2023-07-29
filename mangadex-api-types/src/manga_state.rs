use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Manga state for approval.
///
/// The purpose of these are to discourage troll entries by requiring staff approval.
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[ts(export)]
pub enum MangaState {
    Draft,
    Published,
    Rejected,
    Submitted,
}
