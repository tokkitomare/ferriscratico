use poise::{command, serenity_prelude::GetMessages};
use tokio::time::{sleep, Duration};
use crate::{Context, Error};

/// Limpa mensagens do canal
#[command(
    prefix_command,
    slash_command,
    user_cooldown = "2",
    track_edits, 
    guild_only = true,
    category = "moderation",
    aliases("clean", "clear"),
    required_permissions = "MANAGE_MESSAGES"
)]
pub async fn limpar(
    ctx: Context<'_>,
    #[description = "Quantas mensagens quer deletar?"] num: u8,
) -> Result<(), Error> { 
    let channel = ctx.channel_id();

    let messages = channel
        .messages(&ctx.serenity_context().http, GetMessages::new().limit(num + 1))
        .await?;

    channel
        .delete_messages(&ctx.serenity_context().http, messages.iter().map(|m| m.id))
        .await?;

    let response = ctx.say(format!("🧹 {num} mensagens apagadas com sucesso!"))
        .await?;

    sleep(Duration::from_secs(3)).await;
    response.delete(ctx).await?;

    Ok(())
}