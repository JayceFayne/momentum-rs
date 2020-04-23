use crate::reply::*;
use crate::routes::*;
use crate::Fallible;
use anyhow::bail;
use bytes::buf::BufExt as _;
use chrono::{Date, TimeZone};
use http::header::{HeaderValue, ACCEPT, CONTENT_TYPE, HOST};
use http::request::Request;
use http::uri::{Authority, Parts as UriParts, PathAndQuery, Scheme, Uri};
use hyper::client::HttpConnector;
use hyper::{Client as HyperClient, Method};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned as Deserialize;

static X_MOMENTUM_CLIENT_ID_KEY: &str = "x-momentum-clientid";
static MOMENTUM_HOST: &str = "api.momentumdash.com";
static ACCEPT_ALL: &str = "*/*";
static X_MOMENTUM_VERSION_KEY: &str = "x-momentum-version";
static X_MOMENTUM_VERSION: &str = "1.15.7";
static JSON_CONTENT: &str = "application/json";

pub struct Client {
    client: HyperClient<HttpsConnector<HttpConnector>>,
    client_id: String,
}

impl Client {
    pub fn new(client_id: impl Into<String>) -> Self {
        Self {
            client: HyperClient::builder().build::<_, hyper::Body>(HttpsConnector::new()),
            client_id: client_id.into(),
        }
    }

    pub fn new_with_client(
        client: HyperClient<HttpsConnector<HttpConnector>>,
        client_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            client_id: client_id.into(),
        }
    }

    pub(crate) async fn request<D: Deserialize, T: AsRef<[u8]> + 'static>(
        &self,
        method: Method,
        path_and_query: T,
    ) -> Fallible<D> {
        let mut uri_parts = UriParts::default();
        uri_parts.authority = Some(Authority::from_static(MOMENTUM_HOST));
        uri_parts.scheme = Some(Scheme::HTTPS);
        uri_parts.path_and_query = Some(PathAndQuery::from_maybe_shared(path_and_query)?);
        let uri = Uri::from_parts(uri_parts)?;
        let mut request = Request::new(Default::default());
        std::mem::replace(request.uri_mut(), uri);
        std::mem::replace(request.method_mut(), method);
        let header = request.headers_mut();
        header.insert(HOST, HeaderValue::from_static(MOMENTUM_HOST));
        header.insert(ACCEPT, HeaderValue::from_static(ACCEPT_ALL));
        header.insert(
            X_MOMENTUM_CLIENT_ID_KEY,
            HeaderValue::from_str(&self.client_id)?,
        );
        header.insert(
            X_MOMENTUM_VERSION_KEY,
            HeaderValue::from_static(X_MOMENTUM_VERSION),
        );
        header.insert(CONTENT_TYPE, HeaderValue::from_static(JSON_CONTENT));
        let resp = self.client.request(request).await?;
        let status_code = resp.status().as_u16();
        if status_code < 200 || status_code > 299 {
            bail!("got http status code '{}'", status_code)
        } else {
            let body = hyper::body::aggregate(resp).await?;
            Ok(serde_json::from_reader(body.reader())?)
        }
    }

    async fn get_feed_from<Tz: TimeZone>(&self, date: Date<Tz>) -> Fallible<Feed>
    where
        Tz::Offset: std::fmt::Display,
    {
        self.request(
            Method::GET,
            format!(
                "{}?syncTypes=backgrounds&localDate={}",
                FEED_PATH,
                date.format("%Y-%m-%d")
            ),
        )
        .await
    }

    pub async fn get_feed(&self) -> Fallible<Feed> {
        self.get_feed_from(chrono::Local::today()).await
    }
}
