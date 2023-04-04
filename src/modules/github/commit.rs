pub async fn list(owner: &str, repo: &str) -> Result<Vec<octocrab::models::repos::RepoCommit>, octocrab::Error> {
    let crab = super::client::create_github_client().await;
    let page = crab.repos(owner, repo).list_commits().per_page(100).send().await?;
    let results = crab.all_pages(page).await?;

    Ok(results)
}
