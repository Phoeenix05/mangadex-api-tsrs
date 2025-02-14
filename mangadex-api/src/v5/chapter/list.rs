//! Builder for the chapter list endpoint.
//!
//! <https://api.mangadex.org/docs/redoc.html#tag/Chapter>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api::v5::MangaDexClient;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let chapter_res = client
//!     .chapter()
//!     .list()
//!     .title("summoning")
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("chapters: {:?}", chapter_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::ChapterListResponse;
use mangadex_api_types::{
    ChapterSortOrder, ContentRating, IncludeExternalUrl, IncludeFuturePages,
    IncludeFuturePublishAt, IncludeFutureUpdates, Language, MangaDexDateTime,
    ReferenceExpansionResource,
};

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), default, pattern = "owned")]
#[non_exhaustive]
pub struct ListChapter {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub(crate) http_client: HttpClientRef,

    pub limit: Option<u32>,
    pub offset: Option<u32>,
    #[serde(rename = "ids")]
    #[builder(setter(each = "add_chapter_id"))]
    pub chapter_ids: Vec<Uuid>,
    pub title: Option<String>,
    #[builder(setter(each = "add_group"))]
    pub groups: Vec<Uuid>,
    #[serde(rename = "uploader")]
    #[builder(setter(each = "uploader"))]
    pub uploaders: Vec<Uuid>,
    #[serde(rename = "manga")]
    pub manga_id: Option<Uuid>,
    #[serde(rename = "volume")]
    #[builder(setter(each = "add_volume"))]
    pub volumes: Vec<String>,
    /// Chapter number in the series or volume.
    #[builder(setter(each = "add_chapter"))]
    #[serde(rename = "chapter")]
    pub chapters: Vec<String>,
    #[serde(rename = "translatedLanguage")]
    #[builder(setter(each = "add_translated_language"))]
    pub translated_languages: Vec<Language>,
    #[serde(rename = "originalLanguage")]
    #[builder(setter(each = "add_original_language"))]
    pub original_languages: Vec<Language>,
    #[serde(rename = "excludedOriginalLanguage")]
    #[builder(setter(each = "exclude_original_language"))]
    pub excluded_original_languages: Vec<Language>,
    #[builder(setter(each = "add_content_rating"))]
    pub content_rating: Vec<ContentRating>,
    /// Groups to exclude from the results.
    #[builder(setter(each = "excluded_group"))]
    pub excluded_groups: Vec<Uuid>,
    /// Uploaders to exclude from the results.
    #[builder(setter(each = "excluded_uploader"))]
    pub excluded_uploaders: Vec<Uuid>,
    /// Flag to include future chapter updates in the results.
    ///
    /// Default: `IncludeFutureUpdates::Include` (1)
    pub include_future_updates: Option<IncludeFutureUpdates>,
    /// DateTime string with following format: `YYYY-MM-DDTHH:MM:SS`.
    pub created_at_since: Option<MangaDexDateTime>,
    /// DateTime string with following format: `YYYY-MM-DDTHH:MM:SS`.
    pub updated_at_since: Option<MangaDexDateTime>,
    /// DateTime string with following format: `YYYY-MM-DDTHH:MM:SS`.
    pub publish_at_since: Option<MangaDexDateTime>,
    /// Include empty pages
    pub include_empty_pages: Option<IncludeFuturePages>,
    /// Include external url chapters
    pub include_external_url: Option<IncludeExternalUrl>,
    /// Include future publish at
    pub include_future_publish_at: Option<IncludeFuturePublishAt>,
    pub order: Option<ChapterSortOrder>,
    #[builder(setter(each = "include"))]
    pub includes: Vec<ReferenceExpansionResource>,
}

endpoint! {
    GET "/chapter",
    #[query] ListChapter,
    #[flatten_result] ChapterListResponse
}

#[cfg(test)]
mod tests {
    use fake::faker::name::en::Name;
    use fake::Fake;
    use serde_json::json;
    use time::OffsetDateTime;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};
    use mangadex_api_types::error::Error;
    use mangadex_api_types::{Language, MangaDexDateTime, ResponseType};

    #[tokio::test]
    async fn list_chapter_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let chapter_id = Uuid::new_v4();
        let uploader_id = Uuid::new_v4();
        let chapter_title: String = Name().fake();

        let datetime = MangaDexDateTime::new(&OffsetDateTime::now_utc());

        let response_body = json!({
            "result": "ok",
            "response": "collection",
            "data": [
                {
                    "id": chapter_id,
                    "type": "chapter",
                    "attributes": {
                        "title": chapter_title,
                        "volume": "1",
                        "chapter": "1.5",
                        "pages": 4,
                        "translatedLanguage": "en",
                        "uploader": uploader_id,
                        "version": 1,
                        "createdAt": datetime.to_string(),
                        "updatedAt": datetime.to_string(),
                        "publishAt": datetime.to_string(),
                        "readableAt": datetime.to_string(),
                    },
                    "relationships": []
                }
            ],
            "limit": 1,
            "offset": 0,
            "total": 1
        });

        Mock::given(method("GET"))
            .and(path("/chapter"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .chapter()
            .search()
            .limit(1u32)
            .build()?
            .send()
            .await?;

        assert_eq!(res.response, ResponseType::Collection);
        let chapter = &res.data[0];
        assert_eq!(chapter.id, chapter_id);
        assert_eq!(chapter.attributes.title, chapter_title);
        assert_eq!(chapter.attributes.volume, Some("1".to_string()));
        assert_eq!(chapter.attributes.chapter, Some("1.5".to_string()));
        assert_eq!(chapter.attributes.pages, 4);
        assert_eq!(chapter.attributes.translated_language, Language::English);
        assert_eq!(chapter.attributes.version, 1);
        assert_eq!(
            chapter.attributes.created_at.to_string(),
            datetime.to_string()
        );
        assert_eq!(
            chapter.attributes.updated_at.as_ref().unwrap().to_string(),
            datetime.to_string()
        );
        assert_eq!(
            chapter.attributes.publish_at.to_string(),
            datetime.to_string()
        );

        Ok(())
    }

    #[tokio::test]
    async fn list_chapter_handles_400() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client: HttpClient = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let error_id = Uuid::new_v4();

        let response_body = json!({
            "result": "error",
            "errors": [{
                "id": error_id.to_string(),
                "status": 400,
                "title": "Invalid limit",
                "detail": "Limit must be between 1 and 100"
            }]
        });

        Mock::given(method("GET"))
            .and(path("/chapter"))
            .respond_with(ResponseTemplate::new(400).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .chapter()
            .search()
            .limit(0u32)
            .build()?
            .send()
            .await
            .expect_err("expected error");

        if let Error::Api(errors) = res {
            assert_eq!(errors.errors.len(), 1);

            assert_eq!(errors.errors[0].id, error_id);
            assert_eq!(errors.errors[0].status, 400);
            assert_eq!(errors.errors[0].title, Some("Invalid limit".to_string()));
            assert_eq!(
                errors.errors[0].detail,
                Some("Limit must be between 1 and 100".to_string())
            );
        }

        Ok(())
    }
}
