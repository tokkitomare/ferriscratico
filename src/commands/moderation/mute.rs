use poise::{command, serenity_prelude::{Colour, CreateEmbed, CreateEmbedAuthor, Timestamp, User}};
use chrono::Duration;
use crate::{Context, Error};

/// Silencia algum infrator de regras
#[command(
    prefix_command,
    slash_command,
    user_cooldown = "2",
    track_edits, 
    guild_only = true,
    category = "moderation",
    aliases("mutar", "silenciar", "xiu", "calado"),
    required_permissions = "MODERATE_MEMBERS"
)]
pub async fn mute(
    ctx: Context<'_>,
    #[description = "Membro para mutar"] user: User,
    #[description = "Tempo para mutar"] time: String,
    #[description = "Motivo da punição"] #[rest] reason: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("O comando deve ser usado em um servidor.")?;
    let http = ctx.serenity_context().http.clone();

    let mut member = guild_id.member(&http, user.id).await?;

    if member.communication_disabled_until.is_none() {
        let (num, unit) = time.split_at(time.len() - 1).into();
        let num = num.trim().parse::<i64>();
        if let Err(_) = num {
            ctx.say(
                "Você digitou algo errado na parte do tempo, tente fazer como nesses exemplos: `10m` para minutos, `15h` para horas, `2d` para dias."
            ).await?;
            return Err("Erro no formato do tempo.".into());
        }
        let num = num.unwrap();

        let duration = match unit {
            "m" => Duration::minutes(num),
            "d" => Duration::days(num),
            _ => Duration::hours(num),
        };

        let timeout_end = Timestamp::from(
            Timestamp::now()
                .checked_add_signed(duration)
                .ok_or("Erro tentando calcular o time do timeout.")?
        );

        member.disable_communication_until_datetime(&http, timeout_end).await?;

        ctx.send(
            poise::CreateReply::default()
            .embed(
                CreateEmbed::new()
                    .title("Mutezinho ai papai")
                    .author(CreateEmbedAuthor::new(ctx.author().clone().name))
                    .color(Colour::ORANGE)
                    .field("Tá de bico fechado:", format!("<@{}> {}", user.id, user.id), false)
                    .field("Quem mutou:", format!("<@{}> mutou por {} {}", ctx.author().id, num, 
                    match unit {
                        "m" => "minuto(s)",
                        "d" => "dia(s)",
                        _ => "hora(s)"
                    })
                    , false)
                    .field("Motivo:", reason.unwrap_or_else(|| "Sem motivo especificado...".to_string()), false)
            )
        ).await?;

        let http_clone = http.clone();
        let guild_id_clone = guild_id.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(num as u64 * 60)).await;
            if let Ok(mut mut_member) = guild_id_clone.member(&http_clone, user.id).await {
                let _ = mut_member.enable_communication(&http_clone).await;
            }
        });
    } else {
        if let Some(timeout_end) = member.communication_disabled_until {
            let current_time = Timestamp::now().unix_timestamp();
            let timeout_end_unix = timeout_end.unix_timestamp();
            let remaining_seconds = timeout_end_unix - current_time;
        
            if remaining_seconds > 0 {
                let hours = remaining_seconds / 3600;
                let minutes = (remaining_seconds % 3600) / 60;
                let seconds = remaining_seconds % 60;
        
                ctx.say(
                    format!(
                        "<@{}> já tá de bico fechado! time restante: {}h {}m {}s.",
                        user.id, hours, minutes, seconds
                    )
                ).await?;
            } else {
                ctx.say(
                    format!("<@{}> já poderia ter sido desmutado, mas está ainda no timeout!", user.id)
                ).await?;
            }
        }
        
    }

    Ok(())
}



/// Desilencia algum infrator de regras
#[command(
    prefix_command,
    slash_command,
    user_cooldown = "2",
    track_edits, 
    guild_only = true,
    category = "moderation",
    aliases("desmutar", "disilenciar", "fale"),
    required_permissions = "MODERATE_MEMBERS"
)]
pub async fn unmute(
    ctx: Context<'_>,
    #[description = "Membro para desmutar"] user: User,
    #[description = "Motivo para deixar falar novamente"] #[rest] reason: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("O comando deve ser usado em um servidor.")?;
    let http = ctx.serenity_context().http.clone();

    let mut member = guild_id.member(&http, user.id).await?;
    if let Some(_) = member.communication_disabled_until {
        member.enable_communication(&http).await?;

        ctx.send(
            poise::CreateReply::default()
            .embed(
                CreateEmbed::new()
                    .title("Pode falar novamente")
                    .author(CreateEmbedAuthor::new(ctx.author().clone().name))
                    .color(Colour::from_rgb(0, 255, 32))
                    .field("Desmutado(a):", format!("<@{}> pode falar novamente.", user.id), false)
                    .field("Mod:", format!("<@{}> desmutou", ctx.author().id), false)
                    .field("Motivo:", reason.unwrap_or_else(|| "Sem motivo especificado...".to_string()), false)
            )
        ).await?;
    } else {
        ctx.say(
            format!("Ué, o <@{}> já tá com o direito de fala? Tá doidão mano?", user.id)
        ).await?;
    }

    Ok(())
}
