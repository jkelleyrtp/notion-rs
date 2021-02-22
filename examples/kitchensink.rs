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
