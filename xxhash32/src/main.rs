use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use clap::Parser;
use xxhash32_lib::xxhash32_custom;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// String to hash
    name: Option<String>,

    /// File to hash
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
}

fn main() {
    let args = Cli::parse();

    if args.name.is_none() && args.file.is_none() {
        eprintln!("You must provide either a name or a file");
    }

    if let Some(name) = args.name {
        println!("{:#010X}", xxhash32_custom(&name));
    } else if let Some(file_path) = args.file {
        let file = File::open(file_path.clone()).expect("file not found");
        let file_reader = BufReader::new(file);

        // Sets output to the file name with .csv appended.
        let filename = file_path.file_stem().expect("file name not found");
        let output_filename = format!("{}.csv", filename.to_str().unwrap());
        let mut output = File::create(output_filename).expect("could not create output file");

        writeln!(output, "hash,value").unwrap();

        for line in file_reader.lines().map(|l| l.unwrap()) {
            let hash = xxhash32_custom(&line);
            writeln!(output, "{:#010X},{}", hash, line).unwrap();
        }
    }
}
