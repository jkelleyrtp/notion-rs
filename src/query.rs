use {
    super::notionclient,
    std::collections::HashMap
};

enum CollectionView {
    Table,
    Calendar,
    List,
    Board
}

enum ErrorType {
    is_private,
    invalid_id,
    none
}

pub struct Collection {
    blocks: HashMap<String, String>,
    view_type: CollectionView,
    error_type: ErrorType
}




fn get_blocks_from_id(client: notionclient::NotionClient, id: String) -> Collection {
    let resp = client::query();

    // match client::


    // let resp: HashMap<String, String> = reqwest::get("https://httpbin.org/ip")

    Collection {


    }


}