#[macro_export]
macro_rules! http_err {
    ($name:ident, $code:expr, $msg:ident) => {{
        let err = $crate::responder::ErrorMessage::new($code, $msg.to_string());
        let msg = rocket::serde::json::to_string(&err).expect("The serialization shouldn't fail");
        Self::$name(msg)
    }};
}
