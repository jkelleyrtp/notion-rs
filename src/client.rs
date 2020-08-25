use {
    crate::query::NotionQuery,
    crate::{ClientConfig, NotionEndpoint},
    anyhow::{anyhow, Result},
    reqwest, serde_json,
};

pub struct NotionClient {
    pub cfg: ClientConfig,
    pub client: reqwest::Client,
}

impl NotionClient {
    pub fn builder() -> ClientConfig {
        ClientConfig::default()
    }

    pub async fn post(
        &mut self,
        endpoint: NotionEndpoint,
        data: serde_json::value::Value,
    ) -> Result<reqwest::Response> {
        let token = self.cfg.auth_token.as_ref().ok_or(anyhow!("No auth token"));
        self.client
            .post(endpoint.as_str())
            .json(&data)
            .header(reqwest::header::COOKIE, format!("token_v2={}", token?))
            .send()
            .await
            .map_err(|_| anyhow!("failed to post"))
    }

    pub async fn post_query(&mut self, query: NotionQuery) -> Result<reqwest::Response> {
        self.post(NotionEndpoint::LoadPageChunk, query.to_data())
            .await
            .map_err(|_f| anyhow!("hello!"))
    }
}

#[tokio::test]
async fn test_notion_client() -> Result<()> {
    let page =
        "https://www.notion.so/jonathankelley/KitchenSink-Test-eb4923253d154dd5adf8a80d773acb15";

    // Make sure to use ENV vars to test
    // let token = std::env::var("NOTION_TOKEN_V2")?;
    let token = "6b28acdd4833c13eac5980ce925691146fb218faeaaee0519ca737d538b264afa3821e630e4b215619fc8d18c05117276b77d1944c3b1618612ece054b437d83e28b7b4427e5a71912e912607254";

    let mut client = NotionClient::builder().token_v2(token).build();

    let myblocks = client
        .post_query(NotionQuery::from_url(page)?)
        .await?
        .json::<serde_json::Value>()
        .await?;

    // println!("My blocks {:#?}", myblocks);

    println!("{}", serde_json::to_string_pretty(&myblocks).unwrap());

    Ok(())
}
