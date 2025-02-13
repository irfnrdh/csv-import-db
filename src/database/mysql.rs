use super::Database;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::error::Error;

pub struct MySQLDB {
    pool: Option<MySqlPool>,
    config: crate::config::DatabaseConfig,
}

impl MySQLDB {
    pub fn new(config: crate::config::DatabaseConfig) -> Self {
        Self { pool: None, config }
    }
}

#[async_trait]
impl Database for MySQLDB {
    async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        let conn_string = format!(
            "mysql://{}:{}@{}:{}/{}",
            self.config.username,
            self.config.password,
            self.config.host,
            self.config.port,
            self.config.dbname
        );
        
        self.pool = Some(
            MySqlPoolOptions::new()
                .max_connections(5)
                .connect(&conn_string)
                .await?
        );
        Ok(())
    }

    async fn insert_batch(&self, table: &str, columns: &[String], rows: &[Vec<String>]) -> Result<(), Box<dyn Error>> {
        let pool = self.pool.as_ref().unwrap();
        let mut tx = pool.begin().await?;
        
        for row in rows.chunks(100) {
            let mut query = format!(
                "INSERT INTO {} ({}) VALUES ",
                table,
                columns.join(", ")
            );
            
            let values: Vec<String> = row.iter()
                .map(|r| format!("({})", r.iter().map(|v| format!("'{}'", v)).collect::<Vec<_>>().join(", ")))
                .collect();
                
            query.push_str(&values.join(", "));
            
            sqlx::query(&query).execute(&mut tx).await?;
        }
        
        tx.commit().await?;
        Ok(())
    }
}