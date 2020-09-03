mod block;
mod cfg;
mod client;
mod query;
mod util;

/**
Read
----
Blocks enum
Page Query enum
Database query enum
Blocks cache

Write
---
??
*/
pub use crate::{
    block::{BlockData, NotionBlock},
    cfg::ClientConfig,
    cfg::NotionEndpoint,
    client::NotionClient,
    query::NotionQuery,
};
