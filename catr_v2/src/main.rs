fn main() {
    if let Err(e) = catr_v2::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
