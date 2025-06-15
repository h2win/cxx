#[cxx::bridge]
mod ffi {
    // Shared structs visible to both Rust and C++
    struct BotConfig {
        token: String,
        intents: u32,
    }

    struct MessageData {
        content: String,
        author_id: String,
        channel_id: String,
        guild_id: String,
    }

    unsafe extern "C++" {
        include!("discord_bridge.h");

        // Opaque C++ types
        type DiscordBot;

        // C++ functions
        fn create_discord_bot(config: &BotConfig) -> UniquePtr<DiscordBot>;
        fn start_bot(bot: Pin<&mut DiscordBot>) -> bool;
        fn stop_bot(bot: Pin<&mut DiscordBot>);
        fn send_message(bot: Pin<&mut DiscordBot>, channel_id: &str, content: &str) -> bool;
        fn get_bot_status(bot: &DiscordBot) -> String;
    }

    extern "Rust" {
        // Rust functions callable from C++
        fn on_message_received(message: MessageData);
        fn on_bot_ready(bot_id: &str);
        fn on_error(error_message: &str);
    }
}

// Rust implementations of the callback functions
pub fn on_message_received(message: ffi::MessageData) {
    println!("Received message: '{}' from user {} in channel {}", 
             message.content, message.author_id, message.channel_id);
    
    // Simple echo bot logic
    if message.content.starts_with("!echo ") {
        let echo_content = &message.content[6..];
        println!("Echoing: {}", echo_content);
        // Note: In a real implementation, you'd need to pass the bot instance
        // This is simplified for demonstration
    }
}

pub fn on_bot_ready(bot_id: &str) {
    println!("Discord bot is ready! Bot ID: {}", bot_id);
}

pub fn on_error(error_message: &str) {
    eprintln!("Discord bot error: {}", error_message);
}

// Public Rust API
pub use ffi::{BotConfig, MessageData, DiscordBot};

pub fn create_bot(token: String, intents: u32) -> cxx::UniquePtr<ffi::DiscordBot> {
    let config = ffi::BotConfig { token, intents };
    ffi::create_discord_bot(&config)
}

pub fn start(bot: std::pin::Pin<&mut ffi::DiscordBot>) -> bool {
    ffi::start_bot(bot)
}

pub fn stop(bot: std::pin::Pin<&mut ffi::DiscordBot>) {
    ffi::stop_bot(bot)
}

pub fn send_message(bot: std::pin::Pin<&mut ffi::DiscordBot>, channel_id: &str, content: &str) -> bool {
    ffi::send_message(bot, channel_id, content)
}

pub fn get_status(bot: &ffi::DiscordBot) -> String {
    ffi::get_bot_status(bot)
}