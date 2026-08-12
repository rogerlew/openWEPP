//! V2 tile-resolved vegetation warm-start state.

use std::collections::BTreeMap;

use openwepp_kernel_contract::{OccupancyId, SoilLayerId};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

/// A root-layer hydraulic potential in millimetres of water.
pub type RootPotentialMmByLayer = (SoilLayerId, f64);

/// The complete set of V2 state lanes, keyed by exact occupancy identity.
pub type OccupancyStateLanes = BTreeMap<OccupancyId, OccupancyState>;

/// One V2 occupancy-local numerical warm-start lane.
///
/// Field declaration order is recursively lexicographic and is therefore the
/// canonical JSON serialization order required by `SC-VEGETATION-001@6`.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OccupancyState {
    pub beta_hyd: f64,
    pub canopy_air_specific_humidity_kg_kg: f64,
    pub canopy_air_temperature_k: f64,
    pub canopy_liquid_kg_h2o_m2_tile_ground: f64,
    pub dry_stem_temperature_k: f64,
    pub last_accepted_transaction_id: Option<u128>,
    pub root_potential_mm_by_layer: Vec<RootPotentialMmByLayer>,
    pub shade_ci_pa: f64,
    pub shade_leaf_potential_mm: f64,
    pub shade_leaf_temperature_k: f64,
    pub stem_potential_mm: f64,
    pub sun_ci_pa: f64,
    pub sun_leaf_potential_mm: f64,
    pub sun_leaf_temperature_k: f64,
    pub wet_surface_temperature_k: f64,
}

impl OccupancyState {
    /// Parses and validates one exact V2 state lane.
    pub fn parse_strict(
        bytes: &[u8],
        configured_root_layer_ids: &[SoilLayerId],
        expected_previous_transaction_id: Option<u128>,
    ) -> Result<Self, OccupancyStateError> {
        let state: Self = serde_json::from_slice(bytes)
            .map_err(|error| OccupancyStateError::Schema(error.to_string()))?;
        state.validate(configured_root_layer_ids, expected_previous_transaction_id)?;
        Ok(state)
    }

    /// Validates all numerical, layer-identity, and transaction-lineage domains.
    pub fn validate(
        &self,
        configured_root_layer_ids: &[SoilLayerId],
        expected_previous_transaction_id: Option<u128>,
    ) -> Result<(), OccupancyStateError> {
        finite_fraction(self.beta_hyd, "beta_hyd")?;
        finite_nonnegative(
            self.canopy_air_specific_humidity_kg_kg,
            "canopy_air_specific_humidity_kg_kg",
        )?;
        finite_positive(self.canopy_air_temperature_k, "canopy_air_temperature_k")?;
        finite_nonnegative(
            self.canopy_liquid_kg_h2o_m2_tile_ground,
            "canopy_liquid_kg_h2o_m2_tile_ground",
        )?;
        finite_positive(self.dry_stem_temperature_k, "dry_stem_temperature_k")?;
        finite_positive(self.shade_ci_pa, "shade_ci_pa")?;
        finite(self.shade_leaf_potential_mm, "shade_leaf_potential_mm")?;
        finite_positive(self.shade_leaf_temperature_k, "shade_leaf_temperature_k")?;
        finite(self.stem_potential_mm, "stem_potential_mm")?;
        finite_positive(self.sun_ci_pa, "sun_ci_pa")?;
        finite(self.sun_leaf_potential_mm, "sun_leaf_potential_mm")?;
        finite_positive(self.sun_leaf_temperature_k, "sun_leaf_temperature_k")?;
        finite_positive(self.wet_surface_temperature_k, "wet_surface_temperature_k")?;

        let found_layer_ids = self
            .root_potential_mm_by_layer
            .iter()
            .map(|(layer_id, _)| layer_id.clone())
            .collect::<Vec<_>>();
        if found_layer_ids != configured_root_layer_ids {
            return Err(OccupancyStateError::RootLayerIdentity {
                expected: configured_root_layer_ids.to_vec(),
                found: found_layer_ids,
            });
        }
        for (_, potential_mm) in &self.root_potential_mm_by_layer {
            finite(*potential_mm, "root_potential_mm_by_layer")?;
        }

        if self.last_accepted_transaction_id != expected_previous_transaction_id {
            return Err(OccupancyStateError::StaleTransaction {
                expected: expected_previous_transaction_id,
                found: self.last_accepted_transaction_id,
            });
        }
        Ok(())
    }

    /// Returns the canonical recursively lexicographic JSON bytes for this lane.
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, OccupancyStateError> {
        serde_json::to_vec(self).map_err(|error| OccupancyStateError::Schema(error.to_string()))
    }

