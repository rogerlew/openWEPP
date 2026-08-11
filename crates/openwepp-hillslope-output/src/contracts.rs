use std::fmt;
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HillslopeOutputConfig {
    pub pass: PathBuf,
    pub loss: PathBuf,
    pub pass_parquet: Option<PathBuf>,
    pub wat: Option<PathBuf>,
    pub wat_subhourly: Option<PathBuf>,
    pub soil: Option<PathBuf>,
    pub plot: Option<PathBuf>,
    pub ebe: Option<PathBuf>,
    pub element: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputContractError {
    MissingRequiredPath {
        output_name: &'static str,
    },
    InvalidExtension {
        output_name: &'static str,
        expected: &'static str,
        observed: String,
    },
    DuplicateOutputPath {
        first_output_name: &'static str,
        second_output_name: &'static str,
        path: PathBuf,
    },
}

impl OutputContractError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::MissingRequiredPath { .. } => "OHOUT-E-001",
            Self::InvalidExtension { .. } => "OHOUT-E-002",
            Self::DuplicateOutputPath { .. } => "OHOUT-E-003",
        }
    }
}

impl fmt::Display for OutputContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRequiredPath { output_name } => {
                write!(
                    f,
                    "{} missing required output path {output_name}",
                    self.code()
                )
            }
            Self::InvalidExtension {
                output_name,
                expected,
                observed,
            } => write!(
                f,
                "{} output {output_name} expected extension {expected} but observed {observed}",
                self.code()
            ),
            Self::DuplicateOutputPath {
                first_output_name,
                second_output_name,
                path,
            } => write!(
                f,
                "{} outputs {first_output_name} and {second_output_name} resolve to the same path {}",
                self.code(),
                path.display()
            ),
        }
    }
}

impl std::error::Error for OutputContractError {}

pub fn validate_output_contract(config: &HillslopeOutputConfig) -> Result<(), OutputContractError> {
    validate_required_path(&config.pass, "pass", ".hbp")?;
    validate_required_path(&config.loss, "loss", ".json")?;

    let optional_outputs = configured_optional_outputs(config);
    for (name, path) in &optional_outputs {
        validate_extension(path, name, ".parquet")?;
    }
    for (index, (first_name, first_path)) in optional_outputs.iter().enumerate() {
        for (second_name, second_path) in optional_outputs.iter().skip(index + 1) {
            let normalized_first = lexically_normalize(first_path);
            let normalized_second = lexically_normalize(second_path);
            if normalized_first == normalized_second {
                return Err(OutputContractError::DuplicateOutputPath {
                    first_output_name: first_name,
                    second_output_name: second_name,
                    path: normalized_first,
                });
            }
        }
    }

    Ok(())
}

fn lexically_normalize(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if matches!(
                    normalized.components().next_back(),
                    Some(Component::Normal(_))
                ) {
                    normalized.pop();
                } else if !normalized.has_root() {
                    normalized.push(component.as_os_str());
                }
            }
            Component::Prefix(_) | Component::RootDir | Component::Normal(_) => {
                normalized.push(component.as_os_str());
            }
        }
    }
    normalized
}

#[must_use]
pub fn configured_optional_outputs(config: &HillslopeOutputConfig) -> Vec<(&'static str, PathBuf)> {
    let mut configured = Vec::new();

    for (name, path) in [
        ("pass_parquet", config.pass_parquet.clone()),
        ("wat", config.wat.clone()),
        ("wat_subhourly", config.wat_subhourly.clone()),
        ("soil", config.soil.clone()),
        ("plot", config.plot.clone()),
        ("ebe", config.ebe.clone()),
        ("element", config.element.clone()),
    ] {
        if let Some(path) = path {
            configured.push((name, path));
        }
    }

    configured
}

fn validate_required_path(
    path: &Path,
    output_name: &'static str,
    required_extension: &'static str,
) -> Result<(), OutputContractError> {
    if path.as_os_str().is_empty() {
        return Err(OutputContractError::MissingRequiredPath { output_name });
    }

    validate_extension(path, output_name, required_extension)
}

