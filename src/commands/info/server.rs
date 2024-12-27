use poise::{command, CreateReply};
use poise::serenity_prelude::{Colour, CreateEmbed, CreateEmbedFooter};
use crate::{Context, Error};

/// Mostra o ícone do servidor
#[command(
    prefix_command,
    slash_command, 
    track_edits, 
    user_cooldown = "2",
    guild_only,
    aliases("av", "avatar", "icon"),
    category = "info"
)]
pub async fn logo(
    ctx: Context<'_>
) -> Result<(), Error> {
    let icon_url = match ctx.guild_id() {
        Some(g_id) => {
            match g_id.to_guild_cached(&ctx.serenity_context().cache) {
                Some(icon) => {
                    icon.icon_url().unwrap()
                }
                None => "https://via.placeholder.com/1080x1080".to_string()
            }
        },
        None => "https://via.placeholder.com/1080x1080".to_string()
    };

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("ícone do servidor")
                .url("https://discord.gg/Xs6bYsqKej")
                .image(icon_url)
                .color(Colour::from_rgb(0, 255, 32))
                .footer(CreateEmbedFooter::new("Apelidos: av, avatar, icon"))
        ),
    ).await?;

    Ok(())
}

/// Mostra informações do servidor
#[command(
    prefix_command,
    slash_command, 
    track_edits, 
    user_cooldown = "2",
    guild_only,
    category = "info"
)]
pub async fn info(
    ctx: Context<'_>
) -> Result<(), Error> {
    let guild_id = ctx.guild_id()
        .ok_or_else(|| poise::serenity_prelude::Error::Other("Faz isso no servidor zé"))?;

    let icon_url = match ctx.guild_id() {
        Some(g_id) => {
            match g_id.to_guild_cached(&ctx.serenity_context().cache) {
                Some(icon) => {
                    icon.icon_url().unwrap_or_else(|| "https://blog.iprocess.com.br/wp-content/uploads/2021/11/placeholder-300x200.png".to_string())
                }
                None => "https://via.placeholder.com/1080x1080".to_string()
            }
        },
        None => "https://via.placeholder.com/1080x1080".to_string()
    };
    
    let guild = guild_id
    .to_partial_guild_with_counts(ctx.serenity_context())
    .await
    .map_err(|_| poise::serenity_prelude::Error::Other("Não foi possível obter o servidor."))?;

    // SERVER
    let members = guild.approximate_member_count.unwrap_or(0);
    let members_on = guild.approximate_presence_count.unwrap_or(0);
    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title(format!("informações sobre o Rustocracia!"))
                // ID
                .field("ID", guild.id.to_string(), false)
                // INFO
                .field("Criado em", "30/09/2024", true)
                .field("Membros no servidor", members.to_string(), true)
                .field("Membros online", members_on.to_string(), false)
                .field("Dona", "`@devmyna` <@855638247937409065>", true)
                .color(Colour::from_rgb(0, 255, 32))
                .footer(CreateEmbedFooter::new("RUSTOCRACIA"))
                .thumbnail(&icon_url)
        ),
    ).await?;
    
    Ok(())
}

/// Mostra o banner do servidor
#[command(
    prefix_command,
    slash_command, 
    track_edits, 
    user_cooldown = "2",
    guild_only,
    category = "info"
)]
pub async fn banner(
    ctx: Context<'_>
) -> Result<(), Error> {
    let banner_url = if let Some(guild_id) = ctx.guild_id() {
        if let Some(guild) = guild_id.to_guild_cached(&ctx.serenity_context().cache) {
            if let Some(banner_hash) = &guild.banner {
                format!(
                    "https://cdn.discordapp.com/banners/{}/{}.png",
                    guild.id, banner_hash
                )
            } else {
                "https://blog.iprocess.com.br/wp-content/uploads/2021/11/placeholder-300x200.png".to_string()
            }
        } else {
            "https://blog.iprocess.com.br/wp-content/uploads/2021/11/placeholder-300x200.png".to_string()
        }
    } else {
        "https://blog.iprocess.com.br/wp-content/uploads/2021/11/placeholder-300x200.png".to_string()
    };

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("bannerzinho")
                .url("https://discord.gg/Xs6bYsqKej")
                .image(banner_url)
                .color(Colour::from_rgb(0, 255, 32))
        )
    ).await?;

    Ok(())
}

/// Sobre o comando `server`
#[command(
    prefix_command,
    slash_command, 
    track_edits, 
    user_cooldown = "2",
    guild_only,
    aliases("servidor"),
    category = "info",
    subcommands("logo", "info", "banner")
)]
pub async fn server(
    ctx: Context<'_>
) -> Result<(), Error> {
    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("Informações")
                .description("
Esse comando te mostra informações sobre o servidor.
- Subcomandos
  - `logo` - Mostra o icon do servidor.
  - `info` - Mostra as informações do servidor.
  - `banner` - Mostra o banner do servidor.
                ")
                .color(Colour::from_rgb(0, 255, 32)),
        ),
    ).await?;

    Ok(())
}