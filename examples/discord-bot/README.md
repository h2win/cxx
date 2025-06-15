# Discord Bot Example with CXX and DiscordCoreAPI

This example demonstrates how to use the DiscordCoreAPI C++ library with CXX for safe Rust-C++ interop in Discord bot development.

## Overview

This example shows how to:
- Bridge Discord bot functionality from C++ (DiscordCoreAPI) to Rust using CXX
- Create safe FFI bindings for Discord API operations
- Handle Discord events and commands through the CXX bridge

## Structure

- `src/main.rs` - Rust main application
- `src/lib.rs` - Rust side of the CXX bridge
- `src/discord_bridge.cpp` - C++ implementation using DiscordCoreAPI
- `include/discord_bridge.h` - C++ header declarations
- `build.rs` - Build script for CXX code generation
- `Cargo.toml` - Rust dependencies and configuration

## Building

1. Ensure you have the DiscordCoreAPI dependencies installed:
   - CMake (3.20+)
   - OpenSSL
   - Opus
   - libsodium

2. Build the example:
   ```bash
   cargo build
   ```

3. Run the bot:
   ```bash
   DISCORD_TOKEN=your_bot_token cargo run
   ```

## Features Demonstrated

- Safe string passing between Rust and C++
- Discord bot initialization and connection
- Message handling and responses
- Error handling across the FFI boundary
- Asynchronous operations with proper lifetime management

## Notes

This example uses the DiscordCoreAPI library added as a submodule in `../../third-party/DiscordCoreAPI`.