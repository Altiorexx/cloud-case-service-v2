use reqwest::{header::{HeaderMap, HeaderValue, CONTENT_TYPE}, Client};
use std::env;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Instant, Duration};
use async_std::{task, sync::RwLock};

use super::token::TokenService;
use crate::types::user_service::{CheckTokenBody, CheckTokenErrorResponse};


pub struct CacheEntry {
    pub expires_at: Instant
}

pub struct UserService {
    client: Client,
    token: TokenService,
    domain: String,
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    ttl: Duration
}

impl UserService {

    pub fn new() -> Arc<Self> {
        let s = Arc::new(Self {
            client: Client::new(),
            token: TokenService::new(),
            domain: env::var("USER_SERVICE_DOMAIN").expect("USER_SERVICE_DOMAIN environment variable"),
            cache: Arc::new(RwLock::new(HashMap::new())),
            ttl: Duration::from_secs(300)
        });
        Self::start_cleanup_task(s.clone(), Duration::from_secs(600));
        s
    }

    // request user serivce for token verification
    pub async fn check(&self, token: String) -> Result<(), Box<dyn std::error::Error>> {
        
         // check cache if the token has been accepted recently
        if self.get(&token.to_string()).await {
            return Ok(())
        }

        // otherwise do a normal check
        let url = format!("{}/api/internal/check_user", self.domain);
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("X-Internal-Token", HeaderValue::from_str(&self.token.new_token(&self.domain)?)?);

        let response = self.client.post(url)
            .headers(headers)
            .json(&CheckTokenBody{token: token.to_string()})
            .send()
            .await?;

        match response.status().is_success() {
            true => {
                // add accepted token to cache
                self.set(&token.to_string()).await;
                Ok(())
            },
            false => {
                let err_body = response.json::<CheckTokenErrorResponse>().await?;
                Err(err_body.error.into())
            }
        }
    }

    async fn get(&self, token: &String) -> bool {
        let mut cache = self.cache.write().await;
        match cache.get(token) {
            Some(entry) => {
                if entry.expires_at > Instant::now() {
                    return true
                } else {
                    cache.remove(token);
                    return false
                }
            },
            None => false
        }
    }

    async fn set(&self, token: &String) {
        let mut cache = self.cache.write().await;
        cache.insert(token.to_string(), CacheEntry { expires_at: Instant::now() + self.ttl });
    }

    async fn cleanup_tokens(&self) {
        let mut cache = self.cache.write().await;
        let now = Instant::now();
        cache.retain(|_, entry| entry.expires_at > now);
    } 

    fn start_cleanup_task(self: Arc<UserService>, interval: Duration) {
        task::spawn(async move {
            println!("started user cache cleanup task");
            loop {
                println!("cleaning up idle tokens in user cache...");
                self.cleanup_tokens().await;
                task::sleep(interval).await;
            }
        });
    }

}