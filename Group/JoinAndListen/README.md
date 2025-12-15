# JoinAndListen Bot

## Overview
This bot demonstrates how to join and listen to MLS (Messaging Layer Security) group conversations using the Vector SDK. It handles private direct messages, group messages, and various commands.

## Features
- Handles private direct messages
- Joins and listens to MLS group conversations
- Supports commands like `/help` and `/cat`
- Sends reactions, typing indicators, and images

## Requirements
- Rust toolchain (stable version)
- Cargo package manager

## Dependencies
- vector_sdk (local version in Vector-SDK directory)
- tokio
- reqwest
- serde_json

## Setup and Usage

### Installation
1. Clone the repository:
   ```
   git clone https://github.com/yourusername/vector-bot.git
   cd vector-bot
   ```

2. Build the project:
   ```
   cargo build --release
   ```

### Configuration
Before running the bot, make sure to configure any necessary environment variables or settings, such as setting a permanent npriv and changing the master npub.

### Running the Bot
To start the bot, use the following command:
```
cargo run --release
```

## Commands
The bot supports several commands that can be sent as private messages or in group chats:

- `/help`: Display available commands
- `/cat`: Get a random cat image

## Project Structure
- `src/main.rs`: Main entry point of the application
- `Cargo.toml`: Project dependencies and configuration
- `Vector-SDK/`: Local version of the Vector SDK

## Contributing
Contributions are welcome! Please feel free to submit issues or pull requests.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.