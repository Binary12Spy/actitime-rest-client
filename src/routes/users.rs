use chrono::NaiveDateTime;

use crate::payload::{Schedule, UserWithAllowedActions, enums::Method};
use crate::routes::Route;

pub fn get_users_me() -> Route<(), UserWithAllowedActions> {
    Route::new(Method::GET, "/users/me")
}

pub fn get_users_schedule(
    user_id: u64,
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
) -> Route<(), Schedule> {
    let url = format!(
        "/users/{}/schedule?start={}&end={}",
        user_id,
        start_date.format("%Y-%m-%d"),
        end_date.format("%Y-%m-%d")
    );

    Route::new(Method::GET, &url)
}
