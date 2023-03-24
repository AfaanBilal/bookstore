/**
 * 📕 BookStore
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/bookstore
 */
use sea_orm::*;

use crate::AppConfig;

pub(super) async fn connect(config: &AppConfig) -> Result<DatabaseConnection, DbErr> {
    let mut opts = ConnectOptions::new(format!(
        "mysql://{}:{}@{}:{}/{}",
        config.db_username, config.db_password, config.db_host, config.db_port, config.db_database,
    ));

    opts.sqlx_logging(false);

    Database::connect(opts).await
}
