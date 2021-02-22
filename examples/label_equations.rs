//! Label all the equations on a Notion page

static PAGE: &'static str =
    "https://www.notion.so/jonathankelley/KitchenSink-Test-eb4923253d154dd5adf8a80d773acb15";

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    // Don't commit your token to git!
    let token = std::env::var("NOTION_TOKEN_V2")?;

    let mut client = notion_rs::builder(token.as_str()).build();
    let myblocks = client.get_page(PAGE).await?;

    println!("{}", serde_json::to_string_pretty(&myblocks).unwrap());

    Ok(())
}
