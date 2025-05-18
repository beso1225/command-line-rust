use clap::{Command, Arg};

fn main() {
    // // println!(std::env::args()); // Does not work
    // // println!("{}", std::env::args()); // Also does not work because Args is not Display
    // println!("{:?}", std::env::args()); // This works because Args implements Debug

    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Ken Younens-Clark <kyclark@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .num_args(0),
        )
        .get_matches();

    println!("{:#?}", matches);
    let text= matches.get_many::<String>("text").unwrap().cloned().collect::<Vec<String>>();
    let omit_newline = matches.contains_id("omit_newline");

    // let mut ending = "\n";
    // if omit_newline {
    //     ending = "";
    // }
    // let ending = if omit_newline { "" } else { "\n" };
    // print!("{}{}", text.join(" "), ending);
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
