use std::fs::File;
use std::io::BufReader;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} [input]", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];
    if input.ends_with(".xml") {
        let xml = BufReader::new(File::open(input).expect("failed to open file"));
        netsblox_ast::parse(xml).expect("failed to translate");
    }
    else {
        eprintln!("unknown input file type");
        std::process::exit(1);
    }
}