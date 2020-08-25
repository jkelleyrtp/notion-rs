use {
    serde::{Deserialize, Serialize},
    serde_json,
    std::collections::HashMap,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct RawInput {
    pub role: String,
    pub value: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawBlock {
    pub id: String,
    version: u32,

    #[serde(alias = "type")]
    pub block_type: String,
    pub properties: Option<serde_json::Value>,
    pub format: Option<serde_json::Value>,
    pub content: Option<Vec<String>>,
    permissions: Option<serde_json::Value>,
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
what the best way to deserialize this is. It's meant for really pre-processing
long titles. For now, we flatten the vecs down before deserializing them so it's
easier to work with.

"properties": {
    "title": [["THISISATODO"]]
},


Text is *real fuckety*
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
pub fn compress_properties(
    vals: serde_json::Value,
    content: Option<Vec<String>>,
) -> anyhow::Result<serde_json::Value> {
    let interpreted: HashMap<String, Vec<Vec<String>>> = serde_json::from_value(vals)?;

    let mut new: HashMap<String, serde_json::Value> = interpreted
        .into_iter()
        .map(|(id, value)| {
            let mut out = String::new();
            for outer in value {
                for inner in outer {
                    out.push_str(inner.as_str())
                }
            }
            (id, serde_json::Value::String(out))
        })
        .collect();

    if let Some(ct) = content {
        new.insert("content".to_string(), serde_json::to_value(ct)?);
    }

    Ok(serde_json::to_value(new)?)
}
