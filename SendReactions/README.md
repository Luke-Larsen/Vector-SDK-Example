# SendReactions Bot

## Overview
This bot demonstrates how to send reactions to messages using the Vector SDK. It responds to any private direct message by sending a checkmark reaction.

## Features
- Responds to private direct messages with reactions

## Requirements
- Rust toolchain (stable version)
- Cargo package manager

## Dependencies
- vector_sdk
- tokio

## Setup and Usage

### Running the Bot
To start the bot, navigate to the project directory and run:

```sh
cd SendReactions
cargo run
```

For release builds:

```sh
cargo build --release
cargo run --release
```

## Project Structure
- `src/main.rs`: Main entry point of the application
- `Cargo.toml`: Project dependencies and configuration

## Contributing
Contributions are welcome! Please feel free to submit issues or pull requests.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
