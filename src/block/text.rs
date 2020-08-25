//! text blocks are not easy
//! We need a custom deserialzer

use serde::{de, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NotionText {}

pub fn deserialize_notion_text<'de, D>(deserializer: D) -> Result<NotionText, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;
    println!("Text input is {:#?}", s);
    serde_json::from_str(s).map_err(de::Error::custom)
}
