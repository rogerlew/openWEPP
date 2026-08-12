use sha2::{Digest, Sha256};

use crate::VegetationError;

pub const MODEL_VERSION: &str = "OPENWEPP_C3_WOODY_V2";
pub const MODEL_SHA256: &str = "38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3";
pub const MODEL_BYTES: &[u8] =
    include_bytes!("../model-registry/openwepp_c3_woody_v2_definition.json");

#[derive(Clone, Debug, PartialEq)]
pub struct ModelDefinition {
    pub version: &'static str,
    pub sha256: String,
    pub bytes: &'static [u8],
}

fn validate_model_definition(
    bytes: &'static [u8],
    expected_version: &'static str,
    expected_sha256: &'static str,
) -> Result<ModelDefinition, VegetationError> {
    let found = format!("{:x}", Sha256::digest(bytes));
    if found != expected_sha256 {
        return Err(VegetationError::ModelDigestMismatch {
            expected: expected_sha256.into(),
            found,
        });
    }
    let value: serde_json::Value = serde_json::from_slice(bytes)
        .map_err(|error| VegetationError::Schema(error.to_string()))?;
    if value
        .get("model_version")
        .and_then(serde_json::Value::as_str)
        != Some(expected_version)
    {
        return Err(VegetationError::Schema(
            "model_version does not match registry identity".into(),
        ));
    }
    Ok(ModelDefinition {
        version: expected_version,
        sha256: found,
        bytes,
    })
}

pub fn load_model_definition() -> Result<ModelDefinition, VegetationError> {
    validate_model_definition(MODEL_BYTES, MODEL_VERSION, MODEL_SHA256)
}

#[cfg(test)]
mod tests {
    use super::*;

    const RELEASED_V2_BYTES: &[u8] = include_bytes!(
        "../../../docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/openwepp_c3_woody_v2_definition.json"
    );
    const HISTORICAL_V1_BYTES: &[u8] =
        include_bytes!("../model-registry/openwepp_c3_woody_v1_definition.json");
    const HISTORICAL_V1_SHA256: &str =
        "003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157";

    #[test]
    fn embedded_definition_has_exact_released_v2_identity() {
        let model = load_model_definition().expect("admitted model bytes");
        assert_eq!(model.version, "OPENWEPP_C3_WOODY_V2");
        assert_eq!(model.sha256, MODEL_SHA256);
        assert_eq!(model.bytes, RELEASED_V2_BYTES);
        assert_eq!(MODEL_BYTES, RELEASED_V2_BYTES);
    }

    #[test]
    fn historical_v1_definition_is_preserved_but_not_executable() {
        assert_eq!(
            format!("{:x}", Sha256::digest(HISTORICAL_V1_BYTES)),
            HISTORICAL_V1_SHA256
        );
        assert_ne!(HISTORICAL_V1_BYTES, MODEL_BYTES);

        let error = validate_model_definition(HISTORICAL_V1_BYTES, MODEL_VERSION, MODEL_SHA256)
            .expect_err("historical V1 bytes must not pass the V2 executable identity gate");
        assert_eq!(
            error,
            VegetationError::ModelDigestMismatch {
                expected: MODEL_SHA256.into(),
                found: HISTORICAL_V1_SHA256.into(),
            }
        );
    }
}
