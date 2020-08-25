use {
    crate::NotionBlock,
    indexmap::IndexMap,
    serde::{Deserialize, Serialize},
};

/**
{
    recordMap: {
        block: {
            [id]: NotionBlock
        },
        space: {
            [id]: Space
        },
        notion_user: {
            [id]: User
        }
    },
    cursor: {
        stack: [?]
    }
}
*/

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlocksResponse {
    #[serde(rename = "recordMap")]
    pub record_map: RecordMap,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordMap {
    /// An ordered hashmap
    pub block: IndexMap<String, NotionBlock>,

    /// TBD actual values on these
    pub space: serde_json::Value,

    /// TBD actual values on these
    pub notion_user: serde_json::Value,
}
