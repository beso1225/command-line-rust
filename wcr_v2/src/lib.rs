use clap::{App, Arg};
use std::{error::Error, fs::File, io::{self, BufRead, BufReader}};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr_v2")
        .version("0.1.0")
        .author("Your Name <yourmail@example.com>")
        .about("rust version of wc with clap 2")
        .arg(
            Arg::with_name("files")
                .help("File(s) to input")
                .value_name("FILE")
                .default_value("-")
                .multiple(true)
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .help("Count lines")
                .takes_value(false) 
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .help("Count words")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("Count bytes")
                .takes_value(false)
                .conflicts_with("chars")
        )
        .arg(
            Arg::with_name("chars")
                .short("m")
                .long("chars")
                .help("Count characters")
                .takes_value(false)
                .conflicts_with("bytes")
        )
        .get_matches();
    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let mut chars = matches.is_present("chars");
    if [lines, words, bytes, chars].iter().all(|v| !v) {
        lines = true;
        words = true;
        bytes = true;
        chars = false;
    }
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total = FileInfo::new();
    for filename in &config.files {
        match open(filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(file) => {
                let file_info = count(file)?;
                println!(
                    "{}{}{}{}{}",
                    format_field(file_info.num_lines, config.lines),
                    format_field(file_info.num_words, config.words),
                    format_field(file_info.num_bytes, config.bytes),
                    format_field(file_info.num_chars, config.chars),
                    if filename == "-" { String::new() } else { format!(" {}", filename) }
                );
                total.add(&file_info);
            }
        }
    }
    if config.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total.num_lines, config.lines),
            format_field(total.num_words, config.words),
            format_field(total.num_bytes, config.bytes),
            format_field(total.num_chars, config.chars)
        );
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

impl FileInfo {
    fn new() -> Self {
        FileInfo {
            num_lines: 0,
            num_words: 0,
            num_bytes: 0,
            num_chars: 0,
        }
    }
    fn add(&mut self, other: &FileInfo) {
        self.num_lines += other.num_lines;
        self.num_words += other.num_words;
        self.num_bytes += other.num_bytes;
        self.num_chars += other.num_chars;
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut line = String::new();
    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break; // EOF
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear(); // Clear the line for the next read
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_count() {
        let text = "I don't want the world, I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10, // "I", "don't", "want", "the", "world,", "I", "just", "want", "your", "half."
            num_bytes: text.len(),
            num_chars: text.chars().count(),
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }
}