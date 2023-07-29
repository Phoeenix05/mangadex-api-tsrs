use mangadex_api_types::UserRole;
use serde::Deserialize;
use ts_rs::TS;

/// General user information.
#[derive(Clone, Debug, Deserialize, TS)]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[ts(export)]
pub struct UserAttributes {
    pub username: String,
    pub roles: Vec<UserRole>,
    pub version: u32,
}
