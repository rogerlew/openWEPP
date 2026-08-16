//! Dependency-neutral V8 carbon receipts and bounded E16--E22 composition.
//!
//! The public V8 boundary consumes accepted solver receipts only. It has no
//! water-arbiter argument and cannot execute or re-enter V7 E01--E15.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{OccupancyId, SoilLayerId, StratumId, TransactionId};

use crate::carbon_nitrogen::update_t10;
use crate::carbon_phase::{ClassCarbonOperands, StratumCarbonOperands, integrate_class_carbon};
use crate::diagnostics::CoupledSolvePass;
use crate::nitrogen_protocol::{
    MineralNitrogenFinalizedUse, MineralNitrogenMaximumAuthorization,
    PotentialMineralNitrogenRequest,
};
use crate::persistent_phase::{
    PersistentForcingInputs, StratumPreallocation, execute_persistent_core,
};
use crate::transaction::NitrogenArbiter;
use crate::v8_state::{V8_MODEL_SHA256, V8CoupledOwnedState};
use crate::{VegetationConfiguration, VegetationError};

/// One exact occupancy carbon receipt from a separately accepted V8 solve.
#[derive(Clone, Debug, PartialEq)]
pub struct V8OccupancyCarbonReceipt {
    pub occupancy_id: OccupancyId,
    pub tile_fraction: f64,
    pub operands: V8OccupancyCarbonOperands,
}

/// Physical carbon operands owned by the accepted V8 solve. T10 is excluded:
/// the persistent owner advances it once from beginning state and forcing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct V8OccupancyCarbonOperands {
    pub sun_leaf_area_m2_m2_tile_ground: f64,
    pub shade_leaf_area_m2_m2_tile_ground: f64,
    pub sun_gross_assimilation_umol_co2_m2_leaf_s: f64,
    pub shade_gross_assimilation_umol_co2_m2_leaf_s: f64,
    pub sun_dark_respiration_umol_co2_m2_leaf_s: f64,
    pub shade_dark_respiration_umol_co2_m2_leaf_s: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct V8StratumCarbonTotals {
    pub gross_primary_production_kg_c_m2: f64,
    pub accepted_leaf_respiration_kg_c_m2: f64,
}

/// A validated complete potential or capped carbon pass.
#[derive(Clone, Debug, PartialEq)]
pub struct ValidatedV8CarbonPass {
    model_definition_sha256: String,
    configuration_sha256: String,
    transaction_id: TransactionId,
    vegetation_beginning_state_sha256: String,
    pass: CoupledSolvePass,
    interval_s: f64,
    occupancies: Vec<V8OccupancyCarbonReceipt>,
    strata: BTreeMap<StratumId, V8StratumCarbonTotals>,
}

