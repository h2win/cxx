fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/discord_bridge.cpp")
        .include("include")
        .include("../../third-party/DiscordCoreAPI/Include")
        .std("c++20")
        .compile("discord-bridge");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/discord_bridge.cpp");
    println!("cargo:rerun-if-changed=include/discord_bridge.h");
}