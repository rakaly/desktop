use anyhow::Context;
use directories::ProjectDirs;
use pico_args::Arguments;
use std::{
    io::{self, Write},
    path::PathBuf,
};

pub enum ParsedArgs {
    Help,
    Version,
    Gui { config: PathBuf },
    Run { config: PathBuf },
    InstallService,
    UninstallService,
    RunService,
}

const HELP: &str = r#"
Automatically upload saves to rakaly.com when a new file is detected

USAGE:
    uploader [COMMAND] [OPTIONS]

OPTIONS:
    -h, --help      Prints help information
        --version   Prints version information
    -c, --config    File location of an uploader's configuration file.
                    Defaults to "uploader-config.toml" sibling file to the uploader executable.
"#;

pub fn parse_args() -> anyhow::Result<ParsedArgs> {
    let mut args = pico_args::Arguments::from_env();

    if args.contains(["-h", "--help"]) {
        return Ok(ParsedArgs::Help);
    }

    if args.contains("--version") {
        return Ok(ParsedArgs::Version);
    }

    let subcommand = args
        .subcommand()
        .context("unable to extract subcommand")?
        .unwrap_or_else(|| String::from("gui"));

    match subcommand.to_ascii_lowercase().as_str() {
        "gui" => Ok(ParsedArgs::Gui {
            config: get_config_path(&mut args)?,
        }),
        "run" => Ok(ParsedArgs::Run {
            config: get_config_path(&mut args)?,
        }),
        "install-service" => Ok(ParsedArgs::InstallService),
        "run-service" => Ok(ParsedArgs::RunService),
        "uninstall-service" => Ok(ParsedArgs::UninstallService),
        x => anyhow::bail!("unrecognized subcommand: {}", x),
    }
}

fn get_config_path(args: &mut Arguments) -> anyhow::Result<PathBuf> {
    let cli_config: Option<PathBuf> = args.opt_value_from_str(["-c", "--config"])?;
    match cli_config {
        Some(path) => Ok(path),
        None => project_home().map(|x| x.config_dir().join("config.toml")),
    }
}

pub fn project_home() -> anyhow::Result<ProjectDirs> {
    if let Some(x) = ProjectDirs::from("com", "", "rakaly") {
        Ok(x)
    } else {
        anyhow::bail!("unable to locate project directory")
    }
}

pub fn print_help() {
    let _ = writeln!(io::stdout(), "{}", HELP);
}
