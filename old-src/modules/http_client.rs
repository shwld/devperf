use reqwest::{Client};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use http_cache_reqwest::{Cache, CacheMode, CACacheManager, HttpCache};

pub fn create_http_client() -> ClientWithMiddleware {
  let client = ClientBuilder::new(Client::new())
        .with(Cache(HttpCache {
            mode: CacheMode::Default,
            manager: CACacheManager::default(),
            options: None,
        }))
        .build();
  client
}
