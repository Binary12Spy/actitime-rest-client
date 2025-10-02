use crate::client_context::ClientContext;
use crate::errors::ClientError;
use crate::payload::{ApiError, UserWithAllowedActions};

pub fn get_users_me(client_context: &ClientContext) -> Result<UserWithAllowedActions, ClientError> {
    let url = format!("{}/users/me", client_context.base_url);

    if client_context.basic_auth_token.is_none() {
        return Err(ClientError::Unauthorized);
    }

    let response = client_context
        .http_client
        .get(&url)
        .send()
        .map_err(ClientError::Reqwest)?;

    let status_code = response.status().as_u16();
    let text = response.text().map_err(ClientError::Reqwest)?;
    match status_code {
        200 => {
            let user = UserWithAllowedActions::from_json(&text).map_err(ClientError::SerdeJson)?;
            Ok(user)
        }
        _ => {
            let api_error = ApiError::from_json(&text).map_err(ClientError::SerdeJson)?;
            Err(ClientError::ApiError(status_code, api_error))
        }
    }
}
