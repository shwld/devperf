use clap::Parser;

mod cli;
mod common_types;
mod metrics_retrieving;
mod project_creating;
mod logger;
mod github;

use cli::four_keys::get_four_keys;
use cli::initializer;
use cli::sub_commands::{Action};
use cli::config::{ConfigAction, get_config_path};

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    action: Action,

    #[clap(short, long, global = true, required = false)]
    verbose: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    logger::config::init(args.verbose);

    match args.action {
        Action::Init {  } => {
          initializer::init::perform();
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
          get_four_keys();
        }
    }
}
