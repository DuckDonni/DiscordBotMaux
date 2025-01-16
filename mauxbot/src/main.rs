use dotenv::dotenv;
use serenity::{
    all::UserId,
    async_trait,
    builder::{CreateCommand, CreateCommandOption},
    model::{
        application::ResolvedOption,
        channel::{Channel, ChannelType, Message},
        gateway::{GatewayIntents, Ready},
        id::ChannelId,
        voice::VoiceState,
    },
    prelude::*,
};
use std::env;

// Creates help message
const HELP_MESSAGE: &str = "
Hello I am Maux
This is a test message for the help function!
";

// Establishes command for help function
const HELP_COMMAND: &str = "!help";

// Target user id #1
const TARGET_ID1: UserId = UserId::new(531554945569259541);

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Checks that bot is connected
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    // Function for command messages
    async fn message(&self, ctx: Context, msg: Message) {
        // Ignores direct dm's to bot
        if msg.guild_id.is_none() {
            return;
        }

        // Logs the server message in the terminal
        println!(
            "Received message: {} from channel {}",
            msg.content, msg.channel_id
        );

        // Checks for the message to see if it is a command
        if msg.content.trim() == HELP_COMMAND {
            println!("Received !help command");

            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
    // Function for checking voice channels
    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        // Checks if a user has joined the voice channel
        if let Some(channel_id) = new.channel_id {
            // Prints user id and channel id to the terminal
            println!("User {} joined voice channel {}", new.user_id, channel_id);

            let welcome_message = format!("Oh my god it's <@{}>!", new.user_id);

            // Checks if the user id is the same as the target id #1
            if new.user_id == TARGET_ID1 {
                if let Err(why) = channel_id.say(&ctx.http, welcome_message).await {
                    println!("Error sending message to voice channel chat: {:?}", why);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Loads the environment variables from the .env file
    dotenv().ok();

    /*

    .env file information
    format for bot token
    DISCORD_TOKEN=*bot_token*

    */

    // Sets the bot token from the .env file
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_VOICE_STATES;

    let mut client = serenity::Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Starts the bot client
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
