use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WatershedOutputConfig {
    pub ebe_pw0: PathBuf,
    pub chan_out: PathBuf,
    pub chanwb: PathBuf,
    pub chnwb: PathBuf,
    pub soil_pw0: PathBuf,
    pub totalwatsed3: PathBuf,
    pub loss_hill: PathBuf,
    pub loss_chn: PathBuf,
    pub loss_out: PathBuf,
    pub loss_class_data: PathBuf,
    pub loss_all_years_hill: PathBuf,
    pub loss_all_years_chn: PathBuf,
    pub loss_all_years_out: PathBuf,
    pub loss_all_years_class_data: PathBuf,
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
}

impl OutputContractError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::MissingRequiredPath { .. } => "OWSOUT-E-001",
            Self::InvalidExtension { .. } => "OWSOUT-E-002",
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
        }
    }
}

impl std::error::Error for OutputContractError {}

pub fn validate_output_contract(config: &WatershedOutputConfig) -> Result<(), OutputContractError> {
    for (output_name, path) in required_output_paths(config) {
        validate_required_path(path, output_name, ".parquet")?;
    }
    Ok(())
}

#[must_use]
pub fn required_output_paths(config: &WatershedOutputConfig) -> [(&'static str, &Path); 14] {
    [
        ("ebe_pw0", &config.ebe_pw0),
        ("chan_out", &config.chan_out),
        ("chanwb", &config.chanwb),
        ("chnwb", &config.chnwb),
        ("soil_pw0", &config.soil_pw0),
        ("totalwatsed3", &config.totalwatsed3),
        ("loss_hill", &config.loss_hill),
        ("loss_chn", &config.loss_chn),
        ("loss_out", &config.loss_out),
        ("loss_class_data", &config.loss_class_data),
        ("loss_all_years_hill", &config.loss_all_years_hill),
        ("loss_all_years_chn", &config.loss_all_years_chn),
        ("loss_all_years_out", &config.loss_all_years_out),
        (
            "loss_all_years_class_data",
            &config.loss_all_years_class_data,
        ),
    ]
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

    fn valid_output_config() -> WatershedOutputConfig {
        WatershedOutputConfig {
            ebe_pw0: PathBuf::from("output/interchange/ebe_pw0.parquet"),
            chan_out: PathBuf::from("output/interchange/chan.out.parquet"),
            chanwb: PathBuf::from("output/interchange/chanwb.parquet"),
            chnwb: PathBuf::from("output/interchange/chnwb.parquet"),
            soil_pw0: PathBuf::from("output/interchange/soil_pw0.parquet"),
            totalwatsed3: PathBuf::from("output/interchange/totalwatsed3.parquet"),
            loss_hill: PathBuf::from("output/interchange/loss_pw0.hill.parquet"),
            loss_chn: PathBuf::from("output/interchange/loss_pw0.chn.parquet"),
            loss_out: PathBuf::from("output/interchange/loss_pw0.out.parquet"),
            loss_class_data: PathBuf::from("output/interchange/loss_pw0.class_data.parquet"),
            loss_all_years_hill: PathBuf::from(
                "output/interchange/loss_pw0.all_years.hill.parquet",
            ),
            loss_all_years_chn: PathBuf::from("output/interchange/loss_pw0.all_years.chn.parquet"),
            loss_all_years_out: PathBuf::from("output/interchange/loss_pw0.all_years.out.parquet"),
            loss_all_years_class_data: PathBuf::from(
                "output/interchange/loss_pw0.all_years.class_data.parquet",
            ),
        }
    }

    #[test]
    fn output_contract_accepts_required_parquet_outputs() {
        validate_output_contract(&valid_output_config()).expect("valid contract should pass");
    }

    #[test]
    fn output_contract_rejects_missing_required_output_path() {
        let mut config = valid_output_config();
        config.chanwb = PathBuf::new();

        let error =
            validate_output_contract(&config).expect_err("missing required output should fail");
        assert_eq!(error.code(), "OWSOUT-E-001");
        assert!(matches!(
            error,
            OutputContractError::MissingRequiredPath {
                output_name: "chanwb"
            }
        ));
    }

    #[test]
    fn output_contract_rejects_extension_mismatch() {
        let mut config = valid_output_config();
        config.totalwatsed3 = PathBuf::from("output/interchange/totalwatsed3.json");

        let error = validate_output_contract(&config).expect_err("invalid extension should fail");
        assert_eq!(error.code(), "OWSOUT-E-002");
        assert!(matches!(
            error,
            OutputContractError::InvalidExtension {
                output_name: "totalwatsed3",
                expected: ".parquet",
                ..
            }
        ));
    }
}
