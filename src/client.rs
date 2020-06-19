use {
    crate::cache,
    crate::prelude::{BlockType, ClientConfig, NotionBlock, NotionEndpoint},
    crate::query::NotionQuery,
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

    fn from_cfg_file() -> NotionClient {
        let cfg = ClientConfig::from_file("Notion.toml").unwrap_or_default();
        Self::new(cfg)
    }

    pub async fn post(
        &mut self,
        endpoint: NotionEndpoint,
        data: serde_json::value::Value,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.client
            .post(endpoint.to_string().as_str())
            .json(&data)
            .header(
                reqwest::header::COOKIE,
                format!("token_v2={}", self.cfg.auth_token),
            )
            .send()
            .await
    }

    pub async fn post_query(&mut self, query: impl NotionQuery) -> Result<reqwest::Response> {
        let data = json!({});

        self.post(NotionEndpoint::loadPageChunk, data)
            .await
            .map_err(|f| anyhow!("hello!"))
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
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_notion_client() -> Result<()> {
        let page = "https://www.notion.so/jonathankelley/KitchenSink-Test-eb4923253d154dd5adf8a80d773acb15";

        let mut client: NotionClient = NotionClient::new(ClientConfig::from_file("Notion.toml")?);

        // let my_blocks: GetBlocksResponse = client.get_page(page).await?;

        // my_blocks.recordMap.block.iter().for_each(|(k, v)| {
        //     println!(
        //         "{:#?}: {:#?} - {:#?}",
        //         v.block_properties_type, v.content, v.properties
        //     )
        // });

        Ok(())
    }

    #[tokio::test]
    async fn test_query_builder() -> Result<()> {
        let mut client = NotionClient::from_cfg_file();
        let page = "https://www.notion.so/jonathankelley/KitchenSink-Test-eb4923253d154dd5adf8a80d773acb15";

        Ok(())
    }
}
