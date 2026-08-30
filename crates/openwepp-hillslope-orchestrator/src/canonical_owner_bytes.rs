#[allow(clippy::wildcard_imports)]
use super::*;

const ADAPTIVE_COMPLETE_OWNER_IDS: [&str; 7] = [
    "bgc",
    "hydrology",
    "land_surface_energy",
    "snow",
    "soil_thermal",
    "surface_liquid",
    "vegetation",
];
const ADAPTIVE_DISCRETE_SCHEMA: &str = "OPENWEPP_ADAPTIVE_EXACT_DISCRETE_SURFACE_V1";
const ADAPTIVE_EXTENSIVE_RELATIVE_TOLERANCE: f64 = 1.0e-3;
const ADAPTIVE_SNOW_EXTENSIVE_RELATIVE_TOLERANCE: f64 = 5.0e-3;
const ADAPTIVE_INTENSIVE_RELATIVE_TOLERANCE: f64 = 1.0e-6;
const ADAPTIVE_TEMPERATURE_ABSOLUTE_TOLERANCE_K: f64 = 1.0e-2;
const ADAPTIVE_TEMPERATURE_RELATIVE_TOLERANCE: f64 = 1.0e-8;
const ADAPTIVE_THERMAL_ENERGY_RELATIVE_TOLERANCE: f64 = 5.0e-3;
const ADAPTIVE_SOIL_THERMAL_ENERGY_RELATIVE_TOLERANCE: f64 = 1.5e-2;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum AdaptiveOwnerDimensionV1 {
    AreaEnergyJM2,
    AreaMassKgM2,
    CarbonDioxidePressurePa,
    HydraulicPotentialMm,
    SpecificHumidityKgKg,
    TemperatureKOrC,
    WaterDepthM,
    DimensionlessState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AdaptiveToleranceAuthorityV1 {
    pub contract_id: &'static str,
    pub tolerance_id: &'static str,
    pub dimension: AdaptiveOwnerDimensionV1,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AdaptiveOwnerScalarV1 {
    pub owner_id: String,
    pub path: String,
    pub value: f64,
    pub tolerance_authority: AdaptiveToleranceAuthorityV1,
    pub absolute_tolerance: f64,
    pub relative_tolerance: f64,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum AdaptiveDiscreteSurfaceKindV1 {
    ActiveSet,
    DerivedIntegrityCache,
    ExactBinary64,
    Identity,
    Membership,
    NumericalWarmStart,
    Ordering,
    Posture,
    ReceiptLineage,
    ReceiptOrdering,
    Schema,
    Topology,
}

impl AdaptiveDiscreteSurfaceKindV1 {
    const fn wire_tag(self) -> &'static [u8] {
        match self {
            Self::ActiveSet => b"active_set",
            Self::DerivedIntegrityCache => b"derived_integrity_cache",
            Self::ExactBinary64 => b"exact_binary64",
            Self::Identity => b"identity",
            Self::Membership => b"membership",
            Self::NumericalWarmStart => b"numerical_warm_start",
            Self::Ordering => b"ordering",
            Self::Posture => b"posture",
            Self::ReceiptLineage => b"receipt_lineage",
            Self::ReceiptOrdering => b"receipt_ordering",
            Self::Schema => b"schema",
            Self::Topology => b"topology",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct AdaptiveNumericalWarmStartAuthorityV1 {
    pub contract_id: &'static str,
    pub invariant_id: &'static str,
    pub classification_id: &'static str,
}

const VEGETATION_NUMERICAL_WARM_START_AUTHORITY: AdaptiveNumericalWarmStartAuthorityV1 =
    AdaptiveNumericalWarmStartAuthorityV1 {
        contract_id: "SC-VEGETATION-001@29",
        invariant_id: "INV-VEGETATION-078",
        classification_id: "OPENWEPP_ADAPTIVE_NUMERICAL_WARM_START_V1-vegetation-sequential",
    };
const LSE_NUMERICAL_WARM_START_AUTHORITY: AdaptiveNumericalWarmStartAuthorityV1 =
    AdaptiveNumericalWarmStartAuthorityV1 {
        contract_id: "SC-LANDSURFACEENERGY-001@11",
        invariant_id: "INV-LANDSURFACEENERGY-103",
        classification_id: "OPENWEPP_ADAPTIVE_NUMERICAL_WARM_START_V1-lse-surface-temperature",
    };
const VEGETATION_DERIVED_CACHE_AUTHORITY: AdaptiveNumericalWarmStartAuthorityV1 =
    AdaptiveNumericalWarmStartAuthorityV1 {
        contract_id: "SC-VEGETATION-001@29",
        invariant_id: "INV-VEGETATION-089",
        classification_id: "OPENWEPP_ADAPTIVE_DERIVED_INTEGRITY_CACHE_V1-vegetation-area",
    };
const SNOW_LAYER_DERIVED_CACHE_AUTHORITY: AdaptiveNumericalWarmStartAuthorityV1 =
    AdaptiveNumericalWarmStartAuthorityV1 {
        contract_id: "SC-SNOWENERGY-001@25",
        invariant_id: "INV-SNOWENERGY-043",
        classification_id: "OPENWEPP_ADAPTIVE_DERIVED_INTEGRITY_CACHE_V1-snow-layer-geometry-cadence",
    };
const ADAPTIVE_RECEIPT_LINEAGE_AUTHORITY: &str = "OPENWEPP_ADAPTIVE_FACTORIZATION_LINEAGE_V1";

pub(crate) fn adaptive_numerical_warm_start_authority(
    owner_id: &str,
    path: &str,
) -> Option<AdaptiveNumericalWarmStartAuthorityV1> {
    let normalized_path = path.to_ascii_lowercase();
    let field = adaptive_field_name(&normalized_path);
    if owner_id == "land_surface_energy"
        && matches!(
            field,
            "surface_temperature_warm_start" | "surface_temperature_warm_start_k"
        )
    {
        return Some(LSE_NUMERICAL_WARM_START_AUTHORITY);
    }
    if owner_id == "vegetation"
        && ((normalized_path.contains("occupancies")
            && matches!(
                field,
                "sun_leaf_temperature_k"
                    | "shade_leaf_temperature_k"
                    | "dry_stem_temperature_k"
                    | "wet_surface_temperature_k"
                    | "canopy_air_temperature_k"
                    | "canopy_air_specific_humidity_kg_kg"
                    | "sun_ci_pa"
                    | "shade_ci_pa"
                    | "beta_hyd"
                    | "root_node_potential_mm"
                    | "stem_potential_mm"
                    | "sun_leaf_potential_mm"
                    | "shade_leaf_potential_mm"
                    | "root_potential_mm_by_layer"
            ))
            || (normalized_path.contains("tile_canopy_air")
                && matches!(
                    field,
                    "canopy_air_temperature_k" | "canopy_air_specific_humidity_kg_kg"
                )))
    {
        return Some(VEGETATION_NUMERICAL_WARM_START_AUTHORITY);
    }
    None
}

fn adaptive_record_numerical_warm_start_classification(
    owner_id: &str,
    path: &str,
    discrete_surfaces: &mut Vec<AdaptiveOwnerDiscreteSurfaceV1>,
) -> bool {
    let Some(authority) = adaptive_numerical_warm_start_authority(owner_id, path) else {
        return false;
    };
    discrete_surfaces.push(AdaptiveOwnerDiscreteSurfaceV1 {
        owner_id: owner_id.to_owned(),
        path: path.to_owned(),
        kind: AdaptiveDiscreteSurfaceKindV1::NumericalWarmStart,
        exact_value: format!(
            "{}\0{}\0{}",
            authority.contract_id, authority.invariant_id, authority.classification_id
        ),
    });
    true
}

fn adaptive_derived_cache_authority(
    owner_id: &str,
    path: &str,
) -> Option<AdaptiveNumericalWarmStartAuthorityV1> {
    let normalized_path = path.to_ascii_lowercase();
    let field = adaptive_field_name(&normalized_path);
    if owner_id == "vegetation"
        && normalized_path.contains("strata")
        && matches!(field, "leaf_area" | "stem_area" | "root_area")
    {
        return Some(VEGETATION_DERIVED_CACHE_AUTHORITY);
    }
    (owner_id == "snow"
        && normalized_path.contains("layers")
        && matches!(field, "density_kg_m3" | "settle_day_count"))
    .then_some(SNOW_LAYER_DERIVED_CACHE_AUTHORITY)
}

fn adaptive_record_derived_cache_classification(
    owner_id: &str,
    path: &str,
    discrete_surfaces: &mut Vec<AdaptiveOwnerDiscreteSurfaceV1>,
) -> bool {
    let Some(authority) = adaptive_derived_cache_authority(owner_id, path) else {
        return false;
    };
    discrete_surfaces.push(AdaptiveOwnerDiscreteSurfaceV1 {
        owner_id: owner_id.to_owned(),
        path: path.to_owned(),
        kind: AdaptiveDiscreteSurfaceKindV1::DerivedIntegrityCache,
        exact_value: format!(
            "{}\0{}\0{}",
            authority.contract_id, authority.invariant_id, authority.classification_id
        ),
    });
    true
}

fn adaptive_digest_hex(value: Digest32) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(64);
    for byte in value.as_bytes() {
        encoded.push(char::from(HEX[(byte >> 4) as usize]));
        encoded.push(char::from(HEX[(byte & 0x0f) as usize]));
    }
    encoded
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct AdaptiveOwnerDiscreteSurfaceV1 {
    pub owner_id: String,
    pub path: String,
    pub kind: AdaptiveDiscreteSurfaceKindV1,
    pub exact_value: String,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AdaptiveCompleteOwnerComparisonV1 {
    pub scalars: Vec<AdaptiveOwnerScalarV1>,
    pub exact_discrete_surfaces: Vec<AdaptiveOwnerDiscreteSurfaceV1>,
    pub exact_discrete_sha256: Digest32,
}

impl AdaptiveCompleteOwnerComparisonV1 {
    pub(crate) fn scaled_error(
        &self,
        other: &Self,
    ) -> Result<(f64, bool), DirectV11RealConsumerError> {
        let direct_cross_path_surfaces = self
            .exact_discrete_surfaces
            .iter()
            .filter(|surface| surface.kind != AdaptiveDiscreteSurfaceKindV1::ReceiptLineage)
            .collect::<Vec<_>>();
        let composed_cross_path_surfaces = other
            .exact_discrete_surfaces
            .iter()
            .filter(|surface| surface.kind != AdaptiveDiscreteSurfaceKindV1::ReceiptLineage)
            .collect::<Vec<_>>();
        let mut discrete_mismatch = direct_cross_path_surfaces != composed_cross_path_surfaces;
        let scalar_key = |scalar: &AdaptiveOwnerScalarV1| {
            (
                scalar.owner_id.clone(),
                scalar.path.clone(),
                scalar.tolerance_authority.contract_id,
                scalar.tolerance_authority.tolerance_id,
                scalar.tolerance_authority.dimension,
                scalar.absolute_tolerance.to_bits(),
                scalar.relative_tolerance.to_bits(),
            )
        };
        let direct_scalars = self
            .scalars
            .iter()
            .map(|scalar| (scalar_key(scalar), scalar))
            .collect::<BTreeMap<_, _>>();
        let composed_scalars = other
            .scalars
            .iter()
            .map(|scalar| (scalar_key(scalar), scalar))
            .collect::<BTreeMap<_, _>>();
        if direct_scalars.len() != self.scalars.len()
            || composed_scalars.len() != other.scalars.len()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "adaptive complete-owner duplicate scalar identity",
            ));
        }
        let direct_scalar_order = self.scalars.iter().map(&scalar_key).collect::<Vec<_>>();
        let composed_scalar_order = other.scalars.iter().map(&scalar_key).collect::<Vec<_>>();
        discrete_mismatch |= direct_scalars.keys().collect::<Vec<_>>()
            != composed_scalars.keys().collect::<Vec<_>>()
            || direct_scalar_order != composed_scalar_order;
        let mut maximum_scaled_error: f64 = 0.0;
        for (key, left) in direct_scalars {
            let Some(right) = composed_scalars.get(&key) else {
                continue;
            };
            let denominator = left.absolute_tolerance
                + left.relative_tolerance * left.value.abs().max(right.value.abs());
            if !denominator.is_finite() || denominator <= 0.0 {
                return Err(DirectV11RealConsumerError::Identity(
                    "adaptive complete-owner tolerance domain",
                ));
            }
            let scaled = (right.value - left.value).abs() / denominator;
            if !scaled.is_finite() {
                return Err(DirectV11RealConsumerError::Identity(
                    "adaptive complete-owner comparison domain",
                ));
            }
            maximum_scaled_error = maximum_scaled_error.max(scaled);
        }
        Ok((maximum_scaled_error, discrete_mismatch))
    }
}

fn adaptive_float_bits(value: &str) -> Option<u64> {
    let digits = value.strip_prefix("0x").unwrap_or(value);
    (digits.len() == 16 && digits.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .then(|| u64::from_str_radix(digits, 16).ok())
        .flatten()
}

fn adaptive_field_name(path: &str) -> &str {
    path.rsplit(['.', '[', ']'])
        .find(|component| {
            !component.is_empty() && !component.bytes().all(|byte| byte.is_ascii_digit())
        })
        .unwrap_or(path)
}

fn adaptive_receipt_ordering_projection(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(fields) => serde_json::Value::Object(
            fields
                .iter()
                .map(|(key, value)| {
                    let normalized = key.to_ascii_lowercase();
                    let transaction_local_identity = normalized.ends_with("_sha256")
                        || normalized.ends_with("_digest")
                        || normalized == "fingerprint"
                        || normalized.ends_with("transaction_id")
                        || normalized.ends_with("proposal_id")
                        || normalized.ends_with("receipt_id");
                    let stable_ordering_identity = matches!(
                        normalized.as_str(),
                        "participant_id"
                            | "lane_id"
                            | "ofe_id"
                            | "tile_id"
                            | "destination_id"
                            | "destination"
                            | "owner_id"
                            | "phase"
                            | "class"
                            | "kind"
                            | "source"
                            | "schema"
                            | "model_version"
                    );
                    let projected = if transaction_local_identity {
                        serde_json::Value::Null
                    } else if stable_ordering_identity {
                        value.clone()
                    } else if value.is_object() || value.is_array() {
                        adaptive_receipt_ordering_projection(value)
                    } else {
                        serde_json::Value::Null
                    };
                    (key.clone(), projected)
                })
                .collect(),
        ),
        serde_json::Value::Array(values) => serde_json::Value::Array(
            values
                .iter()
                .map(adaptive_receipt_ordering_projection)
                .collect(),
        ),
        _ => serde_json::Value::Null,
    }
}

fn adaptive_receipt_container_kind(
    owner_id: &str,
    normalized_path: &str,
) -> AdaptiveDiscreteSurfaceKindV1 {
    let wb14_per_ofe_receipts = owner_id == "surface_liquid"
        && normalized_path
            .strip_prefix("wb14_parent_working_state.per_ofe_authorities.")
            .and_then(|path| path.strip_suffix(".receipts"))
            .is_some_and(|ofe_id| !ofe_id.is_empty());
    if wb14_per_ofe_receipts {
        // SC-SURFACELIQUID-001@13: this exact receipt history records the
        // accepted children inside one trial factorization. Direct H and
        // composed H/2 + H/2 must each bind their chronology, but their child
        // receipt key set/order is not cross-path physical state.
        AdaptiveDiscreteSurfaceKindV1::ReceiptLineage
    } else {
        AdaptiveDiscreteSurfaceKindV1::ReceiptOrdering
    }
}

pub(crate) fn adaptive_scalar_policy(
    owner_id: &str,
    path: &str,
) -> Option<(AdaptiveToleranceAuthorityV1, f64, f64)> {
    if adaptive_numerical_warm_start_authority(owner_id, path).is_some() {
        return None;
    }
    let field = adaptive_field_name(path).to_ascii_lowercase();
    let dimension = if field.ends_with("_temperature_k")
        || field.ends_with("_temperature_c")
        || field == "temperature_k"
        || field == "temperature_c"
        || field == "t10_k"
    {
        AdaptiveOwnerDimensionV1::TemperatureKOrC
    } else if field.ends_with("_specific_humidity_kg_kg") {
        AdaptiveOwnerDimensionV1::SpecificHumidityKgKg
    } else if field.ends_with("_potential_mm") {
        AdaptiveOwnerDimensionV1::HydraulicPotentialMm
    } else if field.ends_with("_ci_pa") {
        AdaptiveOwnerDimensionV1::CarbonDioxidePressurePa
    } else if field == "beta_hyd" {
        AdaptiveOwnerDimensionV1::DimensionlessState
    } else if field.ends_with("_j_m2")
        || field.contains("_j_m2_")
        || field.ends_with("enthalpy_j_m2_ofe_ground")
    {
        AdaptiveOwnerDimensionV1::AreaEnergyJM2
    } else if field.ends_with("_kg_m2")
        || field.contains("_kg_m2_")
        || field.contains("_kg_h2o_m2_")
        || matches!(
            field.as_str(),
            "ammonium_n" | "nitrate_n" | "carbon" | "nitrogen" | "dry_matter"
        )
    {
        AdaptiveOwnerDimensionV1::AreaMassKgM2
    } else if field.ends_with("_m")
        && !field.ends_with("area_m")
        && !field.contains("roughness")
        && !field.contains("height")
        && !field.contains("thickness")
    {
        AdaptiveOwnerDimensionV1::WaterDepthM
    } else if owner_id == "snow" && matches!(field.as_str(), "mass_swe_m" | "thickness_m") {
        AdaptiveOwnerDimensionV1::WaterDepthM
    } else {
        return None;
    };

    let (contract_id, tolerance_id, absolute_tolerance, relative_tolerance) =
        match (owner_id, dimension) {
            ("snow", AdaptiveOwnerDimensionV1::WaterDepthM) => (
                "SC-SNOWENERGY-001@25",
                "TOL-SNOWENERGY-004-adaptive-depth",
                1.0e-9_f64,
                ADAPTIVE_SNOW_EXTENSIVE_RELATIVE_TOLERANCE,
            ),
            ("snow", AdaptiveOwnerDimensionV1::AreaMassKgM2) => (
                "SC-SNOWENERGY-001@25",
                "TOL-SNOWENERGY-004-adaptive-mass",
                5.0e-6_f64,
                ADAPTIVE_SNOW_EXTENSIVE_RELATIVE_TOLERANCE,
            ),
            ("snow", AdaptiveOwnerDimensionV1::AreaEnergyJM2) => (
                "SC-SNOWENERGY-001@25",
                "TOL-SNOWENERGY-004-adaptive-energy",
                1.0e-6_f64,
                ADAPTIVE_THERMAL_ENERGY_RELATIVE_TOLERANCE,
            ),
            ("snow", AdaptiveOwnerDimensionV1::TemperatureKOrC) => (
                "SC-SNOWENERGY-001@25",
                "TOL-SNOWENERGY-004-adaptive-temperature",
                ADAPTIVE_TEMPERATURE_ABSOLUTE_TOLERANCE_K,
                ADAPTIVE_TEMPERATURE_RELATIVE_TOLERANCE,
            ),
            ("vegetation", AdaptiveOwnerDimensionV1::TemperatureKOrC) => (
                "SC-VEGETATION-001@29",
                "OPENWEPP_ADAPTIVE_COMPOSITION_V1-temperature",
                ADAPTIVE_TEMPERATURE_ABSOLUTE_TOLERANCE_K,
                ADAPTIVE_TEMPERATURE_RELATIVE_TOLERANCE,
            ),
            ("vegetation", AdaptiveOwnerDimensionV1::SpecificHumidityKgKg) => (
                "SC-VEGETATION-001",
                "OPENWEPP_ADAPTIVE_COMPOSITION_V1-humidity",
                1.0e-12,
                ADAPTIVE_INTENSIVE_RELATIVE_TOLERANCE,
            ),
            ("vegetation", AdaptiveOwnerDimensionV1::HydraulicPotentialMm) => (
                "SC-VEGETATION-001",
                "OPENWEPP_ADAPTIVE_COMPOSITION_V1-hydraulic",
                1.0e-7,
                ADAPTIVE_INTENSIVE_RELATIVE_TOLERANCE,
            ),
            ("vegetation", AdaptiveOwnerDimensionV1::CarbonDioxidePressurePa) => (
                "SC-VEGETATION-001",
                "OPENWEPP_ADAPTIVE_COMPOSITION_V1-ci",
                1.0e-6_f64,
                ADAPTIVE_INTENSIVE_RELATIVE_TOLERANCE,
            ),
            ("vegetation", AdaptiveOwnerDimensionV1::DimensionlessState) => (
                "SC-VEGETATION-001",
                "OPENWEPP_ADAPTIVE_COMPOSITION_V1-beta",
                1.0e-10,
                ADAPTIVE_INTENSIVE_RELATIVE_TOLERANCE,
            ),
            ("vegetation", AdaptiveOwnerDimensionV1::AreaMassKgM2) => (
                "SC-VEGETATION-001",
                "OPENWEPP_ADAPTIVE_COMPOSITION_V1-mass",
                1.0e-12,
                ADAPTIVE_EXTENSIVE_RELATIVE_TOLERANCE,
            ),
            ("bgc", AdaptiveOwnerDimensionV1::AreaMassKgM2) => (
                "SC-BIOGEOCHEM-001",
                "OPENWEPP_ADAPTIVE_COMPOSITION_V1-mass",
                1.0e-14,
                ADAPTIVE_EXTENSIVE_RELATIVE_TOLERANCE,
            ),
            ("hydrology", AdaptiveOwnerDimensionV1::WaterDepthM) => (
                "SC-WATBAL-001",
                "OPENWEPP_ADAPTIVE_COMPOSITION_V1-depth",
                1.0e-9,
                ADAPTIVE_EXTENSIVE_RELATIVE_TOLERANCE,
            ),
            ("surface_liquid", AdaptiveOwnerDimensionV1::WaterDepthM) => (
                "SC-LANDSURFACEENERGY-001",
                "OPENWEPP_ADAPTIVE_COMPOSITION_V1-depth",
                1.0e-17,
                ADAPTIVE_EXTENSIVE_RELATIVE_TOLERANCE,
            ),
            ("surface_liquid", AdaptiveOwnerDimensionV1::AreaMassKgM2) => (
                "SC-LANDSURFACEENERGY-001",
                "OPENWEPP_ADAPTIVE_COMPOSITION_V1-mass",
                1.0e-14,
                ADAPTIVE_EXTENSIVE_RELATIVE_TOLERANCE,
            ),
            ("surface_liquid", AdaptiveOwnerDimensionV1::AreaEnergyJM2) => (
                "SC-LANDSURFACEENERGY-001",
                "OPENWEPP_ADAPTIVE_COMPOSITION_V1-energy",
                1.0e-9,
                ADAPTIVE_EXTENSIVE_RELATIVE_TOLERANCE,
            ),
            ("land_surface_energy" | "soil_thermal", AdaptiveOwnerDimensionV1::TemperatureKOrC) => {
                (
                    "SC-LANDSURFACEENERGY-001",
                    "OPENWEPP_ADAPTIVE_COMPOSITION_V1-temperature",
                    ADAPTIVE_TEMPERATURE_ABSOLUTE_TOLERANCE_K,
                    ADAPTIVE_TEMPERATURE_RELATIVE_TOLERANCE,
                )
            }
            ("soil_thermal", AdaptiveOwnerDimensionV1::AreaEnergyJM2) => (
                "SC-LANDSURFACEENERGY-001",
                "OPENWEPP_ADAPTIVE_COMPOSITION_V1-energy",
                1.0e-6,
                ADAPTIVE_SOIL_THERMAL_ENERGY_RELATIVE_TOLERANCE,
            ),
            ("land_surface_energy", AdaptiveOwnerDimensionV1::AreaEnergyJM2) => (
                "SC-LANDSURFACEENERGY-001",
                "OPENWEPP_ADAPTIVE_COMPOSITION_V1-energy",
                1.0e-6,
                ADAPTIVE_THERMAL_ENERGY_RELATIVE_TOLERANCE,
            ),
            (_, AdaptiveOwnerDimensionV1::WaterDepthM) => (
                "SC-WATBAL-001",
                "OPENWEPP_ADAPTIVE_COMPOSITION_V1-depth",
                1.0e-9,
                ADAPTIVE_EXTENSIVE_RELATIVE_TOLERANCE,
            ),
            (_, AdaptiveOwnerDimensionV1::AreaMassKgM2) => (
                "SC-BIOGEOCHEM-001",
                "OPENWEPP_ADAPTIVE_COMPOSITION_V1-mass",
                1.0e-14,
                ADAPTIVE_EXTENSIVE_RELATIVE_TOLERANCE,
            ),
            (_, AdaptiveOwnerDimensionV1::AreaEnergyJM2) => (
                "SC-LANDSURFACEENERGY-001",
                "OPENWEPP_ADAPTIVE_COMPOSITION_V1-energy",
                1.0e-6,
                ADAPTIVE_EXTENSIVE_RELATIVE_TOLERANCE,
            ),
            (_, AdaptiveOwnerDimensionV1::TemperatureKOrC) => (
                "SC-LANDSURFACEENERGY-001",
                "OPENWEPP_ADAPTIVE_COMPOSITION_V1-temperature",
                ADAPTIVE_TEMPERATURE_ABSOLUTE_TOLERANCE_K,
                ADAPTIVE_TEMPERATURE_RELATIVE_TOLERANCE,
            ),
            (_, AdaptiveOwnerDimensionV1::SpecificHumidityKgKg) => (
                "SC-VEGETATION-001",
                "OPENWEPP_ADAPTIVE_COMPOSITION_V1-humidity",
                1.0e-12,
                ADAPTIVE_INTENSIVE_RELATIVE_TOLERANCE,
            ),
            (_, AdaptiveOwnerDimensionV1::HydraulicPotentialMm) => (
                "SC-VEGETATION-001",
                "OPENWEPP_ADAPTIVE_COMPOSITION_V1-hydraulic",
                1.0e-7,
                ADAPTIVE_INTENSIVE_RELATIVE_TOLERANCE,
            ),
            (_, AdaptiveOwnerDimensionV1::CarbonDioxidePressurePa) => (
                "SC-VEGETATION-001",
                "OPENWEPP_ADAPTIVE_COMPOSITION_V1-ci",
                1.0e-6,
                ADAPTIVE_INTENSIVE_RELATIVE_TOLERANCE,
            ),
            (_, AdaptiveOwnerDimensionV1::DimensionlessState) => (
                "SC-VEGETATION-001",
                "OPENWEPP_ADAPTIVE_COMPOSITION_V1-beta",
                1.0e-10,
                ADAPTIVE_INTENSIVE_RELATIVE_TOLERANCE,
            ),
        };
    Some((
        AdaptiveToleranceAuthorityV1 {
            contract_id,
            tolerance_id,
            dimension,
        },
        absolute_tolerance,
        relative_tolerance,
    ))
}

fn adaptive_collect_snow_lane_cold_content(
    value: &serde_json::Value,
    scalars: &mut Vec<AdaptiveOwnerScalarV1>,
) -> Result<(), DirectV11RealConsumerError> {
    let lanes = value
        .get("lanes")
        .and_then(serde_json::Value::as_array)
        .ok_or(DirectV11RealConsumerError::Identity(
            "adaptive snow owner lane projection",
        ))?;
    for (lane_index, lane) in lanes.iter().enumerate() {
        let state = lane.as_array().and_then(|entry| entry.get(1)).ok_or(
            DirectV11RealConsumerError::Identity("adaptive snow owner lane tuple"),
        )?;
        let layers = state
            .get("layers")
            .and_then(serde_json::Value::as_array)
            .ok_or(DirectV11RealConsumerError::Identity(
                "adaptive snow owner represented layers",
            ))?;
        let mut lane_cold_content_j_m2 = 0.0;
        for layer in layers {
            let value =
                layer
                    .get("cold_content_j_m2")
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "adaptive snow owner layer cold content",
                    ))?;
            let cold_content_j_m2 = match value {
                serde_json::Value::Number(value) => value.as_f64(),
                serde_json::Value::String(value) => adaptive_float_bits(value).map(f64::from_bits),
                _ => None,
            }
            .ok_or(DirectV11RealConsumerError::Identity(
                "adaptive snow owner layer cold-content scalar",
            ))?;
            if !cold_content_j_m2.is_finite() || cold_content_j_m2 < 0.0 {
                return Err(DirectV11RealConsumerError::Identity(
                    "adaptive snow owner layer cold-content domain",
                ));
            }
            lane_cold_content_j_m2 += cold_content_j_m2;
        }
        let path = format!("lanes[{lane_index}][1].aggregate_cold_content_j_m2");
        let (tolerance_authority, absolute_tolerance, relative_tolerance) =
            adaptive_scalar_policy("snow", &path).ok_or(DirectV11RealConsumerError::Identity(
                "adaptive snow aggregate cold-content tolerance",
            ))?;
        scalars.push(AdaptiveOwnerScalarV1 {
            owner_id: "snow".to_owned(),
            path,
            value: lane_cold_content_j_m2,
            tolerance_authority,
            absolute_tolerance,
            relative_tolerance,
        });
    }
    Ok(())
}

fn adaptive_collect_value(
    owner_id: &str,
    path: &str,
    value: &serde_json::Value,
    scalars: &mut Vec<AdaptiveOwnerScalarV1>,
    discrete_surfaces: &mut Vec<AdaptiveOwnerDiscreteSurfaceV1>,
) -> Result<(), DirectV11RealConsumerError> {
    let normalized_path = path.to_ascii_lowercase();
    let field = adaptive_field_name(&normalized_path);
    let receipt_container = field.contains("receipt")
        && !field.ends_with("_sha256")
        && !field.ends_with("_digest")
        && !field.ends_with("_id");
    let derived_identity = field.ends_with("_sha256")
        || field.ends_with("_digest")
        || field == "fingerprint"
        || field.ends_with("transaction_id")
        || field.ends_with("proposal_id")
        || field.ends_with("receipt_id");
    if derived_identity {
        // Direct and composed trials deliberately have different transaction,
        // receipt, and derived-state identities. Those identities seal each
        // trial but are not physical numerical state. Their field presence is
        // retained by the containing schema surface.
        return Ok(());
    }
    match value {
        serde_json::Value::Object(fields) => {
            let keys = fields.keys().cloned().collect::<Vec<_>>().join("\0");
            discrete_surfaces.push(AdaptiveOwnerDiscreteSurfaceV1 {
                owner_id: owner_id.to_owned(),
                path: path.to_owned(),
                kind: if receipt_container {
                    adaptive_receipt_container_kind(owner_id, &normalized_path)
                } else {
                    AdaptiveDiscreteSurfaceKindV1::Schema
                },
                exact_value: keys,
            });
            if receipt_container {
                // Stable receipt membership/order remains exact across paths;
                // the narrowly classified WB14 history above is trial
                // factorization lineage. In both cases bind the complete
                // lineage independently within each trial path.
                discrete_surfaces.push(AdaptiveOwnerDiscreteSurfaceV1 {
                    owner_id: owner_id.to_owned(),
                    path: path.to_owned(),
                    kind: AdaptiveDiscreteSurfaceKindV1::ReceiptLineage,
                    exact_value: format!(
                        "{}\0{}",
                        ADAPTIVE_RECEIPT_LINEAGE_AUTHORITY,
                        adaptive_digest_hex(digest_bytes(&serde_json::to_vec(value)?))
                    ),
                });
                return Ok(());
            }
            for (key, value) in fields {
                let child = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{path}.{key}")
                };
                adaptive_collect_value(owner_id, &child, value, scalars, discrete_surfaces)?;
            }
        }
        serde_json::Value::Array(values) => {
            let ordering_value = if receipt_container {
                adaptive_digest_hex(digest_bytes(&serde_json::to_vec(
                    &adaptive_receipt_ordering_projection(value),
                )?))
            } else {
                values.len().to_string()
            };
            discrete_surfaces.push(AdaptiveOwnerDiscreteSurfaceV1 {
                owner_id: owner_id.to_owned(),
                path: path.to_owned(),
                kind: if receipt_container {
                    adaptive_receipt_container_kind(owner_id, &normalized_path)
                } else {
                    AdaptiveDiscreteSurfaceKindV1::Ordering
                },
                exact_value: ordering_value,
            });
            if receipt_container {
                discrete_surfaces.push(AdaptiveOwnerDiscreteSurfaceV1 {
                    owner_id: owner_id.to_owned(),
                    path: path.to_owned(),
                    kind: AdaptiveDiscreteSurfaceKindV1::ReceiptLineage,
                    exact_value: format!(
                        "{}\0{}",
                        ADAPTIVE_RECEIPT_LINEAGE_AUTHORITY,
                        adaptive_digest_hex(digest_bytes(&serde_json::to_vec(value)?))
                    ),
                });
                return Ok(());
            }
            for (index, value) in values.iter().enumerate() {
                adaptive_collect_value(
                    owner_id,
                    &format!("{path}[{index}]"),
                    value,
                    scalars,
                    discrete_surfaces,
                )?;
            }
        }
        serde_json::Value::String(value) => {
            if adaptive_float_bits(value).is_some()
                && adaptive_record_derived_cache_classification(owner_id, path, discrete_surfaces)
            {
                return Ok(());
            }
            if adaptive_float_bits(value).is_some()
                && adaptive_record_numerical_warm_start_classification(
                    owner_id,
                    path,
                    discrete_surfaces,
                )
            {
                return Ok(());
            }
            if let (
                Some(bits),
                Some((tolerance_authority, absolute_tolerance, relative_tolerance)),
            ) = (
                adaptive_float_bits(value),
                adaptive_scalar_policy(owner_id, path),
            ) {
                let value = f64::from_bits(bits);
                if !value.is_finite() {
                    return Err(DirectV11RealConsumerError::Identity(
                        "adaptive comparison non-finite owner scalar",
                    ));
                }
                scalars.push(AdaptiveOwnerScalarV1 {
                    owner_id: owner_id.to_owned(),
                    path: path.to_owned(),
                    value,
                    tolerance_authority,
                    absolute_tolerance,
                    relative_tolerance,
                });
            } else {
                discrete_surfaces.push(AdaptiveOwnerDiscreteSurfaceV1 {
                    owner_id: owner_id.to_owned(),
                    path: path.to_owned(),
                    kind: adaptive_discrete_kind(path),
                    exact_value: value.clone(),
                });
            }
        }
        serde_json::Value::Bool(value) => {
            discrete_surfaces.push(AdaptiveOwnerDiscreteSurfaceV1 {
                owner_id: owner_id.to_owned(),
                path: path.to_owned(),
                kind: adaptive_discrete_kind(path),
                exact_value: value.to_string(),
            });
        }
        serde_json::Value::Number(value) => {
            if value.is_f64() {
                let value = value.as_f64().ok_or(DirectV11RealConsumerError::Identity(
                    "adaptive comparison numeric owner scalar",
                ))?;
                if !value.is_finite() {
                    return Err(DirectV11RealConsumerError::Identity(
                        "adaptive comparison non-finite owner scalar",
                    ));
                }
                if adaptive_record_derived_cache_classification(owner_id, path, discrete_surfaces) {
                    return Ok(());
                }
                if adaptive_record_numerical_warm_start_classification(
                    owner_id,
                    path,
                    discrete_surfaces,
                ) {
                    return Ok(());
                }
                if let Some((tolerance_authority, absolute_tolerance, relative_tolerance)) =
                    adaptive_scalar_policy(owner_id, path)
                {
                    scalars.push(AdaptiveOwnerScalarV1 {
                        owner_id: owner_id.to_owned(),
                        path: path.to_owned(),
                        value,
                        tolerance_authority,
                        absolute_tolerance,
                        relative_tolerance,
                    });
                } else {
                    discrete_surfaces.push(AdaptiveOwnerDiscreteSurfaceV1 {
                        owner_id: owner_id.to_owned(),
                        path: path.to_owned(),
                        kind: AdaptiveDiscreteSurfaceKindV1::ExactBinary64,
                        exact_value: format!("{:016x}", value.to_bits()),
                    });
                }
            } else {
                discrete_surfaces.push(AdaptiveOwnerDiscreteSurfaceV1 {
                    owner_id: owner_id.to_owned(),
                    path: path.to_owned(),
                    kind: adaptive_discrete_kind(path),
                    exact_value: value.to_string(),
                });
            }
        }
        serde_json::Value::Null => {
            discrete_surfaces.push(AdaptiveOwnerDiscreteSurfaceV1 {
                owner_id: owner_id.to_owned(),
                path: path.to_owned(),
                kind: adaptive_discrete_kind(path),
                exact_value: "null".to_owned(),
            });
        }
    }
    Ok(())
}

