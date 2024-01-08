# ModLog Plugin

Monitors the modlog for actions taken against local users.

## Config

### `enabled`

Toggles the plugin.

### `interval`

Frequency, in seconds, for invoking the plugin.

### `notify_bans`

Notify local admins anytime a local user has been banned on a remote instance.

### `notify_federated_bans`

Notify local admins anytime a ban has been federated from a remote instance.

This works in conjunction with `allowlist_federated_actions`.

### `notify_removals`

Notify local admins anytime a local user's content has been removed on a remote instance.

### `allowlist_federated_actions`

A list of instance hostnames where any `site_ban` will also be performed locally.
