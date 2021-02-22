//! A query to make to the notion client
use crate::error::Result;
use serde_json::json;
use uuid::Uuid;

pub enum NotionQuery {
    GetPage { page_id: Uuid },
    GetBlock { block_id: Uuid },
    GetCollection { collection_id: Uuid },
}

impl NotionQuery {
    /// Returns the contents of a notion page
    pub fn from_url(url: &str) -> Result<Self> {
        Ok(NotionQuery::GetPage {
            page_id: link_to_uuid(url)?,
        })
    }

    pub fn to_data(self) -> serde_json::Value {
        use NotionQuery::*;
        match self {
            GetCollection { collection_id } => json!({
                "collectionId": collection_id.to_string(),
                "collectionViewId": "",
                "loader": {
                    "limit": 10000,
                    "loadContentCover": true,
                    "query": "",
                    "userLocale": "en",
                    "userTimeZone": "",
                    "type": "type",
                },
                "query": {
                    "aggregate": "",
                    "filter": "",
                    "filter_operator": "",
                    "sort": "",
                },
            }),

            GetBlock { block_id } => json!({
                "pageId": block_id.to_hyphenated().to_string(),
                "limit": 100000,
                "cursor": {"stack": []},
                "chunkNumber": 0,
                "verticalColumns": false,
            }),

            GetPage { page_id } => json!({
                "pageId": page_id.to_hyphenated().to_string(),
                "limit": 100000,
                "cursor": {"stack": []},
                "chunkNumber": 0,
                "verticalColumns": false,
            }),
        }
    }
}

fn link_to_uuid(url: &str) -> Result<Uuid> {
    let chunks = url.split("/");
    let slug = chunks.last().unwrap();
    let id = slug.split("-").last().unwrap();

    // TODO @Jon remap the error to the crate's error type
    Ok(Uuid::parse_str(id)?)
}
