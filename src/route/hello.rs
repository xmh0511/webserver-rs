use crate::web_core::{
    config::Config,
    error_catch::{AnyResult, HttpErrorKind, Option2AnyHttpResult, Result2AnyHttpResult},
};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::web_core::authorization::gen_token;

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaim {
    username: String,
    exp: i64,
}

#[handler]
pub fn hello(req: &mut Request, res: &mut Response) -> AnyResult<()> {
    let c = req.query::<String>("id");
    println!("{c:?}");
    let p = req.query::<String>("id").to_result(|| {
        (
            400,
            HttpErrorKind::Json(json!({
                "status":"fail",
                "msg":"id not found"
            })),
        )
    })?;
    res.render(Text::Plain(format!("hello, {p}")));
    Ok(())
}

#[handler]
pub fn login(depot: &mut Depot, res: &mut Response) -> AnyResult<()> {
    let config = depot.obtain::<Config>().to_result(|| {
        (
            400,
            HttpErrorKind::Json(json!({
                "status":"fail",
                "msg":"config not found"
            })),
        )
    })?;
    let token = gen_token(
        config.secret_key.clone(),
        JwtClaim {
            exp: crate::expire_time!(Days(1)),
            username: "a".to_string(),
        },
    )
    .to_result(|e| {
        (
            400,
            HttpErrorKind::Json(json!({
                "status":"fail",
                "msg":e
            })),
        )
    })?;
    res.render(Text::Plain(token));
    Ok(())
}


pub mod ab{
    use salvo::handler;

	#[handler]
	pub fn show()->&'static str{
		"abc"
	}
}