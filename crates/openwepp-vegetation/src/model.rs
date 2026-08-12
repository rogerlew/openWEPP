use sha2::{Digest, Sha256};

use crate::VegetationError;

pub const MODEL_VERSION: &str = "OPENWEPP_C3_WOODY_V1";
pub const MODEL_SHA256: &str = "003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157";
pub const MODEL_BYTES: &[u8] =
    include_bytes!("../model-registry/openwepp_c3_woody_v1_definition.json");

#[derive(Clone, Debug, PartialEq)]
pub struct ModelDefinition {
    pub version: &'static str,
    pub sha256: String,
    pub bytes: &'static [u8],
}

pub fn load_model_definition() -> Result<ModelDefinition, VegetationError> {
    let found = format!("{:x}", Sha256::digest(MODEL_BYTES));
    if found != MODEL_SHA256 {
        return Err(VegetationError::ModelDigestMismatch {
            expected: MODEL_SHA256.into(),
            found,
        });
    }
    let value: serde_json::Value = serde_json::from_slice(MODEL_BYTES)
        .map_err(|error| VegetationError::Schema(error.to_string()))?;
    if value
        .get("model_version")
        .and_then(serde_json::Value::as_str)
        != Some(MODEL_VERSION)
    {
        return Err(VegetationError::Schema(
            "model_version does not match registry identity".into(),
        ));
    }
    Ok(ModelDefinition {
        version: MODEL_VERSION,
        sha256: found,
        bytes: MODEL_BYTES,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_definition_has_frozen_identity() {
        let model = load_model_definition().expect("admitted model bytes");
        assert_eq!(model.sha256, MODEL_SHA256);
    }
}