fn adaptive_discrete_kind(path: &str) -> AdaptiveDiscreteSurfaceKindV1 {
    let normalized_path = path.to_ascii_lowercase();
    let field = adaptive_field_name(path).to_ascii_lowercase();
    if normalized_path.contains("wb14_parent_working_state") && field == "next_child_ordinal" {
        // SC-SURFACELIQUID-001@8: this cursor enumerates accepted WB14 child
        // receipts inside one trial factorization. Direct H and composed H/2
        // + H/2 therefore bind different exact ordinals while representing
        // the same parent-cadence physical state.
        AdaptiveDiscreteSurfaceKindV1::ReceiptLineage
    } else if field.contains("schema") || field.contains("model_version") {
        AdaptiveDiscreteSurfaceKindV1::Schema
    } else if field.contains("posture") || field.contains("status") || field.contains("phase") {
        AdaptiveDiscreteSurfaceKindV1::Posture
    } else if field.contains("active") || field.contains("mode") || field.contains("class") {
        AdaptiveDiscreteSurfaceKindV1::ActiveSet
    } else if field.contains("topology") || field.contains("destination") {
        AdaptiveDiscreteSurfaceKindV1::Topology
    } else if field.contains("lane")
        || field.contains("ofe")
        || field.contains("tile")
        || field.contains("layer_id")
    {
        AdaptiveDiscreteSurfaceKindV1::Membership
    } else {
        AdaptiveDiscreteSurfaceKindV1::Identity
    }
}

