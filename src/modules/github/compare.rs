pub async fn get_first_commit_committer_date(owner: &str, repo: &str, base: &str, head: &str) -> Result<chrono::DateTime<chrono::Utc>, octocrab::Error> {
    if base.is_empty() || head.is_empty() {
        return super::repo::get_created_at(owner, repo).await
    }
    let crab = super::client::create_github_client().await;
    let path = format!("https://api.github.com/repos/{owner}/{repo}/compare/{base}...{head}", owner = owner, repo = repo, base = base, head = head);
    let result = crab._get(path, None::<&()>).await?;
    let status = result.status();
    if status.is_success() == false {
        return super::repo::get_created_at(owner, repo).await
    }
    let res = result.json::<serde_json::Value>().await.expect("Could not parse response");
    let date = res.get("commits").expect("Could not get commits")[0]
        .get("commit").expect("Could not get commit")
        .get("committer").expect("Could not get committer")
        .get("date").expect("Could not get date")
        .as_str().expect("Could not get date as string");
    let parsed_date = chrono::DateTime::parse_from_rfc3339(date).expect("Could not parse date").with_timezone(&chrono::Utc);

    Ok(parsed_date)
}
