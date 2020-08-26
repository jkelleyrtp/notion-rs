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
