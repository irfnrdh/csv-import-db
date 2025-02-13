use anyhow::Result;
use log::{info, error};

mod config;
mod database;
mod notifications;
mod importers;

use crate::config::Settings;
use database::{Database, postgres::PostgresDB, mysql::MySQLDB};
use notifications::telegram::TelegramNotifier;
use importers::csv_processor::CSVProcessor;
use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // env_logger::init();
    let settings = Settings::new()?;
    let mut databases: Vec<Arc<Mutex<Box<dyn Database>>>> = Vec::new();

    // Initialize databases
    for db_type in &settings.database.types {
        match db_type {
            config::DatabaseType::PostgreSQL => {
                let mut pg = PostgresDB::new(settings.database.clone());
                pg.connect().await?;
                databases.push(Arc::new(Mutex::new(Box::new(pg))));
            }
            config::DatabaseType::MySQL => {
                let mut mysql = MySQLDB::new(settings.database.clone());
                mysql.connect().await?;
                databases.push(Arc::new(Mutex::new(Box::new(mysql))));
            }
        }
    }

    // Initialize notifications
    let telegram = settings.notification.telegram_token
        .and_then(|token| settings.notification.telegram_chat_id.map(|chat_id| TelegramNotifier::new(token, chat_id)));

    // Start CSV processor
    let processor = CSVProcessor::new("./data");
    
    loop {
        match processor.watch().recv() {
            Ok(event) => {
                if let Ok(event) = event {
                    for path in event.paths {
                        if path.extension().and_then(|s| s.to_str()) == Some("csv") {
                            if let Err(e) = processor.process_file(&path).await {
                                eprintln!("Error processing file: {}", e);
                                if let Some(tg) = &telegram {
                                    tg.send(&format!("Error processing {}: {}", path.display(), e)).await?;
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("Watch error: {}", e),
        }
    }
}