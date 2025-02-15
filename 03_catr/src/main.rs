use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `cat`. Concatenate and pring files
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number the output lines, starting at 1
    #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
    number_lines: bool,

    /// Number the non-blank output lines, starting at 1
    #[arg(short('b'), long("number-nonblank"), conflicts_with("number_lines"))]
    number_nonblank_lines: bool,
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(buffer) => {
                let mut num = 1;

                for line in buffer.lines() {
                    let line = line?;

                    if args.number_lines || (args.number_nonblank_lines && !line.is_empty()) {
                        println!("{:>6}\t{line}", num);
                        num += 1;
                    } else {
                        println!("{line}");
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
