use octocrab::{Octocrab};

pub async fn create_github_client() -> Octocrab {
    let config = crate::modules::config::load_config().await;
    if config.github_personal_token.is_none() {
        panic!("You must login first.");
    }
    let octocrab = Octocrab::builder()
        .personal_token(config.github_personal_token.expect("Could not get github personal token"))
        .build().expect("Could not create github client");

    octocrab
}
