// All query results are mapped to a collection of blocks
// Queries for individual blocks are still mapped to a collection but with length 1

use {
    crate::cache,
    crate::prelude::{BlockType, NotionBlock},
    reqwest, serde,
    serde::{Deserialize, Serialize},
    serde_json,
    serde_json::json,
    std::collections::HashMap,
};
// use crate::ac
pub struct NotionClient {
    // User set-ables
    /// token_v2 found by inspecting browser cookies on a logged-in session of Notion
    /// Be careful to not let this leak out into the world
    auth_token: String,

    /// Should the notionclient maintain a record store for faster indexing?
    /// Required for monitoring updates on a workspace
    enable_caching: bool,

    /// ID of the workspace the notion client uses to do things like monitoring and searching
    workspace_id: Option<String>,

    /// Should the client autodetect the current workspace from blocks its currently working with?
    auto_detect_ws: bool,

    // Mechanical items for doing things
    client: reqwest::Client,
    cache: cache::RecordCache,

    // Defaults for notion that change with each version
    // Override these if the rust library is out of date with the actual endpoints
    api_url: String,
    base_url: String,

    signed_url_prefix: String,
    s3_url_prefix: String,
    s3_url_prefix_encoded: String,
}

pub enum API_Endpoints {
    base,
    loadPageChunk,
    loadUserContent,
}
impl ToString for API_Endpoints {
    fn to_string(&self) -> String {
        use API_Endpoints::*;
        match self {
            base => "https://www.notion.so/api/v3/".to_string(),
            loadPageChunk => "https://www.notion.so/api/v3/loadPageChunk".to_string(),
            loadUserContent => "https://www.notion.so/api/v3/loadUserContent".to_string(),
        }
    }
}

impl NotionClient {
    pub fn from_config() {}

    pub fn new<S>(token: S) -> NotionClient
    where
        S: Into<String>,
    {
        // Start a new client to make requests to the notion api

        let mut client = reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();

        NotionClient {
            auth_token: token.into(),
            enable_caching: true,
            workspace_id: Some("".to_string()),
            auto_detect_ws: true,

            client: client,
            cache: cache::RecordCache::default(),

            // api_url: "https://www.notion.so/api/v3/".to_string(),
            api_url: "https://www.notion.so/api/v3/".to_string(),
            base_url: "https://www.notion.so".to_string(),
            signed_url_prefix: "https://www.notion.so/signed/".to_string(),
            s3_url_prefix: "https://s3-us-west-2.amazonaws.com/secure.notion-static.com/"
                .to_string(),
            s3_url_prefix_encoded: "https://s3.us-west-2.amazonaws.com/secure.notion-static.com/"
                .to_string(),
        }
    }

    pub fn post(
        &mut self,
        endpoint: API_Endpoints,
        data: serde_json::value::Value,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let req = self
            .client
            .post(endpoint.to_string().as_str())
            .json(&data)
            .header(
                reqwest::header::COOKIE,
                format!("token_v2={}", self.auth_token),
            );

        println!("my request: {:?}", &req);
        req.send()
    }

    fn search_blocks(&mut self, search: String, limit: i32) {
        let data = json! ({
            "query": search,
            "table": "space",
            "id": "",
            "limit": limit
        });
    }

    /// Set's the client's workspace ID based on the blocks it's processing
    fn set_workspace_from_block() {}

    fn get_id_from_url(&self) -> String {
        "".to_string()
    }

