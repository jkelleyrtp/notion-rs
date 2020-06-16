//! Configuration for notion client that makes sense to change on deployment
use crate::prelude::NotionBlock;
use _config::CombinedConfig;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use toml::Value;

pub struct ClientConfig {
    // User set-ables
    /// token_v2 found by inspecting browser cookies on a logged-in session of Notion
    /// Be careful to not let this leak out into the world
    pub auth_token: String,

    /// Should the notionclient maintain a record store for faster indexing?
    /// Required for monitoring updates on a workspace
    pub enable_caching: bool,

    /// ID of the workspace the notion client uses to do things like monitoring and searching
    pub workspace_id: Option<String>,

    /// Should the client autodetect the current workspace from blocks its currently working with?
    pub auto_detect_ws: bool,

    // Defaults for notion that change with each version
    // Override these if the rust library is out of date with the actual endpoints
    pub api_url: String,
    pub base_url: String,

    pub signed_url_prefix: String,
    pub s3_url_prefix: String,
    pub s3_url_prefix_encoded: String,
}

impl ClientConfig {
    pub fn from_file<S>(file_path: S) -> Result<Self>
    where
        S: Into<String>,
    {
        let mut data = String::new();
        File::open(file_path.into())?.read_to_string(&mut data)?;
        let val: CombinedConfig = toml::from_str(data.as_str()).unwrap();

        Ok(ClientConfig {
            auth_token: val.auth.token_v2,
            enable_caching: val.config.cache,
            workspace_id: None,
            auto_detect_ws: false,
            api_url: val.client.api_url,
            base_url: val.client.base_url,
            signed_url_prefix: val.client.signed_url_prefix,
            s3_url_prefix: val.client.s3_url_prefix,
            s3_url_prefix_encoded: val.client.s3_url_prefix_encoded,
        })
    }

    fn token_v2(mut self, token: String) -> Self {
        self.auth_token = token;
        self
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        ClientConfig {
            auth_token: "".to_string(),
            enable_caching: true,
            workspace_id: None,
            auto_detect_ws: true,
            api_url: "https://www.notion.so/api/v3/".to_string(),
            base_url: "https://www.notion.so".to_string(),
            signed_url_prefix: "https://www.notion.so/signed/".to_string(),
            s3_url_prefix: "https://s3-us-west-2.amazonaws.com/secure.notion-static".to_string(),
            s3_url_prefix_encoded: "https://s3.us-west-2.amazonaws.com/secure.notion-static"
                .to_string(),
        }
    }
}

pub enum NotionEndpoint {
    base,
    loadPageChunk,
    loadUserContent,
}
impl ToString for NotionEndpoint {
    fn to_string(&self) -> String {
        use NotionEndpoint::*;
        match self {
            base => "https://www.notion.so/api/v3/".to_string(),
            loadPageChunk => "https://www.notion.so/api/v3/loadPageChunk".to_string(),
            loadUserContent => "https://www.notion.so/api/v3/loadUserContent".to_string(),
        }
    }
}

mod _config {
    use super::*;

    #[derive(Deserialize, Serialize)]
    pub struct CombinedConfig {
        pub auth: auth,
        pub client: client,
        pub config: config,
    }
    #[derive(Deserialize, Serialize)]
    pub struct auth {
        pub token_v2: String,
    }

    #[derive(Deserialize, Serialize)]
    pub struct client {
        pub api_url: String,
        pub base_url: String,
        pub signed_url_prefix: String,
        pub s3_url_prefix: String,
        pub s3_url_prefix_encoded: String,
    }

    #[derive(Deserialize, Serialize)]
    pub struct config {
        pub cache: bool,
    }
}
