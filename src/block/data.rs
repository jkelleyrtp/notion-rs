use serde::{Deserialize, Serialize};

/// Internal data for Notion Blocks
/// Note that every field might not be captured
#[serde(tag = "type", content = "properties", rename_all = "snake_case")]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum BlockData {
    Divider {},
    TableOfContents {},
    Breadcrumb {},
    Callout {},
    Header {},
    SubHeader {},
    SubSubHeader {},
    Quote {},
    ToDo {},
    BulletedList {},
    Page {},
    Toggle {},
    NumberedList {},
    Text {},
    Equation {},
    Factory {},
    ColumnList {},
    Column {},
    Code {
        language: Option<String>,
    },
    Bookmark {
        link: Option<String>,
    },
    Image {
        source: Option<String>,
        caption: Option<String>,
    },
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
    assert_eq!(block_data, BlockData::Header {});

    let block_data: BlockData = serde_json::from_value(serde_json::json!({
        "type": "code",
        "properties": {
            "title": "CODEITEM",
            "language": "Python"
        }
    }))?;
    assert_eq!(
        block_data,
        BlockData::Code {
            language: "Python".to_string().into()
        }
    );

    Ok(())
}