impl ValidatedV8CarbonPass {
    #[allow(clippy::too_many_arguments)]
    pub fn try_new(
        model_definition_sha256: String,
        configuration_sha256: String,
        transaction_id: TransactionId,
        vegetation_beginning_state_sha256: String,
        pass: CoupledSolvePass,
        interval_s: f64,
        occupancies: Vec<V8OccupancyCarbonReceipt>,
        configuration: &VegetationConfiguration,
        beginning: &V8CoupledOwnedState,
    ) -> Result<Self, VegetationError> {
        configuration.validate_v8()?;
        beginning.validate(configuration).map_err(|error| {
            VegetationError::Receipt(format!("invalid V8 beginning state: {error}"))
        })?;
        if model_definition_sha256 != V8_MODEL_SHA256
            || configuration_sha256 != configuration.configuration_sha256
            || beginning.model_definition_sha256 != model_definition_sha256
            || beginning.configuration_sha256 != configuration_sha256
            || vegetation_beginning_state_sha256 != beginning.state_sha256
            || transaction_id.0 != beginning.last_transaction_id + 1
            || !interval_s.is_finite()
            || interval_s <= 0.0
            || interval_s.to_bits() != configuration.dt_s.to_bits()
        {
            return Err(VegetationError::Receipt("V8 carbon receipt lineage".into()));
        }
        if occupancies
            .windows(2)
            .any(|pair| pair[0].occupancy_id >= pair[1].occupancy_id)
            || occupancies
                .iter()
                .map(|receipt| receipt.occupancy_id.clone())
                .collect::<BTreeSet<_>>()
                != configuration.expected_occupancies()
        {
            return Err(VegetationError::Receipt(
                "V8 carbon receipt occupancy set".into(),
            ));
        }

        let fractions = configuration
            .topology_tiles
            .iter()
            .map(|tile| (&tile.tile_id, tile.fraction))
            .collect::<BTreeMap<_, _>>();
        let mut strata = BTreeMap::<StratumId, V8StratumCarbonTotals>::new();
        for receipt in &occupancies {
            let expected_fraction =
                fractions
                    .get(&receipt.occupancy_id.tile_id)
                    .ok_or(VegetationError::Receipt(
                        "V8 carbon receipt tile identity".into(),
                    ))?;
            if receipt.tile_fraction.to_bits() != expected_fraction.to_bits() {
                return Err(VegetationError::Receipt(
                    "V8 carbon receipt tile fraction".into(),
                ));
            }
            let (gpp, respiration) = integrate_class_carbon(
                ClassCarbonOperands {
                    sun_leaf_area_m2_m2_tile_ground: receipt
                        .operands
                        .sun_leaf_area_m2_m2_tile_ground,
                    shade_leaf_area_m2_m2_tile_ground: receipt
                        .operands
                        .shade_leaf_area_m2_m2_tile_ground,
                    sun_gross_assimilation_umol_co2_m2_leaf_s: receipt
                        .operands
                        .sun_gross_assimilation_umol_co2_m2_leaf_s,
                    shade_gross_assimilation_umol_co2_m2_leaf_s: receipt
                        .operands
                        .shade_gross_assimilation_umol_co2_m2_leaf_s,
                    sun_dark_respiration_umol_co2_m2_leaf_s: receipt
                        .operands
                        .sun_dark_respiration_umol_co2_m2_leaf_s,
                    shade_dark_respiration_umol_co2_m2_leaf_s: receipt
                        .operands
                        .shade_dark_respiration_umol_co2_m2_leaf_s,
                },
                interval_s,
                receipt.tile_fraction,
            )?;
            let entry = strata
                .entry(receipt.occupancy_id.stratum_id.clone())
                .or_default();
            entry.gross_primary_production_kg_c_m2 += gpp;
            entry.accepted_leaf_respiration_kg_c_m2 += respiration;
        }
        let configured_strata = configuration
            .strata
            .iter()
            .map(|stratum| stratum.stratum_id.clone())
            .collect::<BTreeSet<_>>();
        if strata.keys().cloned().collect::<BTreeSet<_>>() != configured_strata {
            return Err(VegetationError::Receipt(
                "V8 carbon receipt stratum set".into(),
            ));
        }
        Ok(Self {
            model_definition_sha256,
            configuration_sha256,
            transaction_id,
            vegetation_beginning_state_sha256,
            pass,
            interval_s,
            occupancies,
            strata,
        })
    }

    #[must_use]
    pub fn pass(&self) -> CoupledSolvePass {
        self.pass
    }

    #[must_use]
    pub fn occupancies(&self) -> &[V8OccupancyCarbonReceipt] {
        &self.occupancies
    }

    #[must_use]
    pub fn strata(&self) -> &BTreeMap<StratumId, V8StratumCarbonTotals> {
        &self.strata
    }
}

/// Minimal persistent forcing receipt needed after the V8 physical solve.
#[derive(Clone, Debug, PartialEq)]
pub struct V8PersistentForcingReceipt {
    pub model_definition_sha256: String,
    pub configuration_sha256: String,
    pub transaction_id: TransactionId,
    pub vegetation_beginning_state_sha256: String,
    pub air_temperature_k: f64,
    pub gsi: f64,
    pub soil_temperature_k_by_layer: BTreeMap<SoilLayerId, f64>,
}

