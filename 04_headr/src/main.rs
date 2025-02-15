use anyhow::Result;
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `head`
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number of lines
    #[arg(short('n'), long, conflicts_with("bytes"), default_value = "10", value_parser = clap::value_parser!(u64).range(1..))]
    lines: u64,

    /// Number of bytes
    #[arg(short('c'), long, conflicts_with("lines"), value_parser = clap::value_parser!(u64).range(1..))]
    bytes: Option<u64>,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let num_of_files = args.files.len();
    for (file_num, filename) in args.files.iter().enumerate() {
        match open(&filename) {
            Ok(mut handle) => {
                if num_of_files > 1 {
                    println!("{}==> {filename} <==", if file_num > 0 { "\n" } else { "" });
                }

                if let Some(bytes) = args.bytes {
                    let mut buf = vec![0; bytes as usize];
                    let bytes_read = handle.read(&mut buf)?;
                    print!("{}", String::from_utf8_lossy(&buf[..bytes_read]));
                } else {
                    let mut buf = String::new();
                    for _ in 0..args.lines {
                        if let Ok(n) = handle.read_line(&mut buf) {
                            if n == 0 {
                                break;
                            }
                            print!("{}", buf);
                            buf.clear();
                        }
                    }
                }
            }
            Err(err) => eprintln!("Error opening {filename}: {err}"),
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
