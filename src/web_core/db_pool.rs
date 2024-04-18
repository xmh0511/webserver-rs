use crate::config::Database as DataBaseInfo;
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::{collections::HashMap, sync::OnceLock};

use super::error_catch::{AnyHttpError, AnyResult};

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
pub fn get_db_pool<F: Fn() -> AnyHttpError>(
    key: &String,
    if_error: F,
) -> AnyResult<&'static DatabaseConnection> {
    DBCONN_POOL
        .get()
        .ok_or(if_error())?
        .get(key)
        .ok_or(if_error())
}

#[allow(dead_code)]
pub async fn init_db_if_enable(db_info: Vec<DataBaseInfo>) -> Result<(), DbErr> {
    if cfg!(feature = "mysql") || cfg!(feature = "sqlite") || cfg!(feature = "postgres") {
        init_db_pool(db_info).await
    } else {
        Ok(())
    }
}
