fn main() {
    if let Err(e) = headr_v2::get_args().and_then(headr_v2::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
