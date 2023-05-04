use tokio::task;

pub async fn write_config(config: Config) -> Result<Config, confy::ConfyError> {
    task::spawn_blocking(|| {
        confy::store("devops-metrics-tools", None, config.clone())?;
        Ok(config)
    })
        .await
        .expect("Blocking task join error")
}
