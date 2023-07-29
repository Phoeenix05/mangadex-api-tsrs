use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, PartialOrd, Serialize, TS)]
#[serde(from = "String")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[ts(export)]
pub enum CustomListVisibility {
    Public,
    Private,
}

impl From<String> for CustomListVisibility {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "public" => Self::Public,
            "private" => Self::Private,
            _ => Self::Public,
        }
    }
}

impl std::fmt::Display for CustomListVisibility {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            Self::Public => "public",
            Self::Private => "private",
        })
    }
}
