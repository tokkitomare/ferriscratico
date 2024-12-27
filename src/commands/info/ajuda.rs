use poise::{
    command,
    serenity_prelude::{
        Colour, CreateAttachment, CreateEmbed, CreateEmbedFooter
    }, 
    CreateReply
};
use crate::{Context, Error};

static COMANDOS: [&str; 5] = [
    "ajuda",
    "avatar",
    "limpar",
    "mute/unmute",
    "ping"
];

#[derive(Debug, poise::ChoiceParameter)]
enum Category {
    Info,
    Moderation,
    Fun,
    Economy
}

/// Subcomando do comando de ajuda: lista todos os comandos
#[command(
    prefix_command,
    slash_command,
    user_cooldown = "2",
    track_edits, 
    guild_only = false,
    category = "info",
    aliases("list", "all", "commands", "comandos", "todos")
)]
pub async fn listar(
    ctx: Context<'_>
) -> Result<(), Error> {
    let comandos = COMANDOS.join("\n");
    ctx.send(
        CreateReply::default()
        .embed(
            CreateEmbed::new()
                .title("Todos os comandos em ordem alfabética")
                .color(Colour::from_rgb(0, 255, 32))
                .description(comandos)
                .footer(CreateEmbedFooter::new("Apelidos: list, all, commands, comandos, todos"))
        )
    ).await?;

    Ok(())
}

/// Subcomando do comando de ajuda: lista todos os comandos de uma categoria
#[command(
    prefix_command,
    slash_command,
    user_cooldown = "2",
    track_edits, 
    guild_only = false,
    category = "info",
    aliases("category", "c")
)]
pub async fn categoria(
    ctx: Context<'_>,
    #[description = "Qual categoria quer ver os comandos?"] category: Category,
) -> Result<(), Error> {
    match category {
        Category::Info => {
            ctx.send(
                CreateReply::default()
                .embed(
                    CreateEmbed::new()
                        .title("Comandos dedicados a informação")
                        .color(Colour::from_rgb(0, 255, 32))
                        .description("
- `ajuda` - Exibe esta lista de comandos. Apelidos: `h`, `help`.
  - Subcomandos: `listar`, `categoria`, `comando`.

- `ping` - Mostra o tempo de resposta do bot. Apelidos: `latencia`.

- `user <subcomando>` - Mostra várias informações sobre um usuário. Apelidos: `usuario`.
  - Subcomandos: `avatar`, `info`, `banner`.

- `server <subcomando>` - Mostra várias informações sobre o servidor. Apelidos: `servidor`.
  - Subcomandos: `logo`, `info`, `banner`.
                        ")
                        .footer(CreateEmbedFooter::new("Apelidos: category, c"))
                )
            ).await?;
        },
        Category::Moderation => {
            ctx.send(
                CreateReply::default()
                .embed(
                    CreateEmbed::new()
                        .title("Comandos dedicados a moderação")
                        .color(Colour::from_rgb(0, 255, 32))
                        .description("
- `limpar <número>` - Limpa um número de mensagens no canal. Apelidos: `clear`, `clean`.
  - Permissões necessárias para executar: Gerenciar mensagens.

- `mute <usuário> <tempo> (motivo)` - Silencia um usuário que esteja quebrando as regras. Apelidos: `mutar`, `silenciar`, `xiu`, `calado`.
  - Permissões necessárias para executar: Silenciar usuários.
  - Para desfazer: `unmute <usuário> (motivo)` - Tira o silêncio do usuário provido. Apelidos: `desmutar`, `disilenciar`, `fale`.
")
                        .footer(CreateEmbedFooter::new("Apelidos: category, c"))
                )
            ).await?;
        },
        Category::Fun => {
            ctx.send(
                CreateReply::default()
                .embed(
                    CreateEmbed::new()
                        .title("Comandos dedicados a diversão")
                        .color(Colour::from_rgb(0, 255, 32))
                        .description("Ainda nenhum comando, volte mais tarde!")
                        .footer(CreateEmbedFooter::new("Apelidos: category, c"))
                )
            ).await?;
        },
        Category::Economy => {
            ctx.send(
                CreateReply::default()
                .embed(
                    CreateEmbed::new()
                        .title("Comandos dedicados a economia")
                        .color(Colour::from_rgb(0, 255, 32))
                        .description("Ainda nenhum comando, volte mais tarde!")
                        .footer(CreateEmbedFooter::new("Apelidos: category, c"))
                )
            ).await?;
        }
    };

    Ok(())
}

/// Comando de ajuda
#[command(
    prefix_command,
    slash_command,
    user_cooldown = "2",
    track_edits, 
    guild_only = false,
    category = "info",
    aliases("h", "help"),
    subcommands("listar", "categoria")
)]
pub async fn ajuda(
    ctx: Context<'_>
) -> Result<(), Error> {
    let response = "
Sou o Ferriscratico, bot feito para o servidor Rustocracia! Meu prefixo é `f.` mas eu aceito slash também.

**Obs.:** *`<>` significa OBRIGATÓRIO, `()` significa OPCIONAL.*
- **Categorias**:
  - Informação: `info`
  - Moderação: `moderation`
  - Diversão: `fun`
  - Economia: `economy`
- Quer ver todos os comandos? `help listar`
- Quer ver todos os comandos de uma categoria? `help categoria <categoria>`
    ";
    let path = "./images/ferris background.png";
    let filename = "av.png";

    let file = tokio::fs::File::open(&path).await?;
    let stream = CreateAttachment::file(&file, filename).await?;
    ctx.send(
        CreateReply::default()
            .attachment(stream)
            .embed(
                CreateEmbed::new()
                    .title("Algúem precisa de ajuda?")
                    .url("https://discord.gg/Xs6bYsqKej")
                    .description(response)
                    .color(Colour::from_rgb(0, 255, 32))
                    .footer(CreateEmbedFooter::new("RUSTOCRACIA"))
                    .thumbnail(format!("attachment://{}", filename))
    )).await?;

    Ok(())
}