# Vector Bot: Nostr SDK Explorer

## Overview
The Vector Bot project is designed to explore and showcase various features of the Vector SDK in conjunction with the Nostr protocol. This bot demonstrates how to create a functional bot that can interact with the Nostr network, send and receive messages, and perform various tasks using the Vector SDK.

## Features
- Nostr integration for decentralized messaging
- Command-based interaction with users
- Secure handling of private messages
- Ability to fetch and display data from external APIs
- Image handling and sharing

## Requirements
- Rust toolchain (stable version)
- Cargo package manager

## Dependencies
The project uses the following main dependencies:
- [vector_sdk](https://github.com/vector-sdk/rust): The core SDK for Vector bot functionality
- [nostr-sdk](https://github.com/nostr-protocol/nostr): SDK for Nostr protocol integration
- [tokio](https://tokio.rs/): Asynchronous runtime for Rust
- [reqwest](https://docs.rs/reqwest/latest/reqwest/): HTTP client for making API requests

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
Before running the bot, make sure to configure any necessary environment variables or settings. This may include setting up Nostr relay URLs or API keys for external services.

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
- Vector SDK team
- Nostr protocol developers
- Rust community