use async_trait::async_trait;
use sqlx::{MySqlPool, PgPool};
use std::error::Error;

#[async_trait]
pub trait Database {
    async fn connect(&mut self) -> Result<(), Box<dyn Error>>;
    async fn insert_batch(&self, table: &str, columns: &[String], rows: &[Vec<String>]) -> Result<(), Box<dyn Error>>;
}

pub mod postgres;
pub mod mysql;