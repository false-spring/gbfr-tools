use xxhash32::xxhash32_custom;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("Usage: <input>");
        std::process::exit(1);
    }

    let input = &args[1];
    let hash = xxhash32_custom(input);
    println!("{:#X}", hash);
}