    fn get_block(&mut self, page_id: String) -> reqwest::Response {
        let data = json!({
            "pageId": page_id,
            "limit": 100000,
            "cursor": {"stack": []},
            "chunkNumber": 0,
            "verticalColumns": false,
        });

        let res = self.post(API_Endpoints::loadPageChunk, data).unwrap();
        res
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct blocks_notion {
    pub recordMap: serde_json::Value,
}

#[test]
fn test_notion_client() {
    // Make a notion client and download the "Whats new page"
    // https://www.notion.so/What-s-New-157765353f2c4705bd45474e5ba8b46c

    let mut client = NotionClient::new("32308c9cf7bcf50c6a5f2d248198bc870c16bcd582acc0521411fd0c99932bf8dc899e1792ea633b38a817aace32eeab0069f9063b04592b242ba1fafefa36d76e302b780e30c0ab351bb02ead1b");

    let mut _res = client.post(API_Endpoints::loadUserContent, json!({}));

    // https://www.notion.so/jonathankelley/The-title-has-now-changed-and-has-live-updated-in-the-browser-46f9f915482a43adb0ca7ca4005255d9
    // https://www.notion.so/jonathankelley/The-title-has-now-changed-and-has-live-updated-in-the-browser-46f9f915482a43adb0ca7ca4005255d9
    let mut res = client.get_block("46f9f915-482a-43ad-b0ca-7ca4005255d9".to_string());

    // 46f9f915-482a-43ad-b0ca-7ca4005255d9
    // 46f9f915-482a-43ad-b0ca-7ca4005255d9
    let my_blocks = res
        .json::<blocks_notion>()
        .expect("Couldnt unwrap my blocks");
    let mut true_blocks = Vec::<NotionBlock>::new();

    // let m: HashMap<String, String> = my_blocks.recordMap.get("block").unwrap().as_object().unwrap();

    // println!("Response: {:?}", );
    // println!(
    //     "Here is my blocks: {:}",
    //     my_blocks.recordMap
    //         .get("block")
    //         .unwrap()
    // );
    for (record_key, record_value) in my_blocks
        .recordMap
        .get("block")
        .expect("can't get block")
        .as_object()
        .expect("can't get object from block")
    {
        // println!("Here is the record: \n{}", serde_json::to_string_pretty(record_value).unwrap());
        // println!("Here is the record: \n{:?}", record_value);
        // .unwrap().get("properties")
        if let Some(props) = record_value.get("value") {
            println!("Here is the record: \n{:?}", record_value);
            let mut new_block: NotionBlock =
                serde_json::from_value(props.clone()).expect("Can't convert sredejson to newblock");

            if new_block.block_type == BlockType::unknown {
                new_block.block_type = BlockType::unregistered(
                    props
                        .get("type")
                        .expect("No prop type")
                        .as_str()
                        .expect("can't get string")
                        .to_string(),
                );
            }
            true_blocks.push(new_block);

            // println!("Record title: {:}", record_value.get("properties"));
            // println!("Record title: {:}", props);
        }
    }

    let mut found_block_types = std::collections::HashSet::<String>::new();
    for block in true_blocks {
        println!("Processed blocks {:?}\n", block);
        match block.block_type {
            BlockType::unregistered(val) => {
                found_block_types.insert(val);
                // println!("Unknown value found: {:?}", val);
            }
            _ => {}
        }
        // block_types.insert(block.block_type);
    }

    println!("All my block types: {:?}", found_block_types);
    // api_url: "https://www.notion.so/api/v3/loadPageChunk".to_string(),
    // https://www.notion.so/api/v3/loadPageChunk {'pageId': '46f9f915-482a-43ad-b0ca-7ca4005255d9', 'limit': 100000, 'cursor': {'stack': []}, 'chunkNumber': 0, 'verticalColumns': False}
    // {
    //     Response: Response { url: "https://www.notion.so/api/v3/loadPageChunk", status: 200, headers: {"date": "Tue, 18 Feb 2020 07:00:31 GMT", "content-type": "application/json; charset=utf-8", "content-length": "38", "connection": "keep-alive", "set-cookie": "__cfduid=d277d99b153c3477e21f8f8e2c2bb1fe31582009231; expires=Thu, 19-Mar-20 07:00:31 GMT; path=/; domain=.notion.so; HttpOnly; SameSite=Lax", "set-cookie": "notion_browser_id=0dde7a90-8a8b-4f4f-96e1-f3594f7bef05; Domain=www.notion.so; Path=/; Expires=Fri, 27 Oct 2051 08:47:11 GMT; Secure", "x-dns-prefetch-control": "off", "x-frame-options": "SAMEORIGIN", "strict-transport-security": "max-age=5184000; includeSubDomains", "x-download-options": "noopen", "x-content-type-options": "nosniff", "x-xss-protection": "1; mode=block", "referrer-policy": "same-origin", "content-security-policy": "script-src 'self' 'unsafe-inline' 'unsafe-eval' https://gist.github.com https://apis.google.com https://api.amplitude.com https://widget.intercom.io https://js.intercomcdn.com https://logs-01.loggly.com https://cdn.segment.com https://checkout.stripe.com https://embed.typeform.com https://admin.typeform.com https://platform.twitter.com https://cdn.syndication.twimg.com; connect-src 'self' https://msgstore.www.notion.so wss://msgstore.www.notion.so https://s3-us-west-2.amazonaws.com https://s3.us-west-2.amazonaws.com https://notion-production-snapshots-2.s3.us-west-2.amazonaws.com https: http: https://api.amplitude.com https://api.embed.ly https://js.intercomcdn.com https://api-iam.intercom.io wss://nexus-websocket-a.intercom.io https://logs-01.loggly.com https://api.segment.io https://checkout.stripe.com https://cdn.contentful.com https://images.ctfassets.net https://api.unsplash.com; font-src 'self' data: https://cdnjs.cloudflare.com https://js.intercomcdn.com; img-src 'self' data: blob: https: https://platform.twitter.com https://syndication.twitter.com https://pbs.twimg.com https://ton.twimg.com; style-src 'self' 'unsafe-inline' https://cdnjs.cloudflare.com https://github.githubassets.com https://platform.twitter.com https://ton.twimg.com; frame-src https: http:; media-src https: http:", "x-content-security-policy": "script-src 'self' 'unsafe-inline' 'unsafe-eval' https://gist.github.com https://apis.google.com https://api.amplitude.com https://widget.intercom.io https://js.intercomcdn.com https://logs-01.loggly.com https://cdn.segment.com https://checkout.stripe.com https://embed.typeform.com https://admin.typeform.com https://platform.twitter.com https://cdn.syndication.twimg.com; connect-src 'self' https://msgstore.www.notion.so wss://msgstore.www.notion.so https://s3-us-west-2.amazonaws.com https://s3.us-west-2.amazonaws.com https://notion-production-snapshots-2.s3.us-west-2.amazonaws.com https: http: https://api.amplitude.com https://api.embed.ly https://js.intercomcdn.com https://api-iam.intercom.io wss://nexus-websocket-a.intercom.io https://logs-01.loggly.com https://api.segment.io https://checkout.stripe.com https://cdn.contentful.com https://images.ctfassets.net https://api.unsplash.com; font-src 'self' data: https://cdnjs.cloudflare.com https://js.intercomcdn.com; img-src 'self' data: blob: https: https://platform.twitter.com https://syndication.twitter.com https://pbs.twimg.com https://ton.twimg.com; style-src 'self' 'unsafe-inline' https://cdnjs.cloudflare.com https://github.githubassets.com https://platform.twitter.com https://ton.twimg.com; frame-src https: http:; media-src https: http:", "x-webkit-csp": "script-src 'self' 'unsafe-inline' 'unsafe-eval' https://gist.github.com https://apis.google.com https://api.amplitude.com https://widget.intercom.io https://js.intercomcdn.com https://logs-01.loggly.com https://cdn.segment.com https://checkout.stripe.com https://embed.typeform.com https://admin.typeform.com https://platform.twitter.com https://cdn.syndication.twimg.com; connect-src 'self' https://msgstore.www.notion.so wss://msgstore.www.notion.so https://s3-us-west-2.amazonaws.com https://s3.us-west-2.amazonaws.com https://notion-production-snapshots-2.s3.us-west-2.amazonaws.com https: http: https://api.amplitude.com https://api.embed.ly https://js.intercomcdn.com https://api-iam.intercom.io wss://nexus-websocket-a.intercom.io https://logs-01.loggly.com https://api.segment.io https://checkout.stripe.com https://cdn.contentful.com https://images.ctfassets.net https://api.unsplash.com; font-src 'self' data: https://cdnjs.cloudflare.com https://js.intercomcdn.com; img-src 'self' data: blob: https: https://platform.twitter.com https://syndication.twitter.com https://pbs.twimg.com https://ton.twimg.com; style-src 'self' 'unsafe-inline' https://cdnjs.cloudflare.com https://github.githubassets.com https://platform.twitter.com https://ton.twimg.com; frame-src https: http:; media-src https: http:", "etag": "W/\"26-Rn6C0EDYlwi44ao9ByNx942VLgA\"", "vary": "Accept-Encoding", "cf-cache-status": "DYNAMIC", "expect-ct": "max-age=604800, report-uri=\"https://report-uri.cloudflare.com/cdn-cgi/beacon/expect-ct\"", "server": "cloudflare", "cf-ray": "566e1fdf8dc3e6bc-EWR"} }
    // }
}
