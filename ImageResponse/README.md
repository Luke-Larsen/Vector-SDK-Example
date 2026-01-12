# ImageResponse Bot

## Overview
This bot demonstrates how to send image responses using the Vector SDK. It responds to any private direct message by fetching a cat image from an API and sending it as a response.

## Features
- Responds to private direct messages with cat images
- Demonstrates how to use external APIs with the Vector SDK

## Requirements
- Rust toolchain (stable version)
- Cargo package manager

## Dependencies
- vector_sdk
- tokio
- reqwest
- serde_json

## Setup and Usage

### Running the Bot
To start the bot, navigate to the project directory and run:

```sh
cd ImageResponse
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
