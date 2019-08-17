// All query results are mapped to a collection of blocks
// Queries for individual blocks are still mapped to a collection but with length 1

use {
    reqwest,
    std::collections::{HashMap},
    // std::collections::
    std::str::FromStr,
    serde_derive,
    serde_json,
    // mod CollectionMethods,
};

pub struct NotionClient {
    auth_token: String,
    endpoint: String,
    client: reqwest::Client
}

pub enum RequestType {
    submitTransaction,
    queryCollection
}


enum ApiEndpoint {
    submitTransaction
}



struct NotionCollection {


}

struct NotionBlock {

}

impl NotionClient {
    fn new () -> NotionClient {
        // Generate a new NotionClient utilizing reqwests' client and a default api

        let token = " ".to_string();
        let endpoint = "https://www.notion.so/api/v3/".to_string();
        let client = reqwest::Client::new();

        let client = NotionClient {
            auth_token: token, 
            endpoint: endpoint,
            client: client,
        };

        client

    }
    
    // Builders for the session client
    pub fn set_auth_token<'a>( &'a mut self, token: &str ) -> &'a mut NotionClient {
        self.auth_token = token.to_string();
        self
    }

    pub fn set_endpoint<'a>(&'a mut self, endpoint: &str ) -> &'a mut NotionClient {
        self.endpoint = endpoint.to_string();
        self
    }

    pub fn say_hi<'a>(&'a mut self) ->  &'a mut NotionClient {
        println!("hello !");
        self
    }


    pub fn query_collection(&self, collection_id: &str ) 
            // -> Result<HashMap<String, String>, reqwest::Error>{
            -> Result<NotionCollection, reqwest::Error> {
        
        let request_body = serde_json::json!({
            "collectionId": "11fc26f0-b132-4728-ae03-5826ee0a78bf",
            "collectionId": "2daf2cdb-a859-4ba4-8e71-a85e5b5cb4e8",
            "query": {
                    "filter_operator":"and",
                    "filter":[],
                    "sort":[],
                    "aggregate":[]
                },
            "loader":{
                "type":"table",
                "limit":70,
                "userTimeZone":"America/Los_Angeles",
                "userLocale":"en",
                "loadContentCover": true
               }
        });
        
        let resp = &self.client.post( 
            &[&self.endpoint.clone(), "queryCollection"].concat() 
        )
        .json(&request_body)
        .bearer_auth(&self.auth_token)
        .send()?;

        // let gist: HashMap<String, String> = resp.text()?;
        let gist = resp.text()?;

        println!("Response: {:?}", &gist);
        Ok(NotionCollection{})

    }


    // pub fn query ( &self )-> Result<HashMap<String, String>, reqwest::Error> {
    //     let resp: HashMap<String, String> = reqwest::get(&self.endpoint)?
    //         .json()?;
    //     println!("{:#?}", resp);

    //     // Return the result of the query in json
    //     // Ok(resp)

    //     // match Ok(resp) {
    //     //     HashMap => println!("hello world!"),
    //     //     _ => ()
    //     // }
    //     Ok()

    // }

    fn modify () {


    }
}



#[test]
fn query_test() {
    // let client = NotionClient::new()
    //     .setAuthToken(token: String)
    println!("aaaaaaaaaaaaaaaaaa");

    let session_client = NotionClient::new()
        .say_hi();
        // .set_endpoint("https://www.notion.so/api/v3/");
        // .set_auth_token("46459cd78367217b4f9d9219a401ea39eea71a6818b9c437f58ca8e06d68053975c3a05e0b3fb1c9d9f94eefc1adf94fc9d7ca21acb04daf0a8ccb075052173ca9ccf2671dba036aa647fead4b12")

    // println!("Session endpoint: {}", &session_client.endpoint);

    println!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
    // let whats_new = client.query_collection();

}