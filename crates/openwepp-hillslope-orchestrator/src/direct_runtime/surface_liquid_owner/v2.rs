//! Immutable SC-SURFACELIQUID-001 v14 surface-owner schema.
//!
//! This module is intentionally limited to owner identity, state, canonical
//! bytes, migration, and custody closure. Litter vapor and phase physics are
//! owned by the LSE V3 transaction and are not implemented here.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{ResourceOwnerId, TransactionId};
use openwepp_land_surface_energy::{SurfaceClass, WaterSourceType};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{
    DirectSurfaceLiquidClosureUnit, DirectSurfaceLiquidConfiguration,
    DirectSurfaceLiquidContinuationState, DirectSurfaceLiquidError, DirectSurfaceLiquidErrorCode,
    DirectSurfaceLiquidErrorContext, DirectSurfaceLiquidOwnedState, DirectSurfaceLiquidPhase,
    DirectSurfaceLiquidRollbackHashes, DirectSurfaceLiquidStateRecord, DirectSurfaceLiquidStoreKey,
    ZERO_SHA256, checked_surface_liquid_add, checked_surface_liquid_close,
    checked_surface_liquid_sub, f64_bits, is_sha256, parse_f64_bits,
};

const OWNER_V2_SCHEMA_NAME: &str = "OPENWEPP_SURFACE_LIQUID_OWNER_ENVELOPE_V2";
const OWNER_V2_MODEL_NAME: &str = "OPENWEPP_SNOW_FREE_FOREST_LITTER_SURFACE_OWNER_V2";
const WATER_DENSITY_KG_M3: f64 = 1000.0;
const LITTER_ICE_VOLUMETRIC_CAPACITY: f64 = 0.85;
const T_REF_K: f64 = 273.15;
const ICE_DENSITY_KG_M3: f64 = 920.0;
const ICE_HEAT_CAPACITY_J_KG_K: f64 = 2106.0;
const FUSION_ENTHALPY_J_KG: f64 = 333_700.0;
const PHASE_TIMESCALE_S: f64 = 3300.0;

const R156_SHA256: &str = "2a8c14d912651457bf9205a4a963b78dd12f1aa7f243bccb025e4b81ce99716d";
const ISBA_MEB_SHA256: &str = "0a300739b5dc660b61d29db144dd92f886e8fdf9934eac8facc022585992087a";
const ISBA_FLUXES_MEB_SHA256: &str =
    "e0378bc89ee0d52cffe14841aac56de1d8d379edf18ad29f24cfdb9ea0dfdbbc";
const INI_CSTS_SHA256: &str = "f39840df4d851efc70044f9e3ad62822371ed743c3c3a8055a4c940e2f86d73a";
const CECILL_C_LICENSE_SHA256: &str =
    "7280115e43fa03917f2f23370519be8c9fb0b57f4c86f8da5f7ac10c070f6aa0";

const FORMULA_IDENTITIES: [&str; 7] = [
    "frozen_fraction=empty?0:wi/(wl+wi)",
    "ice_capacity=0.85*rho_w*litter_depth",
    "signed_phase=freeze-melt",
    "u_end=u_star+lf*freeze-lf*melt",
    "t_end=t_ref+u_end/c_end",
    "liquid_vapor_enthalpy=cw*(t-t_ref)+lv(t)",
    "ice_vapor_enthalpy=ci*(t-t_ref)+ls(t)",
];
const ORDER_IDENTITY: &str =
    "beginning->phase_specific_vapor->bounded_phase->current_ingress->wb14";
