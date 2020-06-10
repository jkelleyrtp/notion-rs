use {
    crate::prelude::BlockType,
    serde,
    serde::{Deserialize, Serialize},
    serde_json,
    serde_json::json,
    std::collections::HashMap,
};

/// The shared info that every notion block has
/// These are automatically generated on the serialization boundary with the notion API
/// Block info is updated when the cache is updated
#[derive(Serialize, Deserialize, Debug)]
pub struct NotionBlock {
    pub alive: bool,
    pub created_by_id: String,
    pub created_time: i64,
    pub last_edited_time: i64,
    pub id: String,
    pub properties: Option<serde_json::Value>,

    #[serde(alias = "type", default = "BlockType::default")]
    pub block_type: BlockType,
}
