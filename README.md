# Shwrap

Shwrap, for "Shell Wrapper", is a manager to [Bubblewrap](https://github.com/containers/bubblewrap) your executables in your shell.

## About

Shwrap allows you to define sandbox profiles (in your directory or globally for your user) for different commands and automatically wraps them using [Bubblewrap](https://github.com/containers/bubblewrap) when executed. Hooks are available for `bash`. Integrations for `zsh`, `fish` and `nushell` are comming.

⚠ **Alpha software**: Shwrap is still in alpha, so breaking changes could happen.

Contributions are welcome! Please feel free to submit issues or pull requests.

## Features

- 📁 **Hierarchical configuration**: Local `.shwrap` files override user config at `~/.config/shwrap/config`
- 🔒 **Secure by default**: All namespaces unshared unless explicitly allowed
- 🎯 **Per-command profiles**: Different sandbox settings for each command
- 🔄 **Shell integration**: Automatic command wrapping via shell hooks
- 📦 **Model system**: Reusable configuration models for common patterns
- 🌐 **Flexible sharing**: Selectively share user, network, or other namespaces

## Installation

Build from source:

```bash
git clone https://github.com/pierrelegall/shwrap.git
cd shwrap
cargo build --release
```

## Quick start

1. **Initialize a configuration file**:

```sh
shwrap init
# Or use a template:
shwrap init --template nodejs
```

2. **Edit `.shwrap` file** to define your commands:

```yaml
commands:
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
shwrap exec node app.js
```

4. **Or use shell hooks** (see below) for automatic wrapping.

## How to Use

### Manual Use

Execute a sandboxed command manually:

```bash
# Execute a command
shwrap exec node app.js

# Show the bwrap command that would be executed
shwrap show node app.js

# List active command configurations
shwrap list

# Check configuration syntax
shwrap check

# Show which .shwrap file is being used
shwrap which
```

### Shell hook

Shell hooks automatically wrap configured commands when you execute them.

#### Bash

Add to your `~/.bashrc`:

```bash
eval "$(shwrap shell-hook bash)"
```

```bash
node app.js
# => run `shwarp exec node app.js` if `node` command configured
```

**Features**:

- Automatically detects directory changes
- Reloads command configurations when changing directories
- Supports debug mode: `export SHWRAP_DEBUG=1`

#### Zsh

🚧 TODO

#### Fish

🚧 TODO

#### Nushell

🚧 TODO

## Configuration

### Configuration file hierarchy

Shwrap searches for configuration in this order:

1. **Local**: `.shwrap` in current directory or parent directories
2. **User**: `~/.config/shwrap/config`

### Configuration syntax

```yaml
# Optional: Define reusable models
models:
  base:
    share:
      - user
    ro_bind:
      - /usr
      - /lib

# Define command-specific configurations
commands:
  node:
    enabled: true           # Optional: enable this command (default: true)
    extends: base           # Optional: extend a template
    share:                  # Share specific namespaces
      - network
    bind:                   # Read-write mounts
      - ~/.npm:~/.npm
      - $PWD:/workspace
    ro_bind:                # Read-only mounts
      - /etc/resolv.conf
    dev_bind:               # Device bind mounts
      - /dev/null
    tmpfs:                  # Temporary filesystems
      - /tmp
    env:                    # Set environment variables
      NODE_ENV: production
    unset_env:              # Unset environment variables
      - DEBUG
```

### Namespace Isolation

By default, **all namespaces are unshared** (isolated). Use `share:` to selectively allow:

- `user` - User/group IDs
- `network` - Network access
- `pid` - Process IDs
- `ipc` - Inter-process communication
- `uts` - Hostname
- `cgroup` - Control groups

### Templates

Available templates (use with `shwrap init --template <name>`):

- `default` - Minimal starter template
- `nodejs` - Node.js development
- `python` - Python development
- `ruby` - Ruby development
- `go` - Go development
- `rust` - Rust development

## Examples

### Sandboxed Node.js

```yaml
commands:
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
commands:
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

### Using Templates

```yaml
models:
  dev_base:
    share:
      - user
      - network
    ro_bind:
      - /usr
      - /lib
      - /etc

commands:
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
- [ ] Zsh hook
- [ ] Fish hook
- [ ] Nushell hook
- [ ] Stabilize configuration file schema

## License

This software is licensed under GNU Public License version 3 or later.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## See Also

- [Bubblewrap](https://github.com/containers/bubblewrap) - The underlying sandboxing tool
- [Bubblejail](https://github.com/igo95862/bubblejail) - Alternative sandboxing solution
- [Firejail](https://github.com/netblue30/firejail) - Alternative sandboxing solution
