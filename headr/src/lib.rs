use std::{fs::File, io::{self, BufRead, BufReader}, vec};

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(
        default_value = "-",
        help = "Input file(s)",
        value_name = "FILE",
        num_args = 0..,
    )]
    files: Vec<String>,
    #[arg(
        short = 'n',
        long,
        value_name = "LINES",
        default_value_t = 10,
        help = "Number of lines",
        value_parser = clap::value_parser!(u64).range(1..),
    )]
    lines: u64,
    #[arg(
        short = 'c',
        long,
        value_name = "BYTES",
        help = "Number of bytes",
        value_parser = clap::value_parser!(u64).range(1..),
        conflicts_with = "lines"
    )]
    bytes: Option<u64>,
}

pub fn run() -> MyResult<()> {
    let args = Args::parse();
    let num_files = args.files.len();
    
    for (i, filename) in args.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut reader) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if i > 0 { "\n" } else { "" },
                        filename
                    );
                }

                if let Some(bytes) = args.bytes {
                    let mut buffer = vec![0; bytes as usize];
                    let bytes_read = reader.read(&mut buffer)?;
                    print!(
                        "{}",
                        String::from_utf8_lossy(&buffer[..bytes_read])
                    )
                } else {
                    let mut line = String::new();
                    for _ in 0..args.lines {
                        let bytes = reader.read_line(&mut line)?;
                        if bytes == 0 {
                            break; // EOF
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}