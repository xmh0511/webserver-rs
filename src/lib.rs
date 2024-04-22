#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod route;
pub mod web_core;

pub use assets::MemoryStream;
pub use web_core::config;
pub use web_core::*;

pub use web_core::http_error::*;

use salvo::prelude::*;

pub use config_file::FromConfigFile;
pub use route::build_cros;

pub use salvo;

pub use anyhow;
pub use chrono;
pub use futures;
pub use serde;
pub use serde_json;
pub use time;
pub use tokio;

#[cfg(feature = "http3")]
use salvo::conn::rustls::{Keycert, RustlsConfig};

pub async fn serve(config: config::Config, serve_route: Router) -> anyhow::Result<()> {
    tokio::fs::create_dir_all(config.pub_dir.clone()).await?;
    let config_provider = config::InjectConfig(config.clone());
    let _log_guard = log::set_log(config.log);

    #[cfg(any(feature = "mysql", feature = "sqlite", feature = "postgres"))]
    if let Some(v) = config.database {
        db::init_db_if_enable(v).await?;
    }

    let root_router = if config.route_root.is_empty() {
        Router::new().hoop(config_provider)
    } else {
        Router::with_path(config.route_root).hoop(config_provider)
    };
    let root_router = root_router
        .push(serve_route)
        .push(assets::common_assets(config.pub_dir, config.assets_listing));
    let root_router = Router::new()
        .push(Router::with_path("favicon.ico").get(assets::favicon_ico))
        .push(root_router);
    #[cfg(feature = "http3")]
    {
        let cert_and_key = web_core::config::read_cert_and_key(config.http3).await?;
        let tls_config = RustlsConfig::new(
            Keycert::new()
                .cert(cert_and_key.0.as_slice())
                .key(cert_and_key.1.as_slice()),
        );
        let listener = TcpListener::new(config.host.clone()).rustls(tls_config.clone());
        let acceptor = QuinnListener::new(tls_config, config.host)
            .join(listener)
            .bind()
            .await;
        Server::new(acceptor).serve(root_router).await;
    }
    #[cfg(not(feature = "http3"))]
    {
        let acceptor = TcpListener::new(config.host).bind().await;
        Server::new(acceptor).serve(root_router).await;
    }
    Ok(())
}

pub mod prelude {
    pub use crate::config::Config;
    pub use crate::http_error::HttpResult;
    pub use anyhow;
    pub use chrono;
    pub use config_file::FromConfigFile;
    pub use salvo;
    pub use serde;
    pub use serde_json;
    pub use time;
    pub use tokio;
}
