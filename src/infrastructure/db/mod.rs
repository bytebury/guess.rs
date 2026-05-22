use log::info;
use sqlx::{
    SqlitePool,
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
};
use std::{env, path::PathBuf, str::FromStr};

pub mod breakout_repository;
pub mod user_repository;

pub use breakout_repository::BreakoutRepository;
pub use user_repository::UserRepository;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub struct Database {}
impl Database {
    pub async fn initialize() -> SqlitePool {
        let database_url =
            env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://db/database.db".to_string());

        if let Some(path) = sqlite_file_path(&database_url)
            && let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent)
                .unwrap_or_else(|e| panic!("Failed to create db directory {parent:?}: {e}"));
        }

        let connect_options = SqliteConnectOptions::from_str(&database_url)
            .expect("Invalid DATABASE_URL")
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .foreign_keys(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(25)
            .connect_with(connect_options)
            .await
            .expect("Failed to connect to database.");

        MIGRATOR.run(&pool).await.expect("Failed to run migrations");

        info!("🎉 Database connected and migrations run successfully.");

        pool
    }
}

/// Extract the filesystem path from a `sqlite://...` URL, if present.
/// Returns `None` for in-memory databases.
fn sqlite_file_path(url: &str) -> Option<PathBuf> {
    let without_scheme = url.strip_prefix("sqlite://").unwrap_or(url);
    let path = without_scheme.split('?').next().unwrap_or(without_scheme);
    if path.is_empty() || path == ":memory:" {
        return None;
    }
    Some(PathBuf::from(path))
}
