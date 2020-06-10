# Notion-rs

A wasm-friendly rust implementation of the Notion.so unofficial API.

Give an auth token and a 







## Not yet supported:
----
Delievers a Graph-QL like experience using Queries and Mutations.


Post is called several times:
- loadUserContent
- submitTransaction
- searchPages
- searchBlocks


Question:

Should it be:
structs vs enums
enum as a field
enum as the type


trait block {
    get_id()
}

struct block_info {

}

struct VideoBlock {
    block_info: block_info
    id: String,   
}

impl block for VideoBlock {
    block.get_info() { self.block_info }
}


TODO:
- [x] Get individual block
- [ ] Coerce block into a block struct
- [ ] Add block to cache
- [ ] Enable graph network for cache
- [ ] Save images/content to dir
- [ ] Consider trait/enum system for types of blocks



Roadmap:
- [ ] Caching requests
- [ ] Subscribe to changes 
- [ ] Use in npm

- [ ] Graph-QL like usage (using the Graph-QL Rust Client)

