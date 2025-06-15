# Discord Integration with CXX and DiscordCoreAPI

This document describes the integration of the DiscordCoreAPI library into the CXX project, enabling safe Rust-C++ interop for Discord bot development.

## Overview

The DiscordCoreAPI library has been added as a git submodule to provide Discord bot functionality through safe FFI bindings using CXX. This integration allows developers to:

- Use DiscordCoreAPI's high-performance C++ Discord bot library from Rust
- Maintain memory safety through CXX's safe FFI mechanisms
- Leverage Rust's async ecosystem alongside C++ Discord operations
- Create hybrid Discord bots with Rust business logic and C++ Discord API handling

## Library Location

The DiscordCoreAPI library is located at:
```
third-party/DiscordCoreAPI/
```

This is a git submodule pointing to: https://github.com/RealTimeChris/DiscordCoreAPI

## Example Implementation

A complete example Discord bot demonstrating the integration is available in:
```
examples/discord-bot/
```

### Example Features

The example demonstrates:
- Safe string passing between Rust and C++
- Discord bot initialization and lifecycle management
- Message handling with callbacks from C++ to Rust
- Error handling across the FFI boundary
- Asynchronous operations with proper resource management

### Running the Example

1. Set your Discord bot token:
   ```bash
   export DISCORD_TOKEN=your_bot_token_here
   ```

2. Build and run:
   ```bash
   cd examples/discord-bot
   cargo run
   ```

## Integration Architecture

### CXX Bridge Definition

The integration uses a CXX bridge (`src/lib.rs`) that defines:

- **Shared Structs**: `BotConfig`, `MessageData` - visible to both languages
- **Opaque Types**: `DiscordBot` - C++ implementation hidden from Rust
- **C++ Functions**: Bot lifecycle, message sending, status queries
- **Rust Callbacks**: Event handlers for messages, bot ready, errors

### C++ Implementation

The C++ side (`src/discord_bridge.cpp`) provides:
- Wrapper around DiscordCoreAPI functionality
- PIMPL pattern for clean separation
- Callback mechanisms to notify Rust of Discord events
- Resource management and lifecycle control

### Rust API

The Rust side exposes a clean, safe API:
```rust
// Create and configure a bot
let mut bot = create_bot(token, intents);

// Start the bot
if start(&mut bot) {
    // Send messages
    send_message(&mut bot, channel_id, "Hello!");
    
    // Check status
    println!("Status: {}", get_status(&bot));
    
    // Stop when done
    stop(&mut bot);
}
```

## Dependencies

To use the Discord integration, you'll need:

### System Dependencies
- CMake 3.20 or greater
- C++20 compatible compiler
- OpenSSL development libraries
- Opus development libraries  
- libsodium development libraries

### Rust Dependencies
- `cxx` - For safe FFI
- `cxx-build` - For build-time code generation
- `tokio` - For async runtime (in examples)

## Building with DiscordCoreAPI

When building projects that use the Discord integration:

1. Ensure all DiscordCoreAPI dependencies are installed
2. Initialize the submodule if not already done:
   ```bash
   git submodule update --init --recursive
   ```
3. Include the DiscordCoreAPI headers in your build script:
   ```rust
   cxx_build::bridge("src/lib.rs")
       .include("../../third-party/DiscordCoreAPI/Include")
       .std("c++20")
       .compile("your-project");
   ```

## Advanced Usage

### Custom Event Handling

You can extend the bridge to handle additional Discord events by:
1. Adding new callback function signatures to the CXX bridge
2. Implementing the callbacks in Rust
3. Calling the callbacks from the C++ DiscordCoreAPI event handlers

### Performance Considerations

- The integration leverages DiscordCoreAPI's custom coroutines for high performance
- CXX provides zero-cost abstractions for most operations
- String conversions between Rust and C++ are optimized by CXX
- Consider using `&str` instead of `String` for read-only operations

### Error Handling

The integration provides multiple error handling mechanisms:
- Boolean return values for simple success/failure
- Error callbacks for asynchronous error reporting
- Rust `Result` types can be added for more sophisticated error handling

## Contributing

When contributing to the Discord integration:

1. Follow CXX best practices for FFI design
2. Maintain compatibility with DiscordCoreAPI updates
3. Add tests for new functionality
4. Update documentation for API changes
5. Consider performance implications of FFI calls

## License

The Discord integration follows the same MIT/Apache-2.0 dual license as the CXX project. The DiscordCoreAPI library maintains its own MIT license.