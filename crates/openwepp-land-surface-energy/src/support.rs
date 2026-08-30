//! Sealed positive-support admission for the default-off V11 physical adopter.

#![allow(
    clippy::cast_precision_loss,
    clippy::missing_errors_doc,
    clippy::too_many_arguments
)]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    LandSurfaceEnergyConfiguration, LandSurfaceEnergyError, LandSurfaceEnergyState,
    MODEL_DEFINITION_SHA256, MODEL_VERSION, Sha256Digest,
};

pub const MINIMUM_SUPPORT_NS: u128 = 60_000_000_000;
const DOMAIN: &[u8] = b"OPENWEPP_LSE_SUPPORT_ADMISSION_V1\0";
const TOLERANCE_POLICY: &str = "energy_absolute=1e-6;energy_relative=1e-10";
const NUMERICAL_POLICY: &str = "iterations=50;backtracking=0..20;strict-decrease";

fn policy_digest(value: &str) -> Sha256Digest {
    Sha256Digest::try_new(format!("{:x}", Sha256::digest(value.as_bytes())))
        .expect("policy digest is always 64 lowercase hex")
}

fn canonical_digest(
    value: &LseSupportAdmissibilityReceiptV1,
) -> Result<Sha256Digest, LandSurfaceEnergyError> {
    // Serialize the declared wire struct directly. Converting through
    // `serde_json::Value` would reorder members through its map representation
    // and would not match the authority's frozen field-order KAT.
    let mut blank = value.clone();
    blank.receipt_sha256 = Sha256Digest::try_new("0".repeat(64))?;
    let mut bytes = serde_json::to_vec(&blank)
        .map_err(|_| LandSurfaceEnergyError::MalformedSerialization("support receipt".into()))?;
    let zero_digest = format!("\"receipt_sha256\":\"{}\"", "0".repeat(64));
    let empty_digest = "\"receipt_sha256\":\"\"";
    let text = std::str::from_utf8(&bytes)
        .map_err(|_| LandSurfaceEnergyError::MalformedSerialization("support receipt".into()))?;
    let replaced = text.replace(&zero_digest, empty_digest);
    if replaced.len() + 64 != text.len() || !replaced.ends_with("\"}") {
        return Err(LandSurfaceEnergyError::MalformedSerialization(
            "support receipt digest field".into(),
        ));
    }
    bytes = replaced.into_bytes();
    let mut preimage = DOMAIN.to_vec();
    preimage.extend_from_slice(&bytes);
    Sha256Digest::try_new(format!("{:x}", Sha256::digest(preimage)))
}

fn canonical_decimal(value: u128) -> String {
    value.to_string()
}

