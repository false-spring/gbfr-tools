use serde::Deserialize;
use std::{fs::File, io::BufReader};

use xxhash32_lib::xxhash32_custom;

fn read_all_rows(csv_writer: &mut csv::Writer<File>, msg: LanguageFile) -> Result<(), csv::Error> {
    csv_writer.write_record(["id", "id_hash_", "sub_id_hash_", "text_"])?;

    for row in msg.rows_ {
        let column = row.column_;
        csv_writer.write_record(&[
            format!("{:#010X}", xxhash32_custom(column.id_hash_.as_bytes())),
            column.id_hash_,
            column.subid_hash_,
            column.text_,
        ])?;
    }

    csv_writer.flush()?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct LanguageRowColumn {
    id_hash_: String,
    subid_hash_: String,
    text_: String,
}

#[derive(Debug, Deserialize)]
struct LanguageRow {
    column_: LanguageRowColumn,
}

#[derive(Debug, Deserialize)]
struct LanguageFile {
    rows_: Vec<LanguageRow>,
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("Usage: <file.msg>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let file = File::open(file_path).expect("file not found");
    let mut reader = BufReader::new(file);
    let msg: LanguageFile = rmp_serde::from_read(&mut reader).expect("Failed to read value");
    let mut csv_writer = csv::Writer::from_path("output.csv").expect("Failed to create CSV writer");

    read_all_rows(&mut csv_writer, msg).unwrap();
}
