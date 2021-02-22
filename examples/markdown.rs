use notion_rs::{BlockData, NotionBlock, NotionClient};
use std::collections::HashMap;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let page =
        "https://www.notion.so/jonathankelley/KitchenSink-Test-eb4923253d154dd5adf8a80d773acb15";

    // Don't commit your token to git!
    let token = std::env::var("NOTION_TOKEN_V2")?;

    let mut client = NotionClient::builder(token.as_str()).build();

    let page_blocks = client.get_page(page).await?;

    let mut explored_blocks = HashMap::<String, NotionBlock>::new();

    for (id, block) in page_blocks {
        explored_blocks.insert(id.clone(), block.clone());

        match block.data {
            BlockData::Page {} => {
                println!("{:?}", block.content);
                for sub_block in block.content {
                    let out: Vec<String> = client
                        .get_block(sub_block)
                        .await?
                        .iter()
                        .map(|f| f.0.clone())
                        .collect();

                    println!("Subpage is {:?}", out);
                }
            }
            _ => {}
        };
    }

    Ok(())
}
