pub async fn list(owner: &str, repo: &str) -> Result<Vec<octocrab::models::pulls::PullRequest>, octocrab::Error> {
    let crab = super::client::create_github_client().await;
    let page = crab.pulls(owner, repo).list().per_page(100).send().await?;
    let results = crab.all_pages(page).await?;

    Ok(results)
}
