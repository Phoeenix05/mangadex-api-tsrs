use serde::Deserialize;
use ts_rs::TS;
use url::Url;
use uuid::Uuid;

use crate::deserialize_null_default;
use mangadex_api_types::{Language, MangaDexDateTime};

/// General chapter information.
/// More details at https://api.mangadex.org/docs/swagger.html#model-ChapterAttributes
#[derive(Clone, Debug, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[ts(export)]
pub struct ChapterAttributes {
    // TODO: Known issue: API doesn't always return an empty string despite the docs saying it's not nullable.
    #[serde(deserialize_with = "deserialize_null_default")]
    pub title: String,
    /// Volume number in the manga.
    pub volume: Option<String>,
    /// Chapter number in the manga.
    pub chapter: Option<String>,
    /// Count of readable images for this chapter.
    pub pages: u32,
    /// Language the text is in.
    pub translated_language: Language,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(type = "string")]
    pub uploader: Option<Uuid>,
    /// Denotes a chapter that links to an external source.
    #[ts(type = "string")]
    pub external_url: Option<Url>,
    pub version: u32,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    pub created_at: MangaDexDateTime,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    pub updated_at: Option<MangaDexDateTime>,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    #[cfg_attr(feature = "specta", specta(type = String))]
    pub publish_at: MangaDexDateTime,
    /// Datetime in `YYYY-MM-DDTHH:MM:SS+HH:MM` format.
    #[cfg_attr(feature = "specta", specta(type = String))]
    pub readable_at: MangaDexDateTime,
}
