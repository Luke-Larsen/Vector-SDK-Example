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

### Running the Bot
To start the bot, navigate to the project directory and run:

```sh
cd Multi-command
cargo run
```

For release builds:

```sh
cargo build --release
cargo run --release
```

### Configuration
The bot uses a hardcoded private key for demonstration purposes. In production, you should:
1. Generate your own keys using `Keys::generate()`
2. Store the private key securely
3. Use environment variables or configuration files for sensitive data

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
