use {
    notion_rs::{notionclient, query}
};




#[test]
fn test_client() {
    println!("aaaaaaaaaaaaaaaaaa");

    let session_client = notionclient::NotionClient::new()
        .set_auth_token("46459cd78367217b4f9d9219a401ea39eea71a6818b9c437f58ca8e06d68053975c3a05e0b3fb1c9d9f94eefc1adf94fc9d7ca21acb04daf0a8ccb075052173ca9ccf2671dba036aa647fead4b12")
        .set_endpoint("https://www.notion.so/api/v3/");

    println!("Session endpoint: {}", session_client.endpoint);

    println!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
    // let whats_new = client.query_collection();

    // let id = "157765353f2c4705bd45474e5ba8b46c";

}


fn main(){

}