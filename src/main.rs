use std::env;
use std::process::Command;
use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, Configuration, CommandResult};

#[group]
#[commands(ping, lisp)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(Configuration::new().prefix("```"));

    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

#[command]
async fn lisp(ctx: &Context, msg: &Message) -> CommandResult {
    let codeblocks 
            = &msg.content.match_indices("```lisp")
                  .map(|(i, _)| i)
                  .zip (msg.content.match_indices("```").skip(1).step_by(2).map(|(i, _)| i))
                  .map(|(x, y)| msg.content[x..y].lines().skip(1).collect::<String>())
                  .collect::<Vec<String>>();
    for codeblock in codeblocks {
        dbg!(codeblock);
        let output 
            = Command::new("powershell")
                .args([".\\interpreters\\insituc.exe \"", codeblock, "\""])
                .output()
                .expect("failed to run");
        dbg!(&output);
        msg.reply(ctx, String::from_utf8_lossy(&output.stdout)).await?;
    }
    Ok(())
}