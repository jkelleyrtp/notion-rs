use crate::prelude::NotionBlock;
use petgraph::Graph;
use std::collections::HashMap;
use std::time;
// use crate::

pub struct RecordCache {
    records: HashMap<String, NotionBlock>,
}

// /// The record cache that stores information requested by the API
// /// Is owned by the client
// pub struct RecordCache {
//     records: Graph<String, NotionBlock>,
// }

// impl RecordCache {
//     fn new() -> RecordCache {
//         let my_graph: Graph<String, NotionBlock> = Graph::new();
//         RecordCache { records: my_graph }
//     }
// }

// impl Default for RecordCache {
//     fn default() -> RecordCache {
//         let my_graph: Graph<String, NotionBlock> = Graph::new();
//         RecordCache { records: my_graph }
//     }
// }

// struct Record {
//     last_updated: time::Instant,
//     blockid: String,
// }