/// Bounded uncommitted V8 result. It deliberately exposes no ending V8 owner
/// state because the complete E04 final occupancy receipts are not yet part of
/// this boundary.
#[derive(Clone, Debug, PartialEq)]
pub struct UncommittedV8PersistentPhase {
    transaction_id: TransactionId,
    configuration_sha256: String,
    beginning_state_sha256: String,
    potential_carbon: Box<ValidatedV8CarbonPass>,
    capped_carbon: Box<ValidatedV8CarbonPass>,
    requests: Vec<PotentialMineralNitrogenRequest>,
    authorizations: Vec<MineralNitrogenMaximumAuthorization>,
    finalized_uses: Vec<MineralNitrogenFinalizedUse>,
    strata: BTreeMap<StratumId, StratumPreallocation>,
}

impl UncommittedV8PersistentPhase {
    #[must_use]
    pub fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    #[must_use]
    pub fn requests(&self) -> &[PotentialMineralNitrogenRequest] {
        &self.requests
    }

    #[must_use]
    pub fn authorizations(&self) -> &[MineralNitrogenMaximumAuthorization] {
        &self.authorizations
    }

    #[must_use]
    pub fn finalized_uses(&self) -> &[MineralNitrogenFinalizedUse] {
        &self.finalized_uses
    }

    /// Persistent preallocation is intentionally read-only and cannot be
    /// converted into an owner state without the future complete E04 receipt.
    pub(crate) fn strata(&self) -> &BTreeMap<StratumId, StratumPreallocation> {
        &self.strata
    }

    pub(crate) fn matches_sources(
        &self,
        potential: &ValidatedV8CarbonPass,
        capped: &ValidatedV8CarbonPass,
        configuration_sha256: &str,
        beginning_state_sha256: &str,
    ) -> bool {
        self.configuration_sha256 == configuration_sha256
            && self.beginning_state_sha256 == beginning_state_sha256
            && self.potential_carbon.as_ref() == potential
            && self.capped_carbon.as_ref() == capped
    }
}

/// Executes only E16--E22 and the one global mineral-N arbitration from
/// separately sealed V8 potential and capped carbon receipts.
pub fn execute_uncommitted_v8_persistent_phase(
    configuration: &VegetationConfiguration,
    beginning: &V8CoupledOwnedState,
    potential: &ValidatedV8CarbonPass,
    capped: &ValidatedV8CarbonPass,
    forcing: &V8PersistentForcingReceipt,
    nitrogen: &dyn NitrogenArbiter,
) -> Result<UncommittedV8PersistentPhase, VegetationError> {
    configuration.validate_v8()?;
    beginning.validate(configuration).map_err(|error| {
        VegetationError::Receipt(format!("invalid V8 beginning state: {error}"))
    })?;
    if potential.pass != CoupledSolvePass::Potential
        || capped.pass != CoupledSolvePass::Capped
        || potential.model_definition_sha256 != V8_MODEL_SHA256
        || capped.model_definition_sha256 != V8_MODEL_SHA256
        || forcing.model_definition_sha256 != V8_MODEL_SHA256
        || potential.configuration_sha256 != configuration.configuration_sha256
        || capped.configuration_sha256 != configuration.configuration_sha256
        || forcing.configuration_sha256 != configuration.configuration_sha256
        || potential.transaction_id != capped.transaction_id
        || potential.transaction_id != forcing.transaction_id
        || potential.vegetation_beginning_state_sha256 != beginning.state_sha256
        || capped.vegetation_beginning_state_sha256 != beginning.state_sha256
        || forcing.vegetation_beginning_state_sha256 != beginning.state_sha256
        || potential.interval_s.to_bits() != capped.interval_s.to_bits()
        || forcing.transaction_id.0 != beginning.last_transaction_id + 1
    {
        return Err(VegetationError::Receipt(
            "incomplete or mismatched V8 persistent receipt".into(),
        ));
    }
    let configured_layers = configuration
        .strata
        .iter()
        .flat_map(|stratum| &stratum.root_layers)
        .map(|root| root.layer_id.clone())
        .collect::<BTreeSet<_>>();
    if forcing
        .soil_temperature_k_by_layer
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>()
        != configured_layers
    {
        return Err(VegetationError::Receipt(
            "V8 persistent soil-temperature layer set".into(),
        ));
    }
    let bind_t10 = |values: &BTreeMap<StratumId, V8StratumCarbonTotals>| {
        values
            .iter()
            .map(|(stratum_id, value)| {
                let beginning_t10 = beginning
                    .strata
                    .get(stratum_id)
                    .ok_or(VegetationError::Receipt(
                        "V8 persistent T10 stratum identity".into(),
                    ))?
                    .t10_k;
                Ok((
                    stratum_id.clone(),
                    StratumCarbonOperands {
                        gross_primary_production_kg_c_m2: value.gross_primary_production_kg_c_m2,
                        accepted_leaf_respiration_kg_c_m2: value.accepted_leaf_respiration_kg_c_m2,
                        advanced_t10_k: update_t10(
                            beginning_t10,
                            forcing.air_temperature_k,
                            potential.interval_s,
                        )?,
                    },
                ))
            })
            .collect::<Result<BTreeMap<_, _>, VegetationError>>()
    };
    let potential_strata = bind_t10(&potential.strata)?;
    let capped_strata = bind_t10(&capped.strata)?;
    let core = execute_persistent_core(
        configuration,
        &beginning.strata,
        forcing.transaction_id,
        &PersistentForcingInputs {
            air_temperature_k: forcing.air_temperature_k,
            gsi: forcing.gsi,
            soil_temperature_k_by_layer: forcing.soil_temperature_k_by_layer.clone(),
        },
        &potential_strata,
        &capped_strata,
        nitrogen,
    )?;
    Ok(UncommittedV8PersistentPhase {
        transaction_id: core.transaction_id,
        configuration_sha256: configuration.configuration_sha256.clone(),
        beginning_state_sha256: beginning.state_sha256.clone(),
        potential_carbon: Box::new(potential.clone()),
        capped_carbon: Box::new(capped.clone()),
        requests: core.requests,
        authorizations: core.authorizations,
        finalized_uses: core.finalized_uses,
        strata: core.strata,
    })
}

