//! Independent component-level energy owner for the default-off V7 diagnostic.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{OccupancyId, TileId, TransactionId};
use openwepp_vegetation::column::OccupancyEnergyProposal;
use openwepp_vegetation::energy::{LATENT_HEAT_VAPORIZATION, STEFAN_BOLTZMANN};
use openwepp_vegetation::energy_proposal::{EnergyProposalBatch, RadiationBoundaryProposal};
use openwepp_vegetation::ledger::{EnergyLedgerOperands, LedgerIdentity};
use openwepp_vegetation::radiation::{IncidentComponent, RadiationBand};
use serde::{Deserialize, Serialize};

const CP_AIR_J_KG_K: f64 = 1_004.64;
const R_DRY_AIR_J_KG_K: f64 = 287.05;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CanopyHeatStorageMode {
    EquilibriumZero,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DiagnosticEnergyState {
    pub model_definition_sha256: String,
    pub configuration_sha256: String,
    pub accepted_vegetation_state_sha256: String,
    pub last_transaction_id: u128,
    pub last_operands: Option<EnergyLedgerOperands>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnergyOwnerTopology {
    model_definition_sha256: String,
    configuration_sha256: String,
    tile_fractions: BTreeMap<TileId, f64>,
    occupancies: BTreeSet<OccupancyId>,
}

impl EnergyOwnerTopology {
    pub fn from_configuration(
        configuration: &openwepp_vegetation::VegetationConfiguration,
    ) -> Result<Self, EnergyOwnerError> {
        configuration
            .validate()
            .map_err(|_| EnergyOwnerError::Identity("configuration"))?;
        let tile_fractions = configuration
            .topology_tiles
            .iter()
            .map(|tile| (tile.tile_id.clone(), tile.fraction))
            .collect::<BTreeMap<_, _>>();
        Ok(Self {
            model_definition_sha256: configuration.model_definition_sha256.clone(),
            configuration_sha256: configuration.configuration_sha256.clone(),
            tile_fractions,
            occupancies: configuration.expected_occupancies(),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OccupancyEnergyReceipt {
    pub occupancy_id: OccupancyId,
    pub transaction_id: TransactionId,
    pub surface_residuals_w_m2_tile: [f64; 4],
    pub canopy_air_sensible_residual_w_m2_tile: f64,
    pub canopy_air_vapor_residual_kg_m2_tile: f64,
    pub transpiration_withdrawal_residual_kg_m2_tile: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DiagnosticEnergyOwnerCandidate {
    transaction_id: TransactionId,
    proposal_identity: openwepp_vegetation::energy_proposal::EnergyProposalIdentity,
    beginning: DiagnosticEnergyState,
    ending: DiagnosticEnergyState,
    occupancy_receipts: Vec<OccupancyEnergyReceipt>,
    stand_ledger: EnergyLedgerOperands,
    heat_storage_mode: CanopyHeatStorageMode,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EnergyOwnerError {
    Weighting(&'static str),
    Identity(&'static str),
    Operand(&'static str),
    Closure {
        component: &'static str,
        residual: f64,
    },
}

impl std::fmt::Display for EnergyOwnerError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Weighting(message) => write!(
                formatter,
                "VEGTXN-E-004: invalid energy tile weighting: {message}"
            ),
            Self::Identity(message) => write!(
                formatter,
                "VEGTXN-E-006: invalid independent energy owner identity: {message}"
            ),
            Self::Operand(message) => write!(
                formatter,
                "VEGTXN-E-002: invalid component-level energy operand: {message}"
            ),
            Self::Closure {
                component,
                residual,
            } => write!(
                formatter,
                "VEGTXN-E-005: independent {component} energy residual {residual} exceeds tolerance"
            ),
        }
    }
}

impl std::error::Error for EnergyOwnerError {}

impl DiagnosticEnergyOwnerCandidate {
    #[must_use]
    pub fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    #[must_use]
    pub fn beginning(&self) -> &DiagnosticEnergyState {
        &self.beginning
    }

    #[must_use]
    pub fn proposal_identity(
        &self,
    ) -> &openwepp_vegetation::energy_proposal::EnergyProposalIdentity {
        &self.proposal_identity
    }

    #[must_use]
    pub fn ending(&self) -> &DiagnosticEnergyState {
        &self.ending
    }

    #[must_use]
    pub fn occupancy_receipts(&self) -> &[OccupancyEnergyReceipt] {
        &self.occupancy_receipts
    }

    #[must_use]
    pub fn stand_ledger(&self) -> &EnergyLedgerOperands {
        &self.stand_ledger
    }

    #[must_use]
    pub fn heat_storage_mode(&self) -> CanopyHeatStorageMode {
        self.heat_storage_mode
    }
}

pub fn construct_energy_owner_candidate(
    beginning: &DiagnosticEnergyState,
    topology: &EnergyOwnerTopology,
    proposals: &EnergyProposalBatch,
    heat_storage_mode: CanopyHeatStorageMode,
) -> Result<DiagnosticEnergyOwnerCandidate, EnergyOwnerError> {
    validate_batch_identity(beginning, topology, proposals, heat_storage_mode)?;
    let mut occupancy_receipts = Vec::with_capacity(proposals.occupancies.len());
    let mut by_tile = BTreeMap::<_, Vec<&OccupancyEnergyProposal>>::new();
    let mut energy = StandEnergyAccumulator::default();
    for proposal in &proposals.occupancies {
        let reconstructed = reconstruct_occupancy(proposal)?;
        energy.add_occupancy(proposal, &reconstructed);
        by_tile
            .entry(proposal.occupancy_id.tile_id.clone())
            .or_default()
            .push(proposal);
        occupancy_receipts.push(reconstructed.receipt);
    }
    for boundary in &proposals.tile_boundaries {
        let occupancies = by_tile.remove(&boundary.tile_id).unwrap_or_default();
        validate_radiation_boundary(boundary.tile_fraction, &boundary.components, &occupancies)?;
        energy.add_boundary(boundary.tile_fraction, &boundary.components);
    }
    if !by_tile.is_empty() {
        return Err(EnergyOwnerError::Identity("missing radiation tile"));
    }
    let stand_ledger = energy.finish(proposals)?;
    require_stand_energy_closed(
        "weighted stand",
        stand_energy_residual(&stand_ledger),
        stand_energy_scale(&stand_ledger),
        proposals.identity.interval_s,
    )?;
    let ending = DiagnosticEnergyState {
        model_definition_sha256: proposals.identity.model_definition_sha256.clone(),
        configuration_sha256: proposals.identity.configuration_sha256.clone(),
        accepted_vegetation_state_sha256: proposals.identity.ending_state_sha256.clone(),
        last_transaction_id: proposals.identity.transaction_id.0,
        last_operands: Some(stand_ledger.clone()),
    };
    Ok(DiagnosticEnergyOwnerCandidate {
        transaction_id: proposals.identity.transaction_id,
        proposal_identity: proposals.identity.clone(),
        beginning: beginning.clone(),
        ending,
        occupancy_receipts,
        stand_ledger,
        heat_storage_mode,
    })
}

fn validate_batch_identity(
    beginning: &DiagnosticEnergyState,
    topology: &EnergyOwnerTopology,
    proposals: &EnergyProposalBatch,
    heat_storage_mode: CanopyHeatStorageMode,
) -> Result<(), EnergyOwnerError> {
    let expected_transaction = beginning
        .last_transaction_id
        .checked_add(1)
        .ok_or(EnergyOwnerError::Identity("transaction overflow"))?;
    validate_beginning_state(beginning)?;
    if heat_storage_mode != CanopyHeatStorageMode::EquilibriumZero
        || proposals.identity.model_definition_sha256 != openwepp_vegetation::MODEL_SHA256
        || proposals.identity.model_definition_sha256 != topology.model_definition_sha256
        || proposals.identity.configuration_sha256 != topology.configuration_sha256
        || proposals.identity.model_definition_sha256 != beginning.model_definition_sha256
        || proposals.identity.configuration_sha256 != beginning.configuration_sha256
        || proposals.identity.beginning_state_sha256 != beginning.accepted_vegetation_state_sha256
        || !is_sha256(&proposals.identity.configuration_sha256)
        || !is_sha256(&proposals.identity.beginning_state_sha256)
        || !is_sha256(&proposals.identity.ending_state_sha256)
        || proposals.identity.transaction_id.0 != expected_transaction
        || !proposals.identity.interval_s.is_finite()
        || proposals.identity.interval_s <= 0.0
    {
        return Err(EnergyOwnerError::Identity("batch lineage"));
    }
    let mut occupancies = BTreeSet::new();
    if proposals.tile_boundaries.is_empty()
        || proposals.occupancies.iter().any(|proposal| {
            proposal.transaction_id != proposals.identity.transaction_id
                || proposal.interval_s.to_bits() != proposals.identity.interval_s.to_bits()
                || !occupancies.insert(proposal.occupancy_id.clone())
        })
        || occupancies != topology.occupancies
    {
        return Err(EnergyOwnerError::Identity("occupancy set"));
    }
    let actual_tiles = proposals
        .tile_boundaries
        .iter()
        .map(|boundary| (boundary.tile_id.clone(), boundary.tile_fraction))
        .collect::<BTreeMap<_, _>>();
    if actual_tiles.len() != proposals.tile_boundaries.len()
        || actual_tiles.len() != topology.tile_fractions.len()
        || actual_tiles.iter().any(|(tile_id, fraction)| {
            topology
                .tile_fractions
                .get(tile_id)
                .is_none_or(|expected| expected.to_bits() != fraction.to_bits())
        })
    {
        return Err(EnergyOwnerError::Weighting("topology tile fractions"));
    }
    Ok(())
}

fn validate_beginning_state(beginning: &DiagnosticEnergyState) -> Result<(), EnergyOwnerError> {
    match (beginning.last_transaction_id, &beginning.last_operands) {
        (0, None) => Ok(()),
        (0, Some(_)) | (_, None) => Err(EnergyOwnerError::Identity("energy state lineage")),
        (transaction_id, Some(ledger)) => {
            let values = [
                ledger.identity.area_m2,
                ledger.identity.interval_s,
                ledger.incident_shortwave_j_m2,
                ledger.incident_longwave_j_m2,
                ledger.reflected_shortwave_j_m2,
                ledger.terminal_shortwave_j_m2,
                ledger.emitted_longwave_j_m2,
                ledger.sensible_j_m2,
                ledger.latent_j_m2,
                ledger.ground_or_storage_j_m2,
            ];
            if ledger.identity.transaction_id.0 != transaction_id
                || ledger.identity.owner_id != "diagnostic-energy"
                || ledger.identity.area_m2.to_bits() != 1.0_f64.to_bits()
                || ledger.identity.interval_s <= 0.0
                || values.iter().any(|value| !value.is_finite())
            {
                return Err(EnergyOwnerError::Identity("energy state operands"));
            }
            require_stand_energy_closed(
                "beginning-state stand",
                stand_energy_residual(ledger),
                stand_energy_scale(ledger),
                ledger.identity.interval_s,
            )
        }
    }
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

struct ReconstructedOccupancy {
    receipt: OccupancyEnergyReceipt,
    incident_longwave_w_m2_tile: f64,
    emitted_longwave_w_m2_tile: f64,
    atmosphere_sensible_w_m2_tile: f64,
    latent_w_m2_tile: f64,
}

#[allow(clippy::too_many_lines)]
fn reconstruct_occupancy(
    proposal: &OccupancyEnergyProposal,
) -> Result<ReconstructedOccupancy, EnergyOwnerError> {
    validate_constants_and_domain(proposal)?;
    let rho = proposal.pressure_pa / (proposal.rdry_j_kg_k * proposal.canopy_air_temperature_k);
    let wet_area = proposal.wet_leaf_area_m2_m2_tile + proposal.wet_stem_area_m2_m2_tile;
    let incoming_lw = proposal.longwave_down_w_m2 + proposal.longwave_up_w_m2;
    let surface = |shortwave: f64,
                   emissivity: f64,
                   area: f64,
                   temperature: f64,
                   conductance: f64,
                   water_kg_m2: f64| {
        let incident_lw = emissivity * area * incoming_lw;
        let emitted_lw = 2.0 * emissivity * area * STEFAN_BOLTZMANN * temperature.powi(4);
        let sensible = rho
            * proposal.cp_air_j_kg_k
            * conductance
            * area
            * (temperature - proposal.canopy_air_temperature_k);
        let latent = proposal.latent_heat_j_kg * water_kg_m2 / proposal.interval_s;
        (
            shortwave + incident_lw - emitted_lw - sensible - latent,
            incident_lw,
            emitted_lw,
            sensible,
            latent,
        )
    };
    let sun = surface(
        proposal.sun_leaf_absorbed_shortwave_w_m2_tile,
        proposal.leaf_emissivity,
        proposal.dry_sun_leaf_area_m2_m2_tile,
        proposal.sun_leaf_temperature_k,
        proposal.gb_leaf_m_s,
        proposal.sun_transpiration_kg_m2_tile,
    );
    let shade = surface(
        proposal.shade_leaf_absorbed_shortwave_w_m2_tile,
        proposal.leaf_emissivity,
        proposal.dry_shade_leaf_area_m2_m2_tile,
        proposal.shade_leaf_temperature_k,
        proposal.gb_leaf_m_s,
        proposal.shade_transpiration_kg_m2_tile,
    );
    let wet = surface(
        proposal.wet_surface_absorbed_shortwave_w_m2_tile,
        proposal.wet_emissivity,
        wet_area,
        proposal.wet_surface_temperature_k,
        proposal.gb_wet_m_s,
        proposal.wet_phase_change_kg_m2_tile,
    );
    let stem = surface(
        proposal.dry_stem_absorbed_shortwave_w_m2_tile,
        proposal.stem_emissivity,
        proposal.dry_stem_area_m2_m2_tile,
        proposal.dry_stem_temperature_k,
        proposal.gb_stem_m_s,
        0.0,
    );
    for (name, value, scale) in [
        (
            "sun leaf",
            sun.0,
            proposal.sun_leaf_absorbed_shortwave_w_m2_tile.abs() + component_scale(&sun),
        ),
        (
            "shade leaf",
            shade.0,
            proposal.shade_leaf_absorbed_shortwave_w_m2_tile.abs() + component_scale(&shade),
        ),
        (
            "wet surface",
            wet.0,
            proposal.wet_surface_absorbed_shortwave_w_m2_tile.abs() + component_scale(&wet),
        ),
        (
            "dry stem",
            stem.0,
            proposal.dry_stem_absorbed_shortwave_w_m2_tile.abs() + component_scale(&stem),
        ),
    ] {
        require_energy_closed(name, value, scale)?;
    }
    let atmosphere_sensible = rho
        * proposal.cp_air_j_kg_k
        * (proposal.canopy_air_temperature_k - proposal.air_temperature_k)
        / proposal.rah_s_m;
    let canopy_sensible = atmosphere_sensible - sun.3 - shade.3 - wet.3 - stem.3;
    require_energy_closed(
        "canopy air sensible",
        canopy_sensible,
        atmosphere_sensible.abs() + sun.3.abs() + shade.3.abs() + wet.3.abs() + stem.3.abs(),
    )?;
    let atmosphere_vapor = rho
        * (proposal.canopy_air_specific_humidity_kg_kg - proposal.air_specific_humidity_kg_kg)
        / proposal.raw_s_m
        * proposal.interval_s;
    let transpiration =
        proposal.sun_transpiration_kg_m2_tile + proposal.shade_transpiration_kg_m2_tile;
    let vapor_residual = atmosphere_vapor - transpiration - proposal.wet_phase_change_kg_m2_tile;
    require_amount_closed(
        "canopy air vapor",
        vapor_residual,
        atmosphere_vapor
            .abs()
            .max(proposal.sun_transpiration_kg_m2_tile.abs())
            .max(proposal.shade_transpiration_kg_m2_tile.abs())
            .max(proposal.wet_phase_change_kg_m2_tile.abs()),
        proposal.interval_s,
    )?;
    let withdrawal = proposal
        .finalized_layer_withdrawal_kg_m2_tile
        .iter()
        .map(|(_, amount)| *amount)
        .sum::<f64>();
    let withdrawal_residual = transpiration - withdrawal;
    require_amount_closed(
        "transpiration withdrawal",
        withdrawal_residual,
        transpiration.abs().max(withdrawal.abs()),
        proposal.interval_s,
    )?;
    Ok(ReconstructedOccupancy {
        receipt: OccupancyEnergyReceipt {
            occupancy_id: proposal.occupancy_id.clone(),
            transaction_id: proposal.transaction_id,
            surface_residuals_w_m2_tile: [sun.0, shade.0, wet.0, stem.0],
            canopy_air_sensible_residual_w_m2_tile: canopy_sensible,
            canopy_air_vapor_residual_kg_m2_tile: vapor_residual,
            transpiration_withdrawal_residual_kg_m2_tile: withdrawal_residual,
        },
        incident_longwave_w_m2_tile: sun.1 + shade.1 + wet.1 + stem.1,
        emitted_longwave_w_m2_tile: sun.2 + shade.2 + wet.2 + stem.2,
        atmosphere_sensible_w_m2_tile: atmosphere_sensible,
        latent_w_m2_tile: sun.4 + shade.4 + wet.4,
    })
}

fn validate_constants_and_domain(
    proposal: &OccupancyEnergyProposal,
) -> Result<(), EnergyOwnerError> {
    validate_scalar_domain(proposal)?;
    validate_inactive_surfaces(proposal)?;
    validate_layer_and_spectral_shape(proposal)?;
    validate_shortwave_ownership(proposal)
}

fn validate_scalar_domain(proposal: &OccupancyEnergyProposal) -> Result<(), EnergyOwnerError> {
    let positive = [
        proposal.tile_fraction,
        proposal.interval_s,
        proposal.pressure_pa,
        proposal.canopy_air_temperature_k,
        proposal.air_temperature_k,
        proposal.sun_leaf_temperature_k,
        proposal.shade_leaf_temperature_k,
        proposal.wet_surface_temperature_k,
        proposal.dry_stem_temperature_k,
        proposal.gb_leaf_m_s,
        proposal.gb_wet_m_s,
        proposal.gb_stem_m_s,
        proposal.rah_s_m,
        proposal.raw_s_m,
    ];
    let nonnegative = [
        proposal.sun_leaf_absorbed_shortwave_w_m2_tile,
        proposal.shade_leaf_absorbed_shortwave_w_m2_tile,
        proposal.wet_surface_absorbed_shortwave_w_m2_tile,
        proposal.dry_stem_absorbed_shortwave_w_m2_tile,
        proposal.dry_sun_leaf_area_m2_m2_tile,
        proposal.dry_shade_leaf_area_m2_m2_tile,
        proposal.wet_leaf_area_m2_m2_tile,
        proposal.wet_stem_area_m2_m2_tile,
        proposal.dry_stem_area_m2_m2_tile,
        proposal.canopy_air_specific_humidity_kg_kg,
        proposal.air_specific_humidity_kg_kg,
        proposal.longwave_down_w_m2,
        proposal.longwave_up_w_m2,
        proposal.sun_transpiration_kg_m2_tile,
        proposal.shade_transpiration_kg_m2_tile,
    ];
    if positive
        .iter()
        .any(|value| !value.is_finite() || *value <= 0.0)
        || nonnegative
            .iter()
            .any(|value| !value.is_finite() || *value < 0.0)
        || !proposal.wet_phase_change_kg_m2_tile.is_finite()
        || proposal.tile_fraction > 1.0
        || proposal.canopy_air_specific_humidity_kg_kg > 1.0
        || proposal.air_specific_humidity_kg_kg > 1.0
        || [
            proposal.leaf_emissivity,
            proposal.wet_emissivity,
            proposal.stem_emissivity,
        ]
        .iter()
        .any(|value| !value.is_finite() || *value < 0.0 || *value > 1.0)
        || proposal.cp_air_j_kg_k.to_bits() != CP_AIR_J_KG_K.to_bits()
        || proposal.rdry_j_kg_k.to_bits() != R_DRY_AIR_J_KG_K.to_bits()
        || proposal.latent_heat_j_kg.to_bits() != LATENT_HEAT_VAPORIZATION.to_bits()
    {
        return Err(EnergyOwnerError::Operand("domain or authority constant"));
    }
    Ok(())
}

fn validate_inactive_surfaces(proposal: &OccupancyEnergyProposal) -> Result<(), EnergyOwnerError> {
    for (area, temperature) in [
        (
            proposal.dry_sun_leaf_area_m2_m2_tile,
            proposal.sun_leaf_temperature_k,
        ),
        (
            proposal.dry_shade_leaf_area_m2_m2_tile,
            proposal.shade_leaf_temperature_k,
        ),
        (
            proposal.wet_leaf_area_m2_m2_tile + proposal.wet_stem_area_m2_m2_tile,
            proposal.wet_surface_temperature_k,
        ),
        (
            proposal.dry_stem_area_m2_m2_tile,
            proposal.dry_stem_temperature_k,
        ),
    ] {
        if area.to_bits() == 0.0_f64.to_bits()
            && temperature.to_bits() != proposal.canopy_air_temperature_k.to_bits()
        {
            return Err(EnergyOwnerError::Operand("inactive surface temperature"));
        }
    }
    Ok(())
}

fn validate_layer_and_spectral_shape(
    proposal: &OccupancyEnergyProposal,
) -> Result<(), EnergyOwnerError> {
    let mut layers = BTreeSet::new();
    if proposal.finalized_layer_withdrawal_kg_m2_tile.is_empty()
        || proposal
            .finalized_layer_withdrawal_kg_m2_tile
            .iter()
            .any(|(layer, amount)| {
                !amount.is_finite() || *amount < 0.0 || !layers.insert(layer.clone())
            })
        || proposal.spectral_absorption.iter().any(|component| {
            [
                component.sun_leaf_w_m2_tile,
                component.shade_leaf_w_m2_tile,
                component.wet_surface_w_m2_tile,
                component.dry_stem_w_m2_tile,
            ]
            .iter()
            .any(|value| !value.is_finite() || *value < 0.0)
        })
    {
        return Err(EnergyOwnerError::Operand("component or layer operand"));
    }
    let spectral_keys = proposal
        .spectral_absorption
        .iter()
        .map(|component| (component.band, component.component))
        .collect::<BTreeSet<_>>();
    let expected_keys = BTreeSet::from([
        (RadiationBand::Visible, IncidentComponent::Direct),
        (RadiationBand::Visible, IncidentComponent::Diffuse),
        (RadiationBand::NearInfrared, IncidentComponent::Direct),
        (RadiationBand::NearInfrared, IncidentComponent::Diffuse),
    ]);
    if spectral_keys != expected_keys {
        return Err(EnergyOwnerError::Identity("spectral absorption identity"));
    }
    Ok(())
}

fn validate_shortwave_ownership(
    proposal: &OccupancyEnergyProposal,
) -> Result<(), EnergyOwnerError> {
    for (name, scalar, spectral) in [
        (
            "sun shortwave ownership",
            proposal.sun_leaf_absorbed_shortwave_w_m2_tile,
            proposal
                .spectral_absorption
                .iter()
                .map(|component| component.sun_leaf_w_m2_tile)
                .sum::<f64>(),
        ),
        (
            "shade shortwave ownership",
            proposal.shade_leaf_absorbed_shortwave_w_m2_tile,
            proposal
                .spectral_absorption
                .iter()
                .map(|component| component.shade_leaf_w_m2_tile)
                .sum::<f64>(),
        ),
        (
            "wet shortwave ownership",
            proposal.wet_surface_absorbed_shortwave_w_m2_tile,
            proposal
                .spectral_absorption
                .iter()
                .map(|component| component.wet_surface_w_m2_tile)
                .sum::<f64>(),
        ),
        (
            "stem shortwave ownership",
            proposal.dry_stem_absorbed_shortwave_w_m2_tile,
            proposal
                .spectral_absorption
                .iter()
                .map(|component| component.dry_stem_w_m2_tile)
                .sum::<f64>(),
        ),
    ] {
        let residual = scalar - spectral;
        if !residual.is_finite()
            || residual.abs() > 2.0e-12 * (scalar.abs() + spectral.abs()).max(1.0)
        {
            return Err(EnergyOwnerError::Closure {
                component: name,
                residual,
            });
        }
    }
    Ok(())
}

fn validate_radiation_boundary(
    tile_fraction: f64,
    boundaries: &[RadiationBoundaryProposal],
    occupancies: &[&OccupancyEnergyProposal],
) -> Result<(), EnergyOwnerError> {
    if !tile_fraction.is_finite()
        || tile_fraction <= 0.0
        || tile_fraction > 1.0
        || boundaries.len() != 4
        || boundaries.iter().any(|boundary| {
            [
                boundary.incident_w_m2_tile,
                boundary.reflected_w_m2_tile,
                boundary.terminal_direct_w_m2_tile,
                boundary.terminal_diffuse_w_m2_tile,
                boundary.ground_albedo,
                boundary.ground_absorbed_w_m2_tile,
            ]
            .iter()
            .any(|value| !value.is_finite() || *value < 0.0)
        })
    {
        return Err(EnergyOwnerError::Identity("radiation boundary shape"));
    }
    let expected = BTreeSet::from([
        (RadiationBand::Visible, IncidentComponent::Direct),
        (RadiationBand::Visible, IncidentComponent::Diffuse),
        (RadiationBand::NearInfrared, IncidentComponent::Direct),
        (RadiationBand::NearInfrared, IncidentComponent::Diffuse),
    ]);
    let actual = boundaries
        .iter()
        .map(|boundary| (boundary.band, boundary.component))
        .collect::<BTreeSet<_>>();
    if actual != expected
        || occupancies
            .iter()
            .any(|proposal| proposal.tile_fraction.to_bits() != tile_fraction.to_bits())
    {
        return Err(EnergyOwnerError::Identity("radiation component identity"));
    }
    for boundary in boundaries {
        let terminal = boundary.terminal_direct_w_m2_tile + boundary.terminal_diffuse_w_m2_tile;
        let ground_residual =
            boundary.ground_absorbed_w_m2_tile - (1.0 - boundary.ground_albedo) * terminal;
        if !(0.0..=1.0).contains(&boundary.ground_albedo)
            || (boundary.component == IncidentComponent::Diffuse
                && boundary.terminal_direct_w_m2_tile.to_bits() != 0.0_f64.to_bits())
            || ground_residual.abs()
                > 2.0e-12 * (boundary.ground_absorbed_w_m2_tile.abs() + terminal.abs()).max(1.0)
        {
            return Err(EnergyOwnerError::Closure {
                component: "terminal ground boundary",
                residual: ground_residual,
            });
        }
        let absorbed = occupancies
            .iter()
            .map(|proposal| {
                proposal
                    .spectral_absorption
                    .iter()
                    .find(|component| {
                        component.band == boundary.band && component.component == boundary.component
                    })
                    .map(|component| {
                        component.sun_leaf_w_m2_tile
                            + component.shade_leaf_w_m2_tile
                            + component.wet_surface_w_m2_tile
                            + component.dry_stem_w_m2_tile
                    })
                    .ok_or(EnergyOwnerError::Identity("spectral absorption identity"))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .sum::<f64>();
        let residual = boundary.incident_w_m2_tile
            - boundary.reflected_w_m2_tile
            - boundary.ground_absorbed_w_m2_tile
            - absorbed;
        let scale = boundary.incident_w_m2_tile.abs()
            + boundary.reflected_w_m2_tile.abs()
            + boundary.ground_absorbed_w_m2_tile.abs()
            + absorbed.abs();
        if !residual.is_finite() || residual.abs() > 2.0e-12 * scale.max(1.0) {
            return Err(EnergyOwnerError::Closure {
                component: "radiation boundary",
                residual,
            });
        }
    }
    Ok(())
}

#[derive(Default)]
struct StandEnergyAccumulator {
    incident_shortwave: f64,
    reflected_shortwave: f64,
    terminal_shortwave: f64,
    incident_longwave: f64,
    emitted_longwave: f64,
    sensible: f64,
    latent: f64,
}

impl StandEnergyAccumulator {
    fn add_occupancy(
        &mut self,
        proposal: &OccupancyEnergyProposal,
        reconstructed: &ReconstructedOccupancy,
    ) {
        let weight = proposal.tile_fraction * proposal.interval_s;
        self.incident_longwave += reconstructed.incident_longwave_w_m2_tile * weight;
        self.emitted_longwave += reconstructed.emitted_longwave_w_m2_tile * weight;
        self.sensible += reconstructed.atmosphere_sensible_w_m2_tile * weight;
        self.latent += reconstructed.latent_w_m2_tile * weight;
    }

    fn add_boundary(&mut self, tile_fraction: f64, boundaries: &[RadiationBoundaryProposal]) {
        for boundary in boundaries {
            self.incident_shortwave += boundary.incident_w_m2_tile * tile_fraction;
            self.reflected_shortwave += boundary.reflected_w_m2_tile * tile_fraction;
            self.terminal_shortwave += boundary.ground_absorbed_w_m2_tile * tile_fraction;
        }
    }

    fn finish(
        mut self,
        proposals: &EnergyProposalBatch,
    ) -> Result<EnergyLedgerOperands, EnergyOwnerError> {
        let interval = proposals.identity.interval_s;
        if proposals
            .occupancies
            .iter()
            .any(|proposal| proposal.interval_s.to_bits() != interval.to_bits())
        {
            return Err(EnergyOwnerError::Identity("mixed energy interval"));
        }
        self.incident_shortwave *= interval;
        self.reflected_shortwave *= interval;
        self.terminal_shortwave *= interval;
        Ok(EnergyLedgerOperands {
            identity: LedgerIdentity {
                transaction_id: proposals.identity.transaction_id,
                owner_id: "diagnostic-energy".into(),
                area_m2: 1.0,
                interval_s: interval,
            },
            incident_shortwave_j_m2: self.incident_shortwave,
            incident_longwave_j_m2: self.incident_longwave,
            reflected_shortwave_j_m2: self.reflected_shortwave,
            terminal_shortwave_j_m2: self.terminal_shortwave,
            emitted_longwave_j_m2: self.emitted_longwave,
            sensible_j_m2: self.sensible,
            latent_j_m2: self.latent,
            ground_or_storage_j_m2: match CanopyHeatStorageMode::EquilibriumZero {
                CanopyHeatStorageMode::EquilibriumZero => 0.0,
            },
        })
    }
}

fn component_scale(component: &(f64, f64, f64, f64, f64)) -> f64 {
    component.1.abs() + component.2.abs() + component.3.abs() + component.4.abs()
}

fn require_energy_closed(
    component: &'static str,
    residual: f64,
    scale: f64,
) -> Result<(), EnergyOwnerError> {
    if !residual.is_finite() || residual.abs() > 1.0e-6 + 1.0e-10 * scale.max(1.0) {
        return Err(EnergyOwnerError::Closure {
            component,
            residual,
        });
    }
    Ok(())
}

fn require_amount_closed(
    component: &'static str,
    residual: f64,
    scale: f64,
    interval_s: f64,
) -> Result<(), EnergyOwnerError> {
    if !residual.is_finite() || residual.abs() > 1.0e-12 * interval_s + 1.0e-9 * scale.max(1.0e-12)
    {
        return Err(EnergyOwnerError::Closure {
            component,
            residual,
        });
    }
    Ok(())
}

fn require_stand_energy_closed(
    component: &'static str,
    residual: f64,
    scale: f64,
    interval_s: f64,
) -> Result<(), EnergyOwnerError> {
    if !residual.is_finite()
        || residual.abs() > 1.0e-6 * interval_s + 1.0e-10 * scale.max(interval_s)
    {
        return Err(EnergyOwnerError::Closure {
            component,
            residual,
        });
    }
    Ok(())
}

fn stand_energy_residual(ledger: &EnergyLedgerOperands) -> f64 {
    ledger.incident_shortwave_j_m2 + ledger.incident_longwave_j_m2
        - ledger.reflected_shortwave_j_m2
        - ledger.terminal_shortwave_j_m2
        - ledger.emitted_longwave_j_m2
        - ledger.sensible_j_m2
        - ledger.latent_j_m2
        - ledger.ground_or_storage_j_m2
}

fn stand_energy_scale(ledger: &EnergyLedgerOperands) -> f64 {
    ledger.incident_shortwave_j_m2.abs()
        + ledger.incident_longwave_j_m2.abs()
        + ledger.reflected_shortwave_j_m2.abs()
        + ledger.terminal_shortwave_j_m2.abs()
        + ledger.emitted_longwave_j_m2.abs()
        + ledger.sensible_j_m2.abs()
        + ledger.latent_j_m2.abs()
        + ledger.ground_or_storage_j_m2.abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use openwepp_kernel_contract::{SoilLayerId, StratumId, TileId};
    use openwepp_vegetation::column::SpectralEnergyAbsorption;
    use openwepp_vegetation::energy_proposal::{
        EnergyProposalIdentity, TileEnergyBoundaryProposal,
    };

    #[allow(clippy::too_many_lines)]
    fn batch() -> EnergyProposalBatch {
        let tile_id = TileId::try_new("tile-a").expect("tile");
        let occupancy_id = OccupancyId {
            stratum_id: StratumId::try_new("canopy").expect("stratum"),
            tile_id: tile_id.clone(),
        };
        let canopy_temperature = 300.0;
        let pressure = R_DRY_AIR_J_KG_K * canopy_temperature;
        let dry_stem_temperature = canopy_temperature + 100.0 / (CP_AIR_J_KG_K * 0.01);
        let incoming_lw = 2.0 * STEFAN_BOLTZMANN * dry_stem_temperature.powi(4);
        let spectral_absorption = vec![
            SpectralEnergyAbsorption {
                band: RadiationBand::Visible,
                component: IncidentComponent::Direct,
                sun_leaf_w_m2_tile: 0.0,
                shade_leaf_w_m2_tile: 0.0,
                wet_surface_w_m2_tile: 0.0,
                dry_stem_w_m2_tile: 100.0,
            },
            SpectralEnergyAbsorption {
                band: RadiationBand::Visible,
                component: IncidentComponent::Diffuse,
                sun_leaf_w_m2_tile: 0.0,
                shade_leaf_w_m2_tile: 0.0,
                wet_surface_w_m2_tile: 0.0,
                dry_stem_w_m2_tile: 0.0,
            },
            SpectralEnergyAbsorption {
                band: RadiationBand::NearInfrared,
                component: IncidentComponent::Direct,
                sun_leaf_w_m2_tile: 0.0,
                shade_leaf_w_m2_tile: 0.0,
                wet_surface_w_m2_tile: 0.0,
                dry_stem_w_m2_tile: 0.0,
            },
            SpectralEnergyAbsorption {
                band: RadiationBand::NearInfrared,
                component: IncidentComponent::Diffuse,
                sun_leaf_w_m2_tile: 0.0,
                shade_leaf_w_m2_tile: 0.0,
                wet_surface_w_m2_tile: 0.0,
                dry_stem_w_m2_tile: 0.0,
            },
        ];
        let occupancy = OccupancyEnergyProposal {
            transaction_id: TransactionId(1),
            occupancy_id,
            tile_fraction: 0.4,
            interval_s: 1.0,
            sun_leaf_absorbed_shortwave_w_m2_tile: 0.0,
            shade_leaf_absorbed_shortwave_w_m2_tile: 0.0,
            wet_surface_absorbed_shortwave_w_m2_tile: 0.0,
            dry_stem_absorbed_shortwave_w_m2_tile: 100.0,
            spectral_absorption,
            dry_sun_leaf_area_m2_m2_tile: 0.0,
            dry_shade_leaf_area_m2_m2_tile: 0.0,
            wet_leaf_area_m2_m2_tile: 0.0,
            wet_stem_area_m2_m2_tile: 0.0,
            dry_stem_area_m2_m2_tile: 1.0,
            sun_leaf_temperature_k: canopy_temperature,
            shade_leaf_temperature_k: canopy_temperature,
            wet_surface_temperature_k: canopy_temperature,
            dry_stem_temperature_k: dry_stem_temperature,
            canopy_air_temperature_k: canopy_temperature,
            canopy_air_specific_humidity_kg_kg: 0.01,
            air_temperature_k: 290.0,
            air_specific_humidity_kg_kg: 0.01,
            pressure_pa: pressure,
            longwave_down_w_m2: incoming_lw / 2.0,
            longwave_up_w_m2: incoming_lw / 2.0,
            gb_leaf_m_s: 0.01,
            gb_wet_m_s: 0.01,
            gb_stem_m_s: 0.01,
            rah_s_m: CP_AIR_J_KG_K / 10.0,
            raw_s_m: 100.0,
            leaf_emissivity: 1.0,
            wet_emissivity: 1.0,
            stem_emissivity: 1.0,
            cp_air_j_kg_k: CP_AIR_J_KG_K,
            rdry_j_kg_k: R_DRY_AIR_J_KG_K,
            latent_heat_j_kg: LATENT_HEAT_VAPORIZATION,
            sun_transpiration_kg_m2_tile: 0.0,
            shade_transpiration_kg_m2_tile: 0.0,
            wet_phase_change_kg_m2_tile: 0.0,
            finalized_layer_withdrawal_kg_m2_tile: vec![(
                SoilLayerId::try_new("soil-1").expect("layer"),
                0.0,
            )],
        };
        let boundary = |band, component, incident| RadiationBoundaryProposal {
            band,
            component,
            incident_w_m2_tile: incident,
            reflected_w_m2_tile: 0.0,
            terminal_direct_w_m2_tile: 0.0,
            terminal_diffuse_w_m2_tile: 0.0,
            ground_albedo: 0.0,
            ground_absorbed_w_m2_tile: 0.0,
        };
        EnergyProposalBatch {
            identity: EnergyProposalIdentity {
                model_definition_sha256: openwepp_vegetation::MODEL_SHA256.into(),
                configuration_sha256: "a".repeat(64),
                beginning_state_sha256: "b".repeat(64),
                ending_state_sha256: "c".repeat(64),
                transaction_id: TransactionId(1),
                interval_s: 1.0,
            },
            occupancies: vec![occupancy],
            tile_boundaries: vec![
                TileEnergyBoundaryProposal {
                    tile_id,
                    tile_fraction: 0.4,
                    components: vec![
                        boundary(RadiationBand::Visible, IncidentComponent::Direct, 100.0),
                        boundary(RadiationBand::Visible, IncidentComponent::Diffuse, 0.0),
                        boundary(RadiationBand::NearInfrared, IncidentComponent::Direct, 0.0),
                        boundary(RadiationBand::NearInfrared, IncidentComponent::Diffuse, 0.0),
                    ],
                },
                TileEnergyBoundaryProposal {
                    tile_id: TileId::try_new("tile-empty").expect("empty tile"),
                    tile_fraction: 0.6,
                    components: vec![
                        boundary(RadiationBand::Visible, IncidentComponent::Direct, 0.0),
                        boundary(RadiationBand::Visible, IncidentComponent::Diffuse, 0.0),
                        boundary(RadiationBand::NearInfrared, IncidentComponent::Direct, 0.0),
                        boundary(RadiationBand::NearInfrared, IncidentComponent::Diffuse, 0.0),
                    ],
                },
            ],
        }
    }

    fn beginning() -> DiagnosticEnergyState {
        DiagnosticEnergyState {
            model_definition_sha256: openwepp_vegetation::MODEL_SHA256.into(),
            configuration_sha256: "a".repeat(64),
            accepted_vegetation_state_sha256: "b".repeat(64),
            last_transaction_id: 0,
            last_operands: None,
        }
    }

    fn topology(proposals: &EnergyProposalBatch) -> EnergyOwnerTopology {
        EnergyOwnerTopology {
            model_definition_sha256: openwepp_vegetation::MODEL_SHA256.into(),
            configuration_sha256: "a".repeat(64),
            tile_fractions: proposals
                .tile_boundaries
                .iter()
                .map(|boundary| (boundary.tile_id.clone(), boundary.tile_fraction))
                .collect(),
            occupancies: proposals
                .occupancies
                .iter()
                .map(|proposal| proposal.occupancy_id.clone())
                .collect(),
        }
    }

    fn wet_batch(wet_phase_change_kg_m2_tile: f64) -> EnergyProposalBatch {
        let mut proposals = batch();
        let occupancy = &mut proposals.occupancies[0];
        let surface_temperature = occupancy.dry_stem_temperature_k;
        let wet_sensible_w_m2 = 100.0;
        let wet_latent_w_m2 = LATENT_HEAT_VAPORIZATION * wet_phase_change_kg_m2_tile;
        let wet_shortwave_w_m2 = wet_sensible_w_m2 + wet_latent_w_m2;
        assert!(wet_shortwave_w_m2 > 0.0);
        occupancy.wet_leaf_area_m2_m2_tile = 1.0;
        occupancy.wet_surface_temperature_k = surface_temperature;
        occupancy.wet_phase_change_kg_m2_tile = wet_phase_change_kg_m2_tile;
        occupancy.wet_surface_absorbed_shortwave_w_m2_tile = wet_shortwave_w_m2;
        occupancy.spectral_absorption[0].wet_surface_w_m2_tile = wet_shortwave_w_m2;
        occupancy.air_temperature_k = 280.0;
        occupancy.air_specific_humidity_kg_kg = occupancy.canopy_air_specific_humidity_kg_kg
            - wet_phase_change_kg_m2_tile * occupancy.raw_s_m;
        proposals.tile_boundaries[0].components[0].incident_w_m2_tile += wet_shortwave_w_m2;
        proposals
    }

    #[test]
    fn independently_reconstructs_component_and_weighted_stand_energy() {
        let beginning = beginning();
        let proposals = batch();
        let candidate = construct_energy_owner_candidate(
            &beginning,
            &topology(&proposals),
            &proposals,
            CanopyHeatStorageMode::EquilibriumZero,
        )
        .expect("component energy candidate");
        assert_eq!(candidate.transaction_id(), TransactionId(1));
        assert_eq!(candidate.beginning(), &beginning);
        assert_eq!(candidate.occupancy_receipts().len(), 1);
        assert!((candidate.stand_ledger().incident_shortwave_j_m2 - 40.0).abs() < 1.0e-12);
        assert!((candidate.stand_ledger().sensible_j_m2 - 40.0).abs() < 1.0e-12);
        assert_eq!(
            candidate.heat_storage_mode(),
            CanopyHeatStorageMode::EquilibriumZero
        );
        assert_eq!(
            candidate.stand_ledger().ground_or_storage_j_m2.to_bits(),
            0.0_f64.to_bits()
        );
        assert!(stand_energy_residual(candidate.stand_ledger()).abs() <= 1.0e-6);
    }

    #[test]
    fn reconstructs_nonzero_evaporation_and_condensation_with_signed_latent_energy() {
        for amount in [4.0e-5, -2.0e-5] {
            let proposals = wet_batch(amount);
            let candidate = construct_energy_owner_candidate(
                &beginning(),
                &topology(&proposals),
                &proposals,
                CanopyHeatStorageMode::EquilibriumZero,
            )
            .expect("signed wet-phase candidate");
            assert_eq!(
                candidate.stand_ledger().latent_j_m2.to_bits(),
                (0.4 * LATENT_HEAT_VAPORIZATION * amount).to_bits()
            );
        }
        let accepted_condensation = wet_batch(-2.0e-5);
        let expected_topology = topology(&accepted_condensation);
        let mut wrong_sign = accepted_condensation;
        wrong_sign.occupancies[0].wet_phase_change_kg_m2_tile = 2.0e-5;
        assert!(
            construct_energy_owner_candidate(
                &beginning(),
                &expected_topology,
                &wrong_sign,
                CanopyHeatStorageMode::EquilibriumZero,
            )
            .is_err()
        );
    }

    #[test]
    fn uses_interval_scaled_stand_threshold_and_typed_transaction_overflow() {
        require_stand_energy_closed("daily stand", 0.04, 1.0, 86_400.0)
            .expect("interval-scaled absolute threshold");
        let mut overflow = beginning();
        overflow.last_transaction_id = u128::MAX;
        let proposals = batch();
        assert!(matches!(
            construct_energy_owner_candidate(
                &overflow,
                &topology(&proposals),
                &proposals,
                CanopyHeatStorageMode::EquilibriumZero,
            ),
            Err(EnergyOwnerError::Identity("transaction overflow"))
        ));
    }

    #[test]
    fn rejects_identity_basis_component_and_finalized_water_poisons() {
        let beginning = beginning();
        let expected_topology = topology(&batch());
        let assert_rejected = |mutated: EnergyProposalBatch| {
            assert!(
                construct_energy_owner_candidate(
                    &beginning,
                    &expected_topology,
                    &mutated,
                    CanopyHeatStorageMode::EquilibriumZero,
                )
                .is_err()
            );
        };

        let mut wrong_fraction = batch();
        wrong_fraction.occupancies[0].tile_fraction = 0.5;
        assert_rejected(wrong_fraction);

        let mut coherent_wrong_fraction = batch();
        coherent_wrong_fraction.occupancies[0].tile_fraction = 0.5;
        coherent_wrong_fraction.tile_boundaries[0].tile_fraction = 0.5;
        assert_rejected(coherent_wrong_fraction);

        let mut doubled_fraction = batch();
        doubled_fraction.occupancies[0].tile_fraction = 2.0;
        assert_rejected(doubled_fraction);

        let mut wrong_tile = batch();
        wrong_tile.occupancies[0].occupancy_id.tile_id =
            TileId::try_new("tile-b").expect("wrong tile");
        assert_rejected(wrong_tile);

        let mut omitted_stem = batch();
        omitted_stem.occupancies[0].dry_stem_absorbed_shortwave_w_m2_tile = 0.0;
        assert_rejected(omitted_stem);

        let mut wrong_spectrum = batch();
        wrong_spectrum.occupancies[0].spectral_absorption[0].band = RadiationBand::NearInfrared;
        assert_rejected(wrong_spectrum);

        let mut forged_configuration = batch();
        forged_configuration.identity.configuration_sha256 = "d".repeat(64);
        assert_rejected(forged_configuration);

        let mut forged_terminal = batch();
        forged_terminal.tile_boundaries[0].components[0].terminal_direct_w_m2_tile = 1.0;
        assert_rejected(forged_terminal);

        let mut direct_diffuse_swap = batch();
        direct_diffuse_swap.occupancies[0].spectral_absorption[0].component =
            IncidentComponent::Diffuse;
        direct_diffuse_swap.occupancies[0].spectral_absorption[1].component =
            IncidentComponent::Direct;
        assert_rejected(direct_diffuse_swap);

        let mut vis_nir_swap = batch();
        vis_nir_swap.occupancies[0].spectral_absorption[0].band = RadiationBand::NearInfrared;
        vis_nir_swap.occupancies[0].spectral_absorption[2].band = RadiationBand::Visible;
        assert_rejected(vis_nir_swap);

        let mut stem_as_sun = batch();
        stem_as_sun.occupancies[0].dry_stem_absorbed_shortwave_w_m2_tile = 0.0;
        stem_as_sun.occupancies[0].sun_leaf_absorbed_shortwave_w_m2_tile = 100.0;
        stem_as_sun.occupancies[0].spectral_absorption[0].dry_stem_w_m2_tile = 0.0;
        stem_as_sun.occupancies[0].spectral_absorption[0].sun_leaf_w_m2_tile = 100.0;
        assert_rejected(stem_as_sun);

        let mut authorization_as_use = batch();
        authorization_as_use.occupancies[0].finalized_layer_withdrawal_kg_m2_tile[0].1 = 0.01;
        assert_rejected(authorization_as_use);

        let mut wet_sign = batch();
        wet_sign.occupancies[0].wet_phase_change_kg_m2_tile = 0.01;
        assert_rejected(wet_sign);

        let mut rate_as_amount = batch();
        rate_as_amount.occupancies[0].interval_s = 2.0;
        assert_rejected(rate_as_amount);
    }
}
