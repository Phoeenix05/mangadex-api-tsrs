//! General CustomList information.

use mangadex_api_types::CustomListVisibility;
use serde::Deserialize;
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[ts(export)]
pub struct CustomListAttributes {
    pub name: String,
    pub visibility: CustomListVisibility,
    pub version: u32,
}
