use salvo::prelude::*;

use serde_json::Value;

/// The unified error process for HTTP. Using macros `json_err` and `html_err` to conveniently return errors
pub type HttpResult<T> = Result<T, AnyHttpError>;

#[allow(dead_code)]
pub enum HttpErrorKind {
    Json(Value),
    Html(String),
}

pub struct AnyHttpError(pub(crate) Option<StatusCode>, pub(crate) HttpErrorKind);

impl AnyHttpError {
    /// Construct an `AnyHttpError` by an HTTP status code and a specific error message
    #[allow(dead_code)]
    pub fn new(code: u16, msg: HttpErrorKind) -> Self {
        Self(
            Some(StatusCode::from_u16(code).unwrap_or(StatusCode::BAD_REQUEST)),
            msg,
        )
    }
    /// Construct an `AnyHttpError` by only a specific error message
    /// HTTP status is context-dependent
    pub fn new_msg(msg: HttpErrorKind) -> Self {
        Self(None, msg)
    }
}

#[async_trait]
impl Writer for AnyHttpError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.status_code(self.0.unwrap_or(StatusCode::BAD_REQUEST));
        match self.1 {
            HttpErrorKind::Json(v) => res.render(Text::Json(v.to_string())),
            HttpErrorKind::Html(v) => res.render(Text::Html(v)),
        }
    }
}

/// Respond to clients an error with JSON data structure
/// 1. Specify HTTP status code and the response data
/// > json_err!(code, data)
///
/// 2. Just specify the response data, the HTTP status code is context-dependent
/// > json_err!(data)
#[macro_export]
macro_rules! json_err {
	($status:expr, {$($t:tt)*}) => {
		{
			use $crate::web_core::http_error;
			http_error::AnyHttpError::new($status,http_error::HttpErrorKind::Json($crate::serde_json::json!({$($t)*})))
		}
	};
	({$($t:tt)*}) => {
		{
			use $crate::web_core::http_error;
			http_error::AnyHttpError::new_msg(http_error::HttpErrorKind::Json($crate::serde_json::json!({$($t)*})))
		}
	};
}

/// Respond to clients an error with html structure string
/// A similar usage as `json_err`
#[macro_export]
macro_rules! html_err {
    ($status:expr, $text:expr) => {{
        use $crate::web_core::http_error;
        http_error::AnyHttpError::new($status, http_error::HttpErrorKind::Html($text.into()))
    }};
    ($text:expr) => {{
        use $crate::web_core::http_error;
        http_error::AnyHttpError::new_msg(http_error::HttpErrorKind::Html($text.into()))
    }};
}
