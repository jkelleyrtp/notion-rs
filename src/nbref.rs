use crate::NotionClient;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
trait NBRef
where
    Self: Sized + Serialize,
{
    fn write(&self, client: &mut NotionClient) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Serialize)]
struct HeaderBlock {
    id: Uuid,
    title: String,
    code: String,
}
impl HeaderBlock {
    fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = title.to_string();
        self
    }

    fn set_code(&mut self, code: &str) -> &mut Self {
        self.code = code.to_string();
        self
    }
}
impl NBRef for HeaderBlock {}

/*
Goal is to provide mutable access to blocks where "sanctioned" mutations are okay

// We can save the ref of the block that we give out in an internal map
// When the client writes, it gives us the ref back
// From there, we can grab the metadata out of the block in our map
//
// Or, we can implement deref and deref_mut and return a smart pointer
let header = client.get_block("asdasd")?;

header
    .set_title("hello world!")
    .set_code("value")
    .write(client);


// For batching updates
update_all(vec![header]);
*/

fn example() -> anyhow::Result<()> {
    let mut header = HeaderBlock {
        id: Uuid::new_v4(),
        title: format!(""),
        code: format!(""),
    };

    let mut client = NotionClient::builder().build();

    header
        .set_code("asdasd")
        .set_title("asdasd")
        .write(&mut client)?;

    header
        .set_code("asdasd")
        .set_title("asdasd")
        .set_title("asdasd")
        .set_title("asdasd");

    Ok(())
}
