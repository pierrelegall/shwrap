use anyhow::Result;
use std::process::Command;

use crate::config::CommandConfig;

pub struct BwrapBuilder {
    config: CommandConfig,
}

impl BwrapBuilder {
    pub fn new(config: CommandConfig) -> Self {
        Self { config }
    }

    /// Build the bwrap command arguments
    pub fn build_args(&self) -> Vec<String> {
        let mut args = Vec::new();

        // Handle unshare options
        for namespace in &self.config.unshare {
            match namespace.as_str() {
                "network" => args.push("--unshare-net".to_string()),
                "pid" => args.push("--unshare-pid".to_string()),
                "ipc" => args.push("--unshare-ipc".to_string()),
                "uts" => args.push("--unshare-uts".to_string()),
                "user" => args.push("--unshare-user".to_string()),
                _ => eprintln!("Warning: unknown namespace '{}'", namespace),
            }
        }

        // Handle share (bind read-write)
        for path in &self.config.share {
            let expanded = shellexpand::full(path).unwrap_or_else(|_| path.into());
            args.push("--bind".to_string());
            args.push(expanded.to_string());
            args.push(expanded.to_string());
        }

        // Handle custom bind mounts
        for bind in &self.config.bind {
            let parts: Vec<&str> = bind.split(':').collect();
            if parts.len() == 2 {
                let src = shellexpand::full(parts[0]).unwrap_or_else(|_| parts[0].into());
                let dst = shellexpand::full(parts[1]).unwrap_or_else(|_| parts[1].into());
                args.push("--bind".to_string());
                args.push(src.to_string());
                args.push(dst.to_string());
            } else {
                eprintln!("Warning: invalid bind format '{}'", bind);
            }
        }

        // Handle read-only binds
        for ro_bind in &self.config.ro_bind {
            let expanded = shellexpand::full(ro_bind).unwrap_or_else(|_| ro_bind.into());
            args.push("--ro-bind".to_string());
            args.push(expanded.to_string());
            args.push(expanded.to_string());
        }

        // Handle device binds
        for dev_bind in &self.config.dev_bind {
            let expanded = shellexpand::full(dev_bind).unwrap_or_else(|_| dev_bind.into());
            args.push("--dev-bind".to_string());
            args.push(expanded.to_string());
            args.push(expanded.to_string());
        }

        // Handle tmpfs
        for tmpfs in &self.config.tmpfs {
            args.push("--tmpfs".to_string());
            args.push(tmpfs.clone());
        }

        // Handle environment variables
        for (key, value) in &self.config.env {
            args.push("--setenv".to_string());
            args.push(key.clone());
            args.push(value.clone());
        }

        // Handle unset environment variables
        for key in &self.config.unset_env {
            args.push("--unsetenv".to_string());
            args.push(key.clone());
        }

        args
    }

    /// Execute a command with bwrap
    pub fn exec(&self, command: &str, command_args: &[String]) -> Result<i32> {
        let bwrap_args = self.build_args();

        let mut cmd = Command::new("bwrap");
        cmd.args(&bwrap_args);
        cmd.arg(command);
        cmd.args(command_args);

        let status = cmd.status()?;
        Ok(status.code().unwrap_or(1))
    }

