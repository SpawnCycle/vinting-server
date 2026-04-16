use rocket::{Request, catch, http::Status, serde::json::Json};

use super::ErrorMessage;

#[catch(default)]
pub fn default_responder(status: Status, _req: &Request) -> Json<ErrorMessage> {
    ErrorMessage::new(
        status.code,
        status.reason().unwrap_or(&status.code.to_string()),
    )
    .into()
}
