use base64::engine::{Engine, general_purpose::STANDARD as BASE64_STANDARD};
use reqwest::blocking::Client as HttpClient;

use crate::errors::ClientError;
use crate::payload::{ApiError, User, enums::Method};
use crate::routes::Route;

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

    pub fn call_route<PayloadType, ResponseType>(
        &self,
        client_context: &ClientContext,
        route: &Route<PayloadType, ResponseType>,
        payload: Option<&PayloadType>,
    ) -> Result<ResponseType, crate::errors::ClientError>
    where
        PayloadType: serde::Serialize + serde::de::DeserializeOwned,
        ResponseType: serde::Serialize + serde::de::DeserializeOwned,
    {
        if client_context.basic_auth_token.is_none() {
            return Err(ClientError::Unauthorized);
        }

        let url = format!("{}{}", self.base_url, route.relative_path);
        let request_builder = match route.method {
            Method::GET => self.http_client.get(&url),
            Method::POST => self.http_client.post(&url),
            Method::PUT => self.http_client.put(&url),
            Method::DELETE => self.http_client.delete(&url),
            Method::PATCH => self.http_client.patch(&url),
        };

        let request_builder = if let Some(ref payload) = payload {
            request_builder.json(payload)
        } else {
            request_builder
        };

        let response = request_builder.send().map_err(ClientError::Reqwest)?;

        let response_code = response.status().as_u16();
        if response.status().is_success() {
            let response_json = response.json().map_err(ClientError::Reqwest)?;
            let response = serde_json::from_value(response_json).map_err(ClientError::SerdeJson)?;
            Ok(response)
        } else {
            let response_json = response.text().unwrap_or_default();
            if !response_json.is_empty() {
                return Err(ClientError::Message(response_json));
            }
            let error =
                serde_json::from_str::<ApiError>(&response_json).map_err(ClientError::SerdeJson)?;
            Err(ClientError::ApiError(response_code, error))
        }
    }
}