    /// Returns the lowercase SHA-256 digest of [`Self::canonical_bytes`].
    pub fn canonical_sha256(&self) -> Result<String, OccupancyStateError> {
        Ok(format!("{:x}", Sha256::digest(self.canonical_bytes()?)))
    }
}

#[derive(Clone, Debug, Error, PartialEq)]
pub enum OccupancyStateError {
    #[error("VEG-E-SCHEMA-001: invalid V2 occupancy state: {0}")]
    Schema(String),
    #[error("VEG-E-071: out-of-domain V2 occupancy state field {0}")]
    Domain(&'static str),
    #[error("VEG-E-071: root-layer identities do not exactly match configuration")]
    RootLayerIdentity {
        expected: Vec<SoilLayerId>,
        found: Vec<SoilLayerId>,
    },
    #[error(
        "VEG-E-071: stale occupancy transaction identity: expected {expected:?}, found {found:?}"
    )]
    StaleTransaction {
        expected: Option<u128>,
        found: Option<u128>,
    },
}

fn finite(value: f64, field: &'static str) -> Result<(), OccupancyStateError> {
    if !value.is_finite() {
        return Err(OccupancyStateError::Domain(field));
    }
    Ok(())
}

fn finite_positive(value: f64, field: &'static str) -> Result<(), OccupancyStateError> {
    if !value.is_finite() || value <= 0.0 {
        return Err(OccupancyStateError::Domain(field));
    }
    Ok(())
}

fn finite_nonnegative(value: f64, field: &'static str) -> Result<(), OccupancyStateError> {
    if !value.is_finite() || value < 0.0 {
        return Err(OccupancyStateError::Domain(field));
    }
    Ok(())
}

