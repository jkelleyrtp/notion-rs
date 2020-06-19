# notion-rs

A wasm-friendly rust implementation of the Notion.so unofficial API.



Generate complex queries of your notion workspace!



```rust
type BlocksView = IndexMap<Uuid, &mut NotionBlock>;

// Configuration is stored in a notions.toml file
// Make sure to ignore the contents so your keys don't get leaked
let client = NotionClient::from_file("notion.toml")?;

// The blocksview is a collection of references to notionblocks
// You cannot own a notion block, but you can get a mutable reference to it
// You can get a mutable reference to a notion block
// You can sync modifcation to a notion block 
let myblocks: BlocksView = client
                            .query_builder()
                            .collection_view("12345") // collection query
                            .view_id("123")
                            .filter_column("bill", TagFilter::Matches(""))
                            .filter_column("date", TagFilter::Matches(""))
                            .post()
                            .await?;
            
// Label all equations on a page!
myblocks
    .iter()
    .filter(|(id, block)| block.block_type == BlockTypes::equation)
    .enumerate()
    .for_each(|(eqnID, (id, block))| {
        match block.block_type {
            BlockTypes::Equation(eqn) =>{
                // Automatically sends a "needs update" flag to the client
                eqn.change_text(|f: String| format!("{:} {:}", f, eqnID) )
            }
            _ => {}
        }
    }); 

// Updates blocks that have been changed via methods
client
    .update()
    .await?;
```


