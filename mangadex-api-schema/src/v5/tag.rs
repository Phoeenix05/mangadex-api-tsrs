use mangadex_api_types::TagGroup;
use serde::Deserialize;
use ts_rs::TS;

use crate::v5::{localizedstring_array_or_map, LocalizedString};

#[derive(Clone, Debug, Deserialize, TS)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[ts(export)]
pub struct TagAttributes {
    pub name: LocalizedString,
    #[serde(with = "localizedstring_array_or_map")]
    pub description: LocalizedString,
    pub group: TagGroup,
    pub version: u32,
}