fn validate_extension(
    path: &Path,
    output_name: &'static str,
    required_extension: &'static str,
) -> Result<(), OutputContractError> {
    let expected = required_extension.trim_start_matches('.');
    let observed = path
        .extension()
        .and_then(|value| value.to_str())
        .map_or_else(|| "<none>".to_string(), ToString::to_string);

    if observed.eq_ignore_ascii_case(expected) {
        return Ok(());
    }

    Err(OutputContractError::InvalidExtension {
        output_name,
        expected: required_extension,
        observed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_output_config() -> HillslopeOutputConfig {
        HillslopeOutputConfig {
            pass: PathBuf::from("output/H1.hbp"),
            loss: PathBuf::from("output/H1.loss.json"),
            pass_parquet: Some(PathBuf::from("output/H1.pass.parquet")),
            wat: Some(PathBuf::from("output/H1.wat.parquet")),
            wat_subhourly: Some(PathBuf::from("output/H1.wat-subhourly.parquet")),
            soil: Some(PathBuf::from("output/H1.soil.parquet")),
            plot: Some(PathBuf::from("output/H1.plot.parquet")),
            ebe: Some(PathBuf::from("output/H1.ebe.parquet")),
            element: Some(PathBuf::from("output/H1.element.parquet")),
        }
    }

    #[test]
    fn output_contract_accepts_required_and_optional_output_extensions() {
        validate_output_contract(&valid_output_config()).expect("valid contract should pass");
    }

    #[test]
    fn output_contract_rejects_missing_required_pass_path() {
        let mut config = valid_output_config();
        config.pass = PathBuf::new();

        let error = validate_output_contract(&config).expect_err("missing pass path should fail");
        assert_eq!(error.code(), "OHOUT-E-001");
        assert!(matches!(
            error,
            OutputContractError::MissingRequiredPath {
                output_name: "pass"
            }
        ));
    }

    #[test]
    fn output_contract_rejects_required_output_extension_mismatch() {
        let mut config = valid_output_config();
        config.loss = PathBuf::from("output/H1.loss.txt");

        let error =
            validate_output_contract(&config).expect_err("invalid loss extension should fail");
        assert_eq!(error.code(), "OHOUT-E-002");
        assert!(matches!(
            error,
            OutputContractError::InvalidExtension {
                output_name: "loss",
                expected: ".json",
                ..
            }
        ));
    }

    #[test]
    fn output_contract_rejects_optional_output_extension_mismatch() {
        let mut config = valid_output_config();
        config.wat = Some(PathBuf::from("output/H1.wat.json"));

        let error =
            validate_output_contract(&config).expect_err("invalid optional extension should fail");
        assert_eq!(error.code(), "OHOUT-E-002");
        assert!(matches!(
            error,
            OutputContractError::InvalidExtension {
                output_name: "wat",
                expected: ".parquet",
                ..
            }
        ));
    }

    #[test]
    fn output_contract_reports_only_configured_optional_outputs() {
        let mut config = valid_output_config();
        config.soil = None;
        config.ebe = None;

        let configured = configured_optional_outputs(&config);
        let names: Vec<&str> = configured.iter().map(|(name, _)| *name).collect();
        assert_eq!(
            names,
            vec!["pass_parquet", "wat", "wat_subhourly", "plot", "element"]
        );
    }

    #[test]
    fn output_contract_rejects_wat5_aliases_before_any_writer_opens() {
        for aliased_name in ["pass_parquet", "wat"] {
            let mut config = valid_output_config();
            let shared = PathBuf::from("output/shared.parquet");
            config.wat_subhourly = Some(shared.clone());
            match aliased_name {
                "pass_parquet" => config.pass_parquet = Some(shared),
                "wat" => config.wat = Some(shared),
                _ => unreachable!("fixed test vector"),
            }
            let error = validate_output_contract(&config).expect_err("alias must fail");
            assert_eq!(error.code(), "OHOUT-E-003");
            assert!(matches!(
                error,
                OutputContractError::DuplicateOutputPath { .. }
            ));
        }
    }

    #[test]
    fn output_contract_rejects_lexical_parent_component_aliases() {
        let mut config = valid_output_config();
        config.wat = Some(PathBuf::from("/run/output/shared.parquet"));
        config.wat_subhourly = Some(PathBuf::from("/run/output/intermediate/../shared.parquet"));
        let error = validate_output_contract(&config).expect_err("lexical alias must fail");
        assert_eq!(error.code(), "OHOUT-E-003");
    }
}
