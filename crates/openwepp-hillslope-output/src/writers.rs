use std::path::PathBuf;

use crate::contracts::{HillslopeOutputConfig, configured_optional_outputs};

#[must_use]
pub fn required_output_paths(config: &HillslopeOutputConfig) -> [PathBuf; 2] {
    [config.pass.clone(), config.loss.clone()]
}

#[must_use]
pub fn optional_output_paths(config: &HillslopeOutputConfig) -> Vec<PathBuf> {
    configured_optional_outputs(config)
        .into_iter()
        .map(|(_, path)| path)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_config() -> HillslopeOutputConfig {
        HillslopeOutputConfig {
            pass: PathBuf::from("output/H1.hbp"),
            loss: PathBuf::from("output/H1.loss.json"),
            pass_parquet: Some(PathBuf::from("output/H1.pass.parquet")),
            wat: Some(PathBuf::from("output/H1.wat.parquet")),
            soil: None,
            plot: Some(PathBuf::from("output/H1.plot.parquet")),
            ebe: None,
            element: Some(PathBuf::from("output/H1.element.parquet")),
        }
    }

    #[test]
    fn writer_paths_include_required_outputs() {
        let config = sample_config();
        let required = required_output_paths(&config);

        assert_eq!(
            required,
            [
                PathBuf::from("output/H1.hbp"),
                PathBuf::from("output/H1.loss.json")
            ]
        );
    }

    #[test]
    fn writer_paths_include_only_configured_optional_outputs() {
        let config = sample_config();
        let optional = optional_output_paths(&config);

        assert_eq!(
            optional,
            vec![
                PathBuf::from("output/H1.pass.parquet"),
                PathBuf::from("output/H1.wat.parquet"),
                PathBuf::from("output/H1.plot.parquet"),
                PathBuf::from("output/H1.element.parquet"),
            ]
        );
    }
}
