use salvo::cors::{Cors, CorsHandler};
use salvo::http::Method;

#[doc(hidden)]
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

#[doc(hidden)]
#[macro_export]
macro_rules! stringlize_path {
	($id:ident {$($t:tt)*})=>{
		concat!($($t)* stringify!($id))
	};
	($id:ident $($rest:ident)* {$($t:tt)*}) => {
		$crate::stringlize_path!($($rest)* {$($t)* stringify!($id),"/", })
	};
}

// #[macro_export]
// macro_rules! debug_route {
// 	([$($method:ident),+] => ... @ $($m:ident)::* $(/<**$rest:ident>)?) => {
// 		concat!($crate::stringlize_path!($($m)* {}))
// 	};
// }

/// Construct a router
/// > `/`<sub>opt</sub> `path1/path2/`<sub>opt</sub> @controller `/<**wildcard_path>`<sub>opt</sub>
#[macro_export]
macro_rules! router {
	([$($method:ident),+] => @ $($m:ident)::* $(/<**$rest:ident>)?) => {
		//Router::with_path(acquire_last_ident!($($m)*)).$method($($m)::*)
		$crate::router!(IN Router::with_path(format!($crate::gen_curly_brace!($($rest)?),$crate::acquire_last_ident!($($m)*),$(format!("/<**{}>",stringify!($rest)))?)), $($m)::* , $($method),+)
	};
	([$($method:ident),+] => ... @ $($m:ident)::* $(/<**$rest:ident>)?) => {
		$crate::router!(IN Router::with_path(format!($crate::gen_curly_brace!($($rest)?),concat!($crate::stringlize_path!($($m)* {})),$(format!("/<**{}>",stringify!($rest)))?)), $($m)::* , $($method),+)
	};
	([$($method:ident),+] => $(/)? $($prefix:ident)/+ / @ $($m:ident)::* $(/<**$rest:ident>)?)=>{
		//Router::with_path(format!("{}{}",$prefix,acquire_last_ident!($($m)*))) $(. $method( $($m)::*  ))+
		$crate::router!(IN Router::with_path(format!($crate::gen_curly_brace!(@ $($rest)?),concat!($(stringify!($prefix),stringify!(/)),+),$crate::acquire_last_ident!($($m)*), $(format!("/<**{}>",stringify!($rest)))?)), $($m)::* , $($method),+)
	};
	(IN $e:expr, $m:path , $($method:ident),+)=>{
		$e $(.$method($m))+
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! acquire_last_ident {
	($ide:ident $($ids:ident)+) => {
		$crate::acquire_last_ident!($($ids)+)
	};
	($ide:ident)=>{
		stringify!($ide).trim()
	}
}

/// Construct a middleware to allow Cross-origin
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

/// Start the service with the provided config and defined routers, optionally followed by a list of middleware(globally)
#[macro_export]
macro_rules! serve_routes {
	($c:expr => [$($e:expr),* $(,)?] $(& [$($hoop:expr),+ $(,)?])?) => {
		{
			use $crate::salvo::prelude::*;
			let router = Router::new() $($(.hoop($hoop))+)? $(.push($e))*;
			$crate::serve($c, router).await.unwrap();
		}
	};
}
