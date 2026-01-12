# JoinAndSayHello Bot

## Overview
This bot demonstrates how to join and participate in MLS (Messaging Layer Security) group conversations using the Vector SDK. It handles both private direct messages and group messages, automatically responding with "Hello World" to any message it receives.

## Features
- Handles private direct messages
- Joins MLS group conversations when invited
- Responds to group messages with "Hello World"
- Demonstrates MLS group messaging functionality

## Requirements
- Rust toolchain (stable version)
- Cargo package manager

## Dependencies
- vector_sdk
- tokio

## Setup and Usage

### Running the Bot
To start the bot, use the following command from the repository root:

```sh
cargo run --manifest-path=Group/JoinAndSayHello/Cargo.toml --bin join_and_say_hello
```

Or navigate to the project directory and run:

```sh
cd Group/JoinAndSayHello
cargo run
```

### Configuration
The bot uses a hardcoded private key for demonstration purposes. In production, you should:
1. Generate your own keys using `Keys::generate()`
2. Store the private key securely
3. Use environment variables or configuration files for sensitive data

## How It Works
1. The bot initializes with a specific private key
2. It listens for gift-wrapped messages (encrypted invitations)
3. When it receives a private direct message, it responds with "Hello World"
4. When it receives an MLS group welcome message, it joins the group
5. When it receives a group message, it responds with "Hello World"

## Project Structure
- `main.rs`: Main entry point containing the bot logic
- `Cargo.toml`: Project dependencies and configuration

## Contributing
Contributions are welcome! Please feel free to submit issues or pull requests.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
