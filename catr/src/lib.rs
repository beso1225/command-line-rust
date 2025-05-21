use std::{fs::File, io::{self, BufRead, BufReader}};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version = "0.1.0",
    author = "My Name <myemailacount@example.eg>",
    about = "Rust Version of cat",
)]
struct Args {
    #[arg(
        value_name = "FILE",
        help = "Input file(s)",
        default_value = "-",
        num_args = 0..
    )]
    files: Vec<String>,

    #[arg(
        short = 'n',
        long = "number",
        help = "Number all output lines",
        conflicts_with = "number_nonblank"
    )]
    number: bool,

    #[arg(
        short = 'b',
        long = "number-nonblank",
        help = "Number nonempty output lines, overrides -n"
    )]
    number_nonblank: bool,
}

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn run() -> MyResult<()> {
    let args = Args::parse();
    print_file(&args)?;
    Ok(())
}

fn print_file(args: &Args) -> MyResult<()> {
    for filename in &args.files {
        match open(&filename) {
            Ok(reader) => {
                let mut counter = 1;
                for line in reader.lines() {
                    match line {
                        Ok(line) => {
                            if args.number {
                                println!("{:>6}\t{}", counter, line);
                                counter += 1;
                            } else if args.number_nonblank {
                                if !line.trim().is_empty() {
                                    println!("{:>6}\t{}", counter, line);
                                    counter += 1;
                                } else {
                                    println!("{}", line);
                                }
                            } else {
                                println!("{}", line);
                            }
                        }
                        Err(e) => {
                            eprintln!("Error reading line {}: {}", counter, e);
                        }
                    } {
                        
                    }
                }
            }
            Err(e) => {
                eprintln!("Error opening file {}: {}", filename, e);
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