fn adaptive_exact_discrete_sha256(surfaces: &[AdaptiveOwnerDiscreteSurfaceV1]) -> Digest32 {
    fn frame(out: &mut Vec<u8>, value: &[u8]) {
        out.extend_from_slice(&(value.len() as u64).to_be_bytes());
        out.extend_from_slice(value);
    }
    let mut preimage = Vec::new();
    frame(&mut preimage, ADAPTIVE_DISCRETE_SCHEMA.as_bytes());
    preimage.extend_from_slice(&(surfaces.len() as u64).to_be_bytes());
    for surface in surfaces {
        frame(&mut preimage, surface.owner_id.as_bytes());
        frame(&mut preimage, surface.path.as_bytes());
        frame(&mut preimage, surface.kind.wire_tag());
        frame(&mut preimage, surface.exact_value.as_bytes());
    }
    digest_bytes(&preimage)
}

fn adaptive_complete_owner_comparison_from_bytes(
    owners: BTreeMap<String, Vec<u8>>,
) -> Result<AdaptiveCompleteOwnerComparisonV1, DirectV11RealConsumerError> {
    let owner_ids = owners.keys().map(String::as_str).collect::<Vec<_>>();
    if owner_ids.as_slice() != ADAPTIVE_COMPLETE_OWNER_IDS {
        return Err(DirectV11RealConsumerError::Identity(
            "adaptive complete prognostic owner set",
        ));
    }
    let mut scalars = Vec::new();
    let mut exact_discrete_surfaces = Vec::new();
    for (owner_id, bytes) in owners {
        let value: serde_json::Value = serde_json::from_slice(&bytes)?;
        if owner_id == "snow" {
            adaptive_collect_snow_lane_cold_content(&value, &mut scalars)?;
        }
        adaptive_collect_value(
            &owner_id,
            "",
            &value,
            &mut scalars,
            &mut exact_discrete_surfaces,
        )?;
    }
    exact_discrete_surfaces.sort();
    let exact_discrete_sha256 = adaptive_exact_discrete_sha256(&exact_discrete_surfaces);
    Ok(AdaptiveCompleteOwnerComparisonV1 {
        scalars,
        exact_discrete_surfaces,
        exact_discrete_sha256,
    })
}

