use clap::{error::ErrorKind, CommandFactory, Parser, Subcommand};
use log::LevelFilter;
use proget::Client;
use url::Url;

mod deb;
mod health;

#[derive(Parser)]
#[command(name = "ProGet CLI", version, about)]
struct Cli {
    /// The URL of the ProGet server.
    #[arg(long, env = "PROGET_SERVER", global = true)]
    proget_server: Option<Url>,
    /// The ProGet token to authenticate requests with.
    #[arg(long, env = "PROGET_TOKEN", global = true)]
    proget_token: Option<String>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, PartialEq)]
enum Command {
    /// Get health/status information.
    Health,
    /// Upload a '.deb' package.
    UploadDeb {
        /// The feed name to upload the package to.
        feed: String,

        /// The component name to upload the package in.
        component: String,

        /// The path to the '.deb' package to upload.
        deb_file: String,
    },
}

/// We put our stuff here so that we can cleanly call [`std::process::exit`], as recommended by
/// [`exitcode`].
async fn run() -> exitcode::ExitCode {
    // Set up logging.
    log::set_logger(&hw_msg::HwLogger).unwrap();
    log::set_max_level(LevelFilter::Info);

    // Parse the CLI.
    let cli = Cli::parse();

    // Contrary to our type definitions above, we actually want 'proget_server' to always be
    // required, and 'proget_token' to be required when we aren't running the 'health' CLI command.
    //
    // We can't make global arguments required though
    // (https://github.com/clap-rs/clap/issues/1546), so we have to do some runtime checking here
    // to make sure everything checks out the way we need it to.
    let mut missing_args = vec![];

    if cli.proget_server.is_none() {
        missing_args.push("--proget-server <PROGET_SERVER>");
    }
    if cli.command != Command::Health && cli.proget_token.is_none() {
        missing_args.push("--proget-token <PROGET_TOKEN>");
    }

    if !missing_args.is_empty() {
        // Based on https://github.com/clap-rs/clap/issues/1546#issuecomment-1441020614.
        let mut missing_args_string =
            "the following required arguments were not provided:\n  \x1b[32m".to_owned();
        missing_args_string += &missing_args.join("\n  ");
        missing_args_string += "\x1b[0m";

        Cli::command()
            .error(ErrorKind::MissingRequiredArgument, missing_args_string)
            .exit()
    }

    let proget_server = cli.proget_server.unwrap();
    let anon_client = if cli.command == Command::Health {
        Some(Client::new_anon(proget_server.clone()))
    } else {
        None
    };
    let auth_client = if cli.command != Command::Health {
        Some(Client::new_auth(proget_server, &cli.proget_token.unwrap()))
    } else {
        None
    };

    // Launch our CLI.
    match cli.command {
        Command::Health => health::health(&anon_client.unwrap()).await,
        Command::UploadDeb {
            feed,
            component,
            deb_file,
        } => deb::upload_deb(&auth_client.unwrap(), &feed, &component, &deb_file).await,
    }
}

#[tokio::main]
async fn main() {
    std::process::exit(run().await);
}
