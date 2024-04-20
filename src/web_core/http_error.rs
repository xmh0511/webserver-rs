use salvo::prelude::*;

use serde_json::Value;

pub type HttpResult<T> = Result<T, AnyHttpError>;

#[allow(dead_code)]
pub enum HttpErrorKind {
    Json(Value),
    Html(String),
}

pub struct AnyHttpError(pub(crate) Option<StatusCode>, pub(crate) HttpErrorKind);

impl AnyHttpError {
    #[allow(dead_code)]
    pub fn new(code: u16, msg: HttpErrorKind) -> Self {
        Self(
            Some(StatusCode::from_u16(code).unwrap_or(StatusCode::BAD_REQUEST)),
            msg,
        )
    }
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

#[macro_export]
macro_rules! json_err {
	($status:expr, {$($t:tt)*}) => {
		{
			use $crate::web_core::http_error;
			http_error::AnyHttpError::new($status,http_error::HttpErrorKind::Json(::serde_json::json!({$($t)*})))
		}
	};
	({$($t:tt)*}) => {
		{
			use $crate::web_core::http_error;
			http_error::AnyHttpError::new_msg(http_error::HttpErrorKind::Json(::serde_json::json!({$($t)*})))
		}
	};
}

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
