
mod hello;

use salvo::Router;
use serde_json::{json, Value};


use crate::web_core::{authorization::AuthGuard, error_catch::HttpErrorKind};


pub fn gen_router(_secret_key:String)->Router{
	let api_router = Router::new();
	let hello = Router::with_path("hello").get(hello::hello).hoop(AuthGuard::new(|_a|{
		HttpErrorKind::Json(json!({
			"status":"fail",
			"msg":Value::Null
		}))
	}));
	let login = Router::with_path("login").get(hello::login);
	api_router.push(hello).push(login)
}


