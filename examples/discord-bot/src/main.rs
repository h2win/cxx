use discord_bot_example::{create_bot, start, stop, send_message, get_status};
use std::env;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Discord Bot Example with CXX and DiscordCoreAPI");
    println!("================================================");
    
    // Get Discord token from environment variable
    let token = env::var("DISCORD_TOKEN")
        .unwrap_or_else(|_| {
            println!("Warning: DISCORD_TOKEN not set, using placeholder");
            "your_discord_bot_token_here".to_string()
        });
    
    // Create bot with basic intents
    let intents = 1 << 9 | 1 << 15; // GUILD_MESSAGES | MESSAGE_CONTENT
    let mut bot = create_bot(token, intents);
    
    println!("Bot status: {}", get_status(&bot));
    
    // Start the bot
    if start(&mut bot) {
        println!("Bot started successfully!");
        println!("Bot status: {}", get_status(&bot));
        
        // Let the bot run for a few seconds
        sleep(Duration::from_secs(5)).await;
        
        // Send a test message (this would need a real channel ID in practice)
        let test_channel_id = "123456789012345678";
        if send_message(&mut bot, test_channel_id, "Hello from Rust via CXX!") {
            println!("Test message sent successfully!");
        } else {
            println!("Failed to send test message");
        }
        
        // Let it run a bit more
        sleep(Duration::from_secs(3)).await;
        
        // Stop the bot
        stop(&mut bot);
        println!("Bot stopped");
        println!("Final bot status: {}", get_status(&bot));
    } else {
        println!("Failed to start bot");
    }
    
    Ok(())
}