fn finite_fraction(value: f64, field: &'static str) -> Result<(), OccupancyStateError> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return Err(OccupancyStateError::Domain(field));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    type DomainMutation = (&'static str, fn(&mut OccupancyState));

    const AUTHORITY_BYTES: &[u8] = br#"{"beta_hyd":0.73,"canopy_air_specific_humidity_kg_kg":0.009,"canopy_air_temperature_k":295.4,"canopy_liquid_kg_h2o_m2_tile_ground":0.17,"dry_stem_temperature_k":294.8,"last_accepted_transaction_id":4,"root_potential_mm_by_layer":[["soil-1",-5100.0],["soil-2",-6200.0]],"shade_ci_pa":27.1,"shade_leaf_potential_mm":-7100.0,"shade_leaf_temperature_k":295.0,"stem_potential_mm":-6600.0,"sun_ci_pa":25.9,"sun_leaf_potential_mm":-7400.0,"sun_leaf_temperature_k":296.2,"wet_surface_temperature_k":294.9}"#;
    const AUTHORITY_SHA256: &str =
        "5cb16721125b5352e4aadb861b5e928f13ce05ff32f34533798648ebc2c4bd4b";

    fn layer(value: &str) -> SoilLayerId {
        SoilLayerId::try_new(value).expect("valid test layer")
    }

    fn configured_layers() -> Vec<SoilLayerId> {
        vec![layer("soil-1"), layer("soil-2")]
    }

    fn authority_state() -> OccupancyState {
        serde_json::from_slice(AUTHORITY_BYTES).expect("authority state parses")
    }

    fn assert_domain(state: &OccupancyState, field: &'static str) {
        assert_eq!(
            state.validate(&configured_layers(), Some(4)),
            Err(OccupancyStateError::Domain(field))
        );
    }

    #[test]
    fn authority_lane_has_exact_canonical_bytes_and_digest() {
        let state = OccupancyState::parse_strict(AUTHORITY_BYTES, &configured_layers(), Some(4))
            .expect("authority state validates");
        assert_eq!(state.canonical_bytes().expect("serialize"), AUTHORITY_BYTES);
        assert_eq!(state.canonical_sha256().expect("digest"), AUTHORITY_SHA256);
    }

    #[test]
    fn rejects_every_finite_positive_domain() {
        let cases: &[DomainMutation] = &[
            ("canopy_air_temperature_k", |s| {
                s.canopy_air_temperature_k = 0.0;
            }),
            ("dry_stem_temperature_k", |s| {
                s.dry_stem_temperature_k = -1.0;
            }),
            ("shade_ci_pa", |s| {
                s.shade_ci_pa = f64::NAN;
            }),
            ("shade_leaf_temperature_k", |s| {
                s.shade_leaf_temperature_k = 0.0;
            }),
            ("sun_ci_pa", |s| {
                s.sun_ci_pa = f64::INFINITY;
            }),
            ("sun_leaf_temperature_k", |s| {
                s.sun_leaf_temperature_k = -1.0;
            }),
            ("wet_surface_temperature_k", |s| {
                s.wet_surface_temperature_k = 0.0;
            }),
        ];
        for (field, mutate) in cases {
            let mut state = authority_state();
            mutate(&mut state);
            assert_domain(&state, field);
        }
    }

    #[test]
    fn rejects_every_nonnegative_fraction_and_potential_domain() {
        let cases: &[DomainMutation] = &[
            ("canopy_air_specific_humidity_kg_kg", |s| {
                s.canopy_air_specific_humidity_kg_kg = -0.1;
            }),
            ("canopy_liquid_kg_h2o_m2_tile_ground", |s| {
                s.canopy_liquid_kg_h2o_m2_tile_ground = f64::NAN;
            }),
            ("beta_hyd", |s| {
                s.beta_hyd = 1.01;
            }),
            ("shade_leaf_potential_mm", |s| {
                s.shade_leaf_potential_mm = f64::NAN;
            }),
            ("stem_potential_mm", |s| {
                s.stem_potential_mm = f64::INFINITY;
            }),
            ("sun_leaf_potential_mm", |s| {
                s.sun_leaf_potential_mm = f64::NEG_INFINITY;
            }),
            ("root_potential_mm_by_layer", |s| {
                s.root_potential_mm_by_layer[0].1 = f64::NAN;
            }),
        ];
        for (field, mutate) in cases {
            let mut state = authority_state();
            mutate(&mut state);
            assert_domain(&state, field);
        }
    }

    #[test]
    fn accepts_domain_boundaries_and_any_finite_mm_potential() {
        let mut state = authority_state();
        state.beta_hyd = 0.0;
        state.canopy_air_specific_humidity_kg_kg = 0.0;
        state.canopy_liquid_kg_h2o_m2_tile_ground = 0.0;
        state.stem_potential_mm = 1.0;
        state
            .validate(&configured_layers(), Some(4))
            .expect("lower bounds and finite potential are valid");
        state.beta_hyd = 1.0;
        state
            .validate(&configured_layers(), Some(4))
            .expect("upper beta bound is valid");
    }

    #[test]
    fn rejects_root_layer_order_cardinality_duplicates_and_extras() {
        let expected = configured_layers();
        for found in [
            vec![(layer("soil-2"), -6200.0), (layer("soil-1"), -5100.0)],
            vec![(layer("soil-1"), -5100.0)],
            vec![(layer("soil-1"), -5100.0), (layer("soil-1"), -6200.0)],
            vec![
                (layer("soil-1"), -5100.0),
                (layer("soil-2"), -6200.0),
                (layer("soil-3"), -7000.0),
            ],
        ] {
            let mut state = authority_state();
            state.root_potential_mm_by_layer = found.clone();
            assert_eq!(
                state.validate(&expected, Some(4)),
                Err(OccupancyStateError::RootLayerIdentity {
                    expected: expected.clone(),
                    found: found.into_iter().map(|entry| entry.0).collect(),
                })
            );
        }
    }

    #[test]
    fn transaction_identity_is_exact_for_initial_and_accepted_lanes() {
        let state = authority_state();
        assert_eq!(
            state.validate(&configured_layers(), Some(3)),
            Err(OccupancyStateError::StaleTransaction {
                expected: Some(3),
                found: Some(4),
            })
        );
        assert_eq!(
            state.validate(&configured_layers(), None),
            Err(OccupancyStateError::StaleTransaction {
                expected: None,
                found: Some(4),
            })
        );

        let mut initial = state;
        initial.last_accepted_transaction_id = None;
        initial
            .validate(&configured_layers(), None)
            .expect("initial lane has exact null identity");
        assert!(
            String::from_utf8(initial.canonical_bytes().expect("serialize initial"))
                .expect("JSON is UTF-8")
                .contains("\"last_accepted_transaction_id\":null")
        );
    }

    #[test]
    fn strict_parser_rejects_unknown_or_missing_fields() {
        let with_unknown = [
            &AUTHORITY_BYTES[..AUTHORITY_BYTES.len() - 1],
            br#",\"potential_mpa\":-0.05}"#,
        ]
        .concat();
        assert!(matches!(
            OccupancyState::parse_strict(&with_unknown, &configured_layers(), Some(4)),
            Err(OccupancyStateError::Schema(_))
        ));
        assert!(matches!(
            OccupancyState::parse_strict(b"{}", &configured_layers(), Some(4)),
            Err(OccupancyStateError::Schema(_))
        ));
    }
}
