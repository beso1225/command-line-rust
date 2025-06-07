fn main() {
    if let Err(e) = headr::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