#[cfg(test)]
mod tests {
    use std::cell::{Cell, RefCell};

    use openwepp_kernel_contract::{MineralNitrogenKey, ResourceRequest};

    use super::*;
    use crate::transaction::{NitrogenAuthorization, NitrogenRequest};

    struct CountingNitrogen {
        calls: Cell<u32>,
        requests: RefCell<Vec<NitrogenRequest>>,
    }

    impl NitrogenArbiter for CountingNitrogen {
        fn beginning_amount(&self, _: &MineralNitrogenKey) -> Result<f64, VegetationError> {
            Ok(1.0)
        }

        fn authorize(
            &self,
            requests: &[NitrogenRequest],
        ) -> Result<Vec<NitrogenAuthorization>, VegetationError> {
            self.calls.set(self.calls.get() + 1);
            self.requests.replace(requests.to_vec());
            Ok(requests
                .iter()
                .map(|request| NitrogenAuthorization {
                    transaction_id: request.transaction_id,
                    owner_id: request.owner_id.clone(),
                    key: request.key.clone(),
                    amount: request.amount,
                    basis: request.basis,
                })
                .collect())
        }
    }

    fn operands(gross: f64) -> V8OccupancyCarbonOperands {
        V8OccupancyCarbonOperands {
            sun_leaf_area_m2_m2_tile_ground: 1.0,
            shade_leaf_area_m2_m2_tile_ground: 1.0,
            sun_gross_assimilation_umol_co2_m2_leaf_s: gross,
            shade_gross_assimilation_umol_co2_m2_leaf_s: gross / 2.0,
            sun_dark_respiration_umol_co2_m2_leaf_s: 0.1,
            shade_dark_respiration_umol_co2_m2_leaf_s: 0.1,
        }
    }

