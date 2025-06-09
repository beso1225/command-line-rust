fn main() {
    if let Err(e) = wcr_v2::get_args().and_then(wcr_v2::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    } 
}
