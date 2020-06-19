use crate::block::NotionBlock;
use indexmap::IndexMap;

struct BlocksView<'a>(IndexMap<String, &'a mut NotionBlock>);

impl<'a> BlocksView<'a> {
    fn new() -> Self {
        BlocksView(IndexMap::new())
    }
}
