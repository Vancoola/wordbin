use crate::settings::{api_token, base_url};
use reqwest::{Client, RequestBuilder};
use std::sync::OnceLock;
use wordbin_types::word::{CreateWord, WordCount, WordCreatedId, WordResponse};

static CLIENT: OnceLock<Client> = OnceLock::new();

fn authed(req: RequestBuilder) -> RequestBuilder {
    let token = api_token();
    if token.is_empty() {
        req
    } else {
        req.bearer_auth(token)
    }
}

fn client() -> &'static Client {
    CLIENT.get_or_init(Client::new)
}
pub async fn add_word(payload: CreateWord) -> anyhow::Result<WordCreatedId> {
    let res = authed(client().post(format!("{}/word/add", base_url())))
        .json(&payload)
        .send()
        .await?
        .json::<WordCreatedId>()
        .await?;

    Ok(res)
}

pub async fn word_count() -> anyhow::Result<i64> {
    let res = authed(client().get(format!("{}/word/count", base_url())))
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

pub async fn fetch_words(
    limit: i64,
    offset: i64,
    status: Option<String>,
) -> anyhow::Result<Vec<WordResponse>> {
    let mut req = authed(client().get(format!("{}/word/active", base_url())))
        .query(&[("limit", limit.to_string()), ("offset", offset.to_string())]);

    if let Some(s) = status {
        req = req.query(&[("status", s)]);
    }

    Ok(req.send().await?.json::<Vec<WordResponse>>().await?)
}
