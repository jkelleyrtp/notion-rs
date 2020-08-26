use serde::{Deserialize, Serialize};

/// Internal data for Notion Blocks
/// Note that every field might not be captured
#[serde(tag = "type", content = "properties", rename_all = "snake_case")]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum BlockData {
    Divider {},
    TableOfContents {},
    Breadcrumb {},
    Callout {},

    Header {},
    SubHeader {},
    SubSubHeader {},
    Quote {},
    Code {
        language: Option<String>,
    },
    ToDo {},
    Bookmark {
        link: Option<String>,
    },
    BulletedList {},
    Image {
        source: Option<String>,
        caption: Option<String>,
    },

    Page {},
    Toggle {},

    NumberedList {},

    Text {},

    Equation {},

    Factory {},

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
