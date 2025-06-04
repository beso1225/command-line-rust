use clap::{App, Arg};
use std::{error::Error, fs::File, io::{self, BufRead, BufReader, Read}};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr_v2")
        .version("0.1.0")
        .author("Your Name <yourEmail@example.com")
        .about("Rust head")
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .takes_value(true)
                .conflicts_with("lines")
                .help("Number of bytes"),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .get_matches();

    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;
    
    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    // println!("{:#?}", config);
    let flag = if config.files.len() > 1 { true } else { false };
    for (num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}{}", filename, err, "".to_string() + if flag {"\n"} else {""}),
            Ok(reader) => {
                if flag {
                    println!(
                        "{}==> {} <==",
                        if num > 0 { "\n" } else { "" },
                        &filename
                    );
                }
                if let Err(err) = output_lines(reader, config.lines, config.bytes) {
                    eprintln!("{}: {}", filename, err);
                }
            }
        }
    }
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)), // Err(val.into()), Err(Into::into(val)),
    }
}

#[test]
fn test_paser_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn output_lines(mut reader: Box<dyn BufRead>, lines: usize, bytes: Option<usize>) -> MyResult<()> {
    match bytes {
        Some(b) => {
            let mut handle = reader.take(b as u64);
            let mut buffer = vec![0; b];
            let bytes_read = handle.read(&mut buffer)?;
            print!(
                "{}",
                String::from_utf8_lossy(&buffer[..bytes_read])
            );
            // let mut count = 0;
            // let mut output_bytes = Vec::new();
            // 'outer_loop: for line in reader.lines() {
            //     let line = line?;
            //     let line = line + "\n"; // Ensure we include the newline character
            //     let line_bytes = line.as_bytes();
            //     for &byte in line_bytes {
            //         if count < b {
            //             output_bytes.push(byte);
            //             count += 1;
            //         } else {
            //             break 'outer_loop;
            //         }
            //     }
            // }
            // if !output_bytes.is_empty() {
            //     print!("{}", String::from_utf8_lossy(&output_bytes));
            // }
        },
        None => {
            let mut line = String::new();
            for _ in 0..lines {
                let bytes = reader.read_line(&mut line)?;
                if bytes == 0 {
                    break; // EOF
                }
                print!("{}", line);
                line.clear();
            }
        }
    }
    Ok(())
}