# Notion-rs

A Rust implementation of the unofficial Notion.so API. This crate provides high quality and portable bindings to the API as a basis for complex notion integrations.

```rust
#[async_std::main]
async fn main() {
    // Don't commit your token to git!
    let token = std::env::var("NOTION_TOKEN_V2").unwrap();

    let mut client = notion_rs::builder(token.as_str()).build();

    let blocks = client
        .get_page("https://www.notion.so/157765353f2c4705bd45474e5ba8b46c")
        .await
        .unwrap();

    println!("{:#?}", blocks);
}
```

This crate provides query builders, a reqwest-based client, and the ability to write changes back to Notion.so. With a built-in diffing mechanism, notion-rs allows you to download block data, modify it in place, and commit the changes back to the notion database.

This crate can be used with and without the `reqwest` client, in case you'd like to use an alternate HTTP client or don't want to bundle two versions of the client together. For those use cases, we provide a query builder which can be integrated with surf and reqwest.

As such, this crate works in a WASM environment, and will happily run paired with a WASM web framework. Take note that the underlying method calls are async, so you'll need to provide your own async runtime or use you own synchronous web client.

## Typescript Bindings

In addition to the Rust client, we also provide a Typescript client via WASM bindings. If you need native bindings, then this crate will not work for you. The WASM binary has been stripped down to be leaner, but will still be heftier than something truly native to Typescript.

Typescript:
```typescript
let client = new NotionClient(token_v2);

let page = client.getPage("https://www.notion.so/157765353f2c4705bd45474e5ba8b46c");

for (const block of page.blocks) {
    console.log(block.title);
}
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