const REFUSALS: [&str; 13] = [
    "zertol_cleanup",
    "xwgmin_regularization",
    "soil_compensation",
    "instantaneous_projection",
    "lower_bound_patch",
    "ice_as_wb14_supply",
    "current_ingress_donation",
    "freeze_only",
    "saturation_over_ice",
    "same_support_phase_resolve",
    "implicit_ice_initialization",
    "production_downgrade",
    "producer_residual_closure",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurfaceLiquidOwnerSourceIdentityV2 {
    r156: String,
    isba_meb: String,
    isba_fluxes_meb: String,
    ini_csts: String,
    cecill_c_license: String,
}

impl SurfaceLiquidOwnerSourceIdentityV2 {
    #[must_use]
    pub fn canonical() -> Self {
        Self {
            r156: R156_SHA256.into(),
            isba_meb: ISBA_MEB_SHA256.into(),
            isba_fluxes_meb: ISBA_FLUXES_MEB_SHA256.into(),
            ini_csts: INI_CSTS_SHA256.into(),
            cecill_c_license: CECILL_C_LICENSE_SHA256.into(),
        }
    }

    #[must_use]
    pub fn r156_sha256(&self) -> &str {
        &self.r156
    }

    #[must_use]
    pub fn isba_meb_sha256(&self) -> &str {
        &self.isba_meb
    }

    #[must_use]
    pub fn isba_fluxes_meb_sha256(&self) -> &str {
        &self.isba_fluxes_meb
    }

    #[must_use]
    pub fn ini_csts_sha256(&self) -> &str {
        &self.ini_csts
    }

    #[must_use]
    pub fn cecill_c_license_sha256(&self) -> &str {
        &self.cecill_c_license
    }

    fn validate(&self) -> Result<(), DirectSurfaceLiquidError> {
        if self != &Self::canonical() {
            return Err(DirectSurfaceLiquidError::Identity(
                "surface-owner V2 retained-source identity mismatch",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurfaceLiquidOwnerModelDefinitionV2 {
    schema_sha256: String,
    model_definition_sha256: String,
    surface_contract_sha256: String,
    lse_contract_sha256: String,
    parent_model_definition_sha256: String,
    sources: SurfaceLiquidOwnerSourceIdentityV2,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CanonicalSurfaceLiquidOwnerModelDefinitionV2 {
    schema_name: String,
    schema_sha256: String,
    model_name: String,
    model_definition_sha256: String,
    surface_contract_sha256: String,
    lse_contract_sha256: String,
    parent_model_definition_sha256: String,
    r156_sha256: String,
    isba_meb_sha256: String,
    isba_fluxes_meb_sha256: String,
    ini_csts_sha256: String,
    cecill_c_license_sha256: String,
    t_ref_k_bits: String,
    rho_i_kg_m3_bits: String,
    c_i_j_kg_k_bits: String,
    l_f_j_kg_bits: String,
    tau_ice_s_bits: String,
    water_density_kg_m3_bits: String,
    litter_ice_volumetric_capacity_bits: String,
    formula_identities: Vec<String>,
    order_identity: String,
    refusals: Vec<String>,
}

impl SurfaceLiquidOwnerModelDefinitionV2 {
    pub fn new(
        surface_contract_sha256: impl Into<String>,
        lse_contract_sha256: impl Into<String>,
        parent_model_definition_sha256: impl Into<String>,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        let mut value = Self {
            schema_sha256: sha256(OWNER_V2_SCHEMA_NAME.as_bytes()),
            model_definition_sha256: ZERO_SHA256.into(),
            surface_contract_sha256: surface_contract_sha256.into(),
            lse_contract_sha256: lse_contract_sha256.into(),
            parent_model_definition_sha256: parent_model_definition_sha256.into(),
            sources: SurfaceLiquidOwnerSourceIdentityV2::canonical(),
        };
        value.validate_input_hashes()?;
        value.model_definition_sha256 = value.recomputed_sha256()?;
        value.validate()?;
        Ok(value)
    }

    #[must_use]
    pub fn schema_sha256(&self) -> &str {
        &self.schema_sha256
    }

    #[must_use]
    pub fn model_definition_sha256(&self) -> &str {
        &self.model_definition_sha256
    }

    #[must_use]
    pub fn parent_model_definition_sha256(&self) -> &str {
        &self.parent_model_definition_sha256
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
        self.validate()?;
        self.canonical_bytes_with_digest(&self.model_definition_sha256)
    }

    fn validate_input_hashes(&self) -> Result<(), DirectSurfaceLiquidError> {
        if !is_sha256(&self.surface_contract_sha256)
            || !is_sha256(&self.lse_contract_sha256)
            || !is_sha256(&self.parent_model_definition_sha256)
            || self.surface_contract_sha256 == ZERO_SHA256
            || self.lse_contract_sha256 == ZERO_SHA256
            || self.parent_model_definition_sha256 == ZERO_SHA256
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "surface-owner V2 contract or parent model digest is invalid",
            ));
        }
        Ok(())
    }

    fn validate(&self) -> Result<(), DirectSurfaceLiquidError> {
        self.validate_input_hashes()?;
        self.sources.validate()?;
        if self.schema_sha256 != sha256(OWNER_V2_SCHEMA_NAME.as_bytes())
            || !is_sha256(&self.model_definition_sha256)
            || self.model_definition_sha256 == ZERO_SHA256
            || self.model_definition_sha256 != self.recomputed_sha256()?
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "surface-owner V2 schema or model-definition digest mismatch",
            ));
        }
        Ok(())
    }

    fn recomputed_sha256(&self) -> Result<String, DirectSurfaceLiquidError> {
        Ok(sha256(&self.canonical_bytes_with_digest(ZERO_SHA256)?))
    }

    fn canonical_bytes_with_digest(
        &self,
        digest: &str,
    ) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
        serde_json::to_vec(&CanonicalSurfaceLiquidOwnerModelDefinitionV2 {
            schema_name: OWNER_V2_SCHEMA_NAME.into(),
            schema_sha256: self.schema_sha256.clone(),
            model_name: OWNER_V2_MODEL_NAME.into(),
            model_definition_sha256: digest.into(),
            surface_contract_sha256: self.surface_contract_sha256.clone(),
            lse_contract_sha256: self.lse_contract_sha256.clone(),
            parent_model_definition_sha256: self.parent_model_definition_sha256.clone(),
            r156_sha256: self.sources.r156.clone(),
            isba_meb_sha256: self.sources.isba_meb.clone(),
            isba_fluxes_meb_sha256: self.sources.isba_fluxes_meb.clone(),
            ini_csts_sha256: self.sources.ini_csts.clone(),
            cecill_c_license_sha256: self.sources.cecill_c_license.clone(),
            t_ref_k_bits: f64_bits(T_REF_K),
            rho_i_kg_m3_bits: f64_bits(ICE_DENSITY_KG_M3),
            c_i_j_kg_k_bits: f64_bits(ICE_HEAT_CAPACITY_J_KG_K),
            l_f_j_kg_bits: f64_bits(FUSION_ENTHALPY_J_KG),
            tau_ice_s_bits: f64_bits(PHASE_TIMESCALE_S),
            water_density_kg_m3_bits: f64_bits(WATER_DENSITY_KG_M3),
            litter_ice_volumetric_capacity_bits: f64_bits(LITTER_ICE_VOLUMETRIC_CAPACITY),
            formula_identities: FORMULA_IDENTITIES.iter().map(ToString::to_string).collect(),
            order_identity: ORDER_IDENTITY.into(),
            refusals: REFUSALS.iter().map(ToString::to_string).collect(),
        })
        .map_err(|_| DirectSurfaceLiquidError::Schema("surface-owner V2 model serialization"))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceLiquidConfigurationRecordV2 {
    pub key: DirectSurfaceLiquidStoreKey,
    pub litter_depth_m: Option<f64>,
    pub litter_ice_capacity_kg_m2_tile: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceLiquidConfigurationV2 {
    parent: DirectSurfaceLiquidConfiguration,
    model_definition: SurfaceLiquidOwnerModelDefinitionV2,
    configuration_sha256: String,
    records: Vec<SurfaceLiquidConfigurationRecordV2>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CanonicalSurfaceLiquidConfigurationRecordV2 {
    key: DirectSurfaceLiquidStoreKey,
    litter_depth_m_bits: Option<String>,
    litter_ice_capacity_kg_m2_tile_bits: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CanonicalSurfaceLiquidConfigurationV2 {
    schema_sha256: String,
    model_definition_sha256: String,
    parent_configuration_sha256: String,
    parent_configuration_bytes_hex: String,
    configuration_sha256: String,
    records: Vec<CanonicalSurfaceLiquidConfigurationRecordV2>,
}

impl SurfaceLiquidConfigurationV2 {
    pub fn new(
        parent: DirectSurfaceLiquidConfiguration,
        model_definition: SurfaceLiquidOwnerModelDefinitionV2,
        litter_depth_m_by_key: &BTreeMap<DirectSurfaceLiquidStoreKey, f64>,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        parent.validate()?;
        model_definition.validate()?;
        let expected_litter_keys = parent
            .records
            .iter()
            .filter(|record| {
                record.key.surface_class == SurfaceClass::ForestLitter
                    && record.key.source_type == WaterSourceType::LitterLiquid
            })
            .map(|record| record.key.clone())
            .collect::<BTreeSet<_>>();
        if litter_depth_m_by_key
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>()
            != expected_litter_keys
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "surface-owner V2 litter-depth key set mismatch",
            ));
        }
        let records = parent
            .records
            .iter()
            .map(|record| {
                let depth = litter_depth_m_by_key.get(&record.key).copied();
                let capacity = match depth {
                    Some(depth) => {
                        require_positive_finite(depth, "surface-owner V2 litter depth")?;
                        let capacity = LITTER_ICE_VOLUMETRIC_CAPACITY * WATER_DENSITY_KG_M3 * depth;
                        require_positive_finite(capacity, "surface-owner V2 litter ice capacity")?;
                        Some(capacity)
                    }
                    None => None,
                };
                Ok(SurfaceLiquidConfigurationRecordV2 {
                    key: record.key.clone(),
                    litter_depth_m: depth,
                    litter_ice_capacity_kg_m2_tile: capacity,
                })
            })
            .collect::<Result<Vec<_>, DirectSurfaceLiquidError>>()?;
        let mut value = Self {
            parent,
            model_definition,
            configuration_sha256: ZERO_SHA256.into(),
            records,
        };
        value.configuration_sha256 = value.recomputed_sha256()?;
        value.validate()?;
        Ok(value)
    }

    #[must_use]
    pub const fn parent(&self) -> &DirectSurfaceLiquidConfiguration {
        &self.parent
    }

    #[must_use]
    pub const fn model_definition(&self) -> &SurfaceLiquidOwnerModelDefinitionV2 {
        &self.model_definition
    }

    #[must_use]
    pub fn configuration_sha256(&self) -> &str {
        &self.configuration_sha256
    }

    #[must_use]
    pub fn records(&self) -> &[SurfaceLiquidConfigurationRecordV2] {
        &self.records
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
        self.validate()?;
        self.canonical_bytes_with_digest(&self.configuration_sha256)
    }

    pub fn from_canonical_bytes(
        parent: DirectSurfaceLiquidConfiguration,
        model_definition: SurfaceLiquidOwnerModelDefinitionV2,
        bytes: &[u8],
    ) -> Result<Self, DirectSurfaceLiquidError> {
        let wire: CanonicalSurfaceLiquidConfigurationV2 = serde_json::from_slice(bytes)
            .map_err(|_| DirectSurfaceLiquidError::Schema("surface-owner V2 config parse"))?;
        let records = wire
            .records
            .into_iter()
            .map(|record| {
                Ok(SurfaceLiquidConfigurationRecordV2 {
                    key: record.key,
                    litter_depth_m: record
                        .litter_depth_m_bits
                        .as_deref()
                        .map(parse_f64_bits)
                        .transpose()?,
                    litter_ice_capacity_kg_m2_tile: record
                        .litter_ice_capacity_kg_m2_tile_bits
                        .as_deref()
                        .map(parse_f64_bits)
                        .transpose()?,
                })
            })
            .collect::<Result<Vec<_>, DirectSurfaceLiquidError>>()?;
        let value = Self {
            parent,
            model_definition,
            configuration_sha256: wire.configuration_sha256,
            records,
        };
        value.validate()?;
        if wire.schema_sha256 != value.model_definition.schema_sha256
            || wire.model_definition_sha256 != value.model_definition.model_definition_sha256
            || wire.parent_configuration_sha256 != value.parent.configuration_sha256
            || decode_hex(&wire.parent_configuration_bytes_hex)?
                != value.parent.canonical_bytes()?
            || value.canonical_bytes()? != bytes
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "surface-owner V2 configuration identity mismatch",
            ));
        }
        Ok(value)
    }

    fn validate(&self) -> Result<(), DirectSurfaceLiquidError> {
        self.parent.validate()?;
        self.model_definition.validate()?;
        if self.records.len() != self.parent.records.len() {
            return Err(DirectSurfaceLiquidError::Identity(
                "surface-owner V2 configuration cardinality mismatch",
            ));
        }
        for (record, parent) in self.records.iter().zip(&self.parent.records) {
            if record.key != parent.key {
                return Err(DirectSurfaceLiquidError::Identity(
                    "surface-owner V2 configuration key/order mismatch",
                ));
            }
            let is_litter = parent.key.surface_class == SurfaceClass::ForestLitter
                && parent.key.source_type == WaterSourceType::LitterLiquid;
            match (
                is_litter,
                record.litter_depth_m,
                record.litter_ice_capacity_kg_m2_tile,
            ) {
                (true, Some(depth), Some(capacity)) => {
                    require_positive_finite(depth, "surface-owner V2 litter depth")?;
                    let expected = LITTER_ICE_VOLUMETRIC_CAPACITY * WATER_DENSITY_KG_M3 * depth;
                    if capacity.to_bits() != expected.to_bits() {
                        return Err(DirectSurfaceLiquidError::Identity(
                            "surface-owner V2 ice capacity identity mismatch",
                        ));
                    }
                }
                (false, None, None) => {}
                _ => {
                    return Err(DirectSurfaceLiquidError::Identity(
                        "surface-owner V2 ice configuration on inadmissible surface",
                    ));
                }
            }
        }
        if !is_sha256(&self.configuration_sha256)
            || self.configuration_sha256 == ZERO_SHA256
            || self.configuration_sha256 != self.recomputed_sha256()?
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "surface-owner V2 configuration digest mismatch",
            ));
        }
        Ok(())
    }

    fn record(
        &self,
        key: &DirectSurfaceLiquidStoreKey,
    ) -> Option<&SurfaceLiquidConfigurationRecordV2> {
        self.records.iter().find(|record| &record.key == key)
    }

    fn recomputed_sha256(&self) -> Result<String, DirectSurfaceLiquidError> {
        Ok(sha256(&self.canonical_bytes_with_digest(ZERO_SHA256)?))
    }

    fn canonical_bytes_with_digest(
        &self,
        digest: &str,
    ) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
        let records = self
            .records
            .iter()
            .map(|record| CanonicalSurfaceLiquidConfigurationRecordV2 {
                key: record.key.clone(),
                litter_depth_m_bits: record.litter_depth_m.map(f64_bits),
                litter_ice_capacity_kg_m2_tile_bits: record
                    .litter_ice_capacity_kg_m2_tile
                    .map(f64_bits),
            })
            .collect();
        serde_json::to_vec(&CanonicalSurfaceLiquidConfigurationV2 {
            schema_sha256: self.model_definition.schema_sha256.clone(),
            model_definition_sha256: self.model_definition.model_definition_sha256.clone(),
            parent_configuration_sha256: self.parent.configuration_sha256.clone(),
            parent_configuration_bytes_hex: encode_hex(&self.parent.canonical_bytes()?),
            configuration_sha256: digest.into(),
            records,
        })
        .map_err(|_| DirectSurfaceLiquidError::Schema("surface-owner V2 config serialization"))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceLiquidStateRecordV2 {
    pub key: DirectSurfaceLiquidStoreKey,
    pub liquid_kg_m2_tile: f64,
    pub litter_ice_kg_m2_tile: f64,
    pub surface_enthalpy_j_m2_tile: f64,
    pub last_accepted_transaction_id: Option<TransactionId>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceLiquidOwnedStateV2 {
    owner_id: ResourceOwnerId,
    configuration_sha256: String,
    model_definition_sha256: String,
    state_sha256: String,
    records: Vec<SurfaceLiquidStateRecordV2>,
    continuations: Vec<DirectSurfaceLiquidContinuationState>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CanonicalSurfaceLiquidStateRecordV2 {
    key: DirectSurfaceLiquidStoreKey,
    liquid_kg_m2_tile_bits: String,
    litter_ice_kg_m2_tile_bits: String,
    surface_enthalpy_j_m2_tile_bits: String,
    last_accepted_transaction_id: Option<TransactionId>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CanonicalSurfaceLiquidOwnedStateV2 {
    owner_id: ResourceOwnerId,
    configuration_sha256: String,
    model_definition_sha256: String,
    state_sha256: String,
    records: Vec<CanonicalSurfaceLiquidStateRecordV2>,
    continuations: Vec<super::CanonicalSurfaceLiquidContinuationState>,
}

/// Performs the one-way checked migration from the frozen V1 owner state.
///
/// The successor initializes every litter-ice field to exact positive zero and
/// requires explicit enthalpy operands. Production intentionally exposes no
/// inverse migration or downgrade path.
pub fn migrate_v1_to_v2(
    configuration: &SurfaceLiquidConfigurationV2,
    state: &DirectSurfaceLiquidOwnedState,
    surface_enthalpy_by_key: &BTreeMap<DirectSurfaceLiquidStoreKey, f64>,
) -> Result<SurfaceLiquidOwnedStateV2, DirectSurfaceLiquidError> {
    SurfaceLiquidOwnedStateV2::migrate_from_v1(configuration, state, surface_enthalpy_by_key)
}

impl SurfaceLiquidOwnedStateV2 {
    pub fn new_initial(
        configuration: &SurfaceLiquidConfigurationV2,
        liquid_by_key: &BTreeMap<DirectSurfaceLiquidStoreKey, f64>,
        litter_ice_by_key: &BTreeMap<DirectSurfaceLiquidStoreKey, f64>,
        surface_enthalpy_by_key: &BTreeMap<DirectSurfaceLiquidStoreKey, f64>,
        day_index: usize,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        let key_set = configuration
            .records
            .iter()
            .map(|record| record.key.clone())
            .collect::<BTreeSet<_>>();
        if liquid_by_key.keys().cloned().collect::<BTreeSet<_>>() != key_set
            || litter_ice_by_key.keys().cloned().collect::<BTreeSet<_>>() != key_set
            || surface_enthalpy_by_key
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>()
                != key_set
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "surface-owner V2 initial state key set mismatch",
            ));
        }
        let v1 = DirectSurfaceLiquidOwnedState::new_initial(
            configuration.parent(),
            liquid_by_key,
            day_index,
        )?;
        Self::from_v1_parts(
            configuration,
            &v1,
            litter_ice_by_key,
            surface_enthalpy_by_key,
        )
    }

    pub fn migrate_from_v1(
        configuration: &SurfaceLiquidConfigurationV2,
        state: &DirectSurfaceLiquidOwnedState,
        surface_enthalpy_by_key: &BTreeMap<DirectSurfaceLiquidStoreKey, f64>,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        let litter_ice_by_key = state
            .records
            .iter()
            .map(|record| (record.key.clone(), f64::from_bits(0)))
            .collect();
        Self::from_v1_parts(
            configuration,
            state,
            &litter_ice_by_key,
            surface_enthalpy_by_key,
        )
    }

    fn from_v1_parts(
        configuration: &SurfaceLiquidConfigurationV2,
        state: &DirectSurfaceLiquidOwnedState,
        litter_ice_by_key: &BTreeMap<DirectSurfaceLiquidStoreKey, f64>,
        surface_enthalpy_by_key: &BTreeMap<DirectSurfaceLiquidStoreKey, f64>,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        state.validate(configuration.parent())?;
        let records = state
            .records
            .iter()
            .map(|record| {
                let litter_ice = litter_ice_by_key.get(&record.key).copied().ok_or(
                    DirectSurfaceLiquidError::Identity(
                        "surface-owner V2 missing explicit litter ice",
                    ),
                )?;
                let enthalpy = surface_enthalpy_by_key.get(&record.key).copied().ok_or(
                    DirectSurfaceLiquidError::Identity(
                        "surface-owner V2 missing explicit surface enthalpy",
                    ),
                )?;
                Ok(SurfaceLiquidStateRecordV2 {
                    key: record.key.clone(),
                    liquid_kg_m2_tile: record.liquid_kg_m2_tile,
                    litter_ice_kg_m2_tile: litter_ice,
                    surface_enthalpy_j_m2_tile: enthalpy,
                    last_accepted_transaction_id: record.last_accepted_transaction_id,
                })
            })
            .collect::<Result<Vec<_>, DirectSurfaceLiquidError>>()?;
        if litter_ice_by_key.len() != records.len()
            || surface_enthalpy_by_key.len() != records.len()
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "surface-owner V2 state key cardinality mismatch",
            ));
        }
        Self::try_new(configuration, records, state.continuations.clone())
    }

    pub fn try_new(
        configuration: &SurfaceLiquidConfigurationV2,
        records: Vec<SurfaceLiquidStateRecordV2>,
        continuations: Vec<DirectSurfaceLiquidContinuationState>,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        let mut value = Self {
            owner_id: configuration.parent.owner_id.clone(),
            configuration_sha256: configuration.configuration_sha256.clone(),
            model_definition_sha256: configuration
                .model_definition
                .model_definition_sha256
                .clone(),
            state_sha256: ZERO_SHA256.into(),
            records,
            continuations,
        };
        value.validate_without_digest(configuration)?;
        value.state_sha256 = value.recomputed_sha256()?;
        value.validate(configuration)?;
        Ok(value)
    }

    #[must_use]
    pub fn state_sha256(&self) -> &str {
        &self.state_sha256
    }

    #[must_use]
    pub fn records(&self) -> &[SurfaceLiquidStateRecordV2] {
        &self.records
    }

    #[must_use]
    pub fn continuations(&self) -> &[DirectSurfaceLiquidContinuationState] {
        &self.continuations
    }

    pub fn canonical_bytes(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
    ) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
        self.validate(configuration)?;
        self.canonical_bytes_with_digest(&self.state_sha256)
    }

    pub fn from_canonical_bytes(
        configuration: &SurfaceLiquidConfigurationV2,
        bytes: &[u8],
    ) -> Result<Self, DirectSurfaceLiquidError> {
        let wire: CanonicalSurfaceLiquidOwnedStateV2 = serde_json::from_slice(bytes)
            .map_err(|_| DirectSurfaceLiquidError::Schema("surface-owner V2 state parse"))?;
        let records = wire
            .records
            .into_iter()
            .map(|record| {
                Ok(SurfaceLiquidStateRecordV2 {
                    key: record.key,
                    liquid_kg_m2_tile: parse_f64_bits(&record.liquid_kg_m2_tile_bits)?,
                    litter_ice_kg_m2_tile: parse_f64_bits(&record.litter_ice_kg_m2_tile_bits)?,
                    surface_enthalpy_j_m2_tile: parse_f64_bits(
                        &record.surface_enthalpy_j_m2_tile_bits,
                    )?,
                    last_accepted_transaction_id: record.last_accepted_transaction_id,
                })
            })
            .collect::<Result<Vec<_>, DirectSurfaceLiquidError>>()?;
        let continuations = wire
            .continuations
            .into_iter()
            .map(|continuation| {
                Ok(DirectSurfaceLiquidContinuationState {
                    ofe_id: continuation.ofe_id,
                    day_index: continuation.day_index,
                    next_interval_index: continuation.next_interval_index,
                    cumulative_supply_m: parse_f64_bits(&continuation.cumulative_supply_m)?,
                    cumulative_infiltration_m: parse_f64_bits(
                        &continuation.cumulative_infiltration_m,
                    )?,
                    last_accepted_transaction_id: continuation.last_accepted_transaction_id,
                })
            })
            .collect::<Result<Vec<_>, DirectSurfaceLiquidError>>()?;
        let value = Self {
            owner_id: wire.owner_id,
            configuration_sha256: wire.configuration_sha256,
            model_definition_sha256: wire.model_definition_sha256,
            state_sha256: wire.state_sha256,
            records,
            continuations,
        };
        value.validate(configuration)?;
        if value.canonical_bytes(configuration)? != bytes {
            return Err(DirectSurfaceLiquidError::Schema(
                "noncanonical surface-owner V2 state bytes",
            ));
        }
        Ok(value)
    }

    fn validate(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
    ) -> Result<(), DirectSurfaceLiquidError> {
        self.validate_without_digest(configuration)?;
        if !is_sha256(&self.state_sha256)
            || self.state_sha256 == ZERO_SHA256
            || self.state_sha256 != self.recomputed_sha256()?
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "surface-owner V2 state digest mismatch",
            ));
        }
        Ok(())
    }

    fn validate_without_digest(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
    ) -> Result<(), DirectSurfaceLiquidError> {
        configuration.validate()?;
        if self.owner_id != configuration.parent.owner_id
            || self.configuration_sha256 != configuration.configuration_sha256
            || self.model_definition_sha256
                != configuration.model_definition.model_definition_sha256
            || self.records.len() != configuration.records.len()
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "surface-owner V2 state owner/config/model/cardinality mismatch",
            ));
        }
        for (record, parent_record) in self.records.iter().zip(&configuration.parent.records) {
            if record.key != parent_record.key {
                return Err(DirectSurfaceLiquidError::Identity(
                    "surface-owner V2 state key/order mismatch",
                ));
            }
            require_nonnegative_finite(record.liquid_kg_m2_tile, "V2 liquid state")?;
            require_nonnegative_finite(record.litter_ice_kg_m2_tile, "V2 litter ice state")?;
            if !record.surface_enthalpy_j_m2_tile.is_finite() {
                return Err(DirectSurfaceLiquidError::Domain(
                    "surface-owner V2 enthalpy is nonfinite",
                ));
            }
            if record.liquid_kg_m2_tile > parent_record.capacity_kg_m2_tile {
                return Err(DirectSurfaceLiquidError::Domain(
                    "surface-owner V2 liquid exceeds capacity",
                ));
            }
            let extension =
                configuration
                    .record(&record.key)
                    .ok_or(DirectSurfaceLiquidError::Identity(
                        "surface-owner V2 missing config record",
                    ))?;
            match extension.litter_ice_capacity_kg_m2_tile {
                Some(capacity) if record.litter_ice_kg_m2_tile <= capacity => {}
                None if record.litter_ice_kg_m2_tile.to_bits() == 0 => {}
                Some(_) => {
                    return Err(DirectSurfaceLiquidError::Domain(
                        "surface-owner V2 litter ice exceeds capacity",
                    ));
                }
                None => {
                    return Err(DirectSurfaceLiquidError::Domain(
                        "surface-owner V2 bare surface carries litter ice",
                    ));
                }
            }
        }
        self.validate_v1_lineage_and_continuations(configuration)
    }

    fn validate_v1_lineage_and_continuations(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
    ) -> Result<(), DirectSurfaceLiquidError> {
        let records = self
            .records
            .iter()
            .map(|record| DirectSurfaceLiquidStateRecord {
                key: record.key.clone(),
                liquid_kg_m2_tile: record.liquid_kg_m2_tile,
                last_accepted_transaction_id: record.last_accepted_transaction_id,
            })
            .collect();
        let mut projected = DirectSurfaceLiquidOwnedState {
            owner_id: self.owner_id.clone(),
            configuration_sha256: configuration.parent.configuration_sha256.clone(),
            state_sha256: ZERO_SHA256.into(),
            records,
            continuations: self.continuations.clone(),
        };
        projected.state_sha256 = projected.recomputed_sha256()?;
        projected.validate(&configuration.parent)
    }

    fn recomputed_sha256(&self) -> Result<String, DirectSurfaceLiquidError> {
        Ok(sha256(&self.canonical_bytes_with_digest(ZERO_SHA256)?))
    }

    fn canonical_bytes_with_digest(
        &self,
        digest: &str,
    ) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
        let records = self
            .records
            .iter()
            .map(|record| CanonicalSurfaceLiquidStateRecordV2 {
                key: record.key.clone(),
                liquid_kg_m2_tile_bits: f64_bits(record.liquid_kg_m2_tile),
                litter_ice_kg_m2_tile_bits: f64_bits(record.litter_ice_kg_m2_tile),
                surface_enthalpy_j_m2_tile_bits: f64_bits(record.surface_enthalpy_j_m2_tile),
                last_accepted_transaction_id: record.last_accepted_transaction_id,
            })
            .collect();
        let continuations = self
            .continuations
            .iter()
            .map(
                |continuation| super::CanonicalSurfaceLiquidContinuationState {
                    ofe_id: continuation.ofe_id.clone(),
                    day_index: continuation.day_index,
                    next_interval_index: continuation.next_interval_index,
                    cumulative_supply_m: f64_bits(continuation.cumulative_supply_m),
                    cumulative_infiltration_m: f64_bits(continuation.cumulative_infiltration_m),
                    last_accepted_transaction_id: continuation.last_accepted_transaction_id,
                },
            )
            .collect();
        serde_json::to_vec(&CanonicalSurfaceLiquidOwnedStateV2 {
            owner_id: self.owner_id.clone(),
            configuration_sha256: self.configuration_sha256.clone(),
            model_definition_sha256: self.model_definition_sha256.clone(),
            state_sha256: digest.into(),
            records,
            continuations,
        })
        .map_err(|_| DirectSurfaceLiquidError::Schema("surface-owner V2 state serialization"))
    }

    #[cfg(test)]
    pub(crate) fn zero_ice_v1_representability_for_test(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
    ) -> Result<DirectSurfaceLiquidOwnedState, DirectSurfaceLiquidError> {
        self.validate(configuration)?;
        if self
            .records
            .iter()
            .any(|record| record.litter_ice_kg_m2_tile.to_bits() != 0)
        {
            return Err(DirectSurfaceLiquidError::unsupported_domain_failure(
                DirectSurfaceLiquidPhase::Restart,
                DirectSurfaceLiquidErrorContext {
                    owner_id: Some(self.owner_id.clone()),
                    ..DirectSurfaceLiquidErrorContext::default()
                },
                Some(self.state_sha256.clone()),
                "surface-owner V2 is not exactly zero-ice representable as V1",
            ));
        }
        let records = self
            .records
            .iter()
            .map(|record| DirectSurfaceLiquidStateRecord {
                key: record.key.clone(),
                liquid_kg_m2_tile: record.liquid_kg_m2_tile,
                last_accepted_transaction_id: record.last_accepted_transaction_id,
            })
            .collect();
        let mut value = DirectSurfaceLiquidOwnedState {
            owner_id: self.owner_id.clone(),
            configuration_sha256: configuration.parent.configuration_sha256.clone(),
            state_sha256: ZERO_SHA256.into(),
            records,
            continuations: self.continuations.clone(),
        };
        value.state_sha256 = value.recomputed_sha256()?;
        value.validate(&configuration.parent)?;
        Ok(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceLiquidOwnerEnvelopeV1Payload {
    schema_sha256: String,
    model_definition_sha256: String,
    parent_identity_sha256: String,
    envelope_sha256: String,
    state: DirectSurfaceLiquidOwnedState,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceLiquidOwnerEnvelopeV2Payload {
    schema_sha256: String,
    model_definition_sha256: String,
    parent_identity_sha256: String,
    envelope_sha256: String,
    state: SurfaceLiquidOwnedStateV2,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SurfaceLiquidOwnerEnvelopeV2 {
    V1(SurfaceLiquidOwnerEnvelopeV1Payload),
    V2(SurfaceLiquidOwnerEnvelopeV2Payload),
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "version", rename_all = "snake_case", deny_unknown_fields)]
enum CanonicalSurfaceLiquidOwnerEnvelopeV2 {
    V1 {
        schema_sha256: String,
        model_definition_sha256: String,
        parent_identity_sha256: String,
        envelope_sha256: String,
        state_bytes_hex: String,
    },
    V2 {
        schema_sha256: String,
        model_definition_sha256: String,
        parent_identity_sha256: String,
        envelope_sha256: String,
        state_bytes_hex: String,
    },
}

impl SurfaceLiquidOwnerEnvelopeV2 {
    pub fn wrap_v1(
        configuration: &DirectSurfaceLiquidConfiguration,
        state: DirectSurfaceLiquidOwnedState,
        parent_model_definition_sha256: impl Into<String>,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        state.validate(configuration)?;
        let model = parent_model_definition_sha256.into();
        if !is_sha256(&model) || model == ZERO_SHA256 {
            return Err(DirectSurfaceLiquidError::Identity(
                "surface-owner V1 parent model digest invalid",
            ));
        }
        let mut value = Self::V1(SurfaceLiquidOwnerEnvelopeV1Payload {
            schema_sha256: sha256(OWNER_V2_SCHEMA_NAME.as_bytes()),
            model_definition_sha256: model,
            parent_identity_sha256: configuration.configuration_sha256.clone(),
            envelope_sha256: ZERO_SHA256.into(),
            state,
        });
        let digest = value.recomputed_sha256(configuration, None)?;
        value.set_envelope_sha256(digest);
        value.validate(configuration, None)?;
        Ok(value)
    }

    pub fn wrap_v2(
        configuration: &SurfaceLiquidConfigurationV2,
        state: SurfaceLiquidOwnedStateV2,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        state.validate(configuration)?;
        let mut value = Self::V2(SurfaceLiquidOwnerEnvelopeV2Payload {
            schema_sha256: configuration.model_definition.schema_sha256.clone(),
            model_definition_sha256: configuration
                .model_definition
                .model_definition_sha256
                .clone(),
            parent_identity_sha256: configuration.configuration_sha256.clone(),
            envelope_sha256: ZERO_SHA256.into(),
            state,
        });
        let digest = value.recomputed_sha256(configuration.parent(), Some(configuration))?;
        value.set_envelope_sha256(digest);
        value.validate(configuration.parent(), Some(configuration))?;
        Ok(value)
    }

    #[must_use]
    pub fn envelope_sha256(&self) -> &str {
        match self {
            Self::V1(payload) => &payload.envelope_sha256,
            Self::V2(payload) => &payload.envelope_sha256,
        }
    }

    #[must_use]
    pub fn model_definition_sha256(&self) -> &str {
        match self {
            Self::V1(payload) => &payload.model_definition_sha256,
            Self::V2(payload) => &payload.model_definition_sha256,
        }
    }

    #[must_use]
    pub fn parent_identity_sha256(&self) -> &str {
        match self {
            Self::V1(payload) => &payload.parent_identity_sha256,
            Self::V2(payload) => &payload.parent_identity_sha256,
        }
    }

    #[must_use]
    pub fn v2_state(&self) -> Option<&SurfaceLiquidOwnedStateV2> {
        match self {
            Self::V1(_) => None,
            Self::V2(payload) => Some(&payload.state),
        }
    }

    #[must_use]
    pub fn v1_state(&self) -> Option<&DirectSurfaceLiquidOwnedState> {
        match self {
            Self::V1(payload) => Some(&payload.state),
            Self::V2(_) => None,
        }
    }

    pub fn canonical_bytes(
        &self,
        v1_configuration: &DirectSurfaceLiquidConfiguration,
        v2_configuration: Option<&SurfaceLiquidConfigurationV2>,
    ) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
        self.validate(v1_configuration, v2_configuration)?;
        self.canonical_bytes_with_digest(v1_configuration, v2_configuration, self.envelope_sha256())
    }

    pub fn from_canonical_bytes(
        v1_configuration: &DirectSurfaceLiquidConfiguration,
        v2_configuration: Option<&SurfaceLiquidConfigurationV2>,
        bytes: &[u8],
    ) -> Result<Self, DirectSurfaceLiquidError> {
        let wire: CanonicalSurfaceLiquidOwnerEnvelopeV2 = serde_json::from_slice(bytes)
            .map_err(|_| DirectSurfaceLiquidError::Schema("surface-owner envelope V2 parse"))?;
        let value = match wire {
            CanonicalSurfaceLiquidOwnerEnvelopeV2::V1 {
                schema_sha256,
                model_definition_sha256,
                parent_identity_sha256,
                envelope_sha256,
                state_bytes_hex,
            } => Self::V1(SurfaceLiquidOwnerEnvelopeV1Payload {
                schema_sha256,
                model_definition_sha256,
                parent_identity_sha256,
                envelope_sha256,
                state: DirectSurfaceLiquidOwnedState::from_canonical_bytes(
                    v1_configuration,
                    &decode_hex(&state_bytes_hex)?,
                )?,
            }),
            CanonicalSurfaceLiquidOwnerEnvelopeV2::V2 {
                schema_sha256,
                model_definition_sha256,
                parent_identity_sha256,
                envelope_sha256,
                state_bytes_hex,
            } => {
                let configuration = v2_configuration.ok_or(DirectSurfaceLiquidError::Identity(
                    "surface-owner envelope V2 configuration absent",
                ))?;
                Self::V2(SurfaceLiquidOwnerEnvelopeV2Payload {
                    schema_sha256,
                    model_definition_sha256,
                    parent_identity_sha256,
                    envelope_sha256,
                    state: SurfaceLiquidOwnedStateV2::from_canonical_bytes(
                        configuration,
                        &decode_hex(&state_bytes_hex)?,
                    )?,
                })
            }
        };
        value.validate(v1_configuration, v2_configuration)?;
        if value.canonical_bytes(v1_configuration, v2_configuration)? != bytes {
            return Err(DirectSurfaceLiquidError::Schema(
                "noncanonical surface-owner envelope V2 bytes",
            ));
        }
        Ok(value)
    }

    pub fn try_replace_v2_state(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
        records: Vec<SurfaceLiquidStateRecordV2>,
        continuations: Vec<DirectSurfaceLiquidContinuationState>,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        if self.v2_state().is_none() {
            return Err(DirectSurfaceLiquidError::unsupported_domain_failure(
                DirectSurfaceLiquidPhase::AtomicEnvelope,
                DirectSurfaceLiquidErrorContext::default(),
                Some(self.envelope_sha256().into()),
                "surface-owner V1 may upgrade only through checked V1-to-V2 migration",
            ));
        }
        let beginning = self.canonical_bytes(configuration.parent(), Some(configuration))?;
        let attempted = raw_state_attempt_sha256(&records, &continuations)?;
        let state = SurfaceLiquidOwnedStateV2::try_new(configuration, records, continuations)
            .map_err(|error| {
                let code = error.code();
                error.complete_context(
                    code,
                    DirectSurfaceLiquidPhase::AtomicEnvelope,
                    DirectSurfaceLiquidErrorContext::default(),
                    Some(sha256(&beginning)),
                    Some(attempted),
                )
            })?;
        Self::wrap_v2(configuration, state)
    }

    fn validate(
        &self,
        v1_configuration: &DirectSurfaceLiquidConfiguration,
        v2_configuration: Option<&SurfaceLiquidConfigurationV2>,
    ) -> Result<(), DirectSurfaceLiquidError> {
        let expected_schema = sha256(OWNER_V2_SCHEMA_NAME.as_bytes());
        match self {
            Self::V1(payload) => {
                payload.state.validate(v1_configuration)?;
                if payload.schema_sha256 != expected_schema
                    || !is_sha256(&payload.model_definition_sha256)
                    || payload.model_definition_sha256 == ZERO_SHA256
                    || payload.parent_identity_sha256 != v1_configuration.configuration_sha256
                {
                    return Err(DirectSurfaceLiquidError::Identity(
                        "surface-owner envelope V1 identity mismatch",
                    ));
                }
            }
            Self::V2(payload) => {
                let configuration = v2_configuration.ok_or(DirectSurfaceLiquidError::Identity(
                    "surface-owner envelope V2 configuration absent",
                ))?;
                payload.state.validate(configuration)?;
                if payload.schema_sha256 != expected_schema
                    || payload.model_definition_sha256
                        != configuration.model_definition.model_definition_sha256
                    || payload.parent_identity_sha256 != configuration.configuration_sha256
                {
                    return Err(DirectSurfaceLiquidError::Identity(
                        "surface-owner envelope V2 identity mismatch",
                    ));
                }
            }
        }
        if !is_sha256(self.envelope_sha256())
            || self.envelope_sha256() == ZERO_SHA256
            || self.envelope_sha256()
                != self.recomputed_sha256(v1_configuration, v2_configuration)?
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "surface-owner envelope V2 digest mismatch",
            ));
        }
        Ok(())
    }

    fn recomputed_sha256(
        &self,
        v1_configuration: &DirectSurfaceLiquidConfiguration,
        v2_configuration: Option<&SurfaceLiquidConfigurationV2>,
    ) -> Result<String, DirectSurfaceLiquidError> {
        Ok(sha256(&self.canonical_bytes_with_digest(
            v1_configuration,
            v2_configuration,
            ZERO_SHA256,
        )?))
    }

    fn set_envelope_sha256(&mut self, digest: String) {
        match self {
            Self::V1(payload) => payload.envelope_sha256 = digest,
            Self::V2(payload) => payload.envelope_sha256 = digest,
        }
    }

    fn canonical_bytes_with_digest(
        &self,
        v1_configuration: &DirectSurfaceLiquidConfiguration,
        v2_configuration: Option<&SurfaceLiquidConfigurationV2>,
        digest: &str,
    ) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
        let wire = match self {
            Self::V1(payload) => CanonicalSurfaceLiquidOwnerEnvelopeV2::V1 {
                schema_sha256: payload.schema_sha256.clone(),
                model_definition_sha256: payload.model_definition_sha256.clone(),
                parent_identity_sha256: payload.parent_identity_sha256.clone(),
                envelope_sha256: digest.into(),
                state_bytes_hex: encode_hex(&payload.state.canonical_bytes(v1_configuration)?),
            },
            Self::V2(payload) => {
                let configuration = v2_configuration.ok_or(DirectSurfaceLiquidError::Identity(
                    "surface-owner envelope V2 configuration absent",
                ))?;
                CanonicalSurfaceLiquidOwnerEnvelopeV2::V2 {
                    schema_sha256: payload.schema_sha256.clone(),
                    model_definition_sha256: payload.model_definition_sha256.clone(),
                    parent_identity_sha256: payload.parent_identity_sha256.clone(),
                    envelope_sha256: digest.into(),
                    state_bytes_hex: encode_hex(&payload.state.canonical_bytes(configuration)?),
                }
            }
        };
        serde_json::to_vec(&wire).map_err(|_| {
            DirectSurfaceLiquidError::Schema("surface-owner envelope V2 serialization")
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceLiquidOwnerClosureRecordV2 {
    pub key: DirectSurfaceLiquidStoreKey,
    pub liquid_debit_kg_m2_tile: f64,
    pub liquid_credit_kg_m2_tile: f64,
    pub ice_debit_kg_m2_tile: f64,
    pub ice_credit_kg_m2_tile: f64,
}

pub fn validate_surface_liquid_owner_mass_closure_v2(
    configuration: &SurfaceLiquidConfigurationV2,
    beginning_state: &SurfaceLiquidOwnedStateV2,
    ending_state: &SurfaceLiquidOwnedStateV2,
    operands: &[SurfaceLiquidOwnerClosureRecordV2],
) -> Result<(), DirectSurfaceLiquidError> {
    beginning_state.validate(configuration)?;
    ending_state.validate(configuration)?;
    if operands.len() != configuration.records.len() {
        return Err(DirectSurfaceLiquidError::Protocol(
            "surface-owner V2 closure operand cardinality mismatch",
        ));
    }
    for ((beginning, ending), operand) in beginning_state
        .records
        .iter()
        .zip(&ending_state.records)
        .zip(operands)
    {
        if beginning.key != ending.key || beginning.key != operand.key {
            return Err(DirectSurfaceLiquidError::Identity(
                "surface-owner V2 closure key/order mismatch",
            ));
        }
        for value in [
            operand.liquid_debit_kg_m2_tile,
            operand.liquid_credit_kg_m2_tile,
            operand.ice_debit_kg_m2_tile,
            operand.ice_credit_kg_m2_tile,
        ] {
            require_nonnegative_finite(value, "surface-owner V2 closure operand")?;
        }
        let expected_liquid = checked_surface_liquid_add(
            checked_surface_liquid_sub(
                beginning.liquid_kg_m2_tile,
                operand.liquid_debit_kg_m2_tile,
            )
            .ok_or(DirectSurfaceLiquidError::Closure(
                "surface-owner V2 liquid closure arithmetic",
            ))?,
            operand.liquid_credit_kg_m2_tile,
        )
        .ok_or(DirectSurfaceLiquidError::Closure(
            "surface-owner V2 liquid closure arithmetic",
        ))?;
        let expected_ice = checked_surface_liquid_add(
            checked_surface_liquid_sub(
                beginning.litter_ice_kg_m2_tile,
                operand.ice_debit_kg_m2_tile,
            )
            .ok_or(DirectSurfaceLiquidError::Closure(
                "surface-owner V2 ice closure arithmetic",
            ))?,
            operand.ice_credit_kg_m2_tile,
        )
        .ok_or(DirectSurfaceLiquidError::Closure(
            "surface-owner V2 ice closure arithmetic",
        ))?;
        let liquid_closed = checked_surface_liquid_close(
            ending.liquid_kg_m2_tile,
            expected_liquid,
            DirectSurfaceLiquidClosureUnit::MassKgM2,
        )
        .ok_or(DirectSurfaceLiquidError::Closure(
            "surface-owner V2 liquid closure comparison",
        ))?;
        let ice_closed = checked_surface_liquid_close(
            ending.litter_ice_kg_m2_tile,
            expected_ice,
            DirectSurfaceLiquidClosureUnit::MassKgM2,
        )
        .ok_or(DirectSurfaceLiquidError::Closure(
            "surface-owner V2 ice closure comparison",
        ))?;
        if !liquid_closed || !ice_closed {
            return Err(DirectSurfaceLiquidError::canonical_failure(
                DirectSurfaceLiquidErrorCode::E010,
                DirectSurfaceLiquidPhase::IndependentClosure,
                DirectSurfaceLiquidErrorContext {
                    owner_id: Some(beginning_owner(beginning, configuration)),
                    ofe_id: Some(beginning.key.ofe_id.clone()),
                    tile_id: Some(beginning.key.tile_id.clone()),
                    surface_id: Some(beginning.key.surface_id.clone()),
                    source_id: Some(beginning.key.source_id.clone()),
                    ..DirectSurfaceLiquidErrorContext::default()
                },
                DirectSurfaceLiquidRollbackHashes {
                    beginning_owner_sha256: Some(beginning_state_hash(
                        configuration,
                        beginning_state,
                    )?),
                    attempted_owner_sha256: Some(beginning_state_hash(
                        configuration,
                        ending_state,
                    )?),
                },
                "surface-owner V2 independent liquid/ice mass closure mismatch",
            ));
        }
    }
    Ok(())
}

fn beginning_owner(
    _record: &SurfaceLiquidStateRecordV2,
    configuration: &SurfaceLiquidConfigurationV2,
) -> ResourceOwnerId {
    configuration.parent.owner_id.clone()
}

fn beginning_state_hash(
    configuration: &SurfaceLiquidConfigurationV2,
    state: &SurfaceLiquidOwnedStateV2,
) -> Result<String, DirectSurfaceLiquidError> {
    Ok(sha256(&state.canonical_bytes(configuration)?))
}

fn raw_state_attempt_sha256(
    records: &[SurfaceLiquidStateRecordV2],
    continuations: &[DirectSurfaceLiquidContinuationState],
) -> Result<String, DirectSurfaceLiquidError> {
    let records = records
        .iter()
        .map(|record| CanonicalSurfaceLiquidStateRecordV2 {
            key: record.key.clone(),
            liquid_kg_m2_tile_bits: f64_bits(record.liquid_kg_m2_tile),
            litter_ice_kg_m2_tile_bits: f64_bits(record.litter_ice_kg_m2_tile),
            surface_enthalpy_j_m2_tile_bits: f64_bits(record.surface_enthalpy_j_m2_tile),
            last_accepted_transaction_id: record.last_accepted_transaction_id,
        })
        .collect::<Vec<_>>();
    let continuations = continuations
        .iter()
        .map(
            |continuation| super::CanonicalSurfaceLiquidContinuationState {
                ofe_id: continuation.ofe_id.clone(),
                day_index: continuation.day_index,
                next_interval_index: continuation.next_interval_index,
                cumulative_supply_m: f64_bits(continuation.cumulative_supply_m),
                cumulative_infiltration_m: f64_bits(continuation.cumulative_infiltration_m),
                last_accepted_transaction_id: continuation.last_accepted_transaction_id,
            },
        )
        .collect::<Vec<_>>();
    let bytes = serde_json::to_vec(&(records, continuations)).map_err(|_| {
        DirectSurfaceLiquidError::Schema("surface-owner V2 candidate-attempt serialization")
    })?;
    Ok(sha256(&bytes))
}

fn require_positive_finite(
    value: f64,
    detail: &'static str,
) -> Result<(), DirectSurfaceLiquidError> {
    if !value.is_finite() || value <= 0.0 {
        return Err(DirectSurfaceLiquidError::Domain(detail));
    }
    Ok(())
}

fn require_nonnegative_finite(
    value: f64,
    detail: &'static str,
) -> Result<(), DirectSurfaceLiquidError> {
    if !value.is_finite() || value < 0.0 {
        return Err(DirectSurfaceLiquidError::Domain(detail));
    }
    Ok(())
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub(super) fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}

pub(super) fn decode_hex(value: &str) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
    if value.len() & 1 != 0
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(DirectSurfaceLiquidError::Schema(
            "surface-owner V2 canonical hex",
        ));
    }
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let pair = std::str::from_utf8(pair)
                .map_err(|_| DirectSurfaceLiquidError::Schema("surface-owner V2 hex utf8"))?;
            u8::from_str_radix(pair, 16)
                .map_err(|_| DirectSurfaceLiquidError::Schema("surface-owner V2 hex parse"))
        })
        .collect()
}
