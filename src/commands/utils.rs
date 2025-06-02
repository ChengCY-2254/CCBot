//! # 工具模块

use crate::{HttpKey, PoiseContext};
use anyhow::Context;
use reqwest::Client;

/// 快速从[PoiseContext]中获取HTTP客户端
pub async fn get_http_client<'a>(ctx: &'a PoiseContext<'a>) -> crate::Result<Client> {
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