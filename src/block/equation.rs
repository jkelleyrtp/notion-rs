//! text blocks are not easy
//! We need a custom deserialzer

use serde::{de, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NotionEquation {}

pub fn deserialize_notion_equation<'de, D>(deserializer: D) -> Result<NotionEquation, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;
    println!("Equation input is {:#?}", s);
    serde_json::from_str(s).map_err(de::Error::custom)
}
