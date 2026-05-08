use crate::settings::base_url;
use reqwest::Client;
use std::sync::OnceLock;
use wordbin_types::{CreateWord, WordCount, WordCreatedId};

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

pub async fn word_count() -> anyhow::Result<i64> {
    let res = client()
        .get(format!("{}/word/count", base_url()))
        .send()
        .await?
        .json::<WordCount>()
        .await?;

    Ok(res.count)
}

pub async fn health_check() -> bool {
    client()
        .get(format!("{}/healthz", base_url()))
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}
