use salvo::cors::{Cors, CorsHandler};
use salvo::http::Method;

#[macro_export]
macro_rules! gen_curly_brace {
    ($i:ident) => {
        "{}{}"
    };
    () => {
        "{}"
    };
    (@ $i:ident) => {
        "{}{}{}"
    };
    (@) => {
        "{}{}"
    };
}

// #[macro_export]
// macro_rules! debug_route {
// 	([$($method:ident),+] => $($prefix:ident)/+ @ $($m:ident)::* $(<**$rest:ident>)?)=>{
// 		format!($crate::gen_curly_brace!(@ $($rest)?),concat!($(stringify!($prefix),stringify!(/)),+),$crate::acquire_last_ident!($($m)*), $(format!("/<**{}>",stringify!($rest)))?)
// 	};
// }

#[macro_export]
macro_rules! create_router {
	([$($method:ident),+] => $($m:ident)::* $(<**$rest:ident>)?) => {
		//Router::with_path(acquire_last_ident!($($m)*)).$method($($m)::*)
		$crate::create_router!(IN Router::with_path(format!($crate::gen_curly_brace!($($rest)?),$crate::acquire_last_ident!($($m)*),$(format!("/<**{}>",stringify!($rest)))?)), $($m)::* , $($method),+)
	};
	([$($method:ident),+] => $(/)? $($prefix:ident)/+ @ $($m:ident)::* $(/<**$rest:ident>)?)=>{
		//Router::with_path(format!("{}{}",$prefix,acquire_last_ident!($($m)*))) $(. $method( $($m)::*  ))+
		$crate::create_router!(IN Router::with_path(format!($crate::gen_curly_brace!(@ $($rest)?),concat!($(stringify!($prefix),stringify!(/)),+),$crate::acquire_last_ident!($($m)*), $(format!("/<**{}>",stringify!($rest)))?)), $($m)::* , $($method),+)
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
		stringify!($ide).trim()
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
macro_rules! serve_route {
	($c:expr => [$($e:expr),* $(,)?]) => {
		{
			use ::salvo::prelude::*;
			let router = Router::new() $(.push($e))*;
			$crate::serve($c, router).await.unwrap();
		}
	};
}