fn valid_hex(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn valid_decimal(value: &str, allow_zero: bool) -> bool {
    value == "0" && allow_zero
        || !value.is_empty()
            && value.as_bytes()[0] != b'0'
            && value.bytes().all(|b| b.is_ascii_digit())
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LseSupportAdmissibilityReceiptV1 {
    pub parent_transaction_id: String,
    pub segment_id: String,
    pub accepted_slab_id: String,
    pub slab_ordinal: String,
    pub support_start_ns: String,
    pub support_end_ns: String,
    pub model_version: String,
    pub model_definition_sha256: Sha256Digest,
    pub configuration_sha256: Sha256Digest,
    pub beginning_state_sha256: Sha256Digest,
    pub beginning_soil_thermal_state_sha256: Sha256Digest,
    pub tolerance_policy_sha256: Sha256Digest,
    pub numerical_policy_sha256: Sha256Digest,
    pub requested_support_ns: String,
    pub duration_s_bits: String,
    pub minimum_support_ns: String,
    pub receipt_sha256: Sha256Digest,
}

impl LseSupportAdmissibilityReceiptV1 {
    pub fn admit(
        configuration: &LandSurfaceEnergyConfiguration,
        beginning: &LandSurfaceEnergyState,
        parent_transaction_id: impl Into<String>,
        segment_id: impl Into<String>,
        accepted_slab_id: impl Into<String>,
        slab_ordinal: u32,
        support_start_ns: u128,
        support_end_ns: u128,
        duration_s_bits: u64,
        beginning_soil_thermal_state_sha256: Sha256Digest,
    ) -> Result<Self, LandSurfaceEnergyError> {
        let requested = support_end_ns.checked_sub(support_start_ns).ok_or(
            LandSurfaceEnergyError::SupportReceipt("support end precedes start"),
        )?;
        if requested == 0 || requested < MINIMUM_SUPPORT_NS {
            return Err(LandSurfaceEnergyError::SupportBelowMinimum {
                requested_ns: requested,
                minimum_ns: MINIMUM_SUPPORT_NS,
            });
        }
        let expected_bits = ((requested as f64) / 1_000_000_000.0).to_bits();
        if duration_s_bits != expected_bits {
            return Err(LandSurfaceEnergyError::SupportReceipt("duration bits"));
        }
        let mut receipt = Self {
            parent_transaction_id: parent_transaction_id.into(),
            segment_id: segment_id.into(),
            accepted_slab_id: accepted_slab_id.into(),
            slab_ordinal: slab_ordinal.to_string(),
            support_start_ns: canonical_decimal(support_start_ns),
            support_end_ns: canonical_decimal(support_end_ns),
            model_version: MODEL_VERSION.into(),
            model_definition_sha256: Sha256Digest::try_new(MODEL_DEFINITION_SHA256)?,
            configuration_sha256: configuration.configuration_sha256.clone(),
            beginning_state_sha256: beginning.state_sha256.clone(),
            beginning_soil_thermal_state_sha256,
            tolerance_policy_sha256: policy_digest(TOLERANCE_POLICY),
            numerical_policy_sha256: policy_digest(NUMERICAL_POLICY),
            requested_support_ns: canonical_decimal(requested),
            duration_s_bits: format!("{duration_s_bits:016x}"),
            minimum_support_ns: MINIMUM_SUPPORT_NS.to_string(),
            receipt_sha256: Sha256Digest::try_new("0".repeat(64))?,
        };
        receipt.receipt_sha256 = canonical_digest(&receipt)?;
        receipt.validate(
            configuration,
            beginning,
            &receipt.beginning_soil_thermal_state_sha256,
        )?;
        Ok(receipt)
    }

    pub fn validate(
        &self,
        configuration: &LandSurfaceEnergyConfiguration,
        beginning: &LandSurfaceEnergyState,
        expected_soil_thermal_state_sha256: &Sha256Digest,
    ) -> Result<(), LandSurfaceEnergyError> {
        if !valid_hex(&self.parent_transaction_id)
            || !valid_hex(&self.segment_id)
            || !valid_hex(&self.accepted_slab_id)
            || !valid_decimal(&self.slab_ordinal, true)
            || !valid_decimal(&self.support_start_ns, true)
            || !valid_decimal(&self.support_end_ns, false)
            || !valid_decimal(&self.requested_support_ns, false)
            || self.duration_s_bits.len() != 16
            || !self
                .duration_s_bits
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
        {
            return Err(LandSurfaceEnergyError::SupportReceipt(
                "noncanonical identity",
            ));
        }
        if self.model_version != MODEL_VERSION
            || self.model_definition_sha256.as_str() != MODEL_DEFINITION_SHA256
            || self.configuration_sha256 != configuration.configuration_sha256
            || self.beginning_state_sha256 != beginning.state_sha256
            || self.beginning_soil_thermal_state_sha256 != *expected_soil_thermal_state_sha256
            || self.tolerance_policy_sha256 != policy_digest(TOLERANCE_POLICY)
            || self.numerical_policy_sha256 != policy_digest(NUMERICAL_POLICY)
            || self.minimum_support_ns != MINIMUM_SUPPORT_NS.to_string()
        {
            return Err(LandSurfaceEnergyError::SupportReceipt("identity policy"));
        }
        let start = self
            .support_start_ns
            .parse::<u128>()
            .map_err(|_| LandSurfaceEnergyError::SupportReceipt("support start"))?;
        let end = self
            .support_end_ns
            .parse::<u128>()
            .map_err(|_| LandSurfaceEnergyError::SupportReceipt("support end"))?;
        let requested = self
            .requested_support_ns
            .parse::<u128>()
            .map_err(|_| LandSurfaceEnergyError::SupportReceipt("requested support"))?;
        if end.checked_sub(start) != Some(requested) {
            return Err(LandSurfaceEnergyError::SupportReceipt("support duration"));
        }
        if requested < MINIMUM_SUPPORT_NS {
            return Err(LandSurfaceEnergyError::SupportBelowMinimum {
                requested_ns: requested,
                minimum_ns: MINIMUM_SUPPORT_NS,
            });
        }
        let bits = u64::from_str_radix(&self.duration_s_bits, 16)
            .map_err(|_| LandSurfaceEnergyError::SupportReceipt("duration bits"))?;
        if bits != ((requested as f64) / 1_000_000_000.0).to_bits() {
            return Err(LandSurfaceEnergyError::SupportReceipt("duration bits"));
        }
        if canonical_digest(self)? != self.receipt_sha256 {
            return Err(LandSurfaceEnergyError::SupportReceipt("receipt digest"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn authority_configuration_and_state()
    -> (LandSurfaceEnergyConfiguration, LandSurfaceEnergyState) {
        let vectors: serde_json::Value = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../docs/work-packages/20260814-snow-free-land-surface-energy-authority-001/artifacts/openwepp_snow_free_lse_v1_vectors.json"
        )))
        .expect("frozen LSE authority vectors");
        let configuration =
            serde_json::from_value(vectors["strict_schema_instances"]["configuration"].clone())
                .expect("authority configuration");
        let state = serde_json::from_value(vectors["strict_schema_instances"]["state"].clone())
            .expect("authority state");
        (configuration, state)
    }

    #[test]
    fn decimal_and_hex_domains_are_closed() {
        assert!(valid_decimal("0", true));
        assert!(!valid_decimal("01", true));
        assert!(valid_hex(&format!("0{}", "a".repeat(63))));
    }

    #[test]
    fn frozen_authority_receipt_digest_and_forgery_are_exact() {
        let bytes = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../docs/work-packages/20260820-c3-woody-v11-segmented-support-001/",
            "artifacts/lse-support-admissibility-baseline.json"
        ));
        let baseline: LseSupportAdmissibilityReceiptV1 =
            serde_json::from_slice(bytes).expect("frozen receipt");
        assert_eq!(
            canonical_digest(&baseline).expect("authority digest"),
            baseline.receipt_sha256
        );
        assert_eq!(
            baseline.receipt_sha256.as_str(),
            "419058014c851ee854a7f432e458306c67cb2f4c640dfdfd0893e521429f54ae"
        );

        let mut forged = baseline.clone();
        forged.segment_id = "d".repeat(64);
        assert_ne!(
            canonical_digest(&forged).expect("forged digest"),
            baseline.receipt_sha256
        );
    }

    #[test]
    fn exact_sixty_second_support_floor_rejects_one_tick_below_and_admits_stable_larger_support() {
        let (configuration, state) = authority_configuration_and_state();
        let soil = Sha256Digest::try_new("a".repeat(64)).expect("soil digest");
        let admit = |duration_ns| {
            LseSupportAdmissibilityReceiptV1::admit(
                &configuration,
                &state,
                "1".repeat(64),
                "2".repeat(64),
                "3".repeat(64),
                0,
                17_u128,
                17_u128 + duration_ns,
                ((duration_ns as f64) / 1_000_000_000.0).to_bits(),
                soil.clone(),
            )
        };

        assert_eq!(
            admit(59_999_999_999),
            Err(LandSurfaceEnergyError::SupportBelowMinimum {
                requested_ns: 59_999_999_999,
                minimum_ns: 60_000_000_000,
            })
        );
        let exact = admit(60_000_000_000).expect("exact 60-second support");
        assert_eq!(exact.requested_support_ns, "60000000000");
        assert_eq!(exact.minimum_support_ns, "60000000000");
        exact
            .validate(&configuration, &state, &soil)
            .expect("exact-floor receipt replay");

        let ordinary = admit(120_000_000_000).expect("ordinary larger support");
        ordinary
            .validate(&configuration, &state, &soil)
            .expect("larger-support receipt replay");
        assert_ne!(ordinary.receipt_sha256, exact.receipt_sha256);
    }
}
