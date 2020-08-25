use super::{
    equation::{deserialize_notion_equation, NotionEquation},
    text::deserialize_notion_text,
    text::NotionText,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[serde(tag = "type", content = "properties", rename_all = "snake_case")]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum BlockData {
    Header {
        title: String,
    },
    SubHeader {
        title: String,
    },
    SubSubHeader {
        title: String,
    },
    Quote {
        title: String,
    },
    Code {
        title: String,
        language: String,
    },
    ToDo {
        title: String,
    },
    Bookmark {
        title: String,
        link: String,
    },
    BulletedList {
        title: String,
    },
    Image {
        source: String,
        caption: String,
    },
    Divider,
    TableOfContents,
    Breadcrumb,
    Page {
        title: String,
        content: Vec<Uuid>,
    },

    // TODO
    NumberedList {
        title: String,
    },

    #[serde(deserialize_with = "deserialize_notion_text")]
    Text(NotionText),

    #[serde(deserialize_with = "deserialize_notion_equation")]
    Equation(NotionEquation),

    // TODO
    Toggle,

    // TODO
    Callout,

    // TODO
    Factory,

    // TODO
    CollectionView,

    #[serde(other)]
    Unknown,
}

#[test]
fn test_adjacently_tagged() -> anyhow::Result<()> {
    let block_data: BlockData = serde_json::from_value(serde_json::json!({
        "type": "header",
        "properties": {
            "title": "HEADERITEM"
        }
    }))?;
    assert_eq!(
        block_data,
        BlockData::Header {
            title: "HEADERITEM".to_string()
        }
    );
    Ok(())
}
