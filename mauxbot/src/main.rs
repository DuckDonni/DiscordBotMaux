use dotenv::dotenv; // Import dotenv function to load the .env file
use serenity::{
    all::UserId, async_trait, builder::{CreateCommand, CreateCommandOption}, model::{
        application::ResolvedOption, channel::{Channel, ChannelType, Message}, gateway::{GatewayIntents, Ready}, id::ChannelId, voice::VoiceState
    }, prelude::*
};
use std::env;

const HELP_MESSAGE: &str = "
Hello I am Maux
This is a test message for the help function!
";



const HELP_COMMAND: &str = "!help";

const TARGET_ID1: UserId = UserId::new(531554945569259541);

struct Handler;

pub fn run(_options: &[ResolvedOption]) -> String {
    "Hey, I'm alive!".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
        
        
    
    async fn message(&self, ctx: Context, msg: Message) {
        // Ignore DMs and respond only in guild channels
        if msg.guild_id.is_none() {
            return;
        }   
        
        // Log the message to confirm it's being received
        println!("Received message: {} from channel {}", msg.content, msg.channel_id);

        // Check if the received message is the help command
        if msg.content.trim() == HELP_COMMAND {
            println!("Received !help command");

            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        // Check if the user joined a voice channel
        if let Some(channel_id) = new.channel_id {
            println!("User {} joined voice channel {}", new.user_id, channel_id);
            
            let welcome_message = format!("Oh my god it's <@{}>!", new.user_id);
            
            if new.user_id == TARGET_ID1{
                if let Err(why) = channel_id.say(&ctx.http, welcome_message).await {
                    println!("Error sending message to voice channel chat: {:?}", why);
                }
            }
            
        }
    }
}

#[tokio::main]
async fn main() {
    // Loads the environment variable from the .env file
    dotenv().ok(); // Ensure correct import here
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_VOICE_STATES;

    let mut client = serenity::Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start the client and log errors
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
