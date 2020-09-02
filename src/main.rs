mod cli;
mod config;
mod gui;
mod logging;
mod service;
mod watcher;

use cli::ParsedArgs;
use log::{error, log_enabled, Level};
use std::env;
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    fn run() -> anyhow::Result<()> {
        let exec_path = env::current_exe()?;
        let args = cli::parse_args()?;
        match args {
            ParsedArgs::Help => {
                cli::print_help();
                Ok(())
            }
            ParsedArgs::Version => {
                let _ = writeln!(io::stdout(), "{}", env!("CARGO_PKG_VERSION"));
                Ok(())
            }
            ParsedArgs::Gui { .. } => {
                gui::run();
                Ok(())
            }
            ParsedArgs::Run {
                config: config_path,
            } => {
                let config = if !config_path.as_path().exists() {
                    let user_input = config::get_user_input();
                    config::write_minimal_config(&user_input, &config_path)?
                } else {
                    config::read_config(&config_path)?
                };

                logging::setup_logger(&exec_path, config.log_level)?;

                let client = watcher::Client {
                    username: config.username,
                    api_key: config.api_key,
                    api_url: config.api_url,
                };

                let watch_path = config
                    .watch_directory
                    .as_deref()
                    .unwrap_or_else(|| exec_path.parent().unwrap_or_else(|| exec_path.as_path()));

                watcher::core_loop(&watch_path, &client)
            }
            ParsedArgs::RunService => {
                service::run_service();
                Ok(())
            }
            ParsedArgs::InstallService => {
                service::run_service();
                Ok(())
            }
            ParsedArgs::UninstallService => {
                service::run_service();
                Ok(())
            }
        }
    }

    let res = run();
    if let Err(ref e) = res {
        if log_enabled!(Level::Debug) {
            error!("{:?}", e);
        } else {
            error!("{}", e);
        }
    }
    log::logger().flush();

    res
}
