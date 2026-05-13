use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use anyhow::Result;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS conversations (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                title TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                conversation_id TEXT NOT NULL,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (conversation_id) REFERENCES conversations(id)
            )
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    pub async fn save_message(
        &self,
        conversation_id: &str,
        id: &str,
        role: &str,
        content: &str,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO messages (id, conversation_id, role, content) VALUES (?, ?, ?, ?)",
        )
        .bind(id)
        .bind(conversation_id)
        .bind(role)
        .bind(content)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
