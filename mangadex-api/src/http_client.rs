#[cfg(not(feature = "multi-thread"))]
use std::cell::RefCell;
#[cfg(not(feature = "multi-thread"))]
use std::rc::Rc;
#[cfg(feature = "multi-thread")]
use std::sync::Arc;

use derive_builder::Builder;
#[cfg(feature = "multi-thread")]
use futures::lock::Mutex;
use mangadex_api_schema::{Endpoint, FromResponse, UrlSerdeQS};
use mangadex_api_types::error::Error;
use reqwest::Client;
use serde::de::DeserializeOwned;
use url::Url;

use crate::v5::AuthTokens;
use crate::{API_URL, API_DEV_URL};
use mangadex_api_types::error::Result;

#[cfg(not(feature = "multi-thread"))]
pub type HttpClientRef = Rc<RefCell<HttpClient>>;
#[cfg(feature = "multi-thread")]
pub type HttpClientRef = Arc<Mutex<HttpClient>>;

#[derive(Debug, Builder, Clone)]
#[builder(setter(into, strip_option), default)]
pub struct HttpClient {
    pub client: Client,
    pub base_url: Url,
    auth_tokens: Option<AuthTokens>,
    captcha: Option<String>,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self {
            client: Client::new(),
            base_url: Url::parse(API_URL).expect("error parsing the base url"),
            auth_tokens: None,
            captcha: None,
        }
    }
}

impl HttpClient {
    /// Create a new `HttpClient` with a custom [`reqwest::Client`](https://docs.rs/reqwest/latest/reqwest/struct.Client.html).
    pub fn new(client: Client) -> Self {
        Self {
            client,
            ..Default::default()
        }
    }

    /// Get a builder struct to customize the `HttpClient` fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use url::Url;
    ///
    /// use mangadex_api::{MangaDexClient, HttpClient};
    ///
    /// # async fn run() -> anyhow::Result<()> {
    /// let http_client = HttpClient::builder()
    ///     .base_url(Url::parse("127.0.0.1:8000")?)
    ///     .build()?;
    ///
    /// let mangadex_client = MangaDexClient::new_with_http_client(http_client);
    /// # Ok(())
    /// # }
    /// ```
    pub fn builder() -> HttpClientBuilder {
        HttpClientBuilder::default()
    }

    /// Send the request to the endpoint but don't deserialize the response.
    ///
    /// This is useful to handle things such as response header data for more control over areas
    /// such as rate limiting.
    pub(crate) async fn send_request_without_deserializing<E>(
        &self,
        endpoint: &E,
    ) -> Result<reqwest::Response>
    where
        E: Endpoint,
    {
        let mut endpoint_url = self.base_url.join(&endpoint.path())?;
        if let Some(query) = endpoint.query() {
            endpoint_url = endpoint_url.query_qs(query);
        }

        let mut req = self.client.request(endpoint.method(), endpoint_url);

        if let Some(body) = endpoint.body() {
            req = req.json(body);
        }

        if let Some(multipart) = endpoint.multipart() {
            req = req.multipart(multipart);
        }

        if let Some(tokens) = self.get_tokens() {
            req = req.bearer_auth(&tokens.session)
        } else if endpoint.require_auth() {
            return Err(Error::MissingTokens);
        }

        if let Some(captcha) = self.get_captcha() {
            req = req.header("X-Captcha-Result", captcha);
        }

        Ok(req.send().await?)
    }

    /// Send the request to the endpoint and deserialize the response body.
    pub(crate) async fn send_request<E>(&self, endpoint: &E) -> Result<E::Response>
    where
        E: Endpoint,
        <<E as Endpoint>::Response as FromResponse>::Response: DeserializeOwned,
    {
        let res = self.send_request_without_deserializing(endpoint).await?;

        let status_code = res.status();

        if status_code.is_server_error() {
            return Err(Error::ServerError(status_code.as_u16(), res.text().await?));
        }

        let res = res
            .json::<<E::Response as FromResponse>::Response>()
            .await?;

        Ok(FromResponse::from_response(res))
    }

    /// Get the authentication tokens stored in the client.
    pub fn get_tokens(&self) -> Option<&AuthTokens> {
        self.auth_tokens.as_ref()
    }

    /// Set new authentication tokens into the client.
    pub fn set_auth_tokens(&mut self, auth_tokens: &AuthTokens) {
        self.auth_tokens = Some(auth_tokens.clone());
    }

    /// Remove all authentication tokens from the client.
    ///
    /// This is effectively the same as logging out, though will not remove the active session from
    /// the MangaDex server. Be sure to call the logout endpoint to ensure your session is removed.
    pub fn clear_auth_tokens(&mut self) {
        self.auth_tokens = None;
    }

    /// Get the captcha solution stored in the client.
    pub fn get_captcha(&self) -> Option<&String> {
        self.captcha.as_ref()
    }

    /// Set a new captcha solution into the client.
    ///
    /// The code needed for this can be found in the "X-Captcha-Sitekey" header field,
    /// or the `siteKey` parameter in the error context of a 403 response,
    /// `captcha_required_exception` error code.
    pub fn set_captcha<T: Into<String>>(&mut self, captcha: T) {
        self.captcha = Some(captcha.into());
    }

    /// Remove the captcha solution from the client.
    pub fn clear_captcha(&mut self) {
        self.captcha = None;
    }
    /// Create a new client of api.mangadex.dev
    pub fn api_dev_client() -> Self{
        Self { 
            client: Client::new(), 
            base_url: Url::parse(API_DEV_URL).expect("error parsing the base url"), 
            auth_tokens: None, 
            captcha: None 
        }
    }
}

