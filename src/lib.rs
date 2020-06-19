pub mod block;
pub mod blocks;
pub mod blocksview;
pub mod cache;
pub mod cfg;
pub mod client;
pub mod collection;
pub mod query;
pub mod util;

pub mod prelude {
    use super::*;
    pub use crate::{
        block::NotionBlock, blocks::BlockType, cfg::ClientConfig, cfg::NotionEndpoint,
        client::NotionClient,
    };
}
