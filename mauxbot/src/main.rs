use serenity::{
    async_trait,
    model::{channel::Message, gateway::{Ready, GatewayIntents}},
    prelude::*,
};
use dotenv::dotenv; // Import dotenv to load the .env file
use std::env;

const HELP_MESSAGE: &str = "
Hello I am Maux
This is a test message for the help function!

";

const HELP_COMMAND: &str = "!help";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Ignore DMs and respond only in guild channels
        if msg.guild_id.is_none() {
            return;
        }

        // Log the message to confirm it's being received
        println!("Received message: {}", msg.content);

        // Check if the received message is the help command
        if msg.content.trim() == HELP_COMMAND {
            println!("Received !help command");

            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Loads the environment variable from the .env file
    dotenv().ok(); 
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");


    let intents = GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES;

    let mut client = serenity::Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start the client and log errors
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
