use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::Instant;
use clap::Parser;

/// A program to process files with uid and mobile numbers
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input file path
    #[arg(short, long)]
    input: String,
    
    /// Output file path
    #[arg(short, long)]
    output: String,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    
    // Start timing
    let start = Instant::now();
    
    // Open input file with 8MB buffer
    let input_file = File::open(&cli.input)?;
    let reader = BufReader::with_capacity(8 * 1024 * 1024, input_file);
    
    // Create output file with 8MB buffer
    let output_file = File::create(&cli.output)?;
    let mut writer = BufWriter::with_capacity(8 * 1024 * 1024, output_file);
    
    // Process first 100,000 lines
    for (i, line) in reader.lines().enumerate() {
        if i >= 100_000 {
            break;
        }
        
        if let Ok(line) = line {
            // Replace ---- with ,
            let processed_line = line.replace("----", ",");
            writeln!(writer, "{}", processed_line)?;
        }
    }
    
    // Ensure all data is written to disk
    writer.flush()?;
    
    // Calculate and display time taken
    let duration = start.elapsed();
    println!("Processing complete!");
    println!("Total time: {:.2?}", duration);
    println!("Processed 100,000 lines");
    
    Ok(())
}