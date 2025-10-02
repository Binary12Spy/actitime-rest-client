use chrono::NaiveDateTime;

use crate::client_context::ClientContext;
use crate::errors::ClientError;
use crate::payload::{ApiError, Schedule};

pub fn get_users_schedule(
    client_context: &ClientContext,
    user_id: u64,
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
) -> Result<Schedule, ClientError> {
    let url = format!(
        "{}/users/{}/schedule?start={}&end={}",
        client_context.base_url,
        user_id,
        start_date.format("%Y-%m-%d"),
        end_date.format("%Y-%m-%d")
    );

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
            let user = Schedule::from_json(&text).map_err(ClientError::SerdeJson)?;
            Ok(user)
        }
        _ => {
            let api_error = ApiError::from_json(&text).map_err(ClientError::SerdeJson)?;
            Err(ClientError::ApiError(status_code, api_error))
        }
    }
}
