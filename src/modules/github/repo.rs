pub async fn get_created_at(owner: &str, repo: &str) -> Result<chrono::DateTime<chrono::Utc>, octocrab::Error> {
    let crab = super::client::create_github_client().await;
    let result = crab.repos(owner, repo).get().await.expect("Could not get repo");
    let created_at = result.created_at.expect("Could not get created_at").with_timezone(&chrono::Utc);

    Ok(created_at)
}
