use std::error::Error;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    // println!("Hello, world!");
    // dbg!(&config);

    // if config.files.len() == 1 {
    //     // if the number of files is 1, then read the file and print it
    //     let filename = &config.files[0];
    //     print_one_file(filename, &config)?;
    // } else {
    //     for filename in &config.files {
    //         // println!("{}", filename);
    //         // match open(&filename) {
    //         //     Err(err) => eprintln!("Failed to open {}: {}", filename, err),
    //         //     Ok(_) => println!("Opened {}", filename),
    //         // }
    //         if let Err(err) = open(&filename) {
    //             eprintln!("Failed to open {}: {}", filename, err);
    //             continue;
    //         }
    //     }
    // }
    for filename in &config.files {
        print_one_file(filename, &config)?;
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr_v2")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number all output lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number all output lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn print_lines(reader: Box<dyn BufRead>) -> MyResult<()> {
    for line in reader.lines() {
        match line {
            Err(err) => eprintln!("Error reading line: {}", err),
            Ok(line) => println!("{}", line),
        }
    }
    Ok(())
}

fn print_lines_with_numbers(reader: Box<dyn BufRead>) -> MyResult<()> {
    let mut line_number = 1;
    for line in reader.lines() {
        match line {
            Err(err) => eprintln!("Error reading line: {}", err),
            Ok(line) => {
                println!("     {}	{}", line_number, line);
                line_number += 1;
            }
        }
    }
    Ok(())
}

fn print_lines_with_numbers_and_nonblank(reader: Box<dyn BufRead>) -> MyResult<()> {
    let mut line_number = 1;
    for line in reader.lines() {
        match line {
            Err(err) => eprintln!("Error reading line: {}", err),
            Ok(line) => {
                if !line.is_empty() {
                    println!("     {}	{}", line_number, line);
                    line_number += 1;
                } else {
                    println!("");
                }
            }
        }
    }
    Ok(())
}

fn print_one_file(filename: &str, config: &Config) -> MyResult<()> {
    match open(&filename) {
        Err(err) => eprintln!("Failed to open {}: {}", filename, err),
        Ok(reader) => {
            if config.number_lines {
                print_lines_with_numbers(reader)?;
            } else if config.number_nonblank_lines {
                print_lines_with_numbers_and_nonblank(reader)?;
            } else {
                print_lines(reader)?;
            }
        }
    }
    Ok(())
}