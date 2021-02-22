//! Configuration for notion client that makes sense to change on deployment

#![allow(unused)]
static API_URL: &'static str = "https://www.notion.so/api/v3/";
static BASE_URL: &'static str = "https://www.notion.so";
static SIGNED_URL_PREFIX: &'static str = "https://www.notion.so/signed/";
static S3_URL_PREFIX: &'static str = "https://s3-us-west-2.amazonaws.com/secure.notion-static";
static S3_URL_PREFIX_ENCODED: &'static str =
    "https://s3.us-west-2.amazonaws.com/secure.notion-static";

static BASE_ENDPOINT: &'static str = "https://www.notion.so/api/v3/";
static LOAD_PAGE_CHUNK_ENDPOINT: &'static str = "https://www.notion.so/api/v3/loadPageChunk";
static LOAD_USER_CONTENT_ENDPOINT: &'static str = "https://www.notion.so/api/v3/loadUserContent";

pub enum NotionEndpoint {
    Base,
    LoadPageChunk,
    LoadUserContent,
}

impl NotionEndpoint {
    pub fn as_str(&self) -> &str {
        match self {
            NotionEndpoint::Base => BASE_ENDPOINT,
            NotionEndpoint::LoadPageChunk => LOAD_PAGE_CHUNK_ENDPOINT,
            NotionEndpoint::LoadUserContent => LOAD_USER_CONTENT_ENDPOINT,
        }
    }
}
