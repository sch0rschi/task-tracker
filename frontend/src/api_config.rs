use api_client::apis::configuration::Configuration;

pub fn config() -> Configuration {
    Configuration {
        base_path: "http://127.0.0.1:8080".to_string(),
        user_agent: None,
        client: reqwest::Client::new(),
        basic_auth: None,
        oauth_access_token: None,
        bearer_access_token: None,
        api_key: None,
    }
}
