use jsonwebtoken::{self, EncodingKey};
use salvo::jwt_auth::{ConstDecoder, JwtTokenFinder};
use salvo::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::web_core::http_error::AnyHttpError;
use crate::HttpErrorKind;

#[allow(dead_code)]
/// Generate a jwt middleware with the provided parameters
pub fn gen_jwt_auth<T: Send + Sync + DeserializeOwned + 'static>(
    secret_key: String,
    finders: Vec<Box<dyn JwtTokenFinder>>,
) -> JwtAuth<T, ConstDecoder> {
    JwtAuth::new(ConstDecoder::from_secret(secret_key.as_bytes()))
        .finders(finders)
        .force_passed(true)
}

#[allow(dead_code)]
/// Generate a jwt with the provided parameters
pub fn gen_token<T: Serialize + Send + Sync + 'static>(
    secret_key: String,
    claim: T,
) -> Result<String, jsonwebtoken::errors::Error> {
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claim,
        &EncodingKey::from_secret(secret_key.as_bytes()),
    )
}

/// JWT Guard middleware that checks the validation of the request before entry of the handler
pub struct AuthGuard<F: Fn(JwtAuthState) -> AnyHttpError + Send + Sync + 'static> {
    f: F,
}
#[allow(dead_code)]
impl<F> AuthGuard<F>
where
    F: Fn(JwtAuthState) -> AnyHttpError + Send + Sync + 'static,
{
    /// Construct the guard with a handler on how to respond to the clients when validation failed
    pub fn new(f: F) -> Self {
        Self { f }
    }
}
#[async_trait]
impl<F> Handler for AuthGuard<F>
where
    F: Fn(JwtAuthState) -> AnyHttpError + Send + Sync + 'static,
{
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        match depot.jwt_auth_state() {
            JwtAuthState::Authorized => {
                ctrl.call_next(req, depot, res).await;
            }
            JwtAuthState::Unauthorized => {
                res.status_code(StatusCode::UNAUTHORIZED);
                match (self.f)(JwtAuthState::Unauthorized).1 {
                    HttpErrorKind::Html(v) => {
                        res.render(Text::Html(v));
                    }
                    HttpErrorKind::Json(v) => {
                        res.render(Text::Json(v.to_string()));
                    }
                };
                ctrl.skip_rest();
            }
            JwtAuthState::Forbidden => {
                res.status_code(StatusCode::FORBIDDEN);
                match (self.f)(JwtAuthState::Forbidden).1 {
                    HttpErrorKind::Html(v) => {
                        res.render(Text::Html(v));
                    }
                    HttpErrorKind::Json(v) => {
                        res.render(Text::Json(v.to_string()));
                    }
                };
                ctrl.skip_rest();
            }
        };
    }
}

/// Conveniently generate an expiration time for a JWT
#[macro_export]
macro_rules! expire_time {
    (Days($day:expr)) => {{
        use $crate::time::{Duration, OffsetDateTime};
        let tmp = OffsetDateTime::now_utc() + Duration::days($day);
        tmp.unix_timestamp()
    }};
    (Weeks($w:expr)) => {{
        use $crate::time::{Duration, OffsetDateTime};
        let tmp = OffsetDateTime::now_utc() + Duration::weeks($w);
        tmp.unix_timestamp()
    }};
    (Hours($h:expr)) => {{
        use $crate::time::{Duration, OffsetDateTime};
        let tmp = OffsetDateTime::now_utc() + Duration::hours($h);
        tmp.unix_timestamp()
    }};
    (Minutes($m:expr)) => {{
        use $crate::time::{Duration, OffsetDateTime};
        let tmp = OffsetDateTime::now_utc() + Duration::minutes($m);
        tmp.unix_timestamp()
    }};
    (Seconds($s:expr)) => {{
        use $crate::time::{Duration, OffsetDateTime};
        let tmp = OffsetDateTime::now_utc() + Duration::seconds($s);
        tmp.unix_timestamp()
    }};
}
