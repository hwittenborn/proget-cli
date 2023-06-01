use colored::Colorize;
use proget::{Auth, Client};
use std::fs;

pub(crate) async fn upload_deb(
    client: &Client<Auth>,
    feed: &str,
    component: &str,
    deb_file: &str,
) -> i32 {
    let deb_bytes = match fs::read(deb_file) {
        Ok(bytes) => bytes,
        Err(err) => {
            log::error!("Failed to read '{}': {err}", deb_file.green().bold());
            return exitcode::UNAVAILABLE;
        }
    };

    if let Err(err) = client
        .upload_deb(feed, component, deb_file, &deb_bytes)
        .await
    {
        log::error!(
            "Failed to upload '{}' to '{}' feed: {err}",
            deb_file.green().bold(),
            feed.green().bold()
        );
        return exitcode::UNAVAILABLE;
    }

    exitcode::OK
}
