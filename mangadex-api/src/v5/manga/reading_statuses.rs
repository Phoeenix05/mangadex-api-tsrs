//! Builder for the manga reading status endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/Manga/get-manga-status>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api_types::ReadingStatus;
//! use mangadex_api::v5::MangaDexClient;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let res = client
//!     .manga()
//!     .reading_statuses()
//!     .status(ReadingStatus::Reading)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("statuses: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;

use crate::HttpClientRef;
use mangadex_api_schema::v5::MangaReadingStatusesResponse;
use mangadex_api_types::ReadingStatus;

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), pattern = "owned", default)]
pub struct MangaReadingStatuses {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,

    pub status: Option<ReadingStatus>,
}

endpoint! {
    GET "/manga/status",
    #[query auth] MangaReadingStatuses,
    #[flatten_result] MangaReadingStatusesResponse
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn manga_reading_status_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let response_body = json!({
            "result": "ok",
            "statuses": {
                "b019ea5d-5fe6-44d4-abbc-f546f210884d": "reading",
                "2394a5c7-1d2e-461f-acde-18726b9e37d6": "dropped"
            }
        });

        Mock::given(method("GET"))
            .and(path(r"/manga/status"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client
            .manga()
            .reading_statuses()
            .build()?
            .send()
            .await?;

        Ok(())
    }
}
