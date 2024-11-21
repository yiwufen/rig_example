use std::env;

use rig::providers::openai;

pub fn create_client_from_env() -> openai::Client {
    let api_key = env::var("OPENAI_API_KEY").unwrap();
    let base_url = env::var("OPENAI_API_BASE").unwrap_or_else(|_| "https://api.openai.com".to_string());
    openai::Client::from_url(&api_key, &base_url)
}
