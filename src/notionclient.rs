// All query results are mapped to a collection of blocks
// Queries for individual blocks are still mapped to a collection but with length 1

use {
    reqwest,
    std::collections::HashMap,
    // mod CollectionMethods,
};

pub struct NotionClient {
    authToken: String,
    endpoint: String,
    client: reqwest::Client
}

enum RequestType {
    submitTransaction,
    queryCollection
}

impl NotionClient {
    fn new () -> NotionClient {
        // Generate a new NotionClient utilizing reqwests' client and a default api
        NotionClient {
            authToken: " ".to_string(),
            endpoint: "https://www.notion.so/api/v3/".to_string(),
            client: reqwest::Client::new()
        }
    }
    
    // Builders for the session client
    pub fn setAuthToken( &self, token: String ) -> &NotionClient {
        self.authToken = token;
        self
    }

    pub fn setEndpoint( &self, endpoint: String ) -> &NotionClient {
        self.endpoint = endpoint;
        self
    }



    pub fn query ( &self )-> Result<HashMap<String, String>, reqwest::Error> {
        let resp: HashMap<String, String> = reqwest::get("https://httpbin.org/ip")?
            .json()?;
        println!("{:#?}", resp);

        // Return the result of the query in json
        Ok(resp)
    }

    fn modify () {


    }
}

#[test]
fn query_test() {

}