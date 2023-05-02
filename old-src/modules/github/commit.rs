use octocrab::models::repos::RepoCommit;


pub async fn list(owner: &str, repo: &str) -> Result<Vec<RepoCommit>, octocrab::Error> {
    let crab = super::client::create_github_client().await;
    let page = crab.repos(owner, repo).list_commits().per_page(100).send().await?;
    let results = crab.all_pages(page).await?;

    Ok(results)
}

pub async fn get(owner: &str, repo: &str, sha: &str) -> Result<RepoCommit, octocrab::Error> {
    let crab = super::client::create_github_client().await;
    let commit: RepoCommit = crab.get(format!("/repos/{owner}/{repo}/commits/{ref}", owner = owner, repo = repo, ref = sha), None::<&()>).await?;

    Ok(commit)
}
