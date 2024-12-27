
#![warn(clippy::str_to_string)]

//////////////////////////////// EXPORTING ////////////////////////////////
mod commands;
mod callback;
mod handlers;

use dotenvy::dotenv;
use colored::*;

use poise::serenity_prelude::{UserId, GatewayIntents, ClientBuilder};
use std::{
    collections::HashMap,
    env::var,
    sync::Arc,
    time::Duration,
};
use tokio::sync::Mutex;

pub type Error = Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
//////////////////////////////// EXPORTING ////////////////////////////////

#[allow(dead_code)]
pub struct Data {
    votes: Mutex<HashMap<String, u32>>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    tracing::info!("{}", format!("Iniciando o bot...").bold().bright_green());

    let owners = vec![UserId::new(674166186136567808), UserId::new(855638247937409065)]
        .into_iter().collect();

    let options = poise::FrameworkOptions {
        commands: vec![
            commands::info::ajuda::ajuda(),
            commands::info::ping::ping(),
            commands::info::user::user(),
            commands::info::faq::faq(),
            commands::info::server::server(),
            commands::moderation::limpar::limpar(),
            commands::moderation::mute::mute(),
            commands::moderation::mute::unmute(),
            commands::test::test()],
        owners,
        prefix_options: poise::PrefixFrameworkOptions {
            mention_as_prefix: true,
            case_insensitive_commands: true,
            prefix: Some("f.".into()),
            additional_prefixes: vec![poise::Prefix::Literal("F.")],
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(120),
            ))),
            ..Default::default()
        },
        on_error: |error: poise::FrameworkError<'_, Data, Error>| Box::pin(callback::on_error(error)),
        pre_command: |ctx| {
            Box::pin(async move {
                tracing::info!("{}", format!("Fazendo o comando {}...", ctx.command().qualified_name).yellow().bold());
            })
        },
        post_command: |ctx| {
            Box::pin(async move {
                tracing::info!("{}", format!("Comando {} executado!", ctx.command().qualified_name).yellow().bold());
            })
        },
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                    return Ok(false);
                }
                Ok(true)
            })
        }),
        skip_checks_for_owners: false,
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                tracing::debug!(
                    "{}", format!("Aconteceu algo, event handler disparado: {:?}",
                    event.snake_case_name()
                ).bright_blue().italic());
                Ok(())
            })
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                tracing::info!("{}", format!("{} está vivo, cada um por si!", _ready.user.name).green().bold());
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    votes: Mutex::new(HashMap::new()),
                })
            })
        })
        .options(options)
        .build();

    let token = var("BOT_TOKEN")
        .expect("Faltando a váriavel BOT_TOKEN");
    let intents =
        GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    if let Err(err) = client.unwrap().start().await {
        tracing::error!("Erro ao iniciar o bot: {}", err);
    }
}
