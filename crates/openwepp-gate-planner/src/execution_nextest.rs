//! Fail-closed derivation of execution-only Nextest configuration.

use std::path::Path;

use crate::error::{ErrorClass, GatePolicyError, Result};

const STORE_DECLARATION: &str = "dir = \"target/nextest\"";
const PUBLICATION_GROUP: &str = "[test-groups.assurance-publication]\nmax-threads = 4";
const QUALIFIED_PUBLICATION_GROUP: &str = "[test-groups.assurance-publication]\nmax-threads = 2";

pub(crate) fn derive_execution_config(source: &str, store: &Path) -> Result<String> {
    if source.matches(STORE_DECLARATION).count() != 1
        || source.matches(PUBLICATION_GROUP).count() != 1
    {
        return Err(config_error(
            "canonical Nextest store or publication-group declaration is missing or ambiguous",
        ));
    }
    let store = store
        .to_str()
        .ok_or_else(|| config_error("external store is non-UTF-8"))?;
    let encoded = serde_json::to_string(store).map_err(|error| config_error(error.to_string()))?;
    Ok(source
        .replacen(PUBLICATION_GROUP, QUALIFIED_PUBLICATION_GROUP, 1)
        .replacen(STORE_DECLARATION, &format!("dir = {encoded}"), 1))
}

fn config_error(message: impl Into<String>) -> GatePolicyError {
    GatePolicyError::new(ErrorClass::Execution, "GATE-EXEC-NEXTEST-CONFIG", message)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::derive_execution_config;

    #[test]
    fn config_derivation_is_strict_and_serializes_publication_cases() {
        let canonical =
            "dir = \"target/nextest\"\n[test-groups.assurance-publication]\nmax-threads = 4\n";
        let derived = derive_execution_config(canonical, Path::new("/tmp/store"))
            .expect("derive execution config");
        assert!(derived.contains("dir = \"/tmp/store\""));
        assert!(derived.contains("max-threads = 2"));
        assert_eq!(
            derive_execution_config(
                &canonical.replace("max-threads = 4", "max-threads = 3"),
                Path::new("/tmp/store"),
            )
            .expect_err("drift must fail")
            .code,
            "GATE-EXEC-NEXTEST-CONFIG"
        );
    }
}
