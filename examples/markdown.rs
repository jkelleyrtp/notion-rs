use notion_rs::{BlockData, NotionBlock, NotionClient, NotionQuery};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let page =
        "https://www.notion.so/jonathankelley/KitchenSink-Test-eb4923253d154dd5adf8a80d773acb15";

    let token = std::env::var("NOTION_TOKEN_V2")?;

    let mut client = NotionClient::builder().token_v2(token.as_str()).build();

    let response = client.get_page(page).await?;

    let mut explored_blocks = HashMap::<String, NotionBlock>::new();

    for (id, block) in response.record_map.block {
        explored_blocks.insert(id.clone(), block.clone());

        match block.data {
            BlockData::Page {} => {
                println!("{:?}", block.content);
                for subblok in block.content {
                    let query = NotionQuery::GetPage {
                        page_id: subblok.clone(),
                    };
                    let b = client.get_block(subblok).await?;
                    let out: Vec<String> = b.record_map.block.iter().map(|f| f.0.clone()).collect();
                    println!("Subpage is {:?}", out);
                }
            }
            _ => {}
        };
    }

    Ok(())
}
