# Notion-rs

A Rust and Typescript implementation of the Notion.so unofficial API. The goal is to provide high quality, fast, and portable bindings to the API as a basis for complex notion integrations.

For portability, we're aiming to support whatever Rust can bind to, especially WASM so the API can be embedded in a web page.

## Usage

Rust:
```rust
use notion_rs::NotionClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let link = "https://www.notion.so/157765353f2c4705bd45474e5ba8b46c";

    let token = std::env::var("NOTION_TOKEN_V2").unwrap();

    let mut client = NotionClient::builder().token_v2(token.as_str()).build();

    let response = client.get_page(link).await?;

    println!("{:#?}", response.record_map.block);

    Ok(())
}
```

Typescript:
```typescript
let client = new NotionClient(token_v2);

let page = client.getPage("https://www.notion.so/157765353f2c4705bd45474e5ba8b46c");

for (const block of page.blocks) {
    console.log(block.title);
}
```



## Example

```rust
// Configuration is stored in a notions.toml file
// Make sure to ignore the contents so your keys don't get leaked
let mut client = NotionClient::from_file("notion.toml")?;


// Label all equations on a page!
client
    .get_page("...")
    .await?
    .blocks
    .iter()
    .filter(|(id, block)| block.block_type == BlockTypes::equation)
    .enumerate()
    .for_each(|(eqnID, (id, block))| {
        // Will do it's best to set the text value
        // Is failable, though you can choose to ignore the error;
        block.set_title()?;
        block.set_title().ok_or(());
    }); 

// Batch changes together before writing
client.write_all().await?;
```

## Support
Current support is:

| Block        | Read  | Write |
| ------------ | :---: | ----: |
| Header       |   ☑️   |       |
| SubHeader    |   ☑️   |       |
| SubSubHeader |   ☑️   |       |
| Quote        |   ☑️   |       |
| Code         |   ☑️   |       |
| ToDo         |   ☑️   |       |
| Bookmark     |   ☑️   |       |
| BulletedList |   ☑️   |       |
| Image        |   ☑️   |       |
| Divider      |   ☑️   |       |
| ToC          |   ☑️   |       |
| Breadcrumb   |   ☑️   |       |
| Page         |   ☑️   |       |
| NumberedList |   ☑️   |       |
| Text         |   ☑️   |       |
| Equation     |   ☑️   |       |
| Toggle       |   ☑️   |       |
| Callout      |   ☑️   |       |
| Factory      |   ☑️   |       |
| Collection   |   🛠   |       |

`☑️ = fully supported`
`🛠 = partially supported`
