use crate::innerlude::{BlockMap, Error, GetBlocksResponse, NotionEndpoint, NotionQuery, Result};

/// This struct provides access to the Notion.so API via a Reqwest client.
///
/// You *don't* need to have this module enabled if you are providing your own HTTP client. If this matches your usecase,
/// then try disabling the "client" feature and using the query system directly.
pub struct NotionClient {
    pub(crate) cfg: ClientConfig,
    pub(crate) client: reqwest::Client,
}

impl NotionClient {
    /// Create a new [`ClientConfig`] object to configure the behavior of the [`NotionClient`]
    pub fn builder(token_v2: &str) -> ClientConfig {
        ClientConfig::new(token_v2)
    }

    /// Make a request to the notion api to get a page by its URL.
    ///
    /// If you already have the page ID as UUID, then [`get_block`] will also work.
    pub async fn get_page(&mut self, url: &str) -> Result<BlockMap> {
        let query = NotionQuery::from_url(url)?;
        let res: GetBlocksResponse = self.post_query(query).await?.json().await?;
        Ok(res.record_map.block)
    }

    /// Get a block directly by its UUID.
    ///
    /// If you don't have the UUID, then try using the [`get_page`] method to query by URL.
    pub async fn get_block(&mut self, id: uuid::Uuid) -> Result<BlockMap> {
        let query = NotionQuery::GetBlock { block_id: id };
        let res: GetBlocksResponse = self.post_query(query).await?.json().await?;
        Ok(res.record_map.block)
    }

    /// This method returns the entire response from the NotionAPI, in case you need extra information about:
    /// - cursor position
    /// - comments
    /// - settings
    /// - more
    pub async fn get_page_raw(&mut self, url: &str) -> Result<GetBlocksResponse> {
        let query = NotionQuery::from_url(url)?;
        let res: GetBlocksResponse = self.post_query(query).await?.json().await?;

        Ok(res)
    }

    /// This method returns the entire response from the NotionAPI, in case you need extra information about:
    /// - cursor position
    /// - comments
    /// - settings
    /// - more
    pub async fn get_block_raw(&mut self, id: uuid::Uuid) -> Result<GetBlocksResponse> {
        let query = NotionQuery::GetBlock { block_id: id };
        let res = self
            .post_query(query)
            .await?
            .json::<GetBlocksResponse>()
            .await?;

        Ok(res)
    }

    /// Internal method: post directly to a given endpoint
    /// This method is modular around endpoint, so you might want to use [`post_query`] instead.
    pub(crate) async fn post(
        &mut self,
        endpoint: NotionEndpoint,
        data: serde_json::value::Value,
    ) -> Result<reqwest::Response> {
        self.client
            .post(endpoint.as_str())
            .json(&data)
            .header(
                reqwest::header::COOKIE,
                format!("token_v2={}", self.cfg.auth_token),
            )
            .send()
            .await
            .map_err(|_| Error::PostError)
    }

    /// Internal method: post directly the LoadPageChunk endpoint.
    pub(crate) async fn post_query(&mut self, query: NotionQuery) -> Result<reqwest::Response> {
        self.post(NotionEndpoint::LoadPageChunk, query.to_data())
            .await
    }
}

/// The `ClientConfig` provides some configuration options to a `NotionClient`
pub struct ClientConfig {
    /// token_v2 found by inspecting browser cookies on a logged-in session of Notion
    /// Be careful to not let this leak out into the world
    pub auth_token: String,

    /// ID of the workspace the notion client uses to do things like monitoring and searching
    pub workspace_id: Option<String>,
}

impl ClientConfig {
    pub fn new(token: &str) -> Self {
        Self {
            auth_token: token.to_string(),
            workspace_id: None,
        }
    }

    pub fn build(self) -> NotionClient {
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();

        NotionClient { cfg: self, client }
    }
}
