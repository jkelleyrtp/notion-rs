//! notion-rs: Rust Wrappers around the Notion.so note-taking application.
//! ---
//! This crate provides useful wrappers and Notion's unofficial API.
//!
//! > **This crate has not been yet updated for the upcoming offical API release!**
//!
//! Once this API is released, you'll be able to perform authorization, reads, and writes valid notion workspaces.
//!
//! For now, if you have your TokenV2, then you too can read and write to a notion workspace.
//!
//! ```ignore
//! #[async_std::main]
//! async fn main() {
//!     // Don't commit your token to git!
//!     let token = std::env::var("NOTION_TOKEN_V2").unwrap();
//!
//!     let mut client = notion_rs::builder(token.as_str()).build();
//!
//!     let blocks = client
//!         .get_page("https://www.notion.so/157765353f2c4705bd45474e5ba8b46c")
//!         .await
//!         .unwrap();
//!
//!     println!("{:#?}", blocks);
//! }
//! ```
//!
//!
//! ## Usage
//! Most functionality of this crate is managed by the NotionClient, NotionQuery, and NotionBlock.
//!
//! - [`NotionBlock`]: A single block in a notion workspace. Has an enum-based type
//! - [`NotionQuery`]: Craft HTTP queries to the Notion API endpoint.
//! - [`NotionClient`]: Wrap a Reqwest HTTP client with functionaity

mod block;
mod cfg;
mod error;
mod query;
mod util;

#[cfg(feature = "client")]
mod client;

// innerludes are cool, eh?
pub(crate) mod innerlude {
    pub use crate::block::raw::{BlockMap, GetBlocksResponse};
    pub use crate::block::NotionBlock;
    pub use crate::error::{Error, Result};
    pub use crate::query::NotionQuery;
    pub use crate::NotionEndpoint;
}

pub use crate::{
    block::{BlockData, NotionBlock},
    cfg::NotionEndpoint,
    client::{ClientConfig, NotionClient},
    query::NotionQuery,
};

/// Create a new ClientBuilder to generate a new `NotionClient
///
/// ```ignore
/// let mut client = notion_rs::builder(TOKEN).build();
/// let blocks = client.get_page("...").await.unwrap();
/// for block in blocks {
///  // do something
/// }
/// ```
pub fn builder(token_v2: &str) -> ClientConfig {
    ClientConfig::new(token_v2)
}
