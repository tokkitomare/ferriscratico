#[allow(warnings)]
use poise::{
    command,
    serenity_prelude::{
        Colour, CreateAttachment, CreateEmbed, CreateEmbedFooter
    }, 
    CreateReply
};
use crate::{Context, Error};

/// Command to test another (only for owners)
#[command(
    prefix_command,
    slash_command,
    track_edits, 
    guild_only = false,
    category = "test",
    owners_only = true,
    aliases("t")
)]
pub async fn test(
    ctx: Context<'_>
) -> Result<(), Error> {
    let test = ctx.data().votes.lock().await;
    ctx.say(format!("{:?}", *test)).await?;

    Ok(())
}