# Vector SDK Examples

This repository contains various examples demonstrating how to use the Vector SDK (https://github.com/VectorPrivacy/Vector-SDK). Each subfolder represents a distinct project with its own objectives and contributions to the overall repository. The projects are implemented in Rust and showcase different functionalities of the Vector SDK.

## Projects Overview

### [HelloWorld](HelloWorld/)
A simple "Hello World" example demonstrating basic setup and usage of the Vector SDK.
- Responds to private direct messages with "Hello World"
- **Files**: `Cargo.toml`, `src/main.rs`

### [ImageResponse](ImageResponse/)
Demonstrates how to send image responses using the Vector SDK.
- Responds to private messages with cat images from an external API
- **Files**: `Cargo.toml`, `src/main.rs`

### [SendReactions](SendReactions/)
Demonstrates sending reactions to messages using the Vector SDK.
- Responds to private messages with checkmark reactions
- **Files**: `Cargo.toml`, `src/main.rs`

### [SendTypingMessage](SendTypingMessage/)
Shows how to send typing message indicators using the Vector SDK.
- Sends typing indicators before responding with "Hello World"
- **Files**: `Cargo.toml`, `src/main.rs`

### [Multi-command](Multi-command/)
Combines multiple Vector SDK features into a single bot.
- Handles private messages and supports commands: `/rand`, `/help`, `/cat`, `/pivx`
- Demonstrates reactions, typing indicators, images, and API integration
- **Files**: `Cargo.toml`, `src/main.rs`

### [Group/JoinAndSayHello](Group/JoinAndSayHello/)
Demonstrates joining and participating in MLS (Messaging Layer Security) group conversations.
- Handles private direct messages and group messages
- Automatically responds with "Hello World" to private messages and group messages
- **Files**: `main.rs`

### [Group/JoinAndMultiCommand](Group/JoinAndMultiCommand/)
Advanced MLS group bot with multiple command support.
- Joins and listens to MLS group conversations
- Supports commands: `/help`, `/cat`
- Demonstrates reactions, typing indicators, and images in group chats
- **Files**: `Cargo.toml`, `src/main.rs`, `Vector-SDK/` (local SDK version)

## Usage

Each project can be built and run independently. Navigate to the project directory and run:

```sh
cd <project-directory>
cargo run
```

For release builds:

```sh
cargo build --release
cargo run --release
```

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for any feature requests or bug reports.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
