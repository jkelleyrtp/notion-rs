mod collection;
mod data;
mod equation;
mod page;
mod raw;
mod text;

use anyhow::Result;
pub use data::BlockData;
use raw::{compress_properties, RawBlock, RawInput};
use serde_json::json;
use uuid::Uuid;
use {serde::Serialize, serde_json};

#[derive(Debug, Serialize, PartialEq)]
pub struct NotionBlock {
    pub id: String,
    pub data: BlockData,
    pub content: Vec<Uuid>,
}

impl<'de> serde::Deserialize<'de> for NotionBlock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw_in: RawInput = RawInput::deserialize(deserializer)?;

        let RawBlock {
            id,
            block_type,
            properties,
            content,
            ..
        } = serde_json::from_value(raw_in.value).expect("Failed to convert serde");

        let compressed_props = compress_properties(properties.unwrap_or(json!({}))).expect("rip");

        let inter = json!({
          "type": block_type,
          "properties": compressed_props,
        });

        let data: BlockData =
            serde_json::from_value(inter.clone()).expect(format!("{:#?}", inter).as_str());

        let content = content
            .unwrap_or(vec![])
            .into_iter()
            .map(|f| Uuid::parse_str(f.as_str()))
            .filter_map(Result::ok)
            .collect();

        let outblock = NotionBlock { id, data, content };

        Ok(outblock)
    }
}

#[test]
fn test_serde() {
    let block = serde_json::json!({
      "role": "editor",
      "value": {
        "id": "eb492325-3d15-4dd5-adf8-a80d773acb15",
        "version": 59,
        "type": "page",
        "properties": { "title": [["KitchenSink Test"]] },
        "content": [
          "f1366603-f22f-40cf-bbe4-dd48dc9a023c",
          "59ca4846-c2f5-4938-9c9f-6a85c175dec8",
          "ab37cc91-5cee-473e-a559-73effa5e8b7b",
          "63ebaf2b-de3b-415f-826c-0a842b3145f7",
          "87ab67ca-8919-4086-98ba-35714248d088"
        ],
        "permissions": [
          {
            "role": "editor",
            "type": "user_permission",
            "user_id": "e7ada895-daf9-4a77-84d3-7e136e3c2ea1"
          }
        ],
        "created_time": 159183,
        "last_edited_time": 159184,
        "parent_id": "9eb2b16c-8502-469d-a4f8-1c11e1d65051",
        "parent_table": "space",
        "alive": true,
        "created_by_table": "notion_user",
        "created_by_id": "e7ada895-daf9-4a77-84d3-7e136e3c2ea1",
        "last_edited_by_table": "notion_user",
        "last_edited_by_id": "e7ada895-daf9-4a77-84d3-7e136e3c2ea1",
        "shard_id": 413777,
        "space_id": "9eb2b16c-8502-469d-a4f8-1c11e1d65051"
      }
    });

    let block: NotionBlock = serde_json::from_value(block).unwrap();

    assert_eq!(block.id.as_str(), "eb492325-3d15-4dd5-adf8-a80d773acb15");
    assert_eq!(
        block.data,
        BlockData::Page {
            title: "KitchenSink Test".to_string().into(),
        },
    );
    assert_eq!(
        block.content,
        vec![
            uuid::Uuid::parse_str("f1366603-f22f-40cf-bbe4-dd48dc9a023c").unwrap(),
            uuid::Uuid::parse_str("59ca4846-c2f5-4938-9c9f-6a85c175dec8").unwrap(),
            uuid::Uuid::parse_str("ab37cc91-5cee-473e-a559-73effa5e8b7b").unwrap(),
            uuid::Uuid::parse_str("63ebaf2b-de3b-415f-826c-0a842b3145f7").unwrap(),
            uuid::Uuid::parse_str("87ab67ca-8919-4086-98ba-35714248d088").unwrap()
        ]
    )
}
