use fake::{Fake};
use fake::locales::*;
use fake::faker::name::raw::*;
use fake::faker::internet::raw::*;
use fake::faker::address::raw::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use rayon::prelude::*;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use std::env;
use std::error::Error;

/// Generate a CSV file with random user data.
fn generate_csv(filename: &str, total_records: usize) -> Result<(), Box<dyn Error>> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    // Write CSV header
    writeln!(writer, "id,name,email,city")?;

    // Generate data in parallel
    let records: Vec<String> = (1..=total_records)
        .into_par_iter()
        .map(|id| {
            let name: String = Name(EN).fake();
            let email: String = SafeEmail(EN).fake();
            let city: String = CityName(EN).fake();
            format!("{},{},{},{}", id, name, email, city)
        })
        .collect();

    // Write data to file
    for record in records {
        writeln!(writer, "{}", record)?;
    }

    info!("✅ CSV generation complete! {} records written to '{}'", total_records, filename);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Predefined sample sizes
    let samples = vec![
        100_000, 500_000, 1_000_000, 3_000_000, 5_000_000, 
        10_000_000, 100_000_000
    ];

    // Check if CLI argument is given
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let custom_size: usize = args[1].parse().unwrap_or(0);
        if custom_size > 0 {
            generate_csv(&format!("data/sample_{}.csv", custom_size), custom_size)?;
            return Ok(());
        }
    }

    // Generate CSVs for predefined sample sizes
    for &size in &samples {
        let filename = format!("data/sample_{}.csv", size);
        generate_csv(&filename, size)?;
    }

    Ok(())
}
