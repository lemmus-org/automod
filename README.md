<h1 align="center"><img src="docs/assets/logo.png"></h1>

# AutoMod

AutoModerator for Lemmy instances. Specifically a bot framework for automating common admin and moderator tasks.

### ⚠️ Use at your own risk ⚠️

This hasn't been extensively tested. It's a *very* rough draft intended to provide a framework for discussion and/or contributions.

There are very likely bugs or edge cases that are not currently being handled.

## Features

AutoMod operates on an opt-in basis using plugins. By default, they are not enabled and must be configured.

* [ModLog](docs/plugins/ModLog.md)
* [PrivateMessage](docs/plugins/PrivateMessage.md)

## Setup

* Requires [Rust](https://www.rust-lang.org/tools/install) stable `v1.75`.
* See [Makefile](Makefile) for additional commands.

#### Fetch dependencies and build project
```bash
make build
```

#### Run project
```bash
make run
```

#### Install binary
```bash
make install
```

## Configure

* See [example.toml](example.toml) for available configuration options.

By default, it will look for `automod.toml` in the current directory. This can be configured at runtime:
```bash
automod --config ~/.config/automod/my_config.toml
```

## References
* [API Reference](https://lemmy.readme.io/reference)
* [Decision Records](docs/decisions/INDEX.md)
* [License](LICENSE.md)
* [TODOs](TODO.md)

## Credits

Original logo made by Andy Cuccaro (@andycuccaro) under the CC-BY-SA 4.0 license.