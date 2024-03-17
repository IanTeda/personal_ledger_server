#[cfg(test)]
pub mod database {
    use personal_ledger_server::configuration::Database;
    use sqlx::{Connection, Executor, PgConnection, PgPool};
    use tracing::debug;

    pub async fn create_test_database(database_config: &Database) -> PgPool {
        debug!(
            "Test database config used to initiate random test database: {:?}",
            database_config
        );

        // Connect to database
        let mut connection = PgConnection::connect_with(&database_config.without_database_name())
            .await
            .expect("Failed to connect to random test database instance...");

        // Create random test database
        connection
            .execute(&*format!(
                r#"CREATE DATABASE "{}";"#,
                database_config.database_name
            ))
            .await
            .expect("Failed to create random test database...");

        // Connect to database pool using random test database
        let connection_pool = sqlx::PgPool::connect_with(database_config.with_database_name())
            .await
            .expect("Failed to connect to random test database connection pool...");

        // Apply database migrations to random test database
        sqlx::migrate!("./migrations")
            .run(&connection_pool)
            .await
            .expect("Failed to apply migrations to random test database...");

        connection_pool
    }

    pub async fn drop_test_database(database_config: &Database) {
        // Connect to database
        let mut connection = PgConnection::connect_with(&database_config.without_database_name())
            .await
            .expect("Failed to connect to database instance...");

        // Create random test database
        connection
            .execute(&*format!(
                r#"DROP DATABASE IF EXISTS "{}";"#,
                database_config.database_name
            ))
            .await
            .expect("Failed to drop random test database...");
    }
}
