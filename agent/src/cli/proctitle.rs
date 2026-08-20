//! Mapping from parsed CLI arguments to an initial process title.
//!
//! This logic depends on the clap `Args`/`Command` types defined in `cli`, so
//! it lives in the CLI layer. The low-level title-setting primitives it uses
//! (`compact_process_title`, `session_name`, `set_title`) live in the
//! `process_title` core module.

use crate::cli::args::{AmbientCommand, Args, Command};
use crate::process_title::{compact_process_title, session_name, set_title};

pub(crate) fn initial_title(args: &Args) -> String {
    match &args.command {
        Some(Command::Serve { .. }) => "abrasax:server".to_string(),
        Some(Command::Acp) => "abrasax acp".to_string(),
        Some(Command::Server { .. }) => "abrasax server".to_string(),
        Some(Command::Connect) => "abrasax:client".to_string(),
        Some(Command::Run { .. }) => "abrasax run".to_string(),
        Some(Command::Login { .. }) => "abrasax login".to_string(),
        Some(Command::Account { .. }) => "abrasax account".to_string(),
        Some(Command::Repl) => "abrasax repl".to_string(),
        Some(Command::Update) => "abrasax update".to_string(),
        Some(Command::Version { .. }) => "abrasax version".to_string(),
        Some(Command::Usage { .. }) => "abrasax usage".to_string(),
        Some(Command::SelfDev { .. }) => "abrasax:selfdev".to_string(),
        Some(Command::Debug { .. }) => "abrasax debug".to_string(),
        Some(Command::Auth(_)) => "abrasax auth".to_string(),
        Some(Command::Provider(_)) => "abrasax provider".to_string(),
        Some(Command::Memory(_)) => "abrasax memory".to_string(),
        Some(Command::Session(_)) => "abrasax session".to_string(),
        Some(Command::Ambient(subcommand)) => match subcommand {
            AmbientCommand::RunVisible => "abrasax ambient visible".to_string(),
            _ => "abrasax ambient".to_string(),
        },
        Some(Command::Cloud(_)) => "abrasax cloud".to_string(),
        Some(Command::Pair { .. }) => "abrasax pair".to_string(),
        Some(Command::Permissions) => "abrasax permissions".to_string(),
        Some(Command::Transcript { .. }) => "abrasax transcript".to_string(),
        Some(Command::Dictate { .. }) => "abrasax dictate".to_string(),
        Some(Command::SetupHotkey {
            listen_macos_hotkey,
            notify_cli_launch,
            listen_windows_hotkey,
            uninstall,
        }) => {
            if *listen_macos_hotkey || *listen_windows_hotkey {
                "abrasax hotkey listener".to_string()
            } else if notify_cli_launch.is_some() {
                "abrasax shortcut reminder".to_string()
            } else if *uninstall {
                "abrasax hotkey uninstall".to_string()
            } else {
                "abrasax hotkey setup".to_string()
            }
        }
        Some(Command::Browser { .. }) => "abrasax browser".to_string(),
        Some(Command::Replay { .. }) => "abrasax replay".to_string(),
        Some(Command::Model(_)) => "abrasax model".to_string(),
        Some(Command::ProviderTestCoverage { .. }) => "abrasax provider-test-coverage".to_string(),
        Some(Command::ProviderDoctor { .. }) => "abrasax provider-doctor".to_string(),
        Some(Command::AuthTest { .. }) => "abrasax auth-test".to_string(),
        Some(Command::Restart { .. }) => "abrasax restart".to_string(),
        Some(Command::Menubar { .. }) => "abrasax menubar".to_string(),
        Some(Command::SetupLauncher) => "abrasax setup-launcher".to_string(),
        None => {
            if let Some(resume) = args.resume.as_deref().filter(|resume| !resume.is_empty()) {
                let prefix = if crate::cli::selfdev::client_selfdev_requested() {
                    "abrasax:d:"
                } else {
                    "abrasax:c:"
                };
                compact_process_title(prefix, Some(&session_name(resume)))
            } else if crate::cli::selfdev::client_selfdev_requested() {
                "abrasax:selfdev".to_string()
            } else {
                "abrasax:client".to_string()
            }
        }
    }
}

pub(crate) fn set_initial_title(args: &Args) {
    set_title(initial_title(args));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::lock_test_env;
    use clap::Parser;

    const SELFDEV_ENV: &str = abrasax_selfdev_types::CLIENT_SELFDEV_ENV;

    fn with_selfdev_env_removed<T>(f: impl FnOnce() -> T) -> T {
        let _guard = lock_test_env();
        let previous = std::env::var_os(SELFDEV_ENV);
        crate::env::remove_var(SELFDEV_ENV);
        let result = f();
        if let Some(value) = previous {
            crate::env::set_var(SELFDEV_ENV, value);
        }
        result
    }

    #[test]
    fn initial_title_labels_server() {
        with_selfdev_env_removed(|| {
            let args = Args::parse_from(["abrasax", "serve"]);
            assert_eq!(initial_title(&args), "abrasax:server");
        });
    }

    #[test]
    fn initial_title_labels_resume_client_with_short_name() {
        with_selfdev_env_removed(|| {
            let args = Args::parse_from(["abrasax", "--resume", "session_fox_123"]);
            assert_eq!(initial_title(&args), "abrasax:c:fox");
        });
    }

    #[test]
    fn initial_title_labels_selfdev_command() {
        with_selfdev_env_removed(|| {
            let args = Args::parse_from(["abrasax", "self-dev"]);
            assert_eq!(initial_title(&args), "abrasax:selfdev");
        });
    }

    #[test]
    fn initial_title_labels_windows_hotkey_listener() {
        let args = Args::parse_from(["abrasax", "setup-hotkey", "--listen-windows-hotkey"]);
        assert_eq!(initial_title(&args), "abrasax hotkey listener");
    }

    #[test]
    fn initial_title_labels_hotkey_uninstall() {
        let args = Args::parse_from(["abrasax", "setup-hotkey", "--uninstall"]);
        assert_eq!(initial_title(&args), "abrasax hotkey uninstall");
    }
}
