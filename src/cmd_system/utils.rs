use anyhow::Context;
use reqwest::Client;
use crate::{Error, HttpKey, PoiseContext};

pub async fn get_http_client<'a>(ctx: &'a PoiseContext<'a>) -> Result<Client, Error> {
    let http_client = ctx
        .serenity_context()
        .data
        .read()
        .await
        .get::<HttpKey>()
        .cloned()
        .with_context(|| "get http client failed")?;
    Ok(http_client)
}