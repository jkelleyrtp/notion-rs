//! Special methods for the various BlockType

use anyhow::Result;
use serde::{Deserialize, Serialize};
mod page;
use page::PageInfo;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum BlockType {
    Page(PageInfo),

    // text(NotionBlocks::text),

    // code(NotionBlocks::code),

    // numbered_list(NotionBlocks::numbered_list),

    // factory(NotionBlocks::factory),

    // equation(NotionBlocks::equation),

    // sub_header(NotionBlocks::sub_header),

    // quote(NotionBlocks::quote),

    // file(NotionBlocks::file),

    // bulleted_list(NotionBlocks::bulleted_list),

    // embed(NotionBlocks::embed),

    // callout(NotionBlocks::callout),

    // breadcrumb(NotionBlocks::breadcrumb),

    // bookmark(NotionBlocks::bookmark),

    // audio(NotionBlocks::audio),

    // divider(NotionBlocks::divider),

    // image(NotionBlocks::image),

    // toggle(NotionBlocks::toggle),

    // to_do(NotionBlocks::to_do),

    // table_of_contents(NotionBlocks::table_of_contents),

    // header(NotionBlocks::header),

    // sub_sub_header(NotionBlocks::sub_sub_header),
    unregistered(String),

    #[serde(other)]
    unknown,
}

impl Default for BlockType {
    fn default() -> Self {
        BlockType::unknown
    }
}
