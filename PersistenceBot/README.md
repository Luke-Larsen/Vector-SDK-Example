# Persistence Bot

This example demonstrates how to create a bot that persists conversation history and user preferences using SQLite database. It showcases state management and data persistence between bot restarts, which is essential for production-grade bots.

## Features

- **Conversation History**: Automatically saves all messages and persists them between bot restarts
- **User Preferences**: Store and retrieve user-specific settings
- **Bot Statistics**: Track and display usage metrics
- **Database Integration**: Uses SQLite for local storage
- **Schema Management**: Demonstrates database schema initialization and versioning
- **Error Handling**: Robust error handling for database operations

## Commands

| Command | Description | Example |
|---------|-------------|---------|
| `/save [key] [value]` | Save a user preference | `/save theme dark` |
| `/load [key]` | Load a user preference | `/load theme` |
| `/history [n]` | Show last n messages in conversation (default: 10) | `/history 5` |
| `/stats` | Show bot usage statistics | `/stats` |
| `/help` | Show help information | `/help` |

## How It Works

1. **Database Initialization**: On startup, the bot creates a SQLite database file (`persistence_bot.db`) with tables for conversations, messages, and user preferences
2. **Conversation Tracking**: Each conversation (user or group) gets its own entry in the database
3. **Message Persistence**: All messages (both from users and the bot) are saved with timestamps
4. **Preference Storage**: User preferences are stored with keys and values, allowing for customization
5. **History Retrieval**: Users can view their conversation history at any time
6. **Statistics**: The bot tracks total conversations, messages, and users

## Database Schema

The bot creates the following tables:

- **schema_version**: Tracks the current database schema version
- **conversations**: Stores information about each conversation (user/group)
- **messages**: Stores all message content with timestamps and sender information
- **user_preferences**: Stores key-value pairs for user preferences

## Technical Details

- Uses `rusqlite` for SQLite database operations
- Uses `chrono` for timestamp handling
- Uses `anyhow` for error handling
- Demonstrates proper database connection management
- Shows how to handle database migrations (schema versioning)
- Implements thread-safe database operations

## Running the Bot

```sh
cd PersistenceBot
cargo run
```

The bot will create a `persistence_bot.db` file in the same directory. This file will persist between bot restarts and contain all conversation history and user preferences.

## Example Usage

```text
User: Hello there!
Bot: Invalid command. Send /help to see available commands.

User: /save theme dark
Bot: Saved preference 'theme' with value 'dark'

User: /save timezone America/New_York
Bot: Saved preference 'timezone' with value 'America/New_York'

User: /load theme
Bot: Preference 'theme': dark

User: /history
Bot: Last 2 messages:

    [2026-01-15 14:30:22] You: Hello there!
    [2026-01-15 14:30:22] Bot: Invalid command. Send /help to see available commands.

User: /stats
Bot: Bot Statistics:

    Total Conversations: 1
    Total Messages: 4
    Total Users: 1
```

## Persistence Between Restarts

One of the key features of this bot is that all data persists between restarts:

```text
# First session
User: Hello
Bot: Invalid command...

User: /save name Alice
Bot: Saved preference 'name' with value 'Alice'

# Restart bot...

# Second session (after restart)
User: /load name
Bot: Preference 'name': Alice

User: /history
Bot: Last 3 messages:

    [2026-01-15 14:30:22] You: Hello
    [2026-01-15 14:30:22] Bot: Invalid command...
    [2026-01-15 14:30:45] You: /save name Alice
    [2026-01-15 14:30:45] Bot: Saved preference 'name' with value 'Alice'
```

## Notes

- The database file (`persistence_bot.db`) will be created in the same directory as the bot executable
- For production use, you might want to store the database in a different location (e.g., `/var/lib/your-bot/`)
- The bot currently uses a simple schema versioning system that can be extended for more complex migrations
- All database operations are synchronous, which is fine for most use cases but could be optimized with async database drivers if needed
- The bot automatically creates the database on first run if it doesn't exist
