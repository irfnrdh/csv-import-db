use super::DatabaseConnector;
use crate::{error::ImportError, models::Record};
use async_trait::async_trait;
use postgres::{Client, NoTls};

pub struct PostgresConnector {
    client: Option<Client>,
    connection_string: String,
}

impl PostgresConnector {
    pub fn new(connection_string: String) -> Self {
        Self {
            client: None,
            connection_string,
        }
    }
}


#[async_trait]
impl DatabaseConnector for PostgresConnector {
    async fn connect(&self) -> Result<(), ImportError> {
        let mut this = self.to_owned();
        this.client = Some(Client::connect(&self.connection_string, NoTls)?);
        Ok(())
    }

    async fn insert_batch(&self, table: &str, records: Vec<Record>) -> Result<(), ImportError> {
        if let Some(client) = &self.client {
            for record in records {
                client.execute(
                    &format!("INSERT INTO {} (id, name, email) VALUES ($1, $2, $3)", table),
                    &[&record.id, &record.name, &record.email],
                )?;
            }
        }
        Ok(())
    }

    async fn disconnect(&self) -> Result<(), ImportError> {
        if let Some(mut client) = self.client.to_owned() {
            client.close()?;
        }
        Ok(())
    }
}
