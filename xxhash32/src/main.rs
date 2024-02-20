use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use libbruteforce::{
    symbols, BasicCrackParameter, CrackParameter, TargetHashAndHashFunction, TargetHashInput,
};
use xxhash32_lib::xxhash32_custom;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Hashes a single string.
    Hash { value: String },

    /// Hashes each line of a file and writes the results to a CSV output file.
    HashFile { file: PathBuf },

    /// Brute forces a hash, trying to find a string up to a given length that hashes to the given hash.
    BruteForce { hash: String, length: u32 },
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Hash { value } => {
            println!("{:#010X}", xxhash32_custom(value.as_bytes()));
        }
        Commands::HashFile { file: file_path } => {
            let file = File::open(file_path.clone()).expect("file not found");
            let file_reader = BufReader::new(file);

            // Sets output to the file name with .csv appended.
            let filename = file_path.file_stem().expect("file name not found");
            let output_filename = format!("{}.csv", filename.to_str().unwrap());
            let mut output = File::create(output_filename).expect("could not create output file");

            writeln!(output, "hash,value").unwrap();

            for line in file_reader.lines().map(|l| l.unwrap()) {
                let hash = xxhash32_custom(line.as_bytes());
                writeln!(output, "{:#010X},{}", hash, line).unwrap();
            }
        }
        Commands::BruteForce { hash, length } => {
            // Lazy, so I'm using libbruteforce to brute force the hash.
            // But xxhash32 has a lot of collisions, so it returns only the first solution found.

            let chars = symbols::Builder::new()
                .with_common_special_chars()
                .with_digits()
                .with_lc_letters()
                .with_uc_letters()
                .with_char('_')
                .build();

            let target: TargetHashAndHashFunction<u32> = TargetHashAndHashFunction::new(
                TargetHashInput::HashAsStr(hash),
                do_hash,
                str_to_hash,
                hash_to_str,
            );

            let res = libbruteforce::crack(CrackParameter::new(
                BasicCrackParameter::new(chars, *length, 0, true),
                target,
            ));

            if let Some(solution) = res.solution() {
                println!("Hash is: {}", solution);
                println!("Took {:.3}s", res.duration_in_seconds());
            } else {
                println!("No solution found");
            }
        }
    }
}

fn do_hash(input: &str) -> u32 {
    xxhash32_custom(input.as_bytes())
}

fn str_to_hash(string: &str) -> u32 {
    u32::from_str_radix(string, 16).unwrap()
}

fn hash_to_str(hash: &u32) -> String {
    format!("{:#010X}", hash)
}
