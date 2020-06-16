use {
    crate::cache,
    crate::prelude::{BlockType, ClientConfig, NotionBlock, NotionEndpoint},
    anyhow::{anyhow, Context, Result},
    reqwest, serde,
    serde::{Deserialize, Serialize},
    serde_json,
    serde_json::json,
    std::collections::HashMap,
    uuid::Uuid,
};

pub struct NotionClient {
    pub cfg: ClientConfig,
    pub client: reqwest::Client,
}

impl NotionClient {
    fn new(cfg: ClientConfig) -> NotionClient {
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();

        NotionClient { cfg, client }
    }

    pub async fn post(
        &mut self,
        endpoint: NotionEndpoint,
        data: serde_json::value::Value,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let req = self
            .client
            .post(endpoint.to_string().as_str())
            .json(&data)
            .header(
                reqwest::header::COOKIE,
                format!("token_v2={}", self.cfg.auth_token),
            );
        req.send().await
    }

    pub fn search_blocks(&mut self, search: String, limit: i32) {
        let data = json! ({
            "query": search,
            "table": "space",
            "id": "",
            "limit": limit
        });
    }

    async fn get_page(&mut self, url: &str) -> Result<GetBlocksResponse> {
        let id = url.rsplit('-').next().context("Oh no!")?;
        let page_id = uuid::Uuid::parse_str(id)?;
        let a: GetBlocksResponse = self
            .get_block(page_id)
            .await
            .json::<GetBlocksResponse>()
            .await?;
        Ok(a)
    }

    async fn get_block(&mut self, page_id: Uuid) -> reqwest::Response {
        let data = json!({
            "pageId": page_id.to_hyphenated().to_string(),
            "limit": 100000,
            "cursor": {"stack": []},
            "chunkNumber": 0,
            "verticalColumns": false,
        });

        let res = self
            .post(NotionEndpoint::loadPageChunk, data)
            .await
            .unwrap();
        res
    }
}

impl Default for NotionClient {
    fn default() -> Self {
        NotionClient::new(ClientConfig::default())
    }
}

impl From<ClientConfig> for NotionClient {
    fn from(cfg: ClientConfig) -> Self {
        NotionClient::new(cfg)
    }
}

use crate::util::GetBlocksResponse;

#[tokio::test]
async fn test_notion_client() -> Result<()> {
    let page =
        "https://www.notion.so/jonathankelley/KitchenSink-Test-eb4923253d154dd5adf8a80d773acb15";

    let mut client = NotionClient::new(ClientConfig::from_file("Notion.toml")?);

    let my_blocks: GetBlocksResponse = client.get_page(page).await?;
    my_blocks
        .recordMap
        .block
        .iter()
        .filter(|(k, v)| v.block_properties_type == BlockType::page)
        .for_each(|(k, v)| println!("{:#?}: {:#?}", v.block_properties_type, v.content));

    Ok(())
}
