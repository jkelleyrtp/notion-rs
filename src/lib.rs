pub mod block;
pub mod blocks;
pub mod cache;
pub mod cfg;
pub mod client;

pub mod prelude {
    use super::*;
    pub use crate::{
        block::NotionBlock, blocks::BlockType, client::API_Endpoints, client::NotionClient,
    };
}
