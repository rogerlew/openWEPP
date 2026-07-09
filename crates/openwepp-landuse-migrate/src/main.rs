fn main() {
    match openwepp_landuse_migrate::cli::run_from_env() {
        Ok(output) => {
            if !output.is_empty() {
                print!("{output}");
            }
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