/// Canonical owner projections used by the complete default-off V11
/// attachment. These bytes are typed JSON projections of actual successor
/// owners; formatting/debug strings are deliberately excluded.
impl DirectV10RealConsumerShadow {
    /// Canonical complete-owner projection for a V11 parent transaction.
    ///
    /// The persistent V10 consumer publishes the successor LSE-V2 owner from
    /// [`Self::canonical_owner_state_bytes`], while the covered V11 executor
    /// operates on the validated V1 runtime held by the inner consumer.  A
    /// V11 parent must therefore begin with the same V1 LSE owner whose
    /// configuration/state digests are named by its support receipts.
    pub(crate) fn canonical_v11_parent_owner_state_bytes(
        &self,
    ) -> Result<BTreeMap<String, Vec<u8>>, DirectV11RealConsumerError> {
        let mut owners = self.canonical_owner_state_bytes()?;
        owners.insert(
            "land_surface_energy".to_owned(),
            serde_json::to_vec(&self.inner.lse_state)?,
        );
        Ok(owners)
    }

    pub(crate) fn adaptive_complete_owner_comparison_v1(
        &self,
        snow_owner_bytes: Vec<u8>,
    ) -> Result<AdaptiveCompleteOwnerComparisonV1, DirectV11RealConsumerError> {
        let mut owners = self.canonical_owner_state_bytes()?;
        owners.insert("snow".to_owned(), snow_owner_bytes);
        adaptive_complete_owner_comparison_from_bytes(owners)
    }

