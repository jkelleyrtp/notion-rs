use {
    crate::prelude::BlockType,
    serde::{Deserialize, Serialize},
    serde_json,
    serde_json::json,
    std::collections::HashMap,
    uuid::Uuid,
};

#[derive(Debug)]
pub struct NotionBlock {
    pub id: String,
    pub block_properties_type: BlockType,
}

impl<'de> serde::Deserialize<'de> for NotionBlock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        #[derive(Serialize, Deserialize, Debug)]
        pub struct RawInput {
            pub role: String,
            pub value: serde_json::Value,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct RawBlock {
            pub id: String,
            pub version: u32,
            #[serde(alias = "type")]
            pub block_type: String,
            pub properties: Option<serde_json::Value>,
            pub content: Option<Vec<String>>,
            pub permissions: Option<serde_json::Value>,
            pub created_time: i64,
            pub last_edited_time: i64,
            pub parent_id: String,
            pub parent_table: String,
            pub alive: bool,
            pub created_by_table: String,
            pub created_by_id: String,
            pub last_edited_by_id: String,
            pub shard_id: Option<i64>,
            pub space_id: Option<String>,
        }

        let raw_in: RawInput = RawInput::deserialize(deserializer)?;

        let raw_block: RawBlock = serde_json::from_value(raw_in.value).unwrap();

        let outblock = match raw_block {
            RawBlock {
                id,
                version,
                block_type,
                properties,
                content,
                ..
            } => {
                let props: BlockType =
                    BlockType::from_props(block_type.as_str(), properties, content).unwrap();

                NotionBlock {
                    id,
                    block_properties_type: props,
                }
            }
        };

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

    let b: NotionBlock = serde_json::from_value(block).unwrap();

    println!("Deserialize is {:#?}", b);
}
