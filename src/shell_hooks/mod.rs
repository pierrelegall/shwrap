// Copyright (C) 2025 Pierre Le Gall
// SPDX-License-Identifier: GPL-3.0-or-later

const BASH_HOOK: &str = include_str!("bash_hook.sh");
const ZSH_HOOK: &str = include_str!("zsh_hook.sh");
const FISH_HOOK: &str = include_str!("fish_hook.fish");

pub enum Shell {
    Bash,
    Zsh,
    Fish,
    Nushell,
}

impl Shell {
    pub fn to_str(&self) -> &str {
        match self {
            Shell::Bash => "bash",
            Shell::Zsh => "zsh",
            Shell::Fish => "fish",
            Shell::Nushell => "nushell",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "bash" => Some(Shell::Bash),
            "zsh" => Some(Shell::Zsh),
            "fish" => Some(Shell::Fish),
            "nushell" => Some(Shell::Nushell),
            _ => None,
        }
    }

    pub fn get_hook(&self) -> Option<&str> {
        match self {
            Shell::Bash => Some(BASH_HOOK),
            Shell::Zsh => Some(ZSH_HOOK),
            Shell::Fish => Some(FISH_HOOK),
            _ => None,
        }
    }
}
