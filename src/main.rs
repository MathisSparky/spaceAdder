use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() -> std::io::Result<()> {
    // Get the input and output file paths from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    // Open the input file
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    // Create the output file
    let file = File::create(output_file)?;
    let mut writer = BufWriter::new(file);

    // Process each line, adding a space at the beginning
    for line in reader.lines() {
        let line = line?;
        writeln!(writer, " {}", line)?;
    }

    Ok(())
}