    fn carbon_pass(
        configuration: &VegetationConfiguration,
        beginning: &V8CoupledOwnedState,
        pass: CoupledSolvePass,
        gross: f64,
    ) -> ValidatedV8CarbonPass {
        let fractions = configuration
            .topology_tiles
            .iter()
            .map(|tile| (&tile.tile_id, tile.fraction))
            .collect::<BTreeMap<_, _>>();
        let receipts = configuration
            .expected_occupancies()
            .into_iter()
            .map(|occupancy_id| V8OccupancyCarbonReceipt {
                tile_fraction: *fractions.get(&occupancy_id.tile_id).expect("tile fraction"),
                occupancy_id,
                operands: operands(gross),
            })
            .collect();
        ValidatedV8CarbonPass::try_new(
            V8_MODEL_SHA256.into(),
            configuration.configuration_sha256.clone(),
            TransactionId(beginning.last_transaction_id + 1),
            beginning.state_sha256.clone(),
            pass,
            configuration.dt_s,
            receipts,
            configuration,
            beginning,
        )
        .expect("validated carbon pass")
    }

    fn forcing(
        configuration: &VegetationConfiguration,
        beginning: &V8CoupledOwnedState,
    ) -> V8PersistentForcingReceipt {
        V8PersistentForcingReceipt {
            model_definition_sha256: V8_MODEL_SHA256.into(),
            configuration_sha256: configuration.configuration_sha256.clone(),
            transaction_id: TransactionId(beginning.last_transaction_id + 1),
            vegetation_beginning_state_sha256: beginning.state_sha256.clone(),
            air_temperature_k: 295.0,
            gsi: 1.0,
            soil_temperature_k_by_layer: configuration
                .strata
                .iter()
                .flat_map(|stratum| &stratum.root_layers)
                .map(|root| (root.layer_id.clone(), 293.0))
                .collect(),
        }
    }

    #[test]
    fn sealed_v8_receipts_run_one_global_n_call_without_water_entry() {
        let (configuration, beginning) = crate::v8_state::v8_test_fixture();
        let potential = carbon_pass(
            &configuration,
            &beginning,
            CoupledSolvePass::Potential,
            12.0,
        );
        let capped = carbon_pass(&configuration, &beginning, CoupledSolvePass::Capped, 9.0);
        let nitrogen = CountingNitrogen {
            calls: Cell::new(0),
            requests: RefCell::new(Vec::new()),
        };
        let phase = execute_uncommitted_v8_persistent_phase(
            &configuration,
            &beginning,
            &potential,
            &capped,
            &forcing(&configuration, &beginning),
            &nitrogen,
        )
        .expect("bounded persistent phase");
        assert_eq!(nitrogen.calls.get(), 1);
        assert_eq!(nitrogen.requests.borrow().as_slice(), phase.requests());
        assert_eq!(phase.transaction_id(), TransactionId(1));
        assert_eq!(phase.strata().len(), configuration.strata.len());
        assert!(!phase.authorizations().is_empty());
        assert!(!phase.finalized_uses().is_empty());
        let beginning_t10 = beginning.strata.values().next().expect("stratum").t10_k;
        let expected_t10 =
            update_t10(beginning_t10, 295.0, configuration.dt_s).expect("canonical T10");
        assert_eq!(
            phase
                .strata()
                .values()
                .next()
                .expect("stratum")
                .candidate_after_growth
                .t10_k
                .to_bits(),
            expected_t10.to_bits()
        );
    }

    #[test]
    fn potential_and_capped_receipts_cannot_be_swapped() {
        let (configuration, beginning) = crate::v8_state::v8_test_fixture();
        let potential = carbon_pass(
            &configuration,
            &beginning,
            CoupledSolvePass::Potential,
            12.0,
        );
        let capped = carbon_pass(&configuration, &beginning, CoupledSolvePass::Capped, 9.0);
        let nitrogen = CountingNitrogen {
            calls: Cell::new(0),
            requests: RefCell::new(Vec::new()),
        };
        assert!(matches!(
            execute_uncommitted_v8_persistent_phase(
                &configuration,
                &beginning,
                &capped,
                &potential,
                &forcing(&configuration, &beginning),
                &nitrogen,
            ),
            Err(VegetationError::Receipt(_))
        ));
        assert_eq!(nitrogen.calls.get(), 0);
    }

