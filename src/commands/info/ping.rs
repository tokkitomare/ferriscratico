use poise::command;
use crate::{Context, Error};

/// Latência do bot
#[command(
    prefix_command, 
    slash_command, 
    track_edits,
    user_cooldown = "2",
    guild_only = true,
    category = "info",
    aliases("latencia"))]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let ping = ctx.ping().await.as_millis();
    ctx.say(format!("Pong! `{}ms`", ping)).await?;

    Ok(())
}