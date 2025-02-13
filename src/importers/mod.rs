use crate::{error::ImportError, models::Record};
use csv::Reader;
use log::{error, info};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use validator::Validate;

pub struct CsvImporter {
    watch_path: String,
    batch_size: usize,
}

impl CsvImporter {
    pub fn new(watch_path: String, batch_size: usize) -> Self {
        Self {
            watch_path,
            batch_size,
        }
    }

    pub fn start_watching(&self) -> Result<(), ImportError> {
        let (tx, rx) = std::sync::mpsc::channel();

        let mut watcher: RecommendedWatcher = Watcher::new(tx)?;
        watcher.watch(Path::new(&self.watch_path), RecursiveMode::Recursive)?;

        for result in rx {
            match result {
                Ok(event) => self.handle_event(event)?,
                Err(e) => error!("Watch error: {:?}", e),
            }
        }

        Ok(())
    }

    fn handle_event(&self, event: notify::Event) -> Result<(), ImportError> {
        match event {
            notify::Event::Create(path) => {
                if path.extension().map_or(false, |ext| ext == "csv") {
                    self.process_file(&path)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn process_file<P: AsRef<Path>>(&self, path: P) -> Result<(), ImportError> {
        let mut reader = Reader::from_path(path)?;
        let mut batch = Vec::with_capacity(self.batch_size);

        for result in reader.deserialize() {
            let record: Record = result?;
            
            // Validate record
            record.validate().map_err(|e| ImportError::Validation(e.to_string()))?;
            
            batch.push(record);

            if batch.len() >= self.batch_size {
                self.process_batch(batch.clone())?;
                batch.clear();
            }
        }

        // Process remaining records
        if !batch.is_empty() {
            self.process_batch(batch)?;
        }

        Ok(())
    }

    fn process_batch(&self, batch: Vec<Record>) -> Result<(), ImportError> {
        info!("Processing batch of {} records", batch.len());
        // Here you would typically send the batch to your database connector
        Ok(())
    }
}
