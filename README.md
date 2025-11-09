# Shwrap

Shwrap, for "Shell Wrapper", is a configuration manager to [Bubblewrap](https://github.com/containers/bubblewrap) your executables in your shell.

## About

⚠ **Alpha software**: Shwrap is an alpha software, so breaking changes will happen.

Shwrap allows you to define sandbox profiles (in your directory or globally for your user) for different commands and automatically wraps them using [Bubblewrap](https://github.com/containers/bubblewrap) when executed. Hooks are available for `bash`, `zsh`, and `fish`. Integration for `nushell` is coming.

Contributions are welcome! Please feel free to submit issues or pull requests.

## Features

- 📁 **Hierarchical configuration**: Local `.shwrap.yaml` files override user config at `~/.config/shwrap/default.yaml`
- 🔒 **Secure by default**: All namespaces unshared unless explicitly allowed
- 🎯 **Per-command rules**: Different sandbox settings for each command
- 📦 **Model system**: Reusable configuration models for common patterns
- 🔄 **Shell integration**: Automatic command wrapping via shell hooks

## Installation

Build from source:

```sh
git clone https://github.com/pierrelegall/shwrap.git
cd shwrap
cargo build --release
```

## Quick start

1. **Initialize a configuration file**:

```sh
shwrap config init
# Or use a template:
shwrap config init --template nodejs
```

2. **Edit `.shwrap.yaml` file** to define your commands:

```yaml
node:
  share:
    - user
    - network
  bind:
    - ~/.npm:~/.npm
    - $PWD:/workspace
  ro_bind:
    - /usr
    - /lib
```

3. **Run manually**:

```sh
shwrap command exec node app.js
```

4. **Or use shell hooks** (see below) for automatic wrapping.

## How to Use

### Manual Use

Execute a sandboxed command manually:

```sh
# Execute a command
shwrap command exec node app.js

# Show the bwrap command that would be executed
shwrap command show node app.js

# List active command configurations
shwrap command list

# Check configuration syntax
shwrap config check

# Show which config file is being used
shwrap config which
```

### Shell hook

Shell hooks automatically wrap configured commands when you execute them.

**Features**:

- Reloads command configurations when changing directories
- Supports debug mode: `export SHWRAP_DEBUG=1`

#### Bash

Add to your `~/.bashrc`:

```sh
eval "$(shwrap shell-hook get bash)"
```

#### Zsh

Add to your `~/.zshrc`:

```sh
eval "$(shwrap shell-hook get zsh)"
```

#### Fish

Add to your `~/.config/fish/config.fish`:

```sh
shwrap shell-hook get fish | source
```

#### Nushell

🚧 TODO

## Configuration

### Configuration file hierarchy

Shwrap searches for configuration in this order:

1. **Local**: `.shwrap.yaml` in current directory or parent directories
2. **User**: `~/.config/shwrap/default.yaml`

### Configuration syntax

```yaml
# Define reusable models
base:
  type: model               # Mark this as a model (not a command)
  share:
    - user
  ro_bind:
    - /usr
    - /lib

# Define command-specific configurations
node:
  extends: base             # Optional: extend a model
  enabled: true             # Optional: enable this command (default: true)
  share:                    # Share specific namespaces
    - network
  bind:                     # Read-write mounts
    - ~/.npm:~/.npm
    - $PWD:/workspace
  ro_bind:                  # Read-only mounts
    - /etc/resolv.conf
  dev_bind:                 # Device bind mounts
    - /dev/null
  tmpfs:                    # Temporary filesystems
    - /tmp
  env:                      # Set environment variables
    NODE_ENV: production
  unset_env:                # Unset environment variables
    - DEBUG
```

### Namespace Isolation

By default, **all namespaces are unshared** (isolated). Use `share` to selectively allow:

- `user` - User/group IDs
- `network` - Network access
- `pid` - Process IDs
- `ipc` - Inter-process communication
- `uts` - Hostname
- `cgroup` - Control groups

### Templates

Available templates (use with `shwrap config init --template <name>`):

- `default` - Minimal starter template
- `nodejs` - Node.js development
- `python` - Python development
- `ruby` - Ruby development
- `go` - Go development
- `rust` - Rust development

## Examples

### Sandboxed Node.js

```yaml
node:
  share:
    - user
    - network
  bind:
    - ~/.npm:~/.npm
    - $PWD:/workspace
  ro_bind:
    - /usr
    - /lib
    - /etc/resolv.conf
```

### Isolated Python (no network)

```yaml
python:
  enabled: true
  share:
    - user
  bind:
    - $PWD:/workspace
  ro_bind:
    - /usr
    - /lib
```

### Using Models

```yaml
dev_base:
  type: model
  share:
    - user
    - network
  ro_bind:
    - /usr
    - /lib
    - /etc

node:
  extends: dev_base
  bind:
    - ~/.npm:~/.npm

python:
  extends: dev_base
  bind:
    - ~/.cache/pip:~/.cache/pip
```

## TODOs

- [X] Use local configuration file
- [X] Use user configuration file if no local configuration
- [ ] Local configuration extends user configuration
- [X] Bash hook
- [X] Zsh hook
- [X] Fish hook
- [ ] Nushell hook

## License

This software is licensed under GNU Public License version 3 or later.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## See Also

- [Bubblewrap](https://github.com/containers/bubblewrap) - The underlying sandboxing tool
- [Bubblejail](https://github.com/igo95862/bubblejail) - Alternative sandboxing solution
- [Firejail](https://github.com/netblue30/firejail) - Alternative sandboxing solution
