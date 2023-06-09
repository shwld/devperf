use clap::Subcommand;

use super::{config::ConfigAction, project::ProjectAction};

#[derive(Subcommand)]
pub enum Action {
    Config {
        #[clap(subcommand)]
        sub_action: ConfigAction,
    },
    Init {},
    FourKeys {
        #[clap(short, long, global = true, required = false)]
        since: Option<String>,

        #[clap(short, long, global = true, required = false)]
        until: Option<String>,

        #[clap(short, long, global = false, required = true)]
        project: String,
    },
    Project {
        #[clap(subcommand)]
        sub_action: ProjectAction,
    },
}
