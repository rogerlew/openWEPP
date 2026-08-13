use sha2::{Digest, Sha256};

use crate::VegetationError;

pub const MODEL_VERSION: &str = "OPENWEPP_C3_WOODY_V5";
pub const MODEL_SHA256: &str = "0ee6a50d5f72da0b9344d8bf1b77674e95a66ab196edc068851bb419eb7b36f3";
pub const MODEL_BYTES: &[u8] =
    include_bytes!("../model-registry/openwepp_c3_woody_v5_definition.json");

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

    const RELEASED_V5_BYTES: &[u8] = include_bytes!(
        "../../../docs/work-packages/20260812-c3-woody-potential-pass-authority-001/artifacts/openwepp_c3_woody_v5_definition.json"
    );
    const HISTORICAL_V1_BYTES: &[u8] =
        include_bytes!("../model-registry/openwepp_c3_woody_v1_definition.json");
    const HISTORICAL_V2_BYTES: &[u8] =
        include_bytes!("../model-registry/openwepp_c3_woody_v2_definition.json");
    const HISTORICAL_V3_BYTES: &[u8] =
        include_bytes!("../model-registry/openwepp_c3_woody_v3_definition.json");
    const HISTORICAL_V4_BYTES: &[u8] =
        include_bytes!("../model-registry/openwepp_c3_woody_v4_definition.json");
    const HISTORICAL_V1_SHA256: &str =
        "003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157";
    const HISTORICAL_V2_SHA256: &str =
        "38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3";
    const HISTORICAL_V3_SHA256: &str =
        "7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852";
    const HISTORICAL_V4_SHA256: &str =
        "8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437";

    #[test]
    fn embedded_definition_has_exact_released_v5_identity() {
        let model = load_model_definition().expect("admitted model bytes");
        assert_eq!(model.version, "OPENWEPP_C3_WOODY_V5");
        assert_eq!(model.sha256, MODEL_SHA256);
        assert_eq!(model.bytes, RELEASED_V5_BYTES);
        assert_eq!(MODEL_BYTES, RELEASED_V5_BYTES);
    }

    #[test]
    fn historical_v1_definition_is_preserved_but_not_executable() {
        assert_eq!(
            format!("{:x}", Sha256::digest(HISTORICAL_V1_BYTES)),
            HISTORICAL_V1_SHA256
        );
        assert_ne!(HISTORICAL_V1_BYTES, MODEL_BYTES);

        let error = validate_model_definition(HISTORICAL_V1_BYTES, MODEL_VERSION, MODEL_SHA256)
            .expect_err("historical V1 bytes must not pass the V5 executable identity gate");
        assert_eq!(
            error,
            VegetationError::ModelDigestMismatch {
                expected: MODEL_SHA256.into(),
                found: HISTORICAL_V1_SHA256.into(),
            }
        );
    }

    #[test]
    fn historical_v2_definition_is_preserved_but_not_executable() {
        assert_eq!(
            format!("{:x}", Sha256::digest(HISTORICAL_V2_BYTES)),
            HISTORICAL_V2_SHA256
        );
        assert_ne!(HISTORICAL_V2_BYTES, MODEL_BYTES);
        assert!(
            validate_model_definition(HISTORICAL_V2_BYTES, MODEL_VERSION, MODEL_SHA256).is_err()
        );
    }

    #[test]
    fn historical_v3_definition_is_preserved_but_not_executable() {
        assert_eq!(
            format!("{:x}", Sha256::digest(HISTORICAL_V3_BYTES)),
            HISTORICAL_V3_SHA256
        );
        assert_ne!(HISTORICAL_V3_BYTES, MODEL_BYTES);

        let error = validate_model_definition(HISTORICAL_V3_BYTES, MODEL_VERSION, MODEL_SHA256)
            .expect_err("historical V3 bytes must not pass the V5 executable identity gate");
        assert_eq!(
            error,
            VegetationError::ModelDigestMismatch {
                expected: MODEL_SHA256.into(),
                found: HISTORICAL_V3_SHA256.into(),
            }
        );
    }

    #[test]
    fn historical_v4_definition_is_preserved_but_not_executable() {
        assert_eq!(
            format!("{:x}", Sha256::digest(HISTORICAL_V4_BYTES)),
            HISTORICAL_V4_SHA256
        );
        assert_ne!(HISTORICAL_V4_BYTES, MODEL_BYTES);

        let error = validate_model_definition(HISTORICAL_V4_BYTES, MODEL_VERSION, MODEL_SHA256)
            .expect_err("historical V4 bytes must not pass the V5 executable identity gate");
        assert_eq!(
            error,
            VegetationError::ModelDigestMismatch {
                expected: MODEL_SHA256.into(),
                found: HISTORICAL_V4_SHA256.into(),
            }
        );
    }
}
