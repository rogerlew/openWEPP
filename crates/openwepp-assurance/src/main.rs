fn main() {
    match openwepp_assurance::cli::run_from_env() {
        Ok(output) => print!("{output}"),
        Err(error) => {
            eprintln!("ERROR: {error}");
            std::process::exit(error.exit_code());
        }
    }
}
