use poise::{command, CreateReply};
use poise::serenity_prelude::{Colour, CreateEmbed, CreateEmbedFooter};
use crate::{Context, Error};

/// Mostra o avatar de um usuário
#[command(
    prefix_command,
    slash_command, 
    track_edits, 
    user_cooldown = "2",
    guild_only,
    aliases("av", "logo", "icon"),
    category = "info"
)]
pub async fn avatar(
    ctx: Context<'_>, 
    #[description = "Quer o avatar de quem?"] user: Option<poise::serenity_prelude::User>,
) -> Result<(), Error> {
    let target = match user {
        Some(ref u) => u.avatar_url(),
        None => ctx.author().avatar_url()
    };

    let target_url = target.as_deref().unwrap_or_else(|| "https://via.placeholder.com/1080x1080");
    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title(format!("avatar de {:?}", match user {
                    Some(ref u) => u.name.clone(),
                    None => ctx.author().name.clone(),
                }))
                .url(target_url)
                .image(target_url)
                .color(Colour::from_rgb(0, 255, 32))
                .footer(CreateEmbedFooter::new("Apelidos: av, logo, icon"))
        ),
    ).await?;

    Ok(())
}

/// Mostra informações de um usuário
#[command(
    prefix_command,
    slash_command, 
    track_edits, 
    user_cooldown = "2",
    guild_only,
    category = "info"
)]
pub async fn info(
    ctx: Context<'_>, 
    #[description = "Quer a informação de quem?"] user: Option<poise::serenity_prelude::User>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id()
        .ok_or_else(|| poise::serenity_prelude::Error::Other("Use esse comando no servidor."))?;
    let target_user = user.as_ref().unwrap_or_else(|| ctx.author());

    let member = guild_id
        .member(ctx.serenity_context(), target_user.id)
        .await
        .map_err(|_| poise::serenity_prelude::Error::Other("Não foi possível obter o membro no servidor."))?;
    let roles_id = member.roles;

    // ROLES
    let guild = guild_id
        .to_partial_guild(ctx.serenity_context())
        .await
        .map_err(|_| poise::serenity_prelude::Error::Other("Não foi possível obter o servidor."))?;

    let mut roles = roles_id
        .iter()
        .filter_map(|role_id| guild.roles.get(role_id))
        .collect::<Vec<_>>();
    roles.sort_by(|a, b| b.position.cmp(&a.position));

    let mention_roles = roles
        .iter()
        .map(|role| format!("<@&{}>", role.id))
        .collect::<Vec<_>>()
        .join(", ");

    let highest_role_color = roles
        .iter()
        .filter(|role| role.colour.0 != 0)
        .max_by_key(|role| role.position)
        .map(|role| role.colour)
        .unwrap_or_else(|| Colour::from_rgb(0, 255, 32));
    //
    // ACCOUNT
    let created = member.user.created_at();
    let join = member.joined_at.unwrap();

    let avatar = match user {
        Some(ref u) => u.avatar_url().clone(),
        None => ctx.author().avatar_url().clone()
    };

    let av_url = avatar.as_deref().unwrap_or_else(|| "https://blog.iprocess.com.br/wp-content/uploads/2021/11/placeholder-300x200.png");
    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title(format!("informações sobre {}", match user {
                    Some(ref u) => u.name.clone(),
                    None => ctx.author().name.clone(),
                }))
                // ID
                .field("ID", match user {
                    Some(ref u) => u.id.clone().to_string(),
                    None => ctx.author().id.clone().to_string()
                }, false)
                // ROLES
                .field("CARGOS", if mention_roles.is_empty() {
                    "Sem cargos".to_string()
                } else {
                    mention_roles
                }, false)
                // ACCOUNT
                .field("Conta criada em", created.format("%d/%m/%Y").to_string(), true)
                .field("Entrou aqui em", join.format("%d/%m/%Y").to_string(), true)
                .color(highest_role_color)
                .footer(CreateEmbedFooter::new("Informações do usuário"))
                .thumbnail(av_url)
        ),
    ).await?;
    
    Ok(())
}

/// Mostra o banner de um usuário
#[command(
    prefix_command,
    slash_command, 
    track_edits, 
    user_cooldown = "2",
    guild_only,
    category = "info"
)]
pub async fn banner(
    ctx: Context<'_>, 
    #[description = "Quer o banner de quem?"] user: Option<poise::serenity_prelude::User>,
) -> Result<(), Error> {
    let target = match user {
        Some(ref u) => u.banner_url(),
        None => ctx.author().banner_url()
    };

    let target_url = target.as_deref().unwrap_or_else(|| "https://blog.iprocess.com.br/wp-content/uploads/2021/11/placeholder-300x200.png");
    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title(format!("banner de {:?}", match user {
                    Some(ref u) => u.name.clone(),
                    None => ctx.author().name.clone(),
                }))
                .url(target_url)
                .image(target_url)
                .color(Colour::from_rgb(0, 255, 32))
                .footer(CreateEmbedFooter::new(format!("Banner de {}", match user {
                    Some(ref u) => u.id.clone(),
                    None => ctx.author().id.clone()
                })))
        ),
    ).await?;

    Ok(())
}

/// Sobre o comando `user`
#[command(
    prefix_command,
    slash_command, 
    track_edits, 
    user_cooldown = "2",
    guild_only,
    aliases("usuario"),
    category = "info",
    subcommands("avatar", "info", "banner")
)]
pub async fn user(
    ctx: Context<'_>
) -> Result<(), Error> {
    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("Informações")
                .description("
Esse comando te mostra informações sobre usuários.
- Subcomandos
  - `avatar` - Mostra o avatar de alguém.
  - `info` - Mostra as informações de uma pessoa aqui dentro do servidor.
  - `banner` - Mostra o banner de alguém.
                ")
                .color(Colour::from_rgb(0, 255, 32)),
        ),
    ).await?;

    Ok(())
}