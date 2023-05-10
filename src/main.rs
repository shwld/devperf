use chrono::{Utc, Duration};
use clap::Parser;

mod cli;
mod common_types;
mod dependencies;
mod metrics_retrieving;
mod project_creating;
mod logger;
mod shared;

use cli::four_keys::get_four_keys;
use cli::initializer;
use cli::sub_commands::{Action};
use cli::config::{ConfigAction, get_config_path};
use shared::datetime_utc;

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
    logger::config::init(args.verbose);

    match args.action {
        Action::Init {  } => {
          initializer::init::perform().await?;
        },
        Action::Config { sub_action } => {
             match sub_action {
                ConfigAction::Get {} => {
                    let config_path = get_config_path().expect("Could not get config path");

                    println!("{:?}", config_path);
                },
            }
        },
        Action::FourKeys { project, since, until, environment } => {
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
            let environment = if let Some(environment) = environment {
                environment
            } else {
                "production".to_string()
            };
            let result = get_four_keys(&project, datetime_since, datetime_until, &environment).await?;
            println!("{:#?}", result);
        }
    }
    Ok(())
}
