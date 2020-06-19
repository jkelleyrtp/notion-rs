use {
    crate::prelude::{BlockType, NotionBlock},
    indexmap::IndexMap,
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlocksResponse {
    pub cursor: serde_json::Value,
    pub recordMap: recordMap,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct recordMap {
    pub block: IndexMap<String, NotionBlock>,
    pub notion_user: serde_json::Value,
    pub space: serde_json::Value,
}
