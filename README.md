# notion-rs

A wasm-friendly rust implementation of the Notion.so unofficial API.

Generate complex queries of your notion workspace!

```rust
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
        // Will do it's best to set the text value
        // Is failable, though you can choose to ignore the error;
        block.set_title()?;
        block.set_title().ok_or(());
    }); 

// Updates blocks that have been changed via methods
client
    .update()
    .await?;
```


## Mutations
Very basic mutation support at the moment, currently done by mutating a block in an ownership scope;

```rust
client.modify("f1366603-f22f-40cf-bbe4-dd48dc9a023c", |block| {
    match &mut block.data {
        BlockData::Header(header) => {
            header.title = "New header!";
        }
    }
    block
})
```

Aiming for something more along the lines of:
```rust
let mut myblock = client.get_mut::<HeaderBlock>("f1366603-f22f-40cf-bbe4-dd48dc9a023c")?
```




## Support
Current support is:

| Block        | Read  | Write |
| ------------ | :---: | ----: |
| Header       |   x   |       |
| SubHeader    |   x   |       |
| SubSubHeader |   x   |       |
| Quote        |   x   |       |
| Code         |   x   |       |
| ToDo         |   x   |       |
| Bookmark     |   x   |       |
| BulletedList |   x   |       |
| Image        |   x   |       |
| Divider      |   x   |       |
| ToC          |   x   |       |
| Breadcrumb   |   x   |       |
| Page         |   x   |       |
| NumberedList |   x   |       |
| Text         |   o   |       |
| Equation     |   o   |       |
| Toggle       |   o   |       |
| Callout      |   o   |       |
| Factory      |   o   |       |
| Collection   |   o   |       |

`x = fully supported`
`o = partially supported`
