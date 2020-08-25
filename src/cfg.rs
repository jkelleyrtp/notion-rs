//! Configuration for notion client that makes sense to change on deployment

use crate::NotionClient;

pub struct ClientConfig {
    // User set-ables
    /// token_v2 found by inspecting browser cookies on a logged-in session of Notion
    /// Be careful to not let this leak out into the world
    pub auth_token: Option<String>,

    /// ID of the workspace the notion client uses to do things like monitoring and searching
    pub workspace_id: Option<String>,

    // Defaults for notion that change with each version
    pub api_url: &'static str,
    pub base_url: &'static str,
    pub signed_url_prefix: &'static str,
    pub s3_url_prefix: &'static str,
    pub s3_url_prefix_encoded: &'static str,
}

impl ClientConfig {
    pub fn default() -> Self {
        ClientConfig {
            auth_token: None,
            workspace_id: None,
            api_url: "https://www.notion.so/api/v3/",
            base_url: "https://www.notion.so",
            signed_url_prefix: "https://www.notion.so/signed/",
            s3_url_prefix: "https://s3-us-west-2.amazonaws.com/secure.notion-static",
            s3_url_prefix_encoded: "https://s3.us-west-2.amazonaws.com/secure.notion-static",
        }
    }

    pub fn token_v2(mut self, token: &str) -> Self {
        self.auth_token = Some(token.to_string());
        self
    }

    pub fn build(self) -> NotionClient {
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();

        NotionClient { cfg: self, client }
    }
}

pub enum NotionEndpoint {
    Base,
    LoadPageChunk,
    LoadUserContent,
}

impl NotionEndpoint {
    pub fn as_str(&self) -> &str {
        use NotionEndpoint::*;
        match self {
            Base => "https://www.notion.so/api/v3/",
            LoadPageChunk => "https://www.notion.so/api/v3/loadPageChunk",
            LoadUserContent => "https://www.notion.so/api/v3/loadUserContent",
        }
    }
}