    pub fn canonical_owner_state_bytes(
        &self,
    ) -> Result<BTreeMap<String, Vec<u8>>, DirectV11RealConsumerError> {
        self.vegetation_state
            .validate(&self.vegetation_configuration)
            .map_err(DirectV10RealConsumerError::from)?;
        #[derive(Serialize)]
        struct HydrologyLayerProjection {
            layer_index: usize,
            theta_m: String,
            frozen_depth_m: String,
            frozen_water_m: String,
        }
        #[derive(Serialize)]
        struct HydrologyLaneProjection {
            lane_id: u32,
            area_m2: String,
            soil_water_m: String,
            infiltration_m: String,
            runoff_m: String,
            evapotranspiration_m: String,
            drainage_m: String,
            lateral_flow_m: String,
            snow_swe_m: String,
            snow_liquid_m: String,
            subsurface_layers: Vec<HydrologyLayerProjection>,
        }
        #[derive(Serialize)]
        struct HydrologyProjection {
            schema: &'static str,
            run_id: u64,
            hillslope_id: u32,
            lane_count: usize,
            day_count: usize,
            lanes: Vec<HydrologyLaneProjection>,
        }
        let frame = self.inner.hydrology_frame();
        let hydrology = HydrologyProjection {
            schema: "OPENWEPP_DIRECT_HYDROLOGY_OWNER_PROJECTION_V1",
            run_id: frame.identity.run_id,
            hillslope_id: frame.identity.hillslope_id,
            lane_count: frame.identity.lane_count,
            day_count: frame.identity.day_count,
            lanes: frame
                .lanes
                .iter()
                .map(|lane| HydrologyLaneProjection {
                    lane_id: lane.lane_id,
                    area_m2: format!("{:016x}", lane.area_m2.to_bits()),
                    soil_water_m: format!("{:016x}", lane.water.soil_water_m.to_bits()),
                    infiltration_m: format!("{:016x}", lane.water.infiltration_m.to_bits()),
                    runoff_m: format!("{:016x}", lane.water.runoff_m.to_bits()),
                    evapotranspiration_m: format!(
                        "{:016x}",
                        lane.water.evapotranspiration_m.to_bits()
                    ),
                    drainage_m: format!("{:016x}", lane.water.drainage_m.to_bits()),
                    lateral_flow_m: format!("{:016x}", lane.water.lateral_flow_m.to_bits()),
                    snow_swe_m: format!("{:016x}", lane.winter_column.snow.runtime_swe_m.to_bits()),
                    snow_liquid_m: format!(
                        "{:016x}",
                        lane.winter_column.snow.liquid_water_retained_m.to_bits()
                    ),
                    subsurface_layers: lane
                        .subsurface_layers
                        .iter()
                        .enumerate()
                        .map(|(layer_index, layer)| HydrologyLayerProjection {
                            layer_index,
                            theta_m: format!("{:016x}", layer.theta_m.to_bits()),
                            frozen_depth_m: format!("{:016x}", layer.frozen_depth_m.to_bits()),
                            frozen_water_m: format!("{:016x}", layer.frozen_water_m.to_bits()),
                        })
                        .collect(),
                })
                .collect(),
        };
        let surface =
            frame
                .surface_liquid_shadow
                .as_ref()
                .ok_or(DirectV11RealConsumerError::Identity(
                    "canonical surface-liquid owner",
                ))?;
        #[derive(Serialize)]
        struct SurfaceLiquidProjection<'a> {
            schema: &'static str,
            persistent_owner: serde_json::Value,
            wb14_parent_working_state:
                Option<&'a crate::direct_runtime::DirectWb14ParentWorkingState>,
        }
        let persistent_owner = serde_json::from_slice(
            &surface
                .canonical_bytes(&self.inner.surface_configuration)
                .map_err(|error| {
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                        DirectV9RealConsumerError::Serialization(error.to_string()),
                    ))
                })?,
        )?;
        let surface_projection = SurfaceLiquidProjection {
            schema: "OPENWEPP_SURFACE_LIQUID_COMPLETE_OWNER_PROJECTION_V2",
            persistent_owner,
            wb14_parent_working_state: self.inner.wb14_parent_working_state.as_ref(),
        };
        let mut owners = BTreeMap::new();
        owners.insert(
            "vegetation".to_owned(),
            serde_json::to_vec(&self.vegetation_state)?,
        );
        owners.insert(
            "land_surface_energy".to_owned(),
            serde_json::to_vec(&self.lse_state)?,
        );
        owners.insert(
            "surface_liquid".to_owned(),
            serde_json::to_vec(&surface_projection)?,
        );
        owners.insert("hydrology".to_owned(), serde_json::to_vec(&hydrology)?);
        owners.insert(
            "bgc".to_owned(),
            serde_json::to_vec(&self.inner.biogeochemistry)?,
        );
        owners.insert(
            "soil_thermal".to_owned(),
            serde_json::to_vec(&self.inner.soil_thermal)?,
        );
        Ok(owners)
    }
}

#[cfg(test)]
mod adaptive_comparison_tests {
    use super::*;

    fn bits(value: f64) -> serde_json::Value {
        serde_json::Value::String(format!("{:016x}", value.to_bits()))
    }

    fn owner_values() -> BTreeMap<String, serde_json::Value> {
        BTreeMap::from([
            (
                "bgc".to_owned(),
                serde_json::json!({
                    "layers": {"soil-1": {"ammonium_n": 0.02, "nitrate_n": 0.03}},
                    "receivers": {"litter": {"carbon": 0.4, "nitrogen": 0.04, "dry_matter": 0.8}},
                    "last_transaction_id": 41
                }),
            ),
            (
                "hydrology".to_owned(),
                serde_json::json!({
                    "schema": "OPENWEPP_DIRECT_HYDROLOGY_OWNER_PROJECTION_V1",
                    "run_id": 4,
                    "hillslope_id": 8,
                    "lane_count": 1,
                    "day_count": 3,
                    "lanes": [{
                        "lane_id": 7,
                        "area_m2": bits(100.0),
                        "soil_water_m": bits(0.25),
                        "infiltration_m": bits(0.02),
                        "runoff_m": bits(0.003),
                        "evapotranspiration_m": bits(0.004),
                        "drainage_m": bits(0.005),
                        "lateral_flow_m": bits(0.006),
                        "snow_swe_m": bits(0.01),
                        "snow_liquid_m": bits(0.001),
                        "subsurface_layers": [{
                            "layer_index": 0,
                            "theta_m": bits(0.2),
                            "frozen_depth_m": bits(0.01),
                            "frozen_water_m": bits(0.02)
                        }]
                    }]
                }),
            ),
            (
                "land_surface_energy".to_owned(),
                serde_json::json!({
                    "model_definition_sha256": "model-a",
                    "configuration_sha256": "config-a",
                    "state_sha256": "derived-a",
                    "owner_id": "lse-owner",
                    "last_accepted_transaction_id": 41,
                    "tiles": [{
                        "ofe_id": "ofe-1",
                        "tile_id": "open",
                        "surface_enthalpy_j_m2_tile_ground": 1200.0,
                        "surface_temperature_warm_start_k": 272.5
                    }]
                }),
            ),
            (
                "snow".to_owned(),
                serde_json::json!({
                    "schema": "OPENWEPP_STAGE3_CANONICAL_SNOW_OWNER_V3",
                    "lanes": [[7, {
                        "schema_version": 22,
                        "terminal_event_model": "enthalpy_event_v1",
                        "fingerprint": 99,
                        "lane_id": 7,
                        "next_interval_index": 5,
                        "layers": [{
                            "mass_swe_m": 0.01,
                            "thickness_m": 0.1,
                            "density_kg_m3": 100.0,
                            "settle_day_count": 1.0,
                            "temperature_c": -2.0,
                            "liquid_water_m": 0.001,
                            "cold_content_j_m2": 42000.0,
                            "refrozen_liquid_m": 0.0002
                        }],
                        "detached_retained_liquid_kg_m2": 0.1,
                        "initial_ice_kg_m2": 10.0,
                        "initial_retained_liquid_kg_m2": 0.2,
                        "cumulative_snowfall_kg_m2": 11.0,
                        "cumulative_external_liquid_kg_m2": 0.3,
                        "cumulative_deposition_kg_m2": 0.4,
                        "cumulative_sublimation_kg_m2": 0.5,
                        "cumulative_melt_kg_m2": 1.0,
                        "cumulative_unresolved_liquid_kg_m2": 0.0,
                        "cumulative_complete_energy_j_m2": 12345.0,
                        "cumulative_cold_energy_change_j_m2": 2345.0,
                        "cumulative_terminal_unallocated_energy_j_m2": 0.0
                    }]],
                    "final_lane_boundary_receipts": {"7": "receipt-one"},
                    "final_boundary_receipts": {"ofe-1\\u0000open": "receipt-two"},
                    "support_receipts": [
                        {"participant_id": "lane-7", "receipt_id": "local-a"},
                        {"participant_id": "lane-8", "receipt_id": "local-b"}
                    ]
                }),
            ),
            (
                "soil_thermal".to_owned(),
                serde_json::json!({
                    "owner_id": "soil-thermal",
                    "configuration_sha256": "config-s",
                    "state_sha256": "derived-s",
                    "snapshot_sha256": "derived-snapshot",
                    "last_accepted_transaction_id": 41,
                    "ofes": [{
                        "ofe_id": "ofe-1",
                        "ordered_layers": [{
                            "layer_id": "soil-1",
                            "temperature_k": 274.0,
                            "enthalpy_j_m2_ofe_ground": 9000.0
                        }]
                    }]
                }),
            ),
            (
                "surface_liquid".to_owned(),
                serde_json::json!({
                    "schema": "OPENWEPP_SURFACE_LIQUID_COMPLETE_OWNER_PROJECTION_V2",
                    "persistent_owner": {
                        "owner_id": "surface-liquid",
                        "configuration_sha256": "config-l",
                        "state_sha256": "derived-l",
                        "records": [{
                            "key": {"run_id": 4, "ofe_id": "ofe-1", "tile_id": "open"},
                            "liquid_kg_m2_tile": bits(0.25),
                            "last_accepted_transaction_id": 41
                        }],
                        "continuations": [{
                            "ofe_id": "ofe-1",
                            "day_index": 3,
                            "next_interval_index": 5,
                            "cumulative_supply_m": bits(0.01),
                            "cumulative_infiltration_m": bits(0.008),
                            "last_accepted_transaction_id": 41
                        }]
                    },
                    "wb14_parent_working_state": {
                        "schema": "OPENWEPP_DIRECT_WB14_PARENT_WORKING_STATE_V2",
                        "production_lane_ids": [7],
                        "accepted_until_ns": 600000000,
                        "next_child_ordinal": 1,
                        "parent_finalizations": null
                    }
                }),
            ),
            (
                "vegetation".to_owned(),
                serde_json::json!({
                    "configuration_sha256": "config-v",
                    "last_transaction_id": 41,
                    "model_definition_sha256": "model-v",
                    "state_sha256": "derived-v",
                    "occupancies": [{
                        "stratum_id": "tree",
                        "tile_id": "forest",
                        "state": {
                            "beta_hyd": 0.8,
                            "canopy_air_specific_humidity_kg_kg": 0.004,
                            "canopy_air_temperature_k": 272.0,
                            "canopy_liquid_kg_h2o_m2_tile_ground": 0.25,
                            "dry_stem_temperature_k": 272.1,
                            "root_node_potential_mm": -120.0,
                            "shade_ci_pa": 27.0,
                            "shade_leaf_potential_mm": -180.0,
                            "shade_leaf_temperature_k": 272.2,
                            "stem_potential_mm": -150.0,
                            "sun_ci_pa": 28.0,
                            "sun_leaf_potential_mm": -190.0,
                            "sun_leaf_temperature_k": 273.0,
                            "wet_surface_temperature_k": 272.3
                        }
                    }],
                    "strata": {"tree": {
                        "leaf_area": 3.0,
                        "stem_area": 0.6,
                        "root_area": 4.5,
                        "t10_k": 291.0,
                        "tissues": {"leaf": {"display": {"carbon": 0.3, "nitrogen": 0.02}}}
                    }},
                    "tile_canopy_air": {"forest": {
                        "canopy_air_specific_humidity_kg_kg": 0.004,
                        "canopy_air_temperature_k": 272.0
                    }}
                }),
            ),
        ])
    }

