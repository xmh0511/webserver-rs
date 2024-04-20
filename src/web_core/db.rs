use crate::config::Database as DataBaseInfo;
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::{collections::HashMap, sync::OnceLock};

static DBCONN_POOL: OnceLock<HashMap<String, DatabaseConnection>> = OnceLock::new();

async fn init_db_pool(db_info: Vec<DataBaseInfo>) -> Result<(), DbErr> {
    let mut map = HashMap::new();
    for db in db_info {
        map.insert(db.name, Database::connect(db.protocol).await?);
    }
    _ = DBCONN_POOL.set(map);
    Ok(())
}
#[allow(dead_code)]
#[cfg(any(feature = "mysql", feature = "sqlite", feature = "postgres"))]
pub fn get(key: &str) -> Result<&'static DatabaseConnection, std::io::Error> {
    DBCONN_POOL
        .get()
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Database Connection Pool wasn't initialized when the program startup",
        ))?
        .get(key)
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("`{key}` is not found in Database Connection Pool"),
        ))
}

#[allow(dead_code)]
pub(crate) async fn init_db_if_enable(db_info: Vec<DataBaseInfo>) -> Result<(), DbErr> {
    if cfg!(feature = "mysql") || cfg!(feature = "sqlite") || cfg!(feature = "postgres") {
        init_db_pool(db_info).await
    } else {
        Ok(())
    }
}
