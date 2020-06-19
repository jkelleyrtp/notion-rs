//! A query to make to the notion client
use crate::client::NotionClient;
use crate::util::GetBlocksResponse;
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[async_trait(?Send)]
pub trait NotionQuery {
    fn get_client_link(&self) -> Option<Arc<RwLock<NotionClient>>>;

    async fn post(self) -> reqwest::Response
    where
        Self: Sized,
    {
        let link = self.get_client_link().context("").unwrap();
        let mut b = link.write().unwrap();
        b.post_query(self).await.unwrap()
    }
}

pub mod NotionQueryImpls {
    use super::*;

    /// Start a new GetBlock query
    pub fn get_block(block_id: Uuid) -> Result<GetBlock> {
        Ok(GetBlock { block_id })
    }

    /// Start a new GetPage query
    pub fn get_page(url: &str) -> Result<GetPage> {
        let id = url.rsplit('-').next().context("Oh no!")?;
        let page_id = uuid::Uuid::parse_str(id)?;

        Ok(GetPage {
            page_url: url.to_string(),
            page_id,
        })
    }

    pub struct GetBlock {
        block_id: Uuid,
    }

    pub struct GetPage {
        page_url: String,
        page_id: Uuid,
    }

    impl GetPage {
        fn url() {}
    }
}

mod tests {
    #[test]
    fn test_query() {
        // let data = json!({
        //     "collectionId": collection_id.to_string(),
        //     "collectionViewId": "",
        //     "loader": {
        //         "limit": 10000,
        //         "loadContentCover": true,
        //         "query": "",
        //         "userLocale": "en",
        //         "userTimeZone": "",
        //         "type": "type",
        //     },
        //     "query": {
        //         "aggregate": "",
        //         "filter": "",
        //         "filter_operator": "",
        //         "sort": "",
        //     },
        // });
        // let data = json!({
        //     "pageId": page_id.to_hyphenated().to_string(),
        //     "limit": 100000,
        //     "cursor": {"stack": []},
        //     "chunkNumber": 0,
        //     "verticalColumns": false,
        // });
    }
}
