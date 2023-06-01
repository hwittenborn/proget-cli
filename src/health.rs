use colored::Colorize;
use proget::{models::Status, Anon, Client};

pub(crate) async fn health(client: &Client<Anon>) -> exitcode::ExitCode {
    let health = match client.health().await {
        Ok(data) => data,
        Err(err) => {
            log::error!("Failed to obtain status information: {err}");
            return exitcode::TEMPFAIL;
        }
    };

    // App name + version.
    println!(
        "{} {}",
        health.application_name.cyan().bold(),
        health.version_number.bold()
    );

    // Service status.
    let service_status = if health.service_status == Status::Ok {
        "Good".green()
    } else if let Some(status) = health.service_status_detail.as_ref() {
        status.purple()
    } else {
        "Unknown".purple()
    };
    println!("Service Status: {service_status}");

    // Database status.
    let database_status = if health.database_status == Status::Ok {
        "Good".green()
    } else if let Some(status) = health.database_status_details.as_ref() {
        status.purple()
    } else {
        "Unknown".purple()
    };
    println!("Database Status: {database_status}");

    // License status.
    let license_status = if health.license_status == Status::Ok {
        "Good".green()
    } else if let Some(status) = health.license_status_detail.as_ref() {
        status.purple()
    } else {
        "Unknown".purple()
    };
    println!("License Status: {license_status}");

    // Extensions.
    if !health.extensions_installed.is_empty() {
        println!("Extensions:");

        for (extension, version) in health.extensions_installed.iter() {
            println!("  - {} ({version})", extension.cyan());
        }
    }

    exitcode::OK
}
