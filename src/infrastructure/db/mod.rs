use log::info;
use sqlx::{Connection, SqlitePool, migrate::Migrator, sqlite::SqlitePoolOptions};

pub mod country_repository;
pub mod user_repository;

pub use country_repository::CountryRepository;
pub use user_repository::UserRepository;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub struct Database {}
impl Database {
    pub async fn initialize() -> SqlitePool {
        let database_url = "sqlite://db/database.db";

        {
            let mut conn = sqlx::SqliteConnection::connect(database_url)
                .await
                .expect("Failed to connect to database for setup");

            // Enable WAL mode (persists in the database file)
            sqlx::query("PRAGMA journal_mode = WAL;")
                .execute(&mut conn)
                .await
                .expect("Failed to enable WAL mode");
        }

        let pool = SqlitePoolOptions::new()
            .max_connections(25)
            .after_connect(|conn, _| {
                Box::pin(async move {
                    // Enable foreign keys on every new pooled connection
                    sqlx::query("PRAGMA foreign_keys = ON;")
                        .execute(&mut *conn)
                        .await?;
                    Ok(())
                })
            })
            .connect(database_url)
            .await
            .expect("Failed to connect to database.");

        // --- Run migrations ---
        MIGRATOR.run(&pool).await.expect("Failed to run migrations");

        info!("🎉 Database connected and migrations run successfully.");

        pool
    }
}
