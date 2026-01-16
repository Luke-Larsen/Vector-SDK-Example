# Scheduled Messages Bot

This example demonstrates how to create a bot that can schedule messages to be sent at specific times or on a recurring basis. It showcases background task management and time-based operations using the Vector SDK.

## Features

- **One-time scheduling**: Schedule messages to be sent at a specific time
- **Daily recurring messages**: Schedule messages to be sent every day at a specific time
- **Weekly recurring messages**: Schedule messages to be sent every week at a specific time
- **List scheduled messages**: View all your pending scheduled messages
- **Cancel scheduled messages**: Remove scheduled messages before they are sent
- **Background task processing**: Demonstrates how to manage background tasks in a Vector bot

## Commands

| Command | Description | Example |
|---------|-------------|---------|
| `/schedule [time] [message]` | Schedule a one-time message | `/schedule 14:30 Hello there!` |
| `/schedule daily [time] [message]` | Schedule a daily recurring message | `/schedule daily 09:00 Good morning!` |
| `/schedule weekly [time] [message]` | Schedule a weekly recurring message | `/schedule weekly 17:00 Have a great weekend!` |
| `/list` | List all your scheduled messages | `/list` |
| `/cancel [id]` | Cancel a scheduled message | `/cancel abc123` |
| `/help` | Show help information | `/help` |

## Time Format

The bot accepts time in two formats:
- **24-hour format**: `HH:MM` (e.g., `14:30` for 2:30 PM)
- **12-hour format**: `HH:MM AM/PM` (e.g., `02:30 PM` or `09:00 AM`)

## How It Works

1. **Background Task**: The bot spawns a background task that periodically checks for messages that need to be sent
2. **Scheduling**: When you schedule a message, it's stored in memory with the scheduled time
3. **Processing**: The background task checks every 5 seconds and sends any messages whose scheduled time has arrived
4. **Recurring**: For recurring messages, after sending, the bot automatically reschedules the next occurrence
5. **User Management**: Each user's scheduled messages are associated with their public key

## Technical Details

- Uses `tokio::spawn` to create a background task
- Uses `Arc<Mutex<HashMap>>` for thread-safe shared state
- Uses `chrono` for time parsing and manipulation
- Uses `uuid` to generate unique IDs for scheduled messages
- Demonstrates asynchronous programming patterns with Tokio

## Running the Bot

```sh
cd ScheduledMessages
cargo run
```

## Example Usage

```text
User: /schedule 15:30 Reminder: Meeting at 3 PM
Bot: Message scheduled! ID: abc123def456
    Time: 2026-01-15 15:30:00 UTC
    Message: Reminder: Meeting at 3 PM

User: /schedule daily 09:00 Good morning!
Bot: Message scheduled! ID: xyz789abc123
    Time: 2026-01-16 09:00:00 UTC
    Message: Good morning!
    Recurrence: Good morning! every daily

User: /list
Bot: Your scheduled messages:

    ID: abc123def456
    Time: 2026-01-15 15:30:00 UTC
    Message: Reminder: Meeting at 3 PM

    ID: xyz789abc123
    Time: 2026-01-16 09:00:00 UTC
    Message: Good morning!
    Recurrence: Good morning! every daily

User: /cancel abc123def456
Bot: Cancelled scheduled message ID: abc123def456
```

## Notes

- This example uses in-memory storage, so scheduled messages will be lost if the bot is restarted
- For production use, you would want to persist scheduled messages to a database
- The background task runs every 5 seconds, so there may be a slight delay (up to 5 seconds) before scheduled messages are sent
- Recurring messages will continue indefinitely until cancelled
