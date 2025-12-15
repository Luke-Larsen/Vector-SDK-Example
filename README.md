# Vector SDK Examples

This repository contains various examples demonstrating how to use the Vector SDK (https://github.com/VectorPrivacy/Vector-SDK). Each subfolder represents a distinct project with its own objectives and contributions to the overall repository. The projects are implemented in Rust and showcase different functionalities of the Vector SDK.

## Projects Overview

### [HelloWorld](HelloWorld/)
- **Objective**: A simple "Hello World" example to demonstrate the basic setup and usage of the Vector SDK.
- **Contents**:
  - `Cargo.toml`: Defines the project dependencies.
  - `src/main.rs`: Contains the main application code that responds to private direct messages with "Hello World".

### [ImageResponse](ImageResponse/)
- **Objective**: Demonstrates how to send image responses using the Vector SDK.
- **Contents**:
  - `Cargo.toml`: Defines the project dependencies.
  - `src/main.rs`: Contains the main application code for sending image responses.

### [SendReactions](SendReactions/)
- **Objective**: Demonstrates sending reactions to messages using the Vector SDK.
- **Contents**:
  - `Cargo.toml`: Defines the project dependencies.
  - `src/main.rs`: Contains the main application code for sending reactions.

### [SendTypingMessage](SendTypingMessage/)
- **Objective**: Shows how to send typing message indicators using the Vector SDK.
- **Contents**:
  - `Cargo.toml`: Defines the project dependencies.
  - `src/main.rs`: Contains the main application code for sending typing message indicators.

### [Multi-command](Multi-command/)
- **Objective**: Shows how to execute multiple commands using the Vector SDK, and combines knowledge from previous examples.
- **Contents**:
  - `Cargo.toml`: Defines the project dependencies.
  - `src/main.rs`: Contains the main application code for executing multiple commands.

### [Group/JoinAndListen](Group/JoinAndListen/)
- **Objective**: Demonstrates how to join and listen to MLS group conversations using the Vector SDK.
- **Contents**:
  - `Cargo.toml`: Defines the project dependencies.
  - `src/main.rs`: Contains the main application code for joining and listening to group conversations.
  - `Vector-SDK/`: A submodule containing the Vector SDK source code.

## Usage

Each project can be built and run independently. Navigate to the project directory and use Cargo to build and run the project:

```sh
cd <project-directory>
cargo run
```

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for any feature requests or bug reports.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.