use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Deserialize)]
pub struct Page {
    title: Vec<Vec<String>>,
    content: Option<Vec<String>>,
}

impl Page {}

// impl<'de> Deserialize<'de> for Page {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         #[derive(Debug, Serialize, Deserialize)]
//         struct Raw {
//             properties: Option<Vec<Vec<String>>>,
//             content: serde_json::Value,
//         }

//         let thisraw: Raw = Raw::deserialize(deserializer)?;
//     }
// }

#[test]
fn test_page_de() -> Result<()> {
    let input = serde_json::json!({
        "properties": { "title": [["KitchenSink Test"]] },
        "content": [
          "f1366603-f22f-40cf-bbe4-dd48dc9a023c",
          "59ca4846-c2f5-4938-9c9f-6a85c175dec8",
          "ab37cc91-5cee-473e-a559-73effa5e8b7b",
          "63ebaf2b-de3b-415f-826c-0a842b3145f7",
          "87ab67ca-8919-4086-98ba-35714248d088"
        ],
    });

    let p: Page = serde_json::from_value(input)?;

    println!("do the p {:#?}", p);
    Ok(())
}