/// Helper macro to quickly implement the `Endpoint` trait,
/// and optionally a `send()` method for the input struct.
///
/// The arguments are ordered as follows:
///
/// 1. HTTP method and endpoint path.
/// 2. Input data to serialize unless `no_data` is specified.
/// 3. Response struct to deserialize into.
///
/// with the following format:
///
/// 1. \<HTTP Method\> "\<ENDPOINT PATH\>"
/// 2. \#\[\<ATTRIBUTE\>\] \<INPUT STRUCT\>
/// 3. \#\[\<OPTIONAL ATTRIBUTE\>\] \<OUTPUT STRUCT\>
///
/// The endpoint is specified by the HTTP method, followed by the path. To get a dynamic path
/// based on the input structure, surround the path with parenthesis:
///
/// ```rust, ignore
/// POST ("/account/activate/{}", id)
/// ```
///
/// The format is the same as the `format!()` macro, except `id` will be substituted by `self.id`,
/// where `self` represents an instance of the second parameter.
///
/// The input structure is preceded by an attribute-like structure.
///
/// - `query`: The input structure will be serialized as the query string.
/// - `body`: The input structure will be serialized as a JSON body.
/// - `no_data`: No data will be sent with the request.
/// - `auth`: If this is included, the request will not be made if the user is not authenticated.
///
/// Some examples of valid tags are:
///
/// ```rust, ignore
/// #[query] QueryReq
/// #[body] BodyReq
/// #[query auth] QueryReq
/// #[no_data] QueryStruct
/// ```
///
/// The input structure itself should implement `serde::Serialize` if it is used as a body or query.
///
/// The third argument is the output type, tagged similarly to the input, to modify the behaviour
/// of the generated `send()` method.
///
/// - \<no tag\>: `send()` will simply return `Result<Output>`.
/// - `flatten_result`: If `Output = Result<T>`, the return type will be simplified to `Result<T>`.
/// - `discard_result`: If `Output = Result<T>`, discard `T`, and return `Result<()>`.
/// - `no_send`: Do not implement a `send()` function.
///
/// # Examples
///
/// ```rust, ignore
/// endpoint! {
///     GET "/path/to/endpoint", // Endpoint.
///     #[query] StructWithData<'_>, // Input data; this example will be serialized as a query string.
///     #[flatten_result] Result<ResponseType> // Response struct; this example will return `Ok(res)` or `Err(e)` instead of `Result<ResponseType>` because of `#[flatten_result]`.
/// }
/// ```
macro_rules! endpoint {
    {
        $method:ident $path:tt,
        #[$payload:ident $($auth:ident)?] $typ:ty,
        $(#[$out_res:ident])? $out:ty
    } => {
        impl mangadex_api_schema::Endpoint for $typ {
            /// The response type.
            type Response = $out;

            /// Get the method of the request.
            fn method(&self) -> reqwest::Method {
                reqwest::Method::$method
            }

            endpoint! { @path $path }
            endpoint! { @payload $payload }
            // If the `auth` attribute is set, make the request require authentication.
            $(endpoint! { @$auth })?
        }

        endpoint! { @send $(:$out_res)?, $typ, $out }
    };

    { @path ($path:expr, $($arg:ident),+) } => {
        /// Get the path of the request.
        fn path(&self) -> std::borrow::Cow<str> {
            std::borrow::Cow::Owned(format!($path, $(self.$arg),+))
        }
    };
    { @path $path:expr } => {
        /// Get the path of the request.
        fn path(&self) -> std::borrow::Cow<str> {
            std::borrow::Cow::Borrowed($path)
        }
    };

    // Set a query string.
    { @payload query } => {
        type Query = Self;
        type Body = ();

        /// Get the query of the request.
        fn query(&self) -> Option<&Self::Query> {
            Some(&self)
        }
    };
    // Set a JSON body.
    { @payload body } => {
        type Query = ();
        type Body = Self;

        /// Get the body of the request.
        fn body(&self) -> Option<&Self::Body> {
            Some(&self)
        }
    };
    // Don't send any additional data with the request.
    { @payload no_data } => {
        type Query = ();
        type Body = ();
    };

    { @auth } => {
        /// Get whether auth is required for this request.
        fn require_auth(&self) -> bool {
            true
        }
    };

    // Return the response as a `Result`.
    { @send, $typ:ty, $out:ty } => {
        impl $typ {
            /// Send the request.
            pub async fn send(&self) -> mangadex_api_types::error::Result<$out> {
                #[cfg(not(feature = "multi-thread"))]
                {
                    self.http_client.try_borrow()?.send_request(self).await
                }
                #[cfg(feature = "multi-thread")]
                {
                    self.http_client.lock().await.send_request(self).await
                }
            }
        }
    };
    // Return the `Result` variants, `Ok` or `Err`.
    { @send:flatten_result, $typ:ty, $out:ty } => {
        impl $typ {
            /// Send the request.
            #[allow(dead_code)]
            pub async fn send(&self) -> $out {
                #[cfg(not(feature = "multi-thread"))]
                {
                    self.http_client.try_borrow()?.send_request(self).await?
                }
                #[cfg(feature = "multi-thread")]
                {
                    self.http_client.lock().await.send_request(self).await?
                }
            }
        }
    };
    // Don't return any data from the response.
    { @send:discard_result, $typ:ty, $out:ty } => {
        impl $typ {
            /// Send the request.
            #[allow(dead_code)]
            pub async fn send(&self) -> mangadex_api_types::error::Result<()> {
                #[cfg(not(feature = "multi-thread"))]
                self.http_client.try_borrow()?.send_request(self).await??;
                #[cfg(feature = "multi-thread")]
                self.http_client.lock().await.send_request(self).await??;

                Ok(())
            }
        }
    };
    // Don't implement `send()` and require manual implementation.
    { @send:no_send, $typ:ty, $out:ty } => { };
}
