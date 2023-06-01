#![allow(non_upper_case_globals)]

use chrono::{Duration, Utc};
use clap::Parser;

mod apps;
mod common_types;
mod dependencies;
mod metrics_retrieving;
mod project_creating;
mod shared;
#[cfg(test)]
mod tests;

use apps::cli::config::{get_config_path, ConfigAction};
use apps::cli::four_keys::get_four_keys;
use apps::cli::sub_commands::Action;
use apps::cli::{initializer, project};
use shared::{datetime_utc, setup_logger};

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    action: Action,

    #[clap(short, long, global = true, required = false)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    setup_logger::init(args.verbose);

    match args.action {
        Action::Init {} => {
            initializer::init::perform().await?;
        }
        Action::Config { sub_action } => match sub_action {
            ConfigAction::Get {} => {
                let config_path = get_config_path().expect("Could not get config path");

                println!("{:?}", config_path);
            }
        },
        Action::FourKeys {
            project,
            since,
            until,
        } => {
            let datetime_since = if let Some(since) = since {
                datetime_utc::parse(&since)
            } else {
                Ok(Utc::now() - Duration::days(90))
            }?;
            let datetime_until = if let Some(until) = until {
                datetime_utc::parse(&until)
            } else {
                Ok(Utc::now())
            }?;
            get_four_keys(&project, datetime_since, datetime_until).await?;
        }
        Action::Project { sub_action } => match sub_action {
            project::ProjectAction::Add {} => {
                project::add().await?;
            }
        },
    }
    Ok(())
}
