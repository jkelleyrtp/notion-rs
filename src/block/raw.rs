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
what the best way to deserialize this is. It's really meant for pre-processing
long titles and working with complex bodies of text. For now, we flatten the
vecs down before deserializing them so it's easier to work with.

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
pub fn compress_properties(vals: serde_json::Value) -> anyhow::Result<serde_json::Value> {
    let interpreted: HashMap<String, serde_json::Value> =
        serde_json::from_value(vals.clone()).expect(format!("vals are \n{:#?}", vals).as_str());

    let new: HashMap<String, String> = interpreted
        .into_iter()
        .map(|(id, value)| (id, recurse_nested_array(value)))
        .collect();

    Ok(serde_json::to_value(new)?)
}

fn recurse_nested_array(val: serde_json::Value) -> String {
    match val {
        serde_json::Value::String(text) => text,
        serde_json::Value::Array(ar) => ar
            .into_iter()
            .map(|f| recurse_nested_array(f))
            .collect::<Vec<String>>()
            .concat(),
        _ => "".to_string(),
    }
}
