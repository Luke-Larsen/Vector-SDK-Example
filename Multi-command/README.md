# Multi-command Bot

## Overview
This bot combines multiple features of the Vector SDK into a single bot. It demonstrates how to handle private direct messages, send reactions, typing indicators, and images. It also includes command-based interaction with users.

## Features
- Handles private direct messages
- Supports commands like `/rand`, `/help`, `/cat`, and `/pivx`
- Sends reactions, typing indicators, and images
- Fetches data from external APIs

## Requirements
- Rust toolchain (stable version)
- Cargo package manager

## Dependencies
- vector_sdk
- tokio
- reqwest
- log
- serde_json
- rand

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
The bot supports several commands that can be sent as private messages:

- `/rand`: Get a random number
- `/help`: Display available commands
- `/cat`: Get a random cat image
- `/pivx [currency]`: Get the current PIVX price in the specified currency (default: USD)

## Project Structure
- `src/main.rs`: Main entry point of the application
- `Cargo.toml`: Project dependencies and configuration

## Contributing
Contributions are welcome! Please feel free to submit issues or pull requests.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments
- Nostr protocol developers
- Rust community