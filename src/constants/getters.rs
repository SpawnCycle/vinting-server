use rocket::http::uri::Host;

use super::JWT_SECRET;

pub fn get_jwt_key() -> &'static str {
    JWT_SECRET.expect("Checked inside `LazyProcFairing`")
}

// Will do for now, the proper way would be to check the given host against a whitelist
pub fn construct_host(_host: &Host<'_>) -> String {
    "http://localhost:8000".to_string()
}
