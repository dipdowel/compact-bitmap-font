pub fn print_verbose(msg: &str, verbose: bool) {
    if verbose {
        println!("[CBF] {msg}");
    }
}
