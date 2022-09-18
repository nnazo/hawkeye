use crate::util::Context;
use anyhow::Result;
use serenity::json::Value;
use tracing::instrument;

#[instrument]
pub async fn send(ctx: &Context, embed: Value) -> Result<()> {
    let wait = false;
    ctx.webhook
        .execute(&ctx.serenity_http, wait, |w| w.embeds(vec![embed]))
        .await?;
    Ok(())
}