    #[test]
    fn incomplete_occupancy_receipt_fails_before_n_authorization() {
        let (configuration, beginning) = crate::v8_state::v8_test_fixture();
        let occupancy_id = configuration
            .expected_occupancies()
            .into_iter()
            .next()
            .expect("occupancy");
        let result = ValidatedV8CarbonPass::try_new(
            V8_MODEL_SHA256.into(),
            configuration.configuration_sha256.clone(),
            TransactionId(1),
            beginning.state_sha256.clone(),
            CoupledSolvePass::Potential,
            configuration.dt_s,
            vec![V8OccupancyCarbonReceipt {
                occupancy_id: OccupancyId {
                    stratum_id: occupancy_id.stratum_id,
                    tile_id: openwepp_kernel_contract::TileId::try_new("missing-tile")
                        .expect("tile"),
                },
                tile_fraction: 1.0,
                operands: operands(12.0),
            }],
            &configuration,
            &beginning,
        );
        assert!(matches!(result, Err(VegetationError::Receipt(_))));
    }

    #[test]
    fn potential_requests_and_capped_growth_remain_distinct() {
        let (configuration, beginning) = crate::v8_state::v8_test_fixture();
        let potential = carbon_pass(
            &configuration,
            &beginning,
            CoupledSolvePass::Potential,
            12.0,
        );
        let capped = carbon_pass(&configuration, &beginning, CoupledSolvePass::Capped, 6.0);
        let nitrogen = CountingNitrogen {
            calls: Cell::new(0),
            requests: RefCell::new(Vec::new()),
        };
        let phase = execute_uncommitted_v8_persistent_phase(
            &configuration,
            &beginning,
            &potential,
            &capped,
            &forcing(&configuration, &beginning),
            &nitrogen,
        )
        .expect("phase");
        let preallocation = phase.strata().values().next().expect("stratum");
        assert!(
            preallocation.potential_carbon_offer.offer > preallocation.final_carbon_offer.offer
        );
        let requested: f64 = phase.requests().iter().map(|request| request.amount).sum();
        assert!(requested >= preallocation.nitrogen_finalization.external_use);
    }

    #[test]
    fn v8_entrypoint_has_no_water_arbiter_or_physical_solver_input() {
        fn accepts_protocol(_: &[ResourceRequest<MineralNitrogenKey, f64>]) {}

        type V8PersistentEntry = fn(
            &VegetationConfiguration,
            &V8CoupledOwnedState,
            &ValidatedV8CarbonPass,
            &ValidatedV8CarbonPass,
            &V8PersistentForcingReceipt,
            &dyn NitrogenArbiter,
        )
            -> Result<UncommittedV8PersistentPhase, VegetationError>;
        let entry: V8PersistentEntry = execute_uncommitted_v8_persistent_phase;
        let _ = entry;

        let empty = Vec::<PotentialMineralNitrogenRequest>::new();
        accepts_protocol(&empty);
    }

    #[test]
    fn carbon_receipt_has_no_caller_owned_t10_lane() {
        let receipt = operands(12.0);
        let V8OccupancyCarbonOperands {
            sun_leaf_area_m2_m2_tile_ground,
            shade_leaf_area_m2_m2_tile_ground,
            sun_gross_assimilation_umol_co2_m2_leaf_s,
            shade_gross_assimilation_umol_co2_m2_leaf_s,
            sun_dark_respiration_umol_co2_m2_leaf_s,
            shade_dark_respiration_umol_co2_m2_leaf_s,
        } = receipt;
        assert!(
            [
                sun_leaf_area_m2_m2_tile_ground,
                shade_leaf_area_m2_m2_tile_ground,
                sun_gross_assimilation_umol_co2_m2_leaf_s,
                shade_gross_assimilation_umol_co2_m2_leaf_s,
                sun_dark_respiration_umol_co2_m2_leaf_s,
                shade_dark_respiration_umol_co2_m2_leaf_s,
            ]
            .iter()
            .all(|value| value.is_finite())
        );
    }
}
