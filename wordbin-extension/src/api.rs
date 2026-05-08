use crate::settings;
use crate::settings::base_url;
use reqwest::Client;
use std::sync::OnceLock;
use wordbin_types::{CreateWord, WordCreatedId};

static CLIENT: OnceLock<Client> = OnceLock::new();

fn client() -> &'static Client {
    CLIENT.get_or_init(Client::new)
}
pub async fn add_word(payload: CreateWord) -> anyhow::Result<WordCreatedId> {
    let res = client()
        .post(format!("{}/word/add", base_url()))
        .json(&payload)
        .send()
        .await?
        .json::<WordCreatedId>()
        .await?;

    Ok(res)
}
