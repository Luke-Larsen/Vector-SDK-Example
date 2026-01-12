# HelloWorld Bot

## Overview
This is the most basic example of using the vector_sdk. It is just a bot that responds to all private direct messages with "Hello World".

## Features
- Responds to private direct messages with "Hello World"

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
cd HelloWorld
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
