//! Isolated execution of the actual test consumer; no alternative parser.
#[path = "../../../../tests/integration/support/sc_contract_text.rs"]
mod sc_contract_text;

fn main() -> std::io::Result<()> {
    let path = std::env::args_os().nth(1).ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "contract path required")
    })?;
    print!("{}", sc_contract_text::read(std::path::Path::new(&path))?);
    Ok(())
}
