use std::fmt::format;
use teloxide::{prelude::*, utils::command::BotCommands};
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::new("");

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Search in Snusbase", parse_with = "split")]
    Snusbase { typer: String, search: String },
    #[command(description = "Search in naz api")]
    Nazapi(String),

}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Snusbase { typer, search } => {
            searchx(typer, search).await.expect("Error");
            bot.send_message(msg.chat.id, format!("Searching in snusbase")).await?;
            let file = teloxide::types::InputFile::file("snusbase.txt");
            bot.send_document(msg.chat.id, file).await?
        }
        Command::Nazapi(search) => {
            bot.send_message(msg.chat.id, format!("Searching in naz api")).await?
        }
    };

    Ok(())
}

async fn searchx(typer: String, search: String)  -> Result<(), reqwest::Error> {

    if typer == "name" {
        search.replace("_", "");
    }

    let mut map = std::collections::HashMap::new();
    map.insert("terms", [&search]);
    map.insert("types", [&typer]);

    let client = reqwest::Client::new();
    let res = client.post("https://api-experimental.snusbase.com/data/search")
        .header("Auth", "")
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await?;

    let response_text = res.text().await?;

    let mut file = File::create("snusbase.txt");
    file.unwrap().write(response_text.as_bytes()).expect("Error while checking snusbase");
    Ok(())
}