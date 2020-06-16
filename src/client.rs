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

        println!("my request: {:?}", &req);
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

impl From<ClientConfig> for NotionClient {
    fn from(cfg: ClientConfig) -> Self {
        NotionClient::new(cfg)
    }
}

use crate::util::{get_all_block_types, GetBlocksResponse};
#[tokio::test]
async fn test_notion_client() -> Result<()> {
    // Make a notion client and download the "Whats new page"
    let page =
        "https://www.notion.so/jonathankelley/KitchenSink-Test-eb4923253d154dd5adf8a80d773acb15";
    let id = page.rsplit('-').next().context("Oh no!")?;
    let a = uuid::Uuid::parse_str(id)?;

    let cfg = ClientConfig::from_file("Notion.toml").unwrap();
    let mut client = NotionClient::new(cfg);

    let res = client.get_block(a).await;

    let my_blocks: GetBlocksResponse = res
        .json::<GetBlocksResponse>()
        .await
        .expect("Couldnt unwrap my blocks");

    let a: HashMap<String, NotionBlock> = my_blocks
        .recordMap
        .block
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| (k.clone(), serde_json::from_value(v.clone()).unwrap()))
        .collect();

    println!("All my block types: {:#?}", a);

    Ok(())
}
