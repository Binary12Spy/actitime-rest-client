use std::default;

use base64::engine::{Engine, general_purpose::STANDARD as BASE64_STANDARD};
use reqwest::blocking::Client as HttpClient;

use crate::payload::User;

pub struct ClientContext {
    pub organization: String,
    pub base_url: String,
    pub basic_auth_token: Option<String>,
    pub user_information: Option<User>,
    pub http_client: HttpClient,
}

impl ClientContext {
    pub fn new(organization: String) -> Self {
        ClientContext {
            organization: organization.clone(),
            base_url: format!("https://online.actitime.com/{}/api/v1", organization).into(),
            basic_auth_token: None,
            user_information: None,
            http_client: HttpClient::new(),
        }
    }

    pub fn set_basic_auth_token(&mut self, username: &str, password: &str) {
        let token = BASE64_STANDARD.encode(format!("{}:{}", username, password));
        self.basic_auth_token = Some(token);

        let default_headers = {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Basic {}", self.basic_auth_token.as_ref().unwrap())
                    .parse()
                    .expect("Failed to parse authorization header"),
            );
            headers.insert(
                reqwest::header::CONTENT_TYPE,
                "application/json"
                    .parse()
                    .expect("Failed to parse content-type header"),
            );
            headers
        };

        self.http_client = HttpClient::builder()
            .timeout(std::time::Duration::from_secs(10))
            .default_headers(default_headers)
            .build()
            .expect("Failed to build HTTP client");
    }

    pub fn clear_basic_auth_token(&mut self) {
        self.basic_auth_token = None;
    }

    pub fn set_user_information(&mut self, user: User) {
        self.user_information = Some(user);
    }
}
