//! Special methods for the various BlockType

use anyhow::Result;
use serde::{Deserialize, Serialize};
mod page;
use page::Page;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum BlockType {
    code,
    to_do,
    sub_sub_header,
    bookmark,
    bulleted_list,
    factory,
    image,
    divider,
    page,
    table_of_contents,
    sub_header,
    numbered_list,
    equation,
    text,
    breadcrumb,
    toggle,
    callout,
    header,
    quote,

    Unregistered(String),

    // #[serde(other)]
    Unknown,
}

impl BlockType {
    pub fn from_props(fieldtype: &str, properties: Option<serde_json::Value>) -> Result<Self> {
        let a = match fieldtype {
            "page" => BlockType::page,
            "code" => BlockType::code,
            "to_do" => BlockType::to_do,
            "sub_sub_header" => BlockType::sub_sub_header,
            "bookmark" => BlockType::bookmark,
            "bulleted_list" => BlockType::bulleted_list,
            "factory" => BlockType::factory,
            "image" => BlockType::image,
            "divider" => BlockType::divider,
            "table_of_contents" => BlockType::table_of_contents,
            "sub_header" => BlockType::sub_header,
            "numbered_list" => BlockType::numbered_list,
            "equation" => BlockType::equation,
            "text" => BlockType::text,
            "breadcrumb" => BlockType::breadcrumb,
            "toggle" => BlockType::toggle,
            "callout" => BlockType::callout,
            "header" => BlockType::header,
            "quote" => BlockType::quote,
            _ => BlockType::Unknown,
        };

        Ok(a)
    }
}

impl Default for BlockType {
    fn default() -> Self {
        BlockType::Unknown
    }
}

// "code",
// "to_do",
// "sub_sub_header",
// "bookmark",
// "bulleted_list",
// "factory",
// "image",
// "divider",
// "page",
// "table_of_contents",
// "sub_header",
// "numbered_list",
// "equation",
// "text",
// "breadcrumb",
// "toggle",
// "callout",
// "header",
// "quote",
