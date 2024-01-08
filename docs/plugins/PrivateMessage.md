# PrivateMessage Plugin

Support common actions for private messages.

## Config

### `enabled`

Toggles the plugin.

### `interval`

Frequency, in seconds, for invoking the plugin.

### `prune_messages`

Delete any private messages sent to the bot after a period of time.

### `forward_messages`

Forwards any private messages from users.

### `allow_message_commands`

Allow commands to be sent to the bot for convenience.

### `audit_message_commands`

Notify local admins any time a message command has been performed.

## Message Commands

When enabled, only local admins are authorized. Any failed commands will be forwarded to sender with an error response.

Since not all clients support admin actions, this can be useful in a pinch.

### site_ban

Performs a site ban against a specific user.

Example:
`!site_ban username reason`

### purge_user

Purges a user's content.

Example:
`!purge_user username reason`