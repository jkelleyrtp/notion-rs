//! This module provides the raw types for serde to serialize/deserialize requests into.
//!
//! Schema
//! ----
//! {
//!     recordMap: {
//!         block: {
//!             [id]: NotionBlock
//!         },
//!         space: {
//!             [id]: Space
//!         },
//!         notion_user: {
//!             [id]: User
//!         }
//!     },
//!     cursor: {
//!         stack: [?]
//!     }
//! }
//!

use crate::innerlude::{NotionBlock, Result};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlocksResponse {
    #[serde(rename = "recordMap")]
    pub record_map: RecordMap,

    pub cursor: Option<JsonValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordMap {
    /// An ordered hashmap
    pub block: BlockMap,

    /// TBD actual values on these
    pub space: JsonValue,

    /// TBD actual values on these
    pub discussion: JsonValue,

    /// TBD actual values on these
    pub comment: JsonValue,

    /// TBD actual values on these
    pub collection: JsonValue,

    /// TBD actual values on these
    pub collection_view: JsonValue,
    // /// TBD actual values on these
    // /// This *might* be deprecated
    // pub notion_user: Option<JsonValue>,
}

pub type BlockMap = IndexMap<String, NotionBlock>;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct RawInput {
    pub role: String,
    pub value: JsonValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct RawBlock {
    #[serde(alias = "type")]
    pub block_type: String,
    pub id: String,
    version: u32,
    pub properties: Option<JsonValue>,
    pub format: Option<JsonValue>,
    pub content: Option<Vec<String>>,
    permissions: Option<JsonValue>,
    created_time: i64,
    last_edited_time: i64,
    parent_id: String,
    parent_table: String,
    alive: bool,
    created_by_table: String,
    created_by_id: String,
    last_edited_by_id: String,
    shard_id: Option<i64>,
    space_id: Option<String>,
}

/*
Notion is really weird and they have this nested title situation. Not sure
what the best way to deserialize this is. It's really meant for pre-processing
long titles and working with complex bodies of text. For now, we flatten the
vecs down before deserializing them so it's easier to work with.

"properties": {
    "title": [["THISISATODO"]]
},

Text is strange
----
A dedicated parser is be needed for text deserialization.
Notion embeds context into the text (number of lines, starting characters, return, etc)

"properties": {
    "title": [
        ["‣", [["u", "e7ada895-daf9-4a77-84d3-7e136e3c2ea1"]]],
        [" "]
    ]
}
*/
pub fn compress_properties(vals: JsonValue) -> Result<JsonValue> {
    let interpreted: HashMap<String, JsonValue> =
        serde_json::from_value(vals.clone()).expect(format!("vals are \n{:#?}", vals).as_str());

    let new: HashMap<String, String> = interpreted
        .into_iter()
        .map(|(id, value)| (id, recurse_nested_array(value)))
        .collect();

    Ok(serde_json::to_value(new)?)
}

fn recurse_nested_array(val: JsonValue) -> String {
    match val {
        JsonValue::String(text) => text,
        JsonValue::Array(ar) => ar
            .into_iter()
            .map(|f| recurse_nested_array(f))
            .collect::<Vec<String>>()
            .concat(),
        _ => "".to_string(),
    }
}
