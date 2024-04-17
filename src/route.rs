use salvo::cors::{Cors, CorsHandler};
use salvo::http::Method;

#[macro_export]
macro_rules! create_router {
	($($m:ident)::*,$($method:ident),+) => {
		//Router::with_path(acquire_last_ident!($($m)*)).$method($($m)::*)
		$crate::create_router!(IN Router::with_path($crate::acquire_last_ident!($($m)*)), $($m)::* , $($method),+)
	};
	($prefix:literal,$($m:ident)::*,$($method:ident),+)=>{
		//Router::with_path(format!("{}{}",$prefix,acquire_last_ident!($($m)*))) $(. $method( $($m)::*  ))+
		$crate::create_router!(IN Router::with_path(format!("{}/{}",$prefix,$crate::acquire_last_ident!($($m)*))), $($m)::* , $($method),+)
	};
	(IN $e:expr, $m:path , $($method:ident),+)=>{
		$e $(.$method($m))+
	};
}

#[macro_export]
macro_rules! acquire_last_ident {
	($ide:ident $($ids:ident)+) => {
		$crate::acquire_last_ident!($($ids)+)
	};
	($ide:ident)=>{
		stringify!($ide)
	}
}

#[allow(dead_code)]
pub fn build_cros(allow_origin: &str) -> CorsHandler {
    Cors::new()
        .allow_origin(allow_origin)
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
            Method::PATCH,
        ])
        .into_handler()
}

#[macro_export]
macro_rules! define_route {
	($($e:expr),* $(,)?) => {
		{
			use ::salvo::prelude::*;
			Router::new() $(.push($e))*
		}
	};
}
