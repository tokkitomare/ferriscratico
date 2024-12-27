use poise::command;
use std::collections::HashMap;
use crate::{Context, Error};

/// Comando para ver as F.A.Q do servidor diretamente pelo bot
#[command(
    prefix_command,
    slash_command, 
    track_edits, 
    user_cooldown = "2",
    guild_only,
    category = "info"
)]
pub async fn faq(
    ctx: Context<'_>,
    #[description = "Qual o número da F.A.Q?"] num: u8,
) -> Result<(), Error> {
    let faqs: HashMap<u8, &str> = [
        (1, "FAQ num 1.")
    ].iter().cloned().collect();

    let response = match faqs.get(&num) {
        Some(&faq) => faq.to_string(),
        None => "Número de FAQ inválido.".to_string()
    };

    ctx.say(&response).await?;
    Ok(())
}