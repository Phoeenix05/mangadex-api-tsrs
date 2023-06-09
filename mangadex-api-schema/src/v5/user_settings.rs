use std::collections::HashMap;

use mangadex_api_types::MangaDexDateTime;
use serde::{Deserialize};
use uuid::Uuid;

/// User Settings response.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
#[allow(unused)]
pub struct UserSettingsAttributes {
    pub updated_at: MangaDexDateTime,
    #[serde(skip)]
    settings: HashMap<String, String>,
    template: Uuid,
}
