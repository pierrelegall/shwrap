#!/bin/bash
# bwrap-manager bash integration
# This hook intercepts commands and wraps them with bwrap if configured

__bwrap_manager_command_not_found_handle() {
    local cmd="$1"
    shift

    # Check if bwrap-manager is available
    if ! command -v bwrap-manager >/dev/null 2>&1; then
        return 127
    fi

    # Find config file
    local config_file
    config_file=$(bwrap-manager which 2>/dev/null)

    # If no config found, run command normally
    if [[ -z "$config_file" ]]; then
        command "$cmd" "$@"
        return $?
    fi

    # Check if this command is configured
    if bwrap-manager check 2>/dev/null | grep -q "- $cmd (enabled)"; then
        # Execute with bwrap-manager
        bwrap-manager exec "$cmd" "$@"
        return $?
    else
        # Run command normally
        command "$cmd" "$@"
        return $?
    fi
}

__bwrap_manager_wrap_command() {
    local cmd="$1"
    shift

    

    # Check if bwrap-manager is available
    if ! command -v bwrap-manager >/dev/null 2>&1; then
        echo "bwrap-manager command unavailable"
        __bwrap_manager_continue_or_exit
        command "$cmd" "$@"
        return $?
    fi

    # Find config file
    local config_file
    config_file=$(bwrap-manager which 2>/dev/null)

    # If no config found, run command normally
    if [[ -z "$config_file" ]]; then
        echo "bwrap-manager config not found"
        __bwrap_manager_continue_or_exit
        command "$cmd" "$@"
        return $?
    fi

    # Check if this command should be wrapped
    # We do this by attempting to show the command - if it succeeds, wrap it
    if bwrap-manager show "$cmd" "$@" >/dev/null 2>&1; then
        echo "Command bwrapped: $cmd"
        bwrap-manager exec "$cmd" "$@"
        return $?
    else
        echo "bwrap-manager do not show `$cmd` command"
        __bwrap_manager_continue_or_exit
        command "$cmd" "$@"
        return $?
    fi
}

__bwrap_manager_continue_or_exit() {
  echo -n "Are you sure you want to continue? (y/N): "
  read -r response
  if [ "$response" != "y" ] && [ "$response" != "Y" ]; then
    echo "Aborting."
    exit 1
  fi
}

# Create wrapper functions for common commands
# This is more reliable than command_not_found_handle
__bwrap_manager_setup_wrappers() {
    local config_file
    config_file=$(bwrap-manager which 2>/dev/null)

    if [[ -z "$config_file" ]]; then
        return
    fi

    # Get list of configured commands
    local commands
    commands=$(bwrap-manager check 2>/dev/null | grep -oP '^\s*-\s+\K\w+(?=\s+\(enabled\))')

    # Create wrapper function for each command
    while IFS= read -r cmd; do
        if [[ -n "$cmd" ]]; then
            eval "
            $cmd() {
                __bwrap_manager_wrap_command '$cmd' \"\$@\"
            }
            "
        fi
    done <<< "$commands"
}

# Set up wrappers when a new directory is entered
__bwrap_manager_chdir() {
    builtin cd "$@"
    local ret=$?
    __bwrap_manager_setup_wrappers
    return $ret
}

# Override cd command
alias cd='__bwrap_manager_chdir'

# Set up wrappers for current directory on shell startup
__bwrap_manager_setup_wrappers

# Export the hook initialization confirmation
export BWRAP_MANAGER_HOOK_LOADED=1
