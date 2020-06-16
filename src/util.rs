use crate::prelude::{BlockType, NotionBlock};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlocksResponse {
    pub cursor: serde_json::Value,
    pub recordMap: recordMap,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct recordMap {
    pub block: serde_json::Value,
    pub notion_user: serde_json::Value,
    pub space: serde_json::Value,
}

pub fn get_all_block_types(blocks: GetBlocksResponse) -> Vec<NotionBlock> {
    let mut true_blocks = Vec::<NotionBlock>::new();

    let a: HashMap<String, NotionBlock> = blocks
        .recordMap
        .block
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| (k.clone(), serde_json::from_value(v.clone()).unwrap()))
        .collect();

    true_blocks
}
