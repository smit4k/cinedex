use poise::serenity_prelude as serenity;
use serenity::{gateway::ActivityData, model::user::OnlineStatus};
use serenity::builder::CreateEmbed;
use dotenv::dotenv;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, (), Error>;

pub mod search;
pub mod rating;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let token = std::env::var("discord_token").expect("Missing discord_token");
    let intents = serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::DIRECT_MESSAGES;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), search::imdb(), rating::ratings()],
            
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("c,".into()),
                ..Default::default()
            },
            ..Default::default()
        })

        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                println!("Bot is connected as {}", ready.user.name);
                
                // Set bot presence
                let activity = ActivityData::watching("movies");
                let status = OnlineStatus::Online;
                ctx.set_presence(Some(activity), status);
                
                // Register slash commands globally
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(())
            })
        })
        .build();

        let client = serenity::ClientBuilder::new(token, intents)
            .framework(framework)
            .await;

        if let Err(err) = client.unwrap().start().await {
            println!("Client error: {:?}", err);
        }
}


/// Checks the bot's latency
#[poise::command(prefix_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let shard_manager = ctx.framework().shard_manager();

    let runners  = shard_manager.runners.lock().await;
    
    let latency = if let Some((_, runner)) = runners.iter().next() {
        runner.latency
    } else {
        None
    };

    let latency_text = match latency {
        Some(duration) => format!("{}ms", duration.as_millis()),
        None => "Unknown".to_string(),
    };
    
    let embed = CreateEmbed::default()
        .title("Pong!")
        .field("Latency:", latency_text.clone(), true)
        .color(if latency_text == "Unknown" { 0xff4444 } else { 0x27ae60 });

        let reply = poise::CreateReply::default().embed(embed);
        ctx.send(reply).await?;
        
    Ok(())
}
