
mod hello;

use salvo::Router;
use serde_json::json;


use crate::web_core::{authorization::AuthGuard, error_catch::HttpErrorKind};

#[allow(unused_macros)]
macro_rules! create_router {
	($($m:ident)::*,$method:ident) => {
		Router::with_path(acquire_last_ident!($($m)*)).$method($($m)::*)
	};
	($prefix:literal,$($m:ident)::*,$method:ident)=>{
		Router::with_path(format!("{}{}",$prefix,acquire_last_ident!($($m)*))).$method($($m)::*)
	};
}
#[allow(unused_macros)]
macro_rules! acquire_last_ident {
	($ide:ident $($ids:ident)+) => {
		acquire_last_ident!($($ids)+)
	};
	($ide:ident)=>{
		stringify!($ide)
	}
}



pub fn gen_router(_secret_key:String)->Router{
	let api_router = Router::new();
	let r = create_router!("a/b/",hello::ab::show,get);
	let hello = create_router!(hello::hello,get).hoop(AuthGuard::new(|_a|{
		HttpErrorKind::Json(json!({
			"status":"fail",
			"msg":"unauthorized"
		}))
	}));
	let login = Router::with_path("login").get(hello::login);
	api_router.push(hello).push(login).push(r)
}


