// // A demo of the notion API
// extern crate notion_rs;
// use notion_rs::prelude::{
//     NotionClientBuilder
// };

// fn main () {
//     let matches = clap::App::new("Notion-rs")
//                                         .version("0.1.0")
//                                         .about("Demo of the Rust Notion.so Client")
//                                         .arg(clap::Arg::with_name("token")
//                                             .short("t")
//                                             .long("token")
//                                             .help("Copy the v2_auth token from your cookies")
//                                             .takes_value(true))
//                                         .get_matches();

//     let token = matches.value_of("token")
//         .expect("No token given, try again by passing ` -t <your_token> `");

//     let client = NotionClientBuilder::default()
//         .auth_token(token.into())
//         .build()
//         .expect("Couldn't properly build a client for Notion");

//     // cv = client.get_collec

// }

fn main() {}
