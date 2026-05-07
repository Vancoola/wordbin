use reqwest::Client;
use std::sync::OnceLock;
use wordbin_types::{CreateWord, WordCreatedId};

static CLIENT: OnceLock<Client> = OnceLock::new();

fn client() -> &'static Client {
    CLIENT.get_or_init(Client::new)
}
pub async fn add_word(payload: CreateWord) -> anyhow::Result<WordCreatedId> {
    let res = client()
        .post("http://localhost:3000/word/add")
        .json(&payload)
        .send()
        .await?
        .json::<WordCreatedId>()
        .await?;

    Ok(res)
}
