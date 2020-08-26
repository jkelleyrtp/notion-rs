use serde::{Deserialize, Serialize};

/// Internal data for Notion Blocks
/// Note that every field might not be captured
#[serde(tag = "type", content = "properties", rename_all = "snake_case")]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum BlockData {
    Divider {},
    TableOfContents {},
    Breadcrumb {},
    Callout {
        title: Option<String>,
    },

    Header {
        title: Option<String>,
    },
    SubHeader {
        title: Option<String>,
    },
    SubSubHeader {
        title: Option<String>,
    },
    Quote {
        title: Option<String>,
    },
    Code {
        title: Option<String>,
        language: Option<String>,
    },
    ToDo {
        title: Option<String>,
    },
    Bookmark {
        title: Option<String>,
        link: Option<String>,
    },
    BulletedList {
        title: Option<String>,
    },
    Image {
        source: Option<String>,
        caption: Option<String>,
    },

    Page {
        title: Option<String>,
    },
    Toggle {
        title: Option<String>,
    },

    NumberedList {
        title: Option<String>,
    },

    Text {
        title: Option<String>,
    },

    Equation {
        title: Option<String>,
    },

    Factory {
        title: Option<String>,
    },

    ColumnList {},
    Column {},
    Video {
        caption: Option<String>,
        source: Option<String>,
    },
    // TODO
    CollectionView {},

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
            title: "HEADERITEM".to_string().into()
        }
    );

    let block_data: BlockData = serde_json::from_value(serde_json::json!({
        "type": "text",
        "properties": {
            "title": "HEADERITEM"
        }
    }))?;
    assert_eq!(
        block_data,
        BlockData::Header {
            title: "HEADERITEM".to_string().into()
        }
    );

    Ok(())
}
