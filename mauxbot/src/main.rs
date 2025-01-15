use dotenv::dotenv; // Import dotenv function to load the .env file
use serenity::{
    async_trait, 
    model::{
        channel::{Channel, ChannelType, Message}, 
        gateway::{GatewayIntents, Ready}, 
        id::ChannelId, 
        voice::VoiceState,
        application::ResolvedOption
    }, 
    prelude::*,
    builder::{CreateCommand, CreateCommandOption},
};
use std::env;

const HELP_MESSAGE: &str = "
Hello I am Maux
This is a test message for the help function!
";

const C_MESSSAGE: &str = "
Oh my god, it's Claire!
";

const D_MESSAGE: &str = "
Big daddy is back!
";
const HELP_COMMAND: &str = "!help";
const C_COMMAND: &str = "!claire";
const D_COMMAND: &str = "!dom";
//const TARGET_CHANNEL_ID: ChannelId = ChannelId::new(605158981543002143); // Use `new` to initialize the ChannelId

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
        println!("Received message: {}", msg.content);

        // Check if the received message is the help command
        if msg.content.trim() == HELP_COMMAND {
            println!("Received !help command");

            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content.trim() == C_COMMAND{
            println!("Revieved !claire command");
            if let Err(why) = msg.channel_id.say(&ctx.http, C_MESSSAGE).await{
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content.trim() == D_COMMAND{
            println!("Revieved !dom command");
            if let Err(why) = msg.channel_id.say(&ctx.http, D_MESSAGE).await{
                println!("Error sending message: {:?}", why);
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
