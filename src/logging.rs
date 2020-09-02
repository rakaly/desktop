use anyhow::Context;
use std::path::PathBuf;

pub fn setup_logger(exec_path: &PathBuf, log_level: log::LevelFilter) -> anyhow::Result<()> {
    let log_file = exec_path.with_file_name("uploader.log");
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now().to_rfc3339(),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_file)?)
        .apply()
        .context("unable to setup logging")?;
    Ok(())
}