    fn comparison(
        owners: &BTreeMap<String, serde_json::Value>,
    ) -> Result<AdaptiveCompleteOwnerComparisonV1, DirectV11RealConsumerError> {
        adaptive_complete_owner_comparison_from_bytes(
            owners
                .iter()
                .map(|(owner, value)| {
                    (
                        owner.clone(),
                        serde_json::to_vec(value).expect("fixture serialization"),
                    )
                })
                .collect(),
        )
    }

    fn scalar<'a>(
        value: &'a AdaptiveCompleteOwnerComparisonV1,
        owner_id: &str,
        path: &str,
    ) -> &'a AdaptiveOwnerScalarV1 {
        value
            .scalars
            .iter()
            .find(|scalar| scalar.owner_id == owner_id && scalar.path == path)
            .expect("required scalar")
    }

    #[test]
    fn comparison_matrix_includes_complete_persistent_owner_state() {
        let value = comparison(&owner_values()).expect("comparison projection");
        for (owner_id, path) in [
            ("snow", "lanes[0][1].layers[0].mass_swe_m"),
            ("snow", "lanes[0][1].layers[0].temperature_c"),
            ("snow", "lanes[0][1].aggregate_cold_content_j_m2"),
            ("snow", "lanes[0][1].cumulative_snowfall_kg_m2"),
            ("snow", "lanes[0][1].cumulative_complete_energy_j_m2"),
            (
                "snow",
                "lanes[0][1].cumulative_terminal_unallocated_energy_j_m2",
            ),
            (
                "vegetation",
                "occupancies[0].state.canopy_liquid_kg_h2o_m2_tile_ground",
            ),
            ("vegetation", "strata.tree.tissues.leaf.display.carbon"),
            ("vegetation", "strata.tree.t10_k"),
            (
                "soil_thermal",
                "ofes[0].ordered_layers[0].enthalpy_j_m2_ofe_ground",
            ),
            ("hydrology", "lanes[0].infiltration_m"),
            ("hydrology", "lanes[0].subsurface_layers[0].theta_m"),
            (
                "surface_liquid",
                "persistent_owner.continuations[0].cumulative_infiltration_m",
            ),
            (
                "land_surface_energy",
                "tiles[0].surface_enthalpy_j_m2_tile_ground",
            ),
            ("bgc", "layers.soil-1.ammonium_n"),
        ] {
            let _ = scalar(&value, owner_id, path);
        }
    }

    #[test]
    fn scalar_identity_mismatch_is_discrete_and_keeps_finite_intersection_error() {
        let direct = comparison(&owner_values()).expect("direct comparison");
        let mut omitted = direct.clone();
        omitted.scalars.remove(0);
        let (error, mismatch) = direct.scaled_error(&omitted).expect("scalar omission");
        assert!(error.is_finite());
        assert!(mismatch);

        let mut substituted = direct.clone();
        substituted.scalars[0].path.push_str(".substituted");
        let (error, mismatch) = direct
            .scaled_error(&substituted)
            .expect("scalar-key substitution");
        assert!(error.is_finite());
        assert!(mismatch);

        let mut authority = direct.clone();
        authority.scalars[0].tolerance_authority.tolerance_id = "SUBSTITUTED-TOLERANCE";
        let (error, mismatch) = direct
            .scaled_error(&authority)
            .expect("tolerance-authority substitution");
        assert!(error.is_finite());
        assert!(mismatch);

        let mut reordered = direct.clone();
        reordered.scalars.swap(0, 1);
        let (error, mismatch) = direct
            .scaled_error(&reordered)
            .expect("scalar ordering poison");
        assert!(error.is_finite());
        assert!(mismatch);
    }

    #[test]
    fn duplicate_scalar_identity_is_rejected_instead_of_aliased() {
        let direct = comparison(&owner_values()).expect("direct comparison");
        let mut duplicate = direct.clone();
        duplicate.scalars.push(duplicate.scalars[0].clone());
        assert!(direct.scaled_error(&duplicate).is_err());
    }

    #[test]
    fn snow_tolerances_are_contract_dimensional_and_cumulatives_are_not_omitted() {
        let value = comparison(&owner_values()).expect("comparison projection");
        let tolerances: [(&str, AdaptiveOwnerDimensionV1, f64); 4] = [
            (
                "lanes[0][1].layers[0].mass_swe_m",
                AdaptiveOwnerDimensionV1::WaterDepthM,
                1.0e-9,
            ),
            (
                "lanes[0][1].cumulative_snowfall_kg_m2",
                AdaptiveOwnerDimensionV1::AreaMassKgM2,
                5.0e-6,
            ),
            (
                "lanes[0][1].cumulative_complete_energy_j_m2",
                AdaptiveOwnerDimensionV1::AreaEnergyJM2,
                1.0e-6,
            ),
            (
                "lanes[0][1].cumulative_terminal_unallocated_energy_j_m2",
                AdaptiveOwnerDimensionV1::AreaEnergyJM2,
                1.0e-6,
            ),
        ];
        for (path, dimension, absolute) in tolerances {
            let scalar = scalar(&value, "snow", path);
            assert_eq!(scalar.tolerance_authority.dimension, dimension);
            assert_eq!(scalar.absolute_tolerance.to_bits(), absolute.to_bits());
            assert_eq!(
                scalar.relative_tolerance.to_bits(),
                ADAPTIVE_SNOW_EXTENSIVE_RELATIVE_TOLERANCE.to_bits()
            );
            assert_eq!(
                scalar.tolerance_authority.contract_id,
                "SC-SNOWENERGY-001@25"
            );
            assert!(
                scalar
                    .tolerance_authority
                    .tolerance_id
                    .starts_with("TOL-SNOWENERGY-004-adaptive-")
            );
        }
    }

    #[test]
    fn numerical_warm_starts_are_authority_bound_but_not_physical_error_scalars() {
        let owners = owner_values();
        let baseline = comparison(&owners).expect("baseline comparison projection");
        let expected_vegetation_fields = [
            "beta_hyd",
            "canopy_air_specific_humidity_kg_kg",
            "canopy_air_temperature_k",
            "dry_stem_temperature_k",
            "root_node_potential_mm",
            "shade_ci_pa",
            "shade_leaf_potential_mm",
            "shade_leaf_temperature_k",
            "stem_potential_mm",
            "sun_ci_pa",
            "sun_leaf_potential_mm",
            "sun_leaf_temperature_k",
            "wet_surface_temperature_k",
        ];
        for field in expected_vegetation_fields {
            let path = format!("occupancies[0].state.{field}");
            assert_eq!(
                adaptive_numerical_warm_start_authority("vegetation", &path),
                Some(VEGETATION_NUMERICAL_WARM_START_AUTHORITY),
                "classification source guard for {path}",
            );
            assert!(baseline.exact_discrete_surfaces.iter().any(|surface| {
                surface.owner_id == "vegetation"
                    && surface.path == path
                    && surface.kind == AdaptiveDiscreteSurfaceKindV1::NumericalWarmStart
                    && surface.exact_value.contains("INV-VEGETATION-078")
            }));
            assert!(
                !baseline
                    .scalars
                    .iter()
                    .any(|scalar| scalar.owner_id == "vegetation" && scalar.path == path)
            );
        }
        for field in [
            "canopy_air_specific_humidity_kg_kg",
            "canopy_air_temperature_k",
        ] {
            let path = format!("tile_canopy_air.forest.{field}");
            assert_eq!(
                adaptive_numerical_warm_start_authority("vegetation", &path),
                Some(VEGETATION_NUMERICAL_WARM_START_AUTHORITY),
                "tile warm-start classification source guard for {path}",
            );
            assert!(baseline.exact_discrete_surfaces.iter().any(|surface| {
                surface.owner_id == "vegetation"
                    && surface.path == path
                    && surface.kind == AdaptiveDiscreteSurfaceKindV1::NumericalWarmStart
            }));
        }
        let lse_path = "tiles[0].surface_temperature_warm_start_k";
        assert_eq!(
            adaptive_numerical_warm_start_authority("land_surface_energy", lse_path),
            Some(LSE_NUMERICAL_WARM_START_AUTHORITY)
        );
        assert!(baseline.exact_discrete_surfaces.iter().any(|surface| {
            surface.owner_id == "land_surface_energy"
                && surface.path == lse_path
                && surface.kind == AdaptiveDiscreteSurfaceKindV1::NumericalWarmStart
                && surface.exact_value.contains("INV-LANDSURFACEENERGY-103")
        }));

        let mut changed_owners = owners.clone();
        *changed_owners
            .get_mut("vegetation")
            .and_then(|owner| owner.pointer_mut("/occupancies/0/state/root_node_potential_mm"))
            .expect("root warm-start field") = serde_json::json!(-9_999.0);
        *changed_owners
            .get_mut("land_surface_energy")
            .and_then(|owner| owner.pointer_mut("/tiles/0/surface_temperature_warm_start_k"))
            .expect("LSE warm-start field") = serde_json::json!(280.0);
        let changed = comparison(&changed_owners).expect("changed comparison projection");
        assert_eq!(
            baseline.scaled_error(&changed).expect("scaled error"),
            (0.0, false)
        );

        let baseline_owner_digest = digest_bytes(
            &serde_json::to_vec(&owners).expect("baseline canonical owner fixture bytes"),
        );
        let changed_owner_digest = digest_bytes(
            &serde_json::to_vec(&changed_owners).expect("changed canonical owner fixture bytes"),
        );
        assert_ne!(
            baseline_owner_digest, changed_owner_digest,
            "numerical warm-start bytes remain exact in owner identity/replay",
        );

        let mut substituted_authority = changed;
        let classified = substituted_authority
            .exact_discrete_surfaces
            .iter_mut()
            .find(|surface| {
                surface.kind == AdaptiveDiscreteSurfaceKindV1::NumericalWarmStart
                    && surface.owner_id == "vegetation"
            })
            .expect("classified numerical warm-start surface");
        classified.exact_value = "SUBSTITUTED_AUTHORITY".to_owned();
        let (_, mismatch) = baseline
            .scaled_error(&substituted_authority)
            .expect("authority substitution comparison");
        assert!(
            mismatch,
            "classification authority substitution fails closed"
        );
    }

    #[test]
    fn derived_area_caches_are_authority_bound_but_not_independent_prognostic_scalars() {
        let owners = owner_values();
        let baseline = comparison(&owners).expect("baseline comparison projection");
        for field in ["leaf_area", "stem_area", "root_area"] {
            let path = format!("strata.tree.{field}");
            assert_eq!(
                adaptive_derived_cache_authority("vegetation", &path),
                Some(VEGETATION_DERIVED_CACHE_AUTHORITY),
                "INV-VEGETATION-089 source guard for {path}",
            );
            assert!(baseline.exact_discrete_surfaces.iter().any(|surface| {
                surface.owner_id == "vegetation"
                    && surface.path == path
                    && surface.kind == AdaptiveDiscreteSurfaceKindV1::DerivedIntegrityCache
                    && surface.exact_value.contains("INV-VEGETATION-089")
            }));
        }

        let mut changed_owners = owners.clone();
        *changed_owners
            .get_mut("vegetation")
            .and_then(|owner| owner.pointer_mut("/strata/tree/leaf_area"))
            .expect("derived leaf-area cache") = serde_json::json!(30.0);
        let changed = comparison(&changed_owners).expect("changed comparison projection");
        assert_eq!(
            baseline.scaled_error(&changed).expect("scaled error"),
            (0.0, false)
        );
        assert_ne!(
            digest_bytes(&serde_json::to_vec(&owners).expect("baseline owner bytes")),
            digest_bytes(&serde_json::to_vec(&changed_owners).expect("changed owner bytes")),
            "derived cache remains exact in selected owner bytes/replay",
        );

        let mut substituted_authority = changed;
        substituted_authority
            .exact_discrete_surfaces
            .iter_mut()
            .find(|surface| {
                surface.kind == AdaptiveDiscreteSurfaceKindV1::DerivedIntegrityCache
                    && surface.path == "strata.tree.leaf_area"
            })
            .expect("derived cache classification")
            .exact_value = "SUBSTITUTED_INV-VEGETATION-089_AUTHORITY".to_owned();
        assert!(
            baseline
                .scaled_error(&substituted_authority)
                .expect("substituted authority comparison")
                .1,
            "derived-cache authority substitution fails closed",
        );
    }

    #[test]
    fn snow_layer_geometry_and_settle_are_inv043_derived_classifications() {
        let owners = owner_values();
        let baseline = comparison(&owners).expect("baseline comparison projection");
        for field in ["density_kg_m3", "settle_day_count"] {
            let path = format!("lanes[0][1].layers[0].{field}");
            assert_eq!(
                adaptive_derived_cache_authority("snow", &path),
                Some(SNOW_LAYER_DERIVED_CACHE_AUTHORITY),
                "INV-SNOWENERGY-043 source guard for {path}",
            );
            assert!(baseline.exact_discrete_surfaces.iter().any(|surface| {
                surface.owner_id == "snow"
                    && surface.path == path
                    && surface.kind == AdaptiveDiscreteSurfaceKindV1::DerivedIntegrityCache
                    && surface.exact_value.contains("INV-SNOWENERGY-043")
            }));
        }

        let mut changed_owners = owners.clone();
        *changed_owners
            .get_mut("snow")
            .and_then(|owner| owner.pointer_mut("/lanes/0/1/layers/0/density_kg_m3"))
            .expect("snow density cache") = serde_json::json!(100.000_000_1);
        *changed_owners
            .get_mut("snow")
            .and_then(|owner| owner.pointer_mut("/lanes/0/1/layers/0/settle_day_count"))
            .expect("snow settle cache") = serde_json::json!(1.000_000_1);
        let changed = comparison(&changed_owners).expect("changed comparison projection");
        assert_eq!(
            baseline.scaled_error(&changed).expect("scaled error"),
            (0.0, false)
        );
        assert_ne!(
            digest_bytes(&serde_json::to_vec(&owners).expect("baseline owner bytes")),
            digest_bytes(&serde_json::to_vec(&changed_owners).expect("changed owner bytes")),
            "derived snow caches remain exact in selected owner bytes/replay",
        );

        let mut substituted_authority = changed;
        substituted_authority
            .exact_discrete_surfaces
            .iter_mut()
            .find(|surface| {
                surface.kind == AdaptiveDiscreteSurfaceKindV1::DerivedIntegrityCache
                    && surface.path == "lanes[0][1].layers[0].density_kg_m3"
            })
            .expect("snow derived-cache classification")
            .exact_value = "SUBSTITUTED_INV-SNOWENERGY-043_AUTHORITY".to_owned();
        assert!(
            baseline
                .scaled_error(&substituted_authority)
                .expect("substituted authority comparison")
                .1,
            "snow derived-cache authority substitution fails closed",
        );
    }

    #[test]
    fn receipt_factorization_lineage_is_bound_per_path_but_not_cross_path_physics() {
        let owners = owner_values();
        let baseline = comparison(&owners).expect("baseline comparison projection");
        let mut changed_owners = owners.clone();
        *changed_owners
            .get_mut("snow")
            .and_then(|owner| owner.pointer_mut("/final_lane_boundary_receipts/7"))
            .expect("lane boundary receipt") = serde_json::json!("different-trial-receipt");
        let changed = comparison(&changed_owners).expect("changed comparison projection");
        assert_eq!(
            baseline
                .scaled_error(&changed)
                .expect("cross-path comparison"),
            (0.0, false),
            "trial-factorization lineage is not a physical semigroup mismatch",
        );
        assert_ne!(
            baseline.exact_discrete_sha256, changed.exact_discrete_sha256,
            "each trial path independently binds its complete receipt lineage",
        );
        let receipt_surface = baseline
            .exact_discrete_surfaces
            .iter()
            .find(|surface| {
                surface.owner_id == "snow"
                    && surface.path == "final_lane_boundary_receipts"
                    && surface.kind == AdaptiveDiscreteSurfaceKindV1::ReceiptLineage
            })
            .expect("receipt-lineage classification surface");
        assert!(
            receipt_surface
                .exact_value
                .starts_with(ADAPTIVE_RECEIPT_LINEAGE_AUTHORITY),
            "receipt lineage source authority is explicit",
        );

        let mut ordinal_owners = owners.clone();
        *ordinal_owners
            .get_mut("surface_liquid")
            .and_then(|owner| owner.pointer_mut("/wb14_parent_working_state/next_child_ordinal"))
            .expect("WB14 trial child ordinal") = serde_json::json!(2);
        let ordinal_changed = comparison(&ordinal_owners).expect("ordinal comparison projection");
        assert_eq!(
            baseline
                .scaled_error(&ordinal_changed)
                .expect("factorization-cursor comparison"),
            (0.0, false),
            "WB14 child receipt ordinal is not parent physical state",
        );
        assert_ne!(
            baseline.exact_discrete_sha256, ordinal_changed.exact_discrete_sha256,
            "each trial still binds its exact WB14 child receipt ordinal",
        );
        let ordinal_surface = baseline
            .exact_discrete_surfaces
            .iter()
            .find(|surface| {
                surface.owner_id == "surface_liquid"
                    && surface.path == "wb14_parent_working_state.next_child_ordinal"
            })
            .expect("WB14 factorization cursor surface");
        assert_eq!(
            ordinal_surface.kind,
            AdaptiveDiscreteSurfaceKindV1::ReceiptLineage,
            "WB14 child ordinal classification is authority-bound factorization lineage",
        );
        let mut substituted_authority = baseline.clone();
        substituted_authority
            .exact_discrete_surfaces
            .iter_mut()
            .find(|surface| {
                surface.owner_id == "surface_liquid"
                    && surface.path == "wb14_parent_working_state.next_child_ordinal"
            })
            .expect("WB14 factorization cursor substitution")
            .kind = AdaptiveDiscreteSurfaceKindV1::Identity;
        substituted_authority.exact_discrete_sha256 =
            adaptive_exact_discrete_sha256(&substituted_authority.exact_discrete_surfaces);
        assert_ne!(
            baseline.exact_discrete_sha256, substituted_authority.exact_discrete_sha256,
            "substituting the WB14 factorization authority changes the bound trial identity",
        );
    }

    #[test]
    fn wb14_per_ofe_receipt_keys_are_lineage_while_other_receipt_ordering_is_exact() {
        let mut direct_owners = owner_values();
        direct_owners
            .get_mut("surface_liquid")
            .and_then(|owner| owner.pointer_mut("/wb14_parent_working_state"))
            .and_then(serde_json::Value::as_object_mut)
            .expect("WB14 parent working state")
            .insert(
                "per_ofe_authorities".to_owned(),
                serde_json::json!({
                    "ofe-1": {
                        "receipts": {
                            "1111111111111111111111111111111111111111111111111111111111111111": {
                                "child_ordinal": 0
                            }
                        }
                    }
                }),
            );
        let direct = comparison(&direct_owners).expect("direct WB14 receipt projection");

        let mut composed_owners = direct_owners.clone();
        *composed_owners
            .get_mut("surface_liquid")
            .and_then(|owner| {
                owner.pointer_mut("/wb14_parent_working_state/per_ofe_authorities/ofe-1/receipts")
            })
            .expect("WB14 per-OFE receipt map") = serde_json::json!({
            "2222222222222222222222222222222222222222222222222222222222222222": {
                "child_ordinal": 0
            }
        });
        let composed = comparison(&composed_owners).expect("composed WB14 receipt projection");
        assert_eq!(
            direct
                .scaled_error(&composed)
                .expect("WB14 factorization comparison"),
            (0.0, false),
            "WB14 child receipt digest keys are transaction-factorization lineage",
        );
        assert_ne!(
            direct.exact_discrete_sha256, composed.exact_discrete_sha256,
            "each WB14 trial still binds its exact receipt map",
        );
        let wb14_receipt_path = "wb14_parent_working_state.per_ofe_authorities.ofe-1.receipts";
        let wb14_receipt_surfaces = direct
            .exact_discrete_surfaces
            .iter()
            .filter(|surface| {
                surface.owner_id == "surface_liquid" && surface.path == wb14_receipt_path
            })
            .collect::<Vec<_>>();
        assert_eq!(wb14_receipt_surfaces.len(), 2);
        assert!(
            wb14_receipt_surfaces
                .iter()
                .all(|surface| { surface.kind == AdaptiveDiscreteSurfaceKindV1::ReceiptLineage })
        );

        let mut direct_history_owners = direct_owners.clone();
        *direct_history_owners
            .get_mut("surface_liquid")
            .and_then(|owner| {
                owner.pointer_mut("/wb14_parent_working_state/per_ofe_authorities/ofe-1/receipts")
            })
            .expect("WB14 direct receipt history") = serde_json::json!([{
            "child_ordinal": 0,
            "receipt_sha256": "1111111111111111111111111111111111111111111111111111111111111111"
        }]);
        let direct_history =
            comparison(&direct_history_owners).expect("direct WB14 receipt history projection");
        let mut composed_history_owners = direct_history_owners;
        *composed_history_owners
            .get_mut("surface_liquid")
            .and_then(|owner| {
                owner.pointer_mut("/wb14_parent_working_state/per_ofe_authorities/ofe-1/receipts")
            })
            .expect("WB14 composed receipt history") = serde_json::json!([
            {
                "child_ordinal": 0,
                "receipt_sha256": "2222222222222222222222222222222222222222222222222222222222222222"
            },
            {
                "child_ordinal": 1,
                "receipt_sha256": "3333333333333333333333333333333333333333333333333333333333333333"
            }
        ]);
        let composed_history =
            comparison(&composed_history_owners).expect("composed WB14 receipt history projection");
        assert_eq!(
            direct_history
                .scaled_error(&composed_history)
                .expect("WB14 receipt-history factorization comparison"),
            (0.0, false),
            "one direct child and two composed children are distinct trial histories",
        );
        assert_ne!(
            direct_history.exact_discrete_sha256, composed_history.exact_discrete_sha256,
            "each child factorization retains its exact ordered receipt history",
        );

        let mut stable_ordering_owners = direct_owners;
        *stable_ordering_owners
            .get_mut("snow")
            .and_then(|owner| owner.pointer_mut("/final_lane_boundary_receipts"))
            .expect("stable receipt ordering") =
            serde_json::json!({"8": "different-valid-receipt-identity"});
        let stable_ordering =
            comparison(&stable_ordering_owners).expect("stable receipt ordering projection");
        assert!(
            direct
                .scaled_error(&stable_ordering)
                .expect("stable receipt ordering comparison")
                .1,
            "non-WB14 receipt membership/order must remain exact",
        );
        assert!(
            stable_ordering
                .exact_discrete_surfaces
                .iter()
                .any(|surface| {
                    surface.owner_id == "snow"
                        && surface.path == "final_lane_boundary_receipts"
                        && surface.kind == AdaptiveDiscreteSurfaceKindV1::ReceiptOrdering
                })
        );
    }

    #[test]
    fn physical_scalar_poison_matrix_changes_the_scaled_error() {
        let owners = owner_values();
        let baseline = comparison(&owners).expect("baseline");
        for (owner_id, pointer, poison) in [
            (
                "snow",
                "/lanes/0/1/cumulative_snowfall_kg_m2",
                serde_json::json!(11.01),
            ),
            (
                "snow",
                "/lanes/0/1/cumulative_complete_energy_j_m2",
                serde_json::json!(12346.0),
            ),
            (
                "snow",
                "/lanes/0/1/cumulative_terminal_unallocated_energy_j_m2",
                serde_json::json!(0.01),
            ),
            (
                "vegetation",
                "/occupancies/0/state/canopy_liquid_kg_h2o_m2_tile_ground",
                serde_json::json!(0.26),
            ),
            (
                "soil_thermal",
                "/ofes/0/ordered_layers/0/enthalpy_j_m2_ofe_ground",
                serde_json::json!(9001.0),
            ),
            ("hydrology", "/lanes/0/infiltration_m", bits(0.021)),
            (
                "hydrology",
                "/lanes/0/subsurface_layers/0/theta_m",
                bits(0.201),
            ),
            (
                "surface_liquid",
                "/persistent_owner/continuations/0/cumulative_infiltration_m",
                bits(0.009),
            ),
            (
                "land_surface_energy",
                "/tiles/0/surface_enthalpy_j_m2_tile_ground",
                serde_json::json!(1201.0),
            ),
            ("bgc", "/layers/soil-1/ammonium_n", serde_json::json!(0.021)),
        ] {
            let mut poisoned = owners.clone();
            *poisoned
                .get_mut(owner_id)
                .and_then(|value| value.pointer_mut(pointer))
                .expect("poison pointer") = poison;
            let poisoned = comparison(&poisoned).expect("poison comparison");
            let (scaled, discrete) = baseline.scaled_error(&poisoned).expect("scaled error");
            assert!(scaled > 0.0, "{owner_id}:{pointer} was omitted");
            assert!(!discrete, "{owner_id}:{pointer} became a discrete alias");
        }
    }

    #[test]
    fn adaptive_tolerance_authority_substitution_fails_closed() {
        let baseline = comparison(&owner_values()).expect("baseline");
        let mut substituted = baseline.clone();
        let scalar = substituted
            .scalars
            .iter_mut()
            .find(|scalar| scalar.owner_id == "snow")
            .expect("snow adaptive scalar");
        scalar.tolerance_authority.tolerance_id = "SUBSTITUTED_TOLERANCE_AUTHORITY";
        assert_eq!(
            baseline
                .scaled_error(&substituted)
                .expect("authority substitution comparison"),
            (0.0, true)
        );
    }

    #[test]
    fn vegetation_t10_is_prognostic_temperature_with_bound_authority() {
        let owners = owner_values();
        let baseline = comparison(&owners).expect("baseline");
        let t10 = scalar(&baseline, "vegetation", "strata.tree.t10_k");
        assert_eq!(
            t10.tolerance_authority.dimension,
            AdaptiveOwnerDimensionV1::TemperatureKOrC,
        );
        assert_eq!(t10.tolerance_authority.contract_id, "SC-VEGETATION-001@29");
        assert_eq!(
            t10.tolerance_authority.tolerance_id,
            "OPENWEPP_ADAPTIVE_COMPOSITION_V1-temperature",
        );

        let mut changed_owners = owners;
        *changed_owners
            .get_mut("vegetation")
            .and_then(|owner| owner.pointer_mut("/strata/tree/t10_k"))
            .expect("T10 persistent state") = serde_json::json!(291.001);
        let changed = comparison(&changed_owners).expect("changed T10 projection");
        let (scaled, discrete) = baseline.scaled_error(&changed).expect("T10 comparison");
        assert!(scaled > 0.0);
        assert!(
            !discrete,
            "T10 is a physical temperature scalar, not exact metadata"
        );

        let mut substituted = baseline.clone();
        substituted
            .scalars
            .iter_mut()
            .find(|scalar| scalar.owner_id == "vegetation" && scalar.path == "strata.tree.t10_k")
            .expect("T10 adaptive scalar")
            .tolerance_authority
            .contract_id = "SUBSTITUTED_T10_AUTHORITY";
        assert_eq!(
            baseline
                .scaled_error(&substituted)
                .expect("T10 authority substitution"),
            (0.0, true),
        );
    }

    #[test]
    fn discrete_poison_matrix_covers_schema_topology_active_set_and_exact_snow_fields() {
        let owners = owner_values();
        let baseline = comparison(&owners).expect("baseline");
        for (owner_id, pointer, poison) in [
            ("snow", "/schema", serde_json::json!("WRONG_SCHEMA")),
            ("snow", "/lanes/0/0", serde_json::json!(8)),
            (
                "snow",
                "/lanes/0/1/terminal_event_model",
                serde_json::json!(null),
            ),
            (
                "surface_liquid",
                "/persistent_owner/records/0/key/tile_id",
                serde_json::json!("forest"),
            ),
            (
                "surface_liquid",
                "/wb14_parent_working_state/production_lane_ids/0",
                serde_json::json!(8),
            ),
        ] {
            let mut poisoned = owners.clone();
            *poisoned
                .get_mut(owner_id)
                .and_then(|value| value.pointer_mut(pointer))
                .expect("poison pointer") = poison;
            let poisoned = comparison(&poisoned).expect("poison comparison");
            let (_, discrete) = baseline.scaled_error(&poisoned).expect("scaled error");
            assert!(discrete, "{owner_id}:{pointer} was not exact");
            assert_ne!(
                baseline.exact_discrete_sha256, poisoned.exact_discrete_sha256,
                "{owner_id}:{pointer} did not poison the typed digest"
            );
        }
    }

    #[test]
    fn receipt_identities_are_not_numerical_state_but_receipt_ordering_is_exact() {
        let owners = owner_values();
        let baseline = comparison(&owners).expect("baseline");

        let mut identity_poison = owners.clone();
        *identity_poison
            .get_mut("snow")
            .and_then(|value| value.pointer_mut("/final_lane_boundary_receipts/7"))
            .expect("snow receipt poison") = serde_json::json!("different-valid-receipt-identity");
        *identity_poison
            .get_mut("land_surface_energy")
            .and_then(|value| value.pointer_mut("/state_sha256"))
            .expect("LSE state identity poison") =
            serde_json::json!("different-derived-state-identity");
        let identity_poison = comparison(&identity_poison).expect("identity poison");
        assert_eq!(
            baseline
                .scaled_error(&identity_poison)
                .expect("identity comparison"),
            (0.0, false)
        );

        let mut ordering_poison = owners;
        *ordering_poison
            .get_mut("snow")
            .and_then(|value| value.pointer_mut("/final_lane_boundary_receipts"))
            .expect("snow receipt ordering poison") =
            serde_json::json!({"8": "different-valid-receipt-identity"});
        let ordering_poison = comparison(&ordering_poison).expect("ordering poison");
        let (_, discrete) = baseline
            .scaled_error(&ordering_poison)
            .expect("ordering comparison");
        assert!(discrete);

        let mut array_identity_poison = owner_values();
        *array_identity_poison
            .get_mut("snow")
            .and_then(|value| value.pointer_mut("/support_receipts/0/receipt_id"))
            .expect("receipt-array identity poison") = serde_json::json!("local-c");
        let array_identity_poison =
            comparison(&array_identity_poison).expect("receipt-array identity comparison");
        assert_eq!(
            baseline
                .scaled_error(&array_identity_poison)
                .expect("receipt-array identity comparison"),
            (0.0, false),
        );

        for poison in [
            serde_json::json!([
                {"participant_id": "lane-8", "receipt_id": "local-b"},
                {"participant_id": "lane-7", "receipt_id": "local-a"}
            ]),
            serde_json::json!([
                {"participant_id": "lane-7", "receipt_id": "local-a"}
            ]),
            serde_json::json!([
                {"participant_id": "lane-7", "receipt_id": "local-a"},
                {"participant_id": "lane-7", "receipt_id": "local-a"}
            ]),
        ] {
            let mut poisoned = owner_values();
            *poisoned
                .get_mut("snow")
                .and_then(|value| value.pointer_mut("/support_receipts"))
                .expect("receipt-array ordering poison") = poison;
            let poisoned = comparison(&poisoned).expect("receipt-array ordering poison");
            assert!(
                baseline
                    .scaled_error(&poisoned)
                    .expect("receipt-array ordering comparison")
                    .1,
                "receipt-array omission/reorder/duplication must remain exact",
            );
        }
    }

    #[test]
    fn malformed_owner_set_and_nonfinite_physical_bits_fail_closed() {
        let mut missing = owner_values();
        missing.remove("bgc");
        assert!(comparison(&missing).is_err());

        let mut extra = owner_values();
        extra.insert("unknown".into(), serde_json::json!({}));
        assert!(comparison(&extra).is_err());

        let mut nonfinite = owner_values();
        *nonfinite
            .get_mut("hydrology")
            .and_then(|value| value.pointer_mut("/lanes/0/soil_water_m"))
            .expect("nonfinite poison") = serde_json::json!("7ff8000000000000");
        assert!(comparison(&nonfinite).is_err());
    }
}
