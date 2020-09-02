use anyhow::{anyhow, Context};
use flate2::bufread::GzEncoder;
use flate2::Compression;
use log::{debug, info, warn};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

pub struct Client {
    pub username: String,
    pub api_key: String,
    pub api_url: String,
}

impl Client {
    fn upload_zip(&self, path: &Path) -> anyhow::Result<()> {
        let file = File::open(path).context("unable to open")?;
        let size = file.metadata().map(|m| m.len()).unwrap_or(0);
        let reader = BufReader::new(file);
        let resp = ureq::post(&self.api_url)
            .auth(&self.username, &self.api_key)
            .set("Content-Length", &size.to_string())
            .set("Content-Type", "application/zip")
            .send(reader);

        if resp.ok() {
            Ok(())
        } else {
            let err = resp
                .into_string()
                .context("unable to interpret eror server response")?;
            Err(anyhow!("server responded with an error: {}", err))
        }
    }

    fn upload_txt(&self, path: &Path) -> anyhow::Result<()> {
        let file = File::open(path).context("unable to open")?;
        let reader = BufReader::new(file);
        let mut buffer = Vec::new();
        let mut gz = GzEncoder::new(reader, Compression::new(4));
        gz.read_to_end(&mut buffer).context("unable to compress")?;
        let resp = ureq::post(&self.api_url)
            .auth(&self.username, &self.api_key)
            .set("Content-Encoding", "gzip")
            .send_bytes(&buffer);

        if resp.ok() {
            Ok(())
        } else {
            let err = resp
                .into_string()
                .context("unable to interpret eror server response")?;
            Err(anyhow!("server responded with an error: {}", err))
        }
    }
}

pub fn core_loop(watch_dir: &Path, client: &Client) -> anyhow::Result<()> {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(5))
        .with_context(|| "unable to create file watcher".to_string())?;

    watcher
        .watch(watch_dir, RecursiveMode::Recursive)
        .with_context(|| format!("unable to watch: {}", watch_dir.display()))?;

    info!("watching directory for save files: {}", watch_dir.display());
    log::logger().flush();

    loop {
        match rx.recv() {
            Ok(DebouncedEvent::Error(e, path)) => {
                if let Some(path) = path {
                    warn!("watch error on {}: {:?}", path.as_path().display(), e);
                } else {
                    warn!("watch error: {:?}", e);
                }
            }
            Ok(DebouncedEvent::Write(path)) | Ok(DebouncedEvent::Create(path)) => {
                if !path.as_path().extension().map_or(false, |x| x == "eu4") {
                    continue;
                }

                let path_display = path.as_path().display();
                info!("detected write: {}", path_display);

                match process_file(client, &path) {
                    Ok(_) => info!("successfully uploaded {}", path_display),
                    Err(e) => warn!("{:?}", e),
                }
            }
            Ok(event) => {
                debug!("{:?}", event);
                continue;
            }
            Err(e) => warn!("watch error: {:?}", e),
        }

        log::logger().flush();
    }
}

fn process_file(client: &Client, path: &Path) -> anyhow::Result<()> {
    let path_display = path.display();
    let magic = {
        let mut buffer = [0; 4];
        let mut file =
            File::open(path).with_context(|| format!("unable to open: {}", path_display))?;
        file.read_exact(&mut buffer)
            .with_context(|| format!("unable to read: {}", path_display))?;
        buffer
    };

    match magic {
        [0x50, 0x4b, 0x03, 0x04] => client
            .upload_zip(&path)
            .with_context(|| format!("unable to upload zip: {}", path_display)),
        [b'E', b'U', b'4', b't'] => client
            .upload_txt(&path)
            .with_context(|| format!("unable to upload txt: {}", path_display)),
        x => Err(anyhow!(
            "unexpected file signature: {:?} - {}",
            x,
            path_display
        )),
    }
}