    /// Show the bwrap command that would be executed (dry-run)
    pub fn show(&self, command: &str, command_args: &[String]) -> String {
        let bwrap_args = self.build_args();

        let mut parts = vec!["bwrap".to_string()];
        parts.extend(bwrap_args);
        parts.push(command.to_string());
        parts.extend(command_args.iter().cloned());

        parts.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_test_config() -> CommandConfig {
        CommandConfig {
            enabled: true,
            extends: None,
            unshare: vec![],
            share: vec![],
            bind: vec![],
            ro_bind: vec![],
            dev_bind: vec![],
            tmpfs: vec![],
            env: HashMap::new(),
            unset_env: vec![],
        }
    }

    #[test]
    fn test_build_args_unshare() {
        let mut config = create_test_config();
        config.unshare = vec![
            "network".to_string(),
            "pid".to_string(),
            "ipc".to_string(),
            "uts".to_string(),
            "user".to_string(),
        ];

        let builder = BwrapBuilder::new(config);
        let args = builder.build_args();

        assert!(args.contains(&"--unshare-net".to_string()));
        assert!(args.contains(&"--unshare-pid".to_string()));
        assert!(args.contains(&"--unshare-ipc".to_string()));
        assert!(args.contains(&"--unshare-uts".to_string()));
        assert!(args.contains(&"--unshare-user".to_string()));
    }

    #[test]
    fn test_build_args_share() {
        let mut config = create_test_config();
        config.share = vec!["/home/user".to_string()];

        let builder = BwrapBuilder::new(config);
        let args = builder.build_args();

        assert!(args.contains(&"--bind".to_string()));
        assert!(args.contains(&"/home/user".to_string()));
    }

    #[test]
    fn test_build_args_bind() {
        let mut config = create_test_config();
        config.bind = vec!["/src:/dest".to_string()];

        let builder = BwrapBuilder::new(config);
        let args = builder.build_args();

        let bind_idx = args.iter().position(|x| x == "--bind").unwrap();
        assert_eq!(args[bind_idx + 1], "/src");
        assert_eq!(args[bind_idx + 2], "/dest");
    }

    #[test]
    fn test_build_args_ro_bind() {
        let mut config = create_test_config();
        config.ro_bind = vec!["/usr".to_string()];

        let builder = BwrapBuilder::new(config);
        let args = builder.build_args();

        assert!(args.contains(&"--ro-bind".to_string()));
        assert!(args.contains(&"/usr".to_string()));
    }

    #[test]
    fn test_build_args_dev_bind() {
        let mut config = create_test_config();
        config.dev_bind = vec!["/dev/null".to_string()];

        let builder = BwrapBuilder::new(config);
        let args = builder.build_args();

        assert!(args.contains(&"--dev-bind".to_string()));
        assert!(args.contains(&"/dev/null".to_string()));
    }

    #[test]
    fn test_build_args_tmpfs() {
        let mut config = create_test_config();
        config.tmpfs = vec!["/tmp".to_string(), "/var/tmp".to_string()];

        let builder = BwrapBuilder::new(config);
        let args = builder.build_args();

        assert!(args.contains(&"--tmpfs".to_string()));
        assert!(args.contains(&"/tmp".to_string()));
        assert!(args.contains(&"/var/tmp".to_string()));
    }

    #[test]
    fn test_build_args_env() {
        let mut config = create_test_config();
        config.env.insert("NODE_ENV".to_string(), "production".to_string());
        config.env.insert("DEBUG".to_string(), "true".to_string());

        let builder = BwrapBuilder::new(config);
        let args = builder.build_args();

        let setenv_count = args.iter().filter(|x| *x == "--setenv").count();
        assert_eq!(setenv_count, 2);
        assert!(args.contains(&"NODE_ENV".to_string()));
        assert!(args.contains(&"production".to_string()));
    }

    #[test]
    fn test_build_args_unset_env() {
        let mut config = create_test_config();
        config.unset_env = vec!["DEBUG".to_string(), "VERBOSE".to_string()];

        let builder = BwrapBuilder::new(config);
        let args = builder.build_args();

        assert!(args.contains(&"--unsetenv".to_string()));
        assert!(args.contains(&"DEBUG".to_string()));
        assert!(args.contains(&"VERBOSE".to_string()));
    }

    #[test]
    fn test_build_args_combined() {
        let mut config = create_test_config();
        config.unshare = vec!["network".to_string()];
        config.ro_bind = vec!["/usr".to_string()];
        config.env.insert("TEST".to_string(), "value".to_string());

        let builder = BwrapBuilder::new(config);
        let args = builder.build_args();

        // Check all types are present
        assert!(args.contains(&"--unshare-net".to_string()));
        assert!(args.contains(&"--ro-bind".to_string()));
        assert!(args.contains(&"--setenv".to_string()));
    }

    #[test]
    fn test_show_command() {
        let mut config = create_test_config();
        config.unshare = vec!["network".to_string()];

        let builder = BwrapBuilder::new(config);
        let cmd = builder.show("node", &["script.js".to_string()]);

        assert!(cmd.starts_with("bwrap"));
        assert!(cmd.contains("--unshare-net"));
        assert!(cmd.contains("node"));
        assert!(cmd.contains("script.js"));
    }

    #[test]
    fn test_show_command_with_multiple_args() {
        let config = create_test_config();
        let builder = BwrapBuilder::new(config);
        let cmd = builder.show("git", &["commit".to_string(), "-m".to_string(), "message".to_string()]);

        assert!(cmd.contains("git"));
        assert!(cmd.contains("commit"));
        assert!(cmd.contains("-m"));
        assert!(cmd.contains("message"));
    }

    #[test]
    fn test_empty_config() {
        let config = create_test_config();
        let builder = BwrapBuilder::new(config);
        let args = builder.build_args();

        // Empty config should produce empty args
        assert_eq!(args.len(), 0);
    }

    #[test]
    fn test_bind_with_tilde() {
        let mut config = create_test_config();
        config.share = vec!["~/.config".to_string()];

        let builder = BwrapBuilder::new(config);
        let args = builder.build_args();

        // shellexpand should expand ~ to home directory
        let bind_idx = args.iter().position(|x| x == "--bind").unwrap();
        // The expanded path should not contain ~
        assert!(!args[bind_idx + 1].contains('~'));
    }

    #[test]
    fn test_invalid_bind_format() {
        let mut config = create_test_config();
        // Invalid bind format (should be src:dest)
        config.bind = vec!["invalid".to_string()];

        let builder = BwrapBuilder::new(config);
        let args = builder.build_args();

        // Should not add invalid bind to args (only warning printed)
        // Count --bind flags, should be 0 for invalid format
        let bind_count = args.iter().filter(|x| *x == "--bind").count();
        assert_eq!(bind_count, 0);
    }
}
