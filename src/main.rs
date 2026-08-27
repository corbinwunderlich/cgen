use std::env;
use clang::Clang;

use crate::backends::{Backend, CHeader};

mod source;
mod backends;

fn main() {
    let mut args = env::args();

    args.next();

    if args.len() < 1 {
        eprintln!("Error: no path given");
        ()
    }

    let clang = Clang::new();

    if let Err(error) = clang {
        eprintln!("Error: failed to initialize Libclang: {}", error);

        return ();
    }

    let clang = clang.unwrap();

    if let Err(error) = args.try_for_each(|path| process_file(&clang, path)) {
        eprintln!("{}", error.to_string());
    }
}

fn process_file(clang: &Clang, path: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    let index = clang::Index::new(clang, false, false);

    let parser = index.parser(&path).parse();

    if let Err(error) = parser {
        return Err(format!("Error: failed to parse file {}: {}", path, error.to_string()).into())
    }

    let parser = parser.unwrap();

    let ranges = source::ranges_from_ast(&parser)
        .ok_or(format!("Error: failed to get source ranges from file {}", path))?;

    let header = CHeader::new(&path);

    header.write(header.generate_content(ranges))?;

    Ok(())
}
