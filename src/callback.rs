use poise::FrameworkError::*;
use crate::{Error, Data};

pub async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        Setup { error, .. } => {
            panic!("Ih rapaz, deu pra ligar não: {:?}", error)
        }
        Command { error, ctx, .. } => {
            println!("Erro no comando `{}`: {:?}", ctx.command().name, error);
        }
        NotAnOwner { ctx, .. } => {
            ctx.say("Só os adm daqui podem executar isso!")
                .await
                .ok();
        }
        MissingBotPermissions { missing_permissions, ctx, .. } => {
            ctx.say(format!("Cara, tô sem permissão pra isso: **{:?}**", missing_permissions))
                .await
                .ok();
        }
        MissingUserPermissions { missing_permissions, ctx, .. } => {
            ctx.say(format!("Você não tem as permissões necessárias: **{:?}**", missing_permissions))
                .await
                .ok();
        } 
        CooldownHit { remaining_cooldown, ctx, .. } => {
            ctx.say(format!("Tá com cooldown! Você pode usar esse comando novamente em {:?}.", remaining_cooldown))
                .await
                .ok();
        }
        ArgumentParse { error, input, ctx, .. } => {
            println!("Erro ao parsear argumentos: {:?}, input: {:?}", error, input);
            ctx.say("Houve um erro ao parsear os argumentos. Verifique se você colocou os argumentos corretamente.")
                .await
                .ok();
        }
        CommandPanic { payload, ctx, .. } => {
            eprintln!("Panico no comando `{}`: {:?}", ctx.command().name, payload);
            ctx.say("Pânico no sistema, ocorreu um erro no comando. Tente novamente mais tarde.")
                .await
                .ok();
        }
        SubcommandRequired { ctx } => {
            ctx.say(format!(
                "Você precisa passar um subcomando. Use `f.help {}` para ver os subcomandos.", ctx.command().name
                ))
                .await
                .ok();
        }
        other => {
            if let Err(e) = poise::builtins::on_error(other).await {
                eprintln!("Deu merda: {}", e)
            }
        }
    }
}