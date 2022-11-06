//! Database.

use error_stack::{IntoReport, Result, ResultExt};
use sqlx::{Pool, Postgres};

use crate::{Config, Error};

/// Connect pool and migrate database.
pub async fn connect(config: &Config) -> Result<Pool<Postgres>, Error> {
    let pool = Pool::connect(&config.database.uri)
        .await
        .into_report()
        .change_context(Error::Sqlx)?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .into_report()
        .change_context(Error::Sqlx)?;

    Ok(pool)
}
