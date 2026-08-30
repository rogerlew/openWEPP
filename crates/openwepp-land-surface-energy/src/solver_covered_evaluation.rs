/// Frozen biochemical constants for one C3 sun or shade class.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LeafBiochemicalInputs {
    pub leaf_area_m2_m2_tile: f64,
    pub absorbed_shortwave_w_m2_tile: f64,
    pub absorbed_par_w_m2_leaf: f64,
    pub vcmax25: f64,
    pub jmax25: f64,
    pub rd25: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BiochemicalConstants {
    pub ha_vcmax_j_mol: f64,
    pub hd_vcmax_j_mol: f64,
    pub entropy_vcmax_j_mol_k: f64,
    pub ha_jmax_j_mol: f64,
    pub hd_jmax_j_mol: f64,
    pub entropy_jmax_j_mol_k: f64,
    pub kc25_pa: f64,
    pub ha_kc_j_mol: f64,
    pub ko25_pa: f64,
    pub ha_ko_j_mol: f64,
    pub gamma25_pa: f64,
    pub ha_gamma_j_mol: f64,
    pub oxygen_partial_pressure_pa: f64,
    pub tp_vcmax_ratio: f64,
    pub electron_quantum_yield: f64,
    pub par_photon_umol_per_j: f64,
    pub electron_curvature: f64,
    pub ac_aj_curvature: f64,
    pub ag_ap_curvature: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RootHydraulicLayer {
    pub layer_id: String,
    pub accessible: bool,
    pub frozen: bool,
    pub root_fraction: f64,
    pub soil_potential_mm: f64,
    pub gravity_head_mm: f64,
    pub z3_m: f64,
    pub dxroot_m: f64,
    pub ksoil_m2_s: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SourceWaterCap {
    pub request_rate_kg_m2_tile_s: f64,
    pub authorization_rate_kg_m2_tile_s: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoveredWaterCaps {
    /// Exact `(occupancy_id, layer_id) -> tile-ground rate` identity.
    pub root: BTreeMap<(String, String), SourceWaterCap>,
    /// The one ground-source authorization for this tile.
    pub ground: SourceWaterCap,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CoveredFrozenBranches {
    pub root: BTreeMap<(String, String), WaterBranch>,
    pub wet: BTreeMap<String, WaterBranch>,
    pub ground: Option<WaterBranch>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SourceWaterFlux {
    pub occupancy_id: String,
    pub layer_id: String,
    pub law_kg_m2_tile_s: f64,
    pub final_kg_m2_tile_s: f64,
    pub request_kg_m2_stand_ground: f64,
    pub authorization_kg_m2_stand_ground: Option<f64>,
    pub finalized_use_kg_m2_stand_ground: f64,
    pub branch: WaterBranch,
}

fn exact_inactive_hydraulic_occupancy(
    authority: CoveredColumnAuthority,
    occupancy: &CoveredOccupancyInputs,
) -> bool {
    authority.admits_nonpositive_assimilation()
        && occupancy.lai.to_bits() == 0.0_f64.to_bits()
        && occupancy.sai.to_bits() == 0.0_f64.to_bits()
        && occupancy.stem_area_m2_m2_tile.to_bits() == 0.0_f64.to_bits()
        && occupancy.sun.leaf_area_m2_m2_tile.to_bits() == 0.0_f64.to_bits()
        && occupancy.shade.leaf_area_m2_m2_tile.to_bits() == 0.0_f64.to_bits()
}

const LIQUID_VAPOR_PHASE_MINIMUM_K: f64 = 273.15;

/// Returns the representational temperature target for an energy coordinate
/// whose physical component has exactly zero area (or is otherwise admitted
/// as numerically inactive).
///
/// The sun, shade, and wet coordinates feed the liquid-vapor saturation law,
/// so their inactive target cannot be colder than that law's phase-domain
/// minimum. The dry-stem coordinate has no such constitutive restriction and
/// continues to track canopy-air temperature exactly.
fn inactive_component_temperature_anchor_k(
    component_index: usize,
    canopy_air_temperature_k: f64,
) -> f64 {
    if component_index < 3 {
        canopy_air_temperature_k.max(LIQUID_VAPOR_PHASE_MINIMUM_K)
    } else {
        canopy_air_temperature_k
    }
}

#[cfg(test)]
mod inactive_component_temperature_anchor_tests {
    use super::{LIQUID_VAPOR_PHASE_MINIMUM_K, inactive_component_temperature_anchor_k};

    #[test]
    fn cold_canopy_zero_area_liquid_vapor_coordinates_stay_inside_phase_domain() {
        let cold_canopy_k = 262.531_500_532_82;

        for component_index in 0..3 {
            assert_eq!(
                inactive_component_temperature_anchor_k(component_index, cold_canopy_k).to_bits(),
                LIQUID_VAPOR_PHASE_MINIMUM_K.to_bits()
            );
        }
        assert_eq!(
            inactive_component_temperature_anchor_k(3, cold_canopy_k).to_bits(),
            cold_canopy_k.to_bits()
        );
    }

    #[test]
    fn warm_canopy_inactive_coordinates_preserve_the_canopy_temperature() {
        let warm_canopy_k = 285.0;

        for component_index in 0..4 {
            assert_eq!(
                inactive_component_temperature_anchor_k(component_index, warm_canopy_k).to_bits(),
                warm_canopy_k.to_bits()
            );
        }
    }
}

pub(crate) fn exact_inactive_source_water(source: &SourceWaterFlux) -> bool {
    source.law_kg_m2_tile_s.to_bits() == 0.0_f64.to_bits()
        && source.final_kg_m2_tile_s.to_bits() == 0.0_f64.to_bits()
        && source.request_kg_m2_stand_ground.to_bits() == 0.0_f64.to_bits()
        && source.finalized_use_kg_m2_stand_ground.to_bits() == 0.0_f64.to_bits()
        && source
            .authorization_kg_m2_stand_ground
            .is_none_or(|value| value.to_bits() == 0.0_f64.to_bits())
        && source.branch == WaterBranch::ConstitutiveLaw
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoveredOccupancyInputs {
    pub occupancy_id: String,
    pub medlyn_g1_kpa_sqrt: f64,
    pub g0_umol_m2_s: f64,
    pub sun: LeafBiochemicalInputs,
    pub shade: LeafBiochemicalInputs,
    pub biochemical: BiochemicalConstants,
    pub stem_area_m2_m2_tile: f64,
    pub stem_absorbed_shortwave_w_m2_tile: f64,
    /// Immutable beginning occupancy store before current top-to-bottom E04.
    pub beginning_canopy_liquid_kg_m2_tile: f64,
    pub liquid_interception_fraction: f64,
    pub liquid_capacity_kg_m2_plant: f64,
    pub stemflow_fraction: f64,
    pub gb_leaf_m_s: f64,
    pub gb_wet_m_s: f64,
    pub gb_stem_m_s: f64,
    pub lai: f64,
    pub sai: f64,
    pub clumping_index: f64,
    pub k1_sun_max_s1: f64,
    pub k1_shade_max_s1: f64,
    pub k2_max: f64,
    pub k3_max_m_s: f64,
    pub height_m: f64,
    pub root_to_leaf_area: f64,
    pub p50_leaf_mm: f64,
    pub p50_xylem_mm: f64,
    pub p50_root_mm: f64,
    pub vulnerability_exponent: f64,
    pub root_layers: Vec<RootHydraulicLayer>,
}

/// Bound E01--E03 band/direction absorption owned by one physical canopy
/// occupancy before wet/dry surface partitioning.
#[derive(Clone, Debug, PartialEq)]
pub struct CoveredOccupancyShortwaveInputs {
    pub occupancy_id: String,
    pub sun_leaf_absorbed_w_m2_tile: BandDirectionalFluxes,
    pub shade_leaf_absorbed_w_m2_tile: BandDirectionalFluxes,
    pub stem_absorbed_w_m2_tile: BandDirectionalFluxes,
}

/// Complete column shortwave boundary receipt from the admitted radiation
/// owner. These are primitive E01--E03 results, not inferred all-wave shares.
#[derive(Clone, Debug, PartialEq)]
pub struct CoveredColumnShortwaveInputs {
    pub incident_w_m2_tile: BandDirectionalFluxes,
    pub top_reflected_w_m2_tile: BandDirectionalFluxes,
    /// Ground absorption attributed to each incident band/direction after
    /// reciprocal ground-canopy reflection. This is distinct from the raw
    /// downward terminal flux at the ground boundary.
    pub ground_absorbed_by_incident_w_m2_tile: BandDirectionalFluxes,
    pub occupancies: Vec<CoveredOccupancyShortwaveInputs>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoveredColumnInputs {
    /// Validated coupled physiology/numerical authority. Historical variants
    /// must never infer successor behavior from forcing values alone.
    pub authority: CoveredColumnAuthority,
    pub interval_s: f64,
    pub tile_fraction: f64,
    pub pressure_pa: f64,
    pub air_temperature_k: f64,
    pub air_specific_humidity_kg_kg: f64,
    pub reference_wind_m_s: f64,
    pub atmospheric_downward_longwave_w_m2: f64,
    pub ca_pa: f64,
    pub canopy_to_atmosphere_heat_resistance_s_m: f64,
    pub canopy_to_atmosphere_vapor_resistance_s_m: f64,
    pub latent_heat_j_kg: f64,
    /// Current interval rain entering the top occupancy on tile-ground basis.
    pub top_rain_kg_m2_tile: f64,
    pub under_canopy_geometry: crate::physics::UnderCanopyGeometry,
    pub ground: OpenSurfaceProblem,
    pub occupancies: Vec<CoveredOccupancyInputs>,
    pub shortwave: CoveredColumnShortwaveInputs,
    /// Stage-3-owned lower-boundary operands for the explicit V11 covered
    /// canopy mode. Historical covered columns leave this absent.
    pub stage3_lower_boundary: Option<Stage3SnowCoveredLowerBoundary>,
    /// Canonical band/direction optical handoff produced by the radiation
    /// owner with the Stage-3 snow albedo already in the column solve.
    pub stage3_optical: Option<Stage3SnowOpticalBoundaryReceiptV1>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoveredColumnAuthority {
    HistoricalV8,
    V10NonpositiveAssimilation,
    V11SnowCovered,
}

impl CoveredColumnAuthority {
    /// Typed authority gate for the exact dark and constructive low-light
    /// branches inherited by V11 from V10 under `INV-VEGETATION-118..120`.
    /// V11's Stage-3 snow lower boundary remains selected independently by
    /// exact `V11SnowCovered` matches at the boundary joins.
    #[must_use]
    pub const fn admits_nonpositive_assimilation(self) -> bool {
        matches!(
            self,
            Self::V10NonpositiveAssimilation | Self::V11SnowCovered
        )
    }
}

/// Stage-3-owned lower-boundary operands for the V11 covered canopy solve.
///
/// The LSE solver consumes the shared carrier's lower-boundary transfer only
/// to close the canopy-air node and to use the released snow temperature in
/// reciprocal longwave. It does not own snow mass, surface liquid, or the
/// Stage-3 energy ledger.
#[derive(Clone, Debug, PartialEq)]
pub struct Stage3SnowCoveredLowerBoundary {
    pub snow_temperature_k: f64,
    pub latent_heat_j_kg: f64,
    pub sensible_to_canopy_air_w_m2: f64,
    pub vapor_to_canopy_air_kg_m2_s: f64,
    pub net_longwave_w_m2: f64,
    pub shortwave_absorbed_w_m2: f64,
    pub precipitation_advection_w_m2: f64,
    pub carrier_receipt_id: Sha256Digest,
    pub snow_vis_albedo: f64,
    pub snow_nir_albedo: f64,
    pub stage3_albedo_state_sha256: Sha256Digest,
    pub forcing_receipt_sha256: Sha256Digest,
    /// Populated only after the keyed optical correction is accepted. The
    /// provisional pass intentionally carries no optical receipt.
    pub optical_receipt_sha256: Option<Sha256Digest>,
    pub reciprocal_longwave_receipt_sha256: Option<Sha256Digest>,
    pub final_canopy_boundary_receipt_sha256: Option<Sha256Digest>,
}

/// Exact VIS/NIR and direct/diffuse optical custody for one covered OFE/tile.
/// The receipt is created from the same two-stream result that supplies the
/// canopy absorption and top reflection; it is not a post-hoc scalar energy
/// correction.
#[derive(Clone, Debug, PartialEq)]
pub struct Stage3SnowOpticalBoundaryReceiptV1 {
    pub ofe_id: OfeId,
    pub tile_id: TileId,
    pub terminal_w_m2_tile: BandDirectionalFluxes,
    pub absorbed_w_m2_tile: BandDirectionalFluxes,
    pub reflected_w_m2_tile: BandDirectionalFluxes,
    pub snow_vis_albedo: f64,
    pub snow_nir_albedo: f64,
    pub stage3_albedo_state_sha256: Sha256Digest,
    pub forcing_receipt_sha256: Sha256Digest,
    pub receipt_sha256: Sha256Digest,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stage3SnowOpticalBoundaryReceiptInputs {
    pub ofe_id: OfeId,
    pub tile_id: TileId,
    pub terminal_w_m2_tile: BandDirectionalFluxes,
    pub absorbed_w_m2_tile: BandDirectionalFluxes,
    pub reflected_w_m2_tile: BandDirectionalFluxes,
    pub snow_vis_albedo: f64,
    pub snow_nir_albedo: f64,
    pub stage3_albedo_state_sha256: Sha256Digest,
    pub forcing_receipt_sha256: Sha256Digest,
}

impl Stage3SnowOpticalBoundaryReceiptV1 {
    pub fn try_new(
        inputs: Stage3SnowOpticalBoundaryReceiptInputs,
    ) -> Result<Self, LandSurfaceEnergyError> {
        let receipt_sha256 = optical_receipt_digest(&inputs)?;
        let receipt = Self {
            ofe_id: inputs.ofe_id,
            tile_id: inputs.tile_id,
            terminal_w_m2_tile: inputs.terminal_w_m2_tile,
            absorbed_w_m2_tile: inputs.absorbed_w_m2_tile,
            reflected_w_m2_tile: inputs.reflected_w_m2_tile,
            snow_vis_albedo: inputs.snow_vis_albedo,
            snow_nir_albedo: inputs.snow_nir_albedo,
            stage3_albedo_state_sha256: inputs.stage3_albedo_state_sha256,
            forcing_receipt_sha256: inputs.forcing_receipt_sha256,
            receipt_sha256,
        };
        receipt.validate()?;
        Ok(receipt)
    }

    pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        if self.ofe_id.as_str().trim().is_empty()
            || self.tile_id.as_str().trim().is_empty()
            || !self.snow_vis_albedo.is_finite()
            || !self.snow_nir_albedo.is_finite()
            || !(0.0..=1.0).contains(&self.snow_vis_albedo)
            || !(0.0..=1.0).contains(&self.snow_nir_albedo)
            || self.stage3_albedo_state_sha256.as_str().is_empty()
            || self.forcing_receipt_sha256.as_str().is_empty()
        {
            return Err(LandSurfaceEnergyError::ComponentClosure(
                "Stage-3 snow optical boundary domain",
            ));
        }
        self.terminal_w_m2_tile.validate_nonnegative()?;
        self.absorbed_w_m2_tile.validate_nonnegative()?;
        self.reflected_w_m2_tile.validate_nonnegative()?;
        let terminal = directional_values(self.terminal_w_m2_tile);
        let absorbed = directional_values(self.absorbed_w_m2_tile);
        let reflected = directional_values(self.reflected_w_m2_tile);
        for index in 0..4 {
            if (terminal[index] - absorbed[index] - reflected[index]).abs()
                > energy_tolerance(
                    terminal[index].abs() + absorbed[index].abs() + reflected[index].abs(),
                )
            {
                return Err(LandSurfaceEnergyError::ComponentClosure(
                    "Stage-3 snow optical terminal partition",
                ));
            }
            let albedo = if index < 2 {
                self.snow_vis_albedo
            } else {
                self.snow_nir_albedo
            };
            if (reflected[index] - albedo * terminal[index]).abs()
                > energy_tolerance(reflected[index].abs() + terminal[index].abs())
            {
                return Err(LandSurfaceEnergyError::ComponentClosure(
                    "Stage-3 snow optical albedo",
                ));
            }
        }
        if optical_receipt_digest(&Stage3SnowOpticalBoundaryReceiptInputs {
            ofe_id: self.ofe_id.clone(),
            tile_id: self.tile_id.clone(),
            terminal_w_m2_tile: self.terminal_w_m2_tile,
            absorbed_w_m2_tile: self.absorbed_w_m2_tile,
            reflected_w_m2_tile: self.reflected_w_m2_tile,
            snow_vis_albedo: self.snow_vis_albedo,
            snow_nir_albedo: self.snow_nir_albedo,
            stage3_albedo_state_sha256: self.stage3_albedo_state_sha256.clone(),
            forcing_receipt_sha256: self.forcing_receipt_sha256.clone(),
        })? != self.receipt_sha256
        {
            return Err(LandSurfaceEnergyError::StateLineage(
                "Stage-3 snow optical receipt digest",
            ));
        }
        Ok(())
    }
}

fn optical_receipt_digest(
    inputs: &Stage3SnowOpticalBoundaryReceiptInputs,
) -> Result<Sha256Digest, LandSurfaceEnergyError> {
    let mut bytes = Vec::with_capacity(256);
    bytes.extend_from_slice(b"OPENWEPP_STAGE3_SNOW_OPTICAL_BOUNDARY_V1\0");
    append_framed_str(&mut bytes, inputs.ofe_id.as_str());
    append_framed_str(&mut bytes, inputs.tile_id.as_str());
    for value in directional_values(inputs.terminal_w_m2_tile)
        .into_iter()
        .chain(directional_values(inputs.absorbed_w_m2_tile))
        .chain(directional_values(inputs.reflected_w_m2_tile))
    {
        bytes.extend_from_slice(&value.to_bits().to_le_bytes());
    }
    bytes.extend_from_slice(&inputs.snow_vis_albedo.to_bits().to_le_bytes());
    bytes.extend_from_slice(&inputs.snow_nir_albedo.to_bits().to_le_bytes());
    append_framed_str(&mut bytes, inputs.stage3_albedo_state_sha256.as_str());
    append_framed_str(&mut bytes, inputs.forcing_receipt_sha256.as_str());
    Sha256Digest::try_new(format!("{:x}", Sha256::digest(bytes)))
}

fn append_framed_str(bytes: &mut Vec<u8>, value: &str) {
    bytes.extend_from_slice(&(value.len() as u64).to_le_bytes());
    bytes.extend_from_slice(value.as_bytes());
}

impl Stage3SnowCoveredLowerBoundary {
    pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        if [
            self.snow_temperature_k,
            self.latent_heat_j_kg,
            self.sensible_to_canopy_air_w_m2,
            self.vapor_to_canopy_air_kg_m2_s,
            self.net_longwave_w_m2,
            self.shortwave_absorbed_w_m2,
            self.precipitation_advection_w_m2,
        ]
        .iter()
        .any(|value| !value.is_finite())
            || !(200.0..=350.0).contains(&self.snow_temperature_k)
            || self.latent_heat_j_kg <= 0.0
            || self.carrier_receipt_id.as_str().is_empty()
            || self.stage3_albedo_state_sha256.as_str().is_empty()
            || self.forcing_receipt_sha256.as_str().is_empty()
            || !self.snow_vis_albedo.is_finite()
            || !self.snow_nir_albedo.is_finite()
            || !(0.0..=1.0).contains(&self.snow_vis_albedo)
            || !(0.0..=1.0).contains(&self.snow_nir_albedo)
        {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "Stage-3 snow-covered lower boundary",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum V10LeafGasBranch {
    Inactive,
    ExactZeroPar,
    RespirationDominated,
    PositiveAssimilation,
}

#[cfg(test)]
pub(crate) fn v10_exact_zero_par_active(beginning: &CoveredColumnInputs) -> bool {
    beginning.authority.admits_nonpositive_assimilation()
        && beginning.occupancies.iter().any(|occupancy| {
            (occupancy.sun.leaf_area_m2_m2_tile > 0.0
                && occupancy.sun.absorbed_par_w_m2_leaf == 0.0)
                || (occupancy.shade.leaf_area_m2_m2_tile > 0.0
                    && occupancy.shade.absorbed_par_w_m2_leaf == 0.0)
        })
}

pub(crate) fn v10_initial_final_residuals_pass(normalized_residuals: &[f64]) -> bool {
    normalized_residuals
        .iter()
        .all(|value| value.is_finite() && value.abs() <= 1.0)
}

pub(crate) fn v10_nonpositive_assimilation_active(evaluation: &CoveredColumnEvaluation) -> bool {
    evaluation.occupancies.iter().any(|occupancy| {
        occupancy.gas_branches.iter().any(|branch| {
            matches!(
                branch,
                V10LeafGasBranch::Inactive
                    | V10LeafGasBranch::ExactZeroPar
                    | V10LeafGasBranch::RespirationDominated
            )
        })
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct LeafTrialState {
    surface_q: f64,
    rs_s_m: f64,
    ci_pa: f64,
    gross_assimilation_umol_co2_m2_leaf_s: f64,
    net_assimilation_umol_co2_m2_leaf_s: f64,
    dark_respiration_umol_co2_m2_leaf_s: f64,
    gas_branch: V10LeafGasBranch,
}

#[derive(Clone, Copy)]
struct LeafGasEnvironment {
    authority: CoveredColumnAuthority,
    pressure_pa: f64,
    ca_pa: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct LeafCarbonState {
    ag: f64,
    an: f64,
    rd: f64,
}

const MOLAR_GAS_CONSTANT: f64 = 8.314_462_618_153_24;

fn log_one_plus_exp(value: f64) -> f64 {
    if value > 0.0 {
        value + (-value).exp().ln_1p()
    } else {
        value.exp().ln_1p()
    }
}

fn arrhenius(temperature: f64, activation: f64) -> Result<f64, LandSurfaceEnergyError> {
    if !temperature.is_finite()
        || !activation.is_finite()
        || temperature <= 0.0
        || activation <= 0.0
    {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "arrhenius_response",
        ));
    }
    Ok((activation / MOLAR_GAS_CONSTANT * (1.0 / 298.15 - 1.0 / temperature)).exp())
}

fn peaked(
    temperature: f64,
    activation: f64,
    deactivation: f64,
    entropy: f64,
) -> Result<f64, LandSurfaceEnergyError> {
    if [temperature, activation, deactivation, entropy]
        .iter()
        .any(|value| !value.is_finite() || *value <= 0.0)
    {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "peaked_response",
        ));
    }
    let reference = 298.15;
    let log_factor = activation * (temperature - reference)
        / (MOLAR_GAS_CONSTANT * temperature * reference)
        + log_one_plus_exp((reference * entropy - deactivation) / (MOLAR_GAS_CONSTANT * reference))
        - log_one_plus_exp(
            (temperature * entropy - deactivation) / (MOLAR_GAS_CONSTANT * temperature),
        );
    let result = log_factor.exp();
    if result.is_finite() {
        Ok(result)
    } else {
        Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "peaked_response",
        ))
    }
}

fn smaller_quadratic_root(a: f64, b: f64, c: f64) -> Result<f64, LandSurfaceEnergyError> {
    if a == 0.0 {
        return if b == 0.0 {
            Err(LandSurfaceEnergyError::ConstitutiveDomain("quadratic"))
        } else {
            Ok(-c / b)
        };
    }
    if c == 0.0 {
        return Ok(0.0_f64.min(-b / a));
    }
    let mut discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        let scale = (b * b).abs().max((4.0 * a * c).abs());
        if discriminant >= -64.0 * f64::EPSILON * scale {
            discriminant = 0.0;
        } else {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "photosynthesis_discriminant",
            ));
        }
    }
    let root = discriminant.sqrt();
    let q = -0.5 * (b + root.copysign(b));
    Ok((q / a).min(c / q))
}

fn canopy_saturation_q(temperature: f64, pressure: f64) -> Result<f64, LandSurfaceEnergyError> {
    let tc = temperature - 273.15;
    if !(0.0..=100.0).contains(&tc) {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "liquid_saturation_polynomial",
        ));
    }
    let coefficients = [
        6.112_134_76,
        4.440_078_56e-1,
        1.430_642_34e-2,
        2.644_614_37e-4,
        3.059_035_58e-6,
        1.962_372_41e-8,
        8.923_447_72e-11,
        -3.732_084_10e-13,
        2.093_399_97e-16,
    ];
    let es = 100.0
        * coefficients
            .iter()
            .scan(1.0, |power, value| {
                let term = value * *power;
                *power *= tc;
                Some(term)
            })
            .sum::<f64>();
    Ok(0.622 * es / (pressure - 0.378 * es))
}

#[allow(clippy::too_many_arguments)]
fn leaf_trial_state(
    inputs: LeafBiochemicalInputs,
    p: BiochemicalConstants,
    temperature: f64,
    qcan: f64,
    beta: f64,
    environment: LeafGasEnvironment,
    gb_leaf: f64,
    g0_umol_m2_s: f64,
    medlyn_g1_kpa_sqrt: f64,
) -> Result<LeafTrialState, LandSurfaceEnergyError> {
    let vcmax_factor = peaked(
        temperature,
        p.ha_vcmax_j_mol,
        p.hd_vcmax_j_mol,
        p.entropy_vcmax_j_mol_k,
    )?;
    let jmax_factor = peaked(
        temperature,
        p.ha_jmax_j_mol,
        p.hd_jmax_j_mol,
        p.entropy_jmax_j_mol_k,
    )?;
    let vcmax = inputs.vcmax25 * vcmax_factor;
    let jmax = inputs.jmax25 * jmax_factor;
    let kc = p.kc25_pa * arrhenius(temperature, p.ha_kc_j_mol)?;
    let ko = p.ko25_pa * arrhenius(temperature, p.ha_ko_j_mol)?;
    let gamma = p.gamma25_pa * arrhenius(temperature, p.ha_gamma_j_mol)?;
    let tp = p.tp_vcmax_ratio * inputs.vcmax25 * vcmax_factor;
    let rd = inputs.rd25 * peaked(temperature, 46_390.0, 150_650.0, 490.0)?;
    let qsurface = canopy_saturation_q(temperature, environment.pressure_pa)?;
    if environment.authority.admits_nonpositive_assimilation() && inputs.leaf_area_m2_m2_tile == 0.0
    {
        let gs_ms =
            g0_umol_m2_s * 1.0e-6 * MOLAR_GAS_CONSTANT * temperature / environment.pressure_pa;
        if !gs_ms.is_finite() || gs_ms <= 0.0 {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "v10_zero_area_stomatal_conductance",
            ));
        }
        return Ok(LeafTrialState {
            surface_q: qsurface,
            rs_s_m: 1.0 / gs_ms,
            ci_pa: environment.ca_pa,
            gross_assimilation_umol_co2_m2_leaf_s: 0.0,
            net_assimilation_umol_co2_m2_leaf_s: 0.0,
            dark_respiration_umol_co2_m2_leaf_s: 0.0,
            gas_branch: V10LeafGasBranch::Inactive,
        });
    }
    if environment.authority.admits_nonpositive_assimilation()
        && inputs.absorbed_par_w_m2_leaf == 0.0
    {
        let gs_ms =
            g0_umol_m2_s * 1.0e-6 * MOLAR_GAS_CONSTANT * temperature / environment.pressure_pa;
        if !gs_ms.is_finite() || gs_ms <= 0.0 {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "v10_zero_par_stomatal_conductance",
            ));
        }
        let rs = 1.0 / gs_ms;
        let an = -rd;
        let rb = 1.0 / gb_leaf;
        let cs = environment.ca_pa - 1.4 * rb * MOLAR_GAS_CONSTANT * temperature * an * 1.0e-6;
        let ci = environment.ca_pa
            - (1.4 * rb + 1.6 * rs) * MOLAR_GAS_CONSTANT * temperature * an * 1.0e-6;
        if !cs.is_finite()
            || cs <= 0.0
            || !ci.is_finite()
            || ci <= environment.ca_pa
            || ci >= environment.pressure_pa
        {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "v10_zero_par_ci",
            ));
        }
        return Ok(LeafTrialState {
            surface_q: qsurface,
            rs_s_m: rs,
            ci_pa: ci,
            gross_assimilation_umol_co2_m2_leaf_s: 0.0,
            net_assimilation_umol_co2_m2_leaf_s: an,
            dark_respiration_umol_co2_m2_leaf_s: rd,
            gas_branch: V10LeafGasBranch::ExactZeroPar,
        });
    }
    let es_leaf = qsurface * environment.pressure_pa / (0.622 + 0.378 * qsurface);
    let e_can = qcan * environment.pressure_pa / (0.622 + 0.378 * qcan);
    let vpd = (es_leaf - e_can) / 1000.0;
    if vpd <= 0.0 {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain("surface_vpd"));
    }
    let carbon_at_ci = |ci: f64| -> Result<LeafCarbonState, LandSurfaceEnergyError> {
        let ipsii = 0.5
            * p.electron_quantum_yield
            * p.par_photon_umol_per_j
            * inputs.absorbed_par_w_m2_leaf;
        let electron = if ipsii > 0.0 {
            smaller_quadratic_root(p.electron_curvature, -(ipsii + jmax), ipsii * jmax)?
        } else {
            0.0
        };
        let ac = vcmax * (ci - gamma) / (ci + kc * (1.0 + p.oxygen_partial_pressure_pa / ko));
        let aj = electron * (ci - gamma) / (4.0 * ci + 8.0 * gamma);
        let ai = smaller_quadratic_root(p.ac_aj_curvature, -(ac + aj), ac * aj)?;
        let ag = smaller_quadratic_root(p.ag_ap_curvature, -(ai + 3.0 * tp), ai * 3.0 * tp)?;
        let an = ag - rd;
        if !ag.is_finite() || !an.is_finite() || !rd.is_finite() {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "photosynthesis_nonfinite",
            ));
        }
        Ok(LeafCarbonState { ag, an, rd })
    };
    let residual = |ci: f64| -> Result<(f64, f64), LandSurfaceEnergyError> {
        let carbon = carbon_at_ci(ci)?;
        let an = carbon.an;
        let rb = 1.0 / gb_leaf;
        let cs = environment.ca_pa - 1.4 * rb * MOLAR_GAS_CONSTANT * temperature * an * 1.0e-6;
        if cs <= 0.0 {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain("surface_co2"));
        }
        let potential = if an <= 0.0 {
            g0_umol_m2_s
        } else {
            g0_umol_m2_s
                + 1.6 * (1.0 + medlyn_g1_kpa_sqrt / vpd.sqrt()) * an
                    / (cs / environment.pressure_pa)
        };
        let gs = g0_umol_m2_s + beta * (potential - g0_umol_m2_s);
        let gs_ms = gs * 1.0e-6 * MOLAR_GAS_CONSTANT * temperature / environment.pressure_pa;
        if gs_ms <= 0.0 {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "stomatal_conductance",
            ));
        }
        let rs = 1.0 / gs_ms;
        let predicted = environment.ca_pa
            - (1.4 * rb + 1.6 * rs) * MOLAR_GAS_CONSTANT * temperature * an * 1.0e-6;
        Ok((ci - predicted, rs))
    };
    let mut a = gamma;
    let mut b = environment.ca_pa;
    let (mut fa, _) = residual(a)?;
    let (mut fb, mut rs) = residual(b)?;
    if fa == 0.0 {
        rs = residual(a)?.1;
        let carbon = carbon_at_ci(a)?;
        return Ok(LeafTrialState {
            surface_q: qsurface,
            rs_s_m: rs,
            ci_pa: a,
            gross_assimilation_umol_co2_m2_leaf_s: carbon.ag,
            net_assimilation_umol_co2_m2_leaf_s: carbon.an,
            dark_respiration_umol_co2_m2_leaf_s: carbon.rd,
            gas_branch: if carbon.an <= 0.0 {
                V10LeafGasBranch::RespirationDominated
            } else {
                V10LeafGasBranch::PositiveAssimilation
            },
        });
    }
    if environment.authority.admits_nonpositive_assimilation() && fb == 0.0 {
        let carbon = carbon_at_ci(b)?;
        return Ok(LeafTrialState {
            surface_q: qsurface,
            rs_s_m: rs,
            ci_pa: b,
            gross_assimilation_umol_co2_m2_leaf_s: carbon.ag,
            net_assimilation_umol_co2_m2_leaf_s: carbon.an,
            dark_respiration_umol_co2_m2_leaf_s: carbon.rd,
            gas_branch: if carbon.an <= 0.0 {
                V10LeafGasBranch::RespirationDominated
            } else {
                V10LeafGasBranch::PositiveAssimilation
            },
        });
    }
    let mut gas_branch = V10LeafGasBranch::PositiveAssimilation;
    if fa * fb > 0.0 {
        if !environment.authority.admits_nonpositive_assimilation() || fb >= 0.0 {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain("ci_bracket"));
        }
        let gs0_m_s =
            g0_umol_m2_s * 1.0e-6 * MOLAR_GAS_CONSTANT * temperature / environment.pressure_pa;
        if !gs0_m_s.is_finite() || gs0_m_s <= 0.0 {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "v10_low_light_stomatal_conductance",
            ));
        }
        let rb = 1.0 / gb_leaf;
        let ci_dark = environment.ca_pa
            + (1.4 * rb + 1.6 / gs0_m_s) * MOLAR_GAS_CONSTANT * temperature * rd * 1.0e-6;
        if !ci_dark.is_finite()
            || ci_dark <= environment.ca_pa
            || ci_dark >= environment.pressure_pa
        {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "v10_low_light_ci_dark",
            ));
        }
        let (f_dark, rs_dark) = residual(ci_dark)?;
        if f_dark < 0.0 {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "v10_low_light_dark_bracket",
            ));
        }
        a = environment.ca_pa;
        fa = fb;
        b = ci_dark;
        fb = f_dark;
        rs = rs_dark;
        gas_branch = V10LeafGasBranch::RespirationDominated;
    }
    let mut c = a;
    let mut fc = fa;
    let mut d = b - a;
    let mut mflag = true;
    for _ in 3..=64 {
        let mut s = if fa != fc && fb != fc {
            a * fb * fc / ((fa - fb) * (fa - fc))
                + b * fa * fc / ((fb - fa) * (fb - fc))
                + c * fa * fb / ((fc - fa) * (fc - fb))
        } else {
            b - fb * (b - a) / (fb - fa)
        };
        let left = ((3.0 * a + b) / 4.0).min(b);
        let right = ((3.0 * a + b) / 4.0).max(b);
        if !(left < s && s < right)
            || (mflag && (s - b).abs() >= (b - c).abs() / 2.0)
            || (!mflag && (s - b).abs() >= (c - d).abs() / 2.0)
            || (mflag && (b - c).abs() < 1.0e-6)
            || (!mflag && (c - d).abs() < 1.0e-6)
        {
            s = 0.5 * (a + b);
            mflag = true;
        } else {
            mflag = false;
        }
        let (fs, state_rs) = residual(s)?;
        rs = state_rs;
        d = c;
        c = b;
        fc = fb;
        if fa * fs < 0.0 {
            b = s;
            fb = fs;
        } else {
            a = s;
            fa = fs;
        }
        if fa.abs() < fb.abs() {
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut fa, &mut fb);
        }
        let scale = a.abs().max(b.abs()).max(1.0);
        if fb.abs() <= 1.0e-8 || (b - a).abs() <= 1.0e-6 + 1.0e-10 * scale {
            rs = residual(b)?.1;
            break;
        }
    }
    let carbon = carbon_at_ci(b)?;
    if gas_branch == V10LeafGasBranch::RespirationDominated
        && (carbon.ag.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater)
            || carbon.an > 0.0
            || b < environment.ca_pa
            || rs.to_bits()
                != (1.0
                    / (g0_umol_m2_s * 1.0e-6 * MOLAR_GAS_CONSTANT * temperature
                        / environment.pressure_pa))
                    .to_bits())
    {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "v10_low_light_accepted_branch",
        ));
    }
    Ok(LeafTrialState {
        surface_q: qsurface,
        rs_s_m: rs,
        ci_pa: b,
        gross_assimilation_umol_co2_m2_leaf_s: carbon.ag,
        net_assimilation_umol_co2_m2_leaf_s: carbon.an,
        dark_respiration_umol_co2_m2_leaf_s: carbon.rd,
        gas_branch,
    })
}

#[must_use]
fn vulnerability(potential: f64, p50: f64, exponent: f64) -> f64 {
    2.0_f64.powf(-(potential / p50).powf(exponent))
}

struct CoveredOccupancyTrialContext<'a> {
    column: &'a CoveredColumnInputs,
    canopy_air_temperature_k: f64,
    canopy_air_q: f64,
    component_longwave_w_m2: [f64; 4],
    caps: Option<&'a CoveredWaterCaps>,
    frozen: Option<&'a CoveredFrozenBranches>,
    liquid: CoveredLiquidPreparation,
}

fn covered_wet_flux(
    column: &CoveredColumnInputs,
    occupancy: &CoveredOccupancyInputs,
    liquid: CoveredLiquidPreparation,
    wet_surface_temperature_k: f64,
    canopy_air_temperature_k: f64,
    canopy_air_q: f64,
    frozen: Option<&CoveredFrozenBranches>,
) -> Result<(f64, WaterBranch), LandSurfaceEnergyError> {
    let rho = column.pressure_pa / (DRY_AIR_GAS_CONSTANT_J_KG_K * canopy_air_temperature_k);
    let wet_area = liquid.wet_fraction
        * (occupancy.sun.leaf_area_m2_m2_tile
            + occupancy.shade.leaf_area_m2_m2_tile
            + occupancy.stem_area_m2_m2_tile);
    let wet_potential = rho
        * occupancy.gb_wet_m_s
        * (canopy_saturation_q(wet_surface_temperature_k, column.pressure_pa)? - canopy_air_q)
        * wet_area;
    let wet_cap = liquid.preliminary_store / column.interval_s;
    let natural_branch = if wet_potential >= 0.0 && wet_cap <= wet_potential {
        WaterBranch::AuthorizationActiveOrTie
    } else if wet_potential < 0.0 {
        WaterBranch::Condensation
    } else {
        WaterBranch::ConstitutiveLaw
    };
    let branch = frozen
        .and_then(|value| value.wet.get(&occupancy.occupancy_id).copied())
        .unwrap_or(natural_branch);
    Ok((
        if branch == WaterBranch::AuthorizationActiveOrTie {
            wet_cap
        } else {
            wet_potential
        },
        branch,
    ))
}

fn evaluate_covered_occupancy(
    context: &CoveredOccupancyTrialContext<'_>,
    occupancy: &CoveredOccupancyInputs,
    block: &[f64],
) -> Result<CoveredOccupancyEvaluation, LandSurfaceEnergyError> {
    let column = context.column;
    let canopy_air_temperature_k = context.canopy_air_temperature_k;
    let canopy_air_q = context.canopy_air_q;
    let component_longwave_w_m2 = context.component_longwave_w_m2;
    if block.len() != 10 {
        return Err(LandSurfaceEnergyError::topology_domain(
            "covered_occupancy_trial",
        ));
    }
    let (psi_sun, psi_shade, psi_stem, psi_root) = (block[0], block[1], block[2], block[3]);
    let (beta_sun, beta_shade) = (block[4], block[5]);
    let (tsun, tshade, twet, tstem) = (block[6], block[7], block[8], block[9]);
    let rho = column.pressure_pa / (DRY_AIR_GAS_CONSTANT_J_KG_K * canopy_air_temperature_k);
    let wet_fraction = context.liquid.wet_fraction;
    let dry_sun = occupancy.sun.leaf_area_m2_m2_tile * (1.0 - wet_fraction);
    let dry_shade = occupancy.shade.leaf_area_m2_m2_tile * (1.0 - wet_fraction);
    let wet_area = wet_fraction
        * (occupancy.sun.leaf_area_m2_m2_tile
            + occupancy.shade.leaf_area_m2_m2_tile
            + occupancy.stem_area_m2_m2_tile);
    let dry_stem = (1.0 - wet_fraction) * occupancy.stem_area_m2_m2_tile;
    let leaf_gas_environment = LeafGasEnvironment {
        authority: column.authority,
        pressure_pa: column.pressure_pa,
        ca_pa: column.ca_pa,
    };
    let sun = leaf_trial_state(
        occupancy.sun,
        occupancy.biochemical,
        tsun,
        canopy_air_q,
        beta_sun,
        leaf_gas_environment,
        occupancy.gb_leaf_m_s,
        occupancy.g0_umol_m2_s,
        occupancy.medlyn_g1_kpa_sqrt,
    )?;
    let shade = leaf_trial_state(
        occupancy.shade,
        occupancy.biochemical,
        tshade,
        canopy_air_q,
        beta_shade,
        leaf_gas_environment,
        occupancy.gb_leaf_m_s,
        occupancy.g0_umol_m2_s,
        occupancy.medlyn_g1_kpa_sqrt,
    )?;
    // V8 maximum demand is an internal beta=1 evaluation at the current
    // leaf/canopy state. It is never a caller-configurable runtime operand.
    let sun_maximum = leaf_trial_state(
        occupancy.sun,
        occupancy.biochemical,
        tsun,
        canopy_air_q,
        1.0,
        leaf_gas_environment,
        occupancy.gb_leaf_m_s,
        occupancy.g0_umol_m2_s,
        occupancy.medlyn_g1_kpa_sqrt,
    )?;
    let shade_maximum = leaf_trial_state(
        occupancy.shade,
        occupancy.biochemical,
        tshade,
        canopy_air_q,
        1.0,
        leaf_gas_environment,
        occupancy.gb_leaf_m_s,
        occupancy.g0_umol_m2_s,
        occupancy.medlyn_g1_kpa_sqrt,
    )?;
    let emax_sun_kg_m2_s = rho * (sun_maximum.surface_q - canopy_air_q)
        / (1.0 / occupancy.gb_leaf_m_s + sun_maximum.rs_s_m)
        * dry_sun;
    let emax_shade_kg_m2_s = rho * (shade_maximum.surface_q - canopy_air_q)
        / (1.0 / occupancy.gb_leaf_m_s + shade_maximum.rs_s_m)
        * dry_shade;
    let sun_e =
        rho * (sun.surface_q - canopy_air_q) / (1.0 / occupancy.gb_leaf_m_s + sun.rs_s_m) * dry_sun;
    let shade_e = rho * (shade.surface_q - canopy_air_q)
        / (1.0 / occupancy.gb_leaf_m_s + shade.rs_s_m)
        * dry_shade;
    let (wet_e, wet_branch) = covered_wet_flux(
        column,
        occupancy,
        context.liquid,
        twet,
        canopy_air_temperature_k,
        canopy_air_q,
        context.frozen,
    )?;
    let q1sun = occupancy.k1_sun_max_s1
        * occupancy.sun.leaf_area_m2_m2_tile
        * vulnerability(
            psi_stem,
            occupancy.p50_xylem_mm,
            occupancy.vulnerability_exponent,
        )
        * (psi_stem - psi_sun);
    let q1shade = occupancy.k1_shade_max_s1
        * occupancy.shade.leaf_area_m2_m2_tile
        * vulnerability(
            psi_stem,
            occupancy.p50_xylem_mm,
            occupancy.vulnerability_exponent,
        )
        * (psi_stem - psi_shade);
    let q2 = occupancy.k2_max / occupancy.height_m
        * vulnerability(
            psi_root,
            occupancy.p50_xylem_mm,
            occupancy.vulnerability_exponent,
        )
        * occupancy.sai
        * (psi_root - psi_stem - 1000.0 * occupancy.height_m);
    let mut root_source_sum = 0.0;
    let mut source_water = Vec::with_capacity(occupancy.root_layers.len());
    let exact_inactive_hydraulics = exact_inactive_hydraulic_occupancy(column.authority, occupancy);
    for layer in &occupancy.root_layers {
        let law = if exact_inactive_hydraulics {
            0.0
        } else if layer.accessible && !layer.frozen && layer.root_fraction > 0.0 {
            let kr = occupancy.k3_max_m_s / layer.z3_m
                * vulnerability(
                    layer.soil_potential_mm,
                    occupancy.p50_root_mm,
                    occupancy.vulnerability_exponent,
                );
            let ks = layer.ksoil_m2_s / layer.dxroot_m;
            let series = kr * ks / (kr + ks);
            let rai =
                (occupancy.lai + occupancy.sai) * layer.root_fraction * occupancy.root_to_leaf_area;
            let flux = series * rai * (layer.soil_potential_mm - psi_root + layer.gravity_head_mm);
            if flux < 0.0 && column.authority == CoveredColumnAuthority::HistoricalV8 {
                return Err(LandSurfaceEnergyError::UnsupportedDomain(
                    "hydraulic_redistribution",
                ));
            }
            flux
        } else {
            0.0
        };
        let key = (occupancy.occupancy_id.clone(), layer.layer_id.clone());
        let supplied = context.caps.and_then(|value| value.root.get(&key));
        let cap_rate = supplied.map(|value| value.authorization_rate_kg_m2_tile_s);
        let natural_branch = if exact_inactive_hydraulics {
            WaterBranch::ConstitutiveLaw
        } else if cap_rate.is_some_and(|cap| cap <= law) {
            WaterBranch::AuthorizationActiveOrTie
        } else {
            WaterBranch::ConstitutiveLaw
        };
        let branch = context
            .frozen
            .and_then(|value| value.root.get(&key).copied())
            .unwrap_or(natural_branch);
        let final_flux = if branch == WaterBranch::AuthorizationActiveOrTie {
            cap_rate.ok_or(LandSurfaceEnergyError::water_cardinality(
                "frozen_root_cap_without_authorization",
            ))?
        } else {
            law
        };
        root_source_sum += final_flux;
        let request_rate = if exact_inactive_hydraulics {
            0.0
        } else {
            supplied.map_or(law.max(0.0), |value| value.request_rate_kg_m2_tile_s)
        };
        let request = request_rate * column.tile_fraction * column.interval_s;
        let authorization =
            cap_rate.map(|amount| amount * column.tile_fraction * column.interval_s);
        let finalized = if branch == WaterBranch::AuthorizationActiveOrTie {
            authorization.ok_or(LandSurfaceEnergyError::water_cardinality(
                "missing_root_authorization",
            ))?
        } else {
            final_flux.max(0.0) * column.tile_fraction * column.interval_s
        };
        source_water.push(SourceWaterFlux {
            occupancy_id: occupancy.occupancy_id.clone(),
            layer_id: layer.layer_id.clone(),
            law_kg_m2_tile_s: law,
            final_kg_m2_tile_s: final_flux,
            request_kg_m2_stand_ground: request,
            authorization_kg_m2_stand_ground: authorization,
            finalized_use_kg_m2_stand_ground: finalized,
            branch,
        });
    }
    let sun_h = rho
        * AIR_HEAT_CAPACITY_J_KG_K
        * occupancy.gb_leaf_m_s
        * dry_sun
        * (tsun - canopy_air_temperature_k);
    let shade_h = rho
        * AIR_HEAT_CAPACITY_J_KG_K
        * occupancy.gb_leaf_m_s
        * dry_shade
        * (tshade - canopy_air_temperature_k);
    let wet_h = rho
        * AIR_HEAT_CAPACITY_J_KG_K
        * occupancy.gb_wet_m_s
        * wet_area
        * (twet - canopy_air_temperature_k);
    let stem_h = rho
        * AIR_HEAT_CAPACITY_J_KG_K
        * occupancy.gb_stem_m_s
        * dry_stem
        * (tstem - canopy_air_temperature_k);
    let component_areas = [dry_sun, dry_shade, wet_area, dry_stem];
    let component_temperatures = [tsun, tshade, twet, tstem];
    let physical_energy_residuals = [
        occupancy.sun.absorbed_shortwave_w_m2_tile * (1.0 - wet_fraction)
            + component_longwave_w_m2[0]
            - sun_h
            - column.latent_heat_j_kg * sun_e,
        occupancy.shade.absorbed_shortwave_w_m2_tile * (1.0 - wet_fraction)
            + component_longwave_w_m2[1]
            - shade_h
            - column.latent_heat_j_kg * shade_e,
        wet_fraction
            * (occupancy.sun.absorbed_shortwave_w_m2_tile
                + occupancy.shade.absorbed_shortwave_w_m2_tile
                + occupancy.stem_absorbed_shortwave_w_m2_tile)
            + component_longwave_w_m2[2]
            - wet_h
            - column.latent_heat_j_kg * wet_e,
        (1.0 - wet_fraction) * occupancy.stem_absorbed_shortwave_w_m2_tile
            + component_longwave_w_m2[3]
            - stem_h,
    ];
    let v10_nonpositive_sun = column.authority.admits_nonpositive_assimilation()
        && matches!(
            sun.gas_branch,
            V10LeafGasBranch::ExactZeroPar | V10LeafGasBranch::RespirationDominated
        );
    let v10_nonpositive_shade = column.authority.admits_nonpositive_assimilation()
        && matches!(
            shade.gas_branch,
            V10LeafGasBranch::ExactZeroPar | V10LeafGasBranch::RespirationDominated
        );
    let wet_energy_tolerance = crate::physics::energy_tolerance(
        physical_energy_residuals[2].abs()
            + component_longwave_w_m2[2].abs()
            + wet_h.abs()
            + (column.latent_heat_j_kg * wet_e).abs(),
    );
    let v10_inactive_wet = column.authority.admits_nonpositive_assimilation()
        && (v10_nonpositive_sun || v10_nonpositive_shade)
        && context.caps.is_none()
        && wet_branch == WaterBranch::AuthorizationActiveOrTie
        && context.liquid.preliminary_store / column.interval_s
            <= crate::physics::water_tolerance(
                context.liquid.preliminary_store / column.interval_s,
            )
        && physical_energy_residuals[2].abs() <= wet_energy_tolerance;
    let energy_residuals: [f64; 4] = std::array::from_fn(|index| {
        if component_areas[index].to_bits() == 0.0_f64.to_bits() || (index == 2 && v10_inactive_wet)
        {
            component_temperatures[index]
                - inactive_component_temperature_anchor_k(index, canopy_air_temperature_k)
        } else {
            physical_energy_residuals[index]
        }
    });
    let v10_inactive_sun = column.authority.admits_nonpositive_assimilation()
        && occupancy.sun.leaf_area_m2_m2_tile == 0.0;
    let v10_inactive_shade = column.authority.admits_nonpositive_assimilation()
        && occupancy.shade.leaf_area_m2_m2_tile == 0.0;
    let residuals = vec![
        if v10_inactive_sun {
            psi_sun - psi_stem
        } else {
            sun_e - q1sun
        },
        if v10_inactive_shade {
            psi_shade - psi_stem
        } else {
            shade_e - q1shade
        },
        if v10_nonpositive_sun || v10_inactive_sun {
            beta_sun - 1.0
        } else {
            sun_e
                - emax_sun_kg_m2_s
                    * vulnerability(
                        psi_sun,
                        occupancy.p50_leaf_mm,
                        occupancy.vulnerability_exponent,
                    )
        },
        if v10_nonpositive_shade || v10_inactive_shade {
            beta_shade - 1.0
        } else {
            shade_e
                - emax_shade_kg_m2_s
                    * vulnerability(
                        psi_shade,
                        occupancy.p50_leaf_mm,
                        occupancy.vulnerability_exponent,
                    )
        },
        q1sun + q1shade - q2,
        q2 - root_source_sum,
        energy_residuals[0],
        energy_residuals[1],
        energy_residuals[2],
        energy_residuals[3],
    ];
    let water_scale = emax_sun_kg_m2_s
        .max(emax_shade_kg_m2_s)
        .max(q1sun.abs())
        .max(q1shade.abs())
        .max(q2.abs())
        .max(root_source_sum.abs());
    let component_operands = [
        occupancy.sun.absorbed_shortwave_w_m2_tile * (1.0 - wet_fraction),
        occupancy.shade.absorbed_shortwave_w_m2_tile * (1.0 - wet_fraction),
        wet_fraction
            * (occupancy.sun.absorbed_shortwave_w_m2_tile
                + occupancy.shade.absorbed_shortwave_w_m2_tile
                + occupancy.stem_absorbed_shortwave_w_m2_tile),
        (1.0 - wet_fraction) * occupancy.stem_absorbed_shortwave_w_m2_tile,
    ];
    let raw_sensible = [sun_h, shade_h, wet_h, stem_h];
    let sensible = std::array::from_fn(|index| {
        if component_areas[index].to_bits() == 0.0_f64.to_bits() {
            0.0
        } else {
            raw_sensible[index]
        }
    });
    let latent = [
        column.latent_heat_j_kg * sun_e,
        column.latent_heat_j_kg * shade_e,
        column.latent_heat_j_kg * wet_e,
        0.0,
    ];
    let mut tolerances = vec![crate::physics::water_tolerance(water_scale); 6];
    if v10_inactive_sun {
        tolerances[0] = 1.0e-7;
    }
    if v10_inactive_shade {
        tolerances[1] = 1.0e-7;
    }
    if v10_nonpositive_sun || v10_inactive_sun {
        tolerances[2] = 1.0e-8;
    }
    if v10_nonpositive_shade || v10_inactive_shade {
        tolerances[3] = 1.0e-8;
    }
    tolerances.extend((0..4).map(|index| {
        if component_areas[index].to_bits() == 0.0_f64.to_bits() {
            crate::physics::energy_tolerance(1.0)
        } else {
            crate::physics::energy_tolerance(
                component_operands[index].abs()
                    + component_longwave_w_m2[index].abs()
                    + sensible[index].abs()
                    + latent[index].abs(),
            )
        }
    }));
    let liquid = finalize_covered_liquid(
        context.liquid,
        wet_e * column.interval_s,
        twet,
        if context.caps.is_some() {
            CoveredLiquidPass::FixedAuthorizationFinal
        } else {
            CoveredLiquidPass::Potential
        },
    )?;
    let wet_surface_q = canopy_saturation_q(twet, column.pressure_pa)?;
    let surface_q = [sun.surface_q, shade.surface_q, wet_surface_q, canopy_air_q];
    let heat_conductance = [
        occupancy.gb_leaf_m_s * dry_sun,
        occupancy.gb_leaf_m_s * dry_shade,
        occupancy.gb_wet_m_s * wet_area,
        occupancy.gb_stem_m_s * dry_stem,
    ];
    let raw_vapor_flux = [sun_e, shade_e, wet_e, 0.0];
    let vapor_flux = std::array::from_fn(|index| {
        if component_areas[index].to_bits() == 0.0_f64.to_bits() {
            0.0
        } else {
            raw_vapor_flux[index]
        }
    });
    // Retain direct constitutive conductances. These operands are independent
    // of the accepted flux and therefore cannot pass validation merely by
    // dividing and multiplying the same result. The wet authorization branch
    // remains explicit in `wet_branch` and is validated against its liquid cap.
    let vapor_conductance = [
        dry_sun / (1.0 / occupancy.gb_leaf_m_s + sun.rs_s_m),
        dry_shade / (1.0 / occupancy.gb_leaf_m_s + shade.rs_s_m),
        occupancy.gb_wet_m_s * wet_area,
        0.0,
    ];
    Ok(CoveredOccupancyEvaluation {
        residuals,
        tolerances,
        source_water,
        canopy_sensible_w_m2: sensible.iter().sum(),
        canopy_vapor_kg_m2_s: vapor_flux.iter().sum(),
        wet_vapor_kg_m2_s: vapor_flux[2],
        wet_branch,
        component_temperatures_k: [tsun, tshade, twet, tstem],
        ci_pa: [sun.ci_pa, shade.ci_pa],
        gas_branches: [sun.gas_branch, shade.gas_branch],
        gross_assimilation_umol_co2_m2_leaf_s: [
            sun.gross_assimilation_umol_co2_m2_leaf_s,
            shade.gross_assimilation_umol_co2_m2_leaf_s,
        ],
        net_assimilation_umol_co2_m2_leaf_s: [
            sun.net_assimilation_umol_co2_m2_leaf_s,
            shade.net_assimilation_umol_co2_m2_leaf_s,
        ],
        dark_respiration_umol_co2_m2_leaf_s: [
            sun.dark_respiration_umol_co2_m2_leaf_s,
            shade.dark_respiration_umol_co2_m2_leaf_s,
        ],
        emax_kg_m2_s: [emax_sun_kg_m2_s, emax_shade_kg_m2_s],
        liquid,
        absorbed_shortwave_w_m2: component_operands,
        net_longwave_w_m2: component_longwave_w_m2,
        sensible_to_canopy_air_w_m2: sensible,
        signed_vapor_to_canopy_air_kg_m2_s: vapor_flux,
        component_areas_m2_m2_tile: component_areas,
        component_emissive_areas_m2_m2_tile: component_areas,
        component_heat_conductance_m_s_tile: heat_conductance,
        component_vapor_conductance_m_s_tile: vapor_conductance,
        component_vapor_authorization_kg_m2_tile_s: [
            None,
            None,
            (wet_branch == WaterBranch::AuthorizationActiveOrTie).then_some(wet_e),
            None,
        ],
        component_surface_specific_humidity_kg_kg: surface_q,
    })
}

pub fn evaluate_covered_occupancy_block(
    column: &CoveredColumnInputs,
    occupancy: &CoveredOccupancyInputs,
    block: &[f64],
    canopy_air_temperature_k: f64,
    canopy_air_q: f64,
    component_longwave_w_m2: [f64; 4],
) -> Result<Vec<f64>, LandSurfaceEnergyError> {
    let liquid = prepare_covered_liquid(occupancy, column.top_rain_kg_m2_tile)?;
    let context = CoveredOccupancyTrialContext {
        column,
        canopy_air_temperature_k,
        canopy_air_q,
        component_longwave_w_m2,
        caps: None,
        frozen: None,
        liquid,
    };
    Ok(evaluate_covered_occupancy(&context, occupancy, block)?.residuals)
}

fn validate_covered_caps(
    column: &CoveredColumnInputs,
    caps: Option<&CoveredWaterCaps>,
) -> Result<(), LandSurfaceEnergyError> {
    let Some(caps) = caps else {
        return Ok(());
    };
    let expected: BTreeSet<(String, String)> = column
        .occupancies
        .iter()
        .flat_map(|occupancy| {
            occupancy
                .root_layers
                .iter()
                .map(|layer| (occupancy.occupancy_id.clone(), layer.layer_id.clone()))
        })
        .collect();
    let actual: BTreeSet<_> = caps.root.keys().cloned().collect();
    if expected != actual {
        return Err(LandSurfaceEnergyError::water_identity(
            "covered_root_authorization_identity",
        ));
    }
    if caps
        .root
        .values()
        .chain(std::iter::once(&caps.ground))
        .any(|value| {
            !value.request_rate_kg_m2_tile_s.is_finite()
                || !value.authorization_rate_kg_m2_tile_s.is_finite()
        })
    {
        return Err(LandSurfaceEnergyError::water_domain(
            "covered_authorization_domain",
        ));
    }
    if caps
        .root
        .values()
        .chain(std::iter::once(&caps.ground))
        .any(|value| {
            value.authorization_rate_kg_m2_tile_s < 0.0
                || value.request_rate_kg_m2_tile_s < value.authorization_rate_kg_m2_tile_s
        })
    {
        return Err(LandSurfaceEnergyError::water_bound(
            "covered_authorization_domain",
        ));
    }
    Ok(())
}

pub(crate) fn covered_ground_uses_liquid_vapor_phase_domain(column: &CoveredColumnInputs) -> bool {
    column.stage3_lower_boundary.is_none()
        && !(column.ground.class == SurfaceClassKind::BareMineralSoil
            && column.ground.surface_liquid_kg_m2_tile == 0.0)
}

pub(crate) fn covered_trial_is_valid(
    trial: &[f64],
    occupancy_count: usize,
    ground_uses_liquid_vapor_phase_domain: bool,
) -> bool {
    if trial.len() < 10 * occupancy_count + 4 || trial.iter().any(|value| !value.is_finite()) {
        return false;
    }
    for index in 0..occupancy_count {
        let block = &trial[index * 10..(index + 1) * 10];
        if !(0.0..=1.0).contains(&block[4])
            || !(0.0..=1.0).contains(&block[5])
            // Sun leaf, shade leaf, and wet-surface vapor use the contract's
            // liquid saturation polynomial, whose exact domain begins at
            // 273.15 K. Reflect that constitutive bound in the trial predicate
            // so every covered Jacobian can select its deterministic inward
            // one-sided difference at the exact closed boundary.
            || block[6..9]
                .iter()
                .any(|value| !(LIQUID_VAPOR_PHASE_MINIMUM_K..=350.0).contains(value))
            || block[9..10]
                .iter()
                .any(|value| !(200.0..=350.0).contains(value))
        {
            return false;
        }
    }
    let common = &trial[10 * occupancy_count..];
    let ground_temperature_minimum = if ground_uses_liquid_vapor_phase_domain {
        LIQUID_VAPOR_PHASE_MINIMUM_K
    } else {
        200.0
    };
    (200.0..=350.0).contains(&common[0])
        && (0.0..=0.1).contains(&common[1])
        && (ground_temperature_minimum..=350.0).contains(&common[2])
        && common[3..]
            .iter()
            .all(|value| (200.0..=350.0).contains(value))
}

fn validate_covered_shortwave_inputs(
    column: &CoveredColumnInputs,
) -> Result<(), LandSurfaceEnergyError> {
    column.shortwave.incident_w_m2_tile.validate_nonnegative()?;
    column
        .shortwave
        .top_reflected_w_m2_tile
        .validate_nonnegative()?;
    column
        .shortwave
        .ground_absorbed_by_incident_w_m2_tile
        .validate_nonnegative()?;
    column
        .ground
        .terminal_shortwave_w_m2_tile
        .validate_nonnegative()?;
    if column.shortwave.occupancies.len() != column.occupancies.len() {
        return Err(LandSurfaceEnergyError::topology_cardinality(
            "covered shortwave occupancy set",
        ));
    }
    for (radiation, occupancy) in column.shortwave.occupancies.iter().zip(&column.occupancies) {
        radiation
            .sun_leaf_absorbed_w_m2_tile
            .validate_nonnegative()?;
        radiation
            .shade_leaf_absorbed_w_m2_tile
            .validate_nonnegative()?;
        radiation.stem_absorbed_w_m2_tile.validate_nonnegative()?;
        if radiation.occupancy_id != occupancy.occupancy_id
            || radiation.sun_leaf_absorbed_w_m2_tile.total().to_bits()
                != occupancy.sun.absorbed_shortwave_w_m2_tile.to_bits()
            || radiation.shade_leaf_absorbed_w_m2_tile.total().to_bits()
                != occupancy.shade.absorbed_shortwave_w_m2_tile.to_bits()
            || radiation.stem_absorbed_w_m2_tile.total().to_bits()
                != occupancy.stem_absorbed_shortwave_w_m2_tile.to_bits()
        {
            return Err(LandSurfaceEnergyError::ComponentClosure(
                "covered bound shortwave component identity",
            ));
        }
    }
    let incident = directional_values(column.shortwave.incident_w_m2_tile);
    let reflected = directional_values(column.shortwave.top_reflected_w_m2_tile);
    let ground_absorbed = if column.authority == CoveredColumnAuthority::V11SnowCovered {
        let optical =
            column
                .stage3_optical
                .as_ref()
                .ok_or(LandSurfaceEnergyError::StateLineage(
                    "missing Stage-3 snow optical boundary",
                ))?;
        optical.validate()?;
        let expected = directional_values(optical.absorbed_w_m2_tile);
        let actual = directional_values(column.shortwave.ground_absorbed_by_incident_w_m2_tile);
        if expected
            .iter()
            .zip(actual)
            .any(|(expected, actual)| expected.to_bits() != actual.to_bits())
        {
            return Err(LandSurfaceEnergyError::ComponentClosure(
                "Stage-3 snow optical/column absorption",
            ));
        }
        expected
    } else {
        directional_values(column.shortwave.ground_absorbed_by_incident_w_m2_tile)
    };
    for direction in 0..4 {
        let canopy_absorbed: f64 = column
            .shortwave
            .occupancies
            .iter()
            .map(|occupancy| {
                directional_values(occupancy.sun_leaf_absorbed_w_m2_tile)[direction]
                    + directional_values(occupancy.shade_leaf_absorbed_w_m2_tile)[direction]
                    + directional_values(occupancy.stem_absorbed_w_m2_tile)[direction]
            })
            .sum();
        let residual = incident[direction]
            - reflected[direction]
            - canopy_absorbed
            - ground_absorbed[direction];
        if residual.abs()
            > energy_tolerance(
                incident[direction].abs()
                    + reflected[direction].abs()
                    + canopy_absorbed.abs()
                    + ground_absorbed[direction].abs(),
            )
        {
            return Err(LandSurfaceEnergyError::ComponentClosure(
                "covered whole-column band/direction shortwave",
            ));
        }
    }
    Ok(())
}

fn directional_values(value: BandDirectionalFluxes) -> [f64; 4] {
    [
        value.direct_vis,
        value.diffuse_vis,
        value.direct_nir,
        value.diffuse_nir,
    ]
}

pub fn evaluate_covered_column(
    column: &CoveredColumnInputs,
    trial: &[f64],
    caps: Option<&CoveredWaterCaps>,
    frozen: Option<&CoveredFrozenBranches>,
) -> Result<CoveredColumnEvaluation, LandSurfaceEnergyError> {
    if column.occupancies.is_empty() || column.ground.soil_nodes.is_empty() {
        return Err(LandSurfaceEnergyError::topology_cardinality(
            "covered_column",
        ));
    }
    validate_covered_caps(column, caps)?;
    validate_covered_shortwave_inputs(column)?;
    let stage3_boundary =
        if column.authority == CoveredColumnAuthority::V11SnowCovered {
            let boundary = column.stage3_lower_boundary.as_ref().ok_or(
                LandSurfaceEnergyError::StateLineage("missing Stage-3 covered lower boundary"),
            )?;
            boundary.validate()?;
            let optical =
                column
                    .stage3_optical
                    .as_ref()
                    .ok_or(LandSurfaceEnergyError::StateLineage(
                        "missing Stage-3 snow optical boundary",
                    ))?;
            if optical.snow_vis_albedo.to_bits() != boundary.snow_vis_albedo.to_bits()
                || optical.snow_nir_albedo.to_bits() != boundary.snow_nir_albedo.to_bits()
                || optical.stage3_albedo_state_sha256 != boundary.stage3_albedo_state_sha256
                || optical.forcing_receipt_sha256 != boundary.forcing_receipt_sha256
            {
                return Err(LandSurfaceEnergyError::StateLineage(
                    "Stage-3 snow optical/lower-boundary identity",
                ));
            }
            Some(boundary)
        } else {
            None
        };
    let expected = 10 * column.occupancies.len() + 3 + column.ground.soil_nodes.len();
    if trial.len() != expected
        || !covered_trial_is_valid(
            trial,
            column.occupancies.len(),
            covered_ground_uses_liquid_vapor_phase_domain(column),
        )
    {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "covered_trial_shape_or_bounds",
        ));
    }
    let common_offset = 10 * column.occupancies.len();
    let canopy_temperature = trial[common_offset];
    let canopy_q = trial[common_offset + 1];
    let ground_temperature = trial[common_offset + 2];
    let soil_temperatures = &trial[common_offset + 3..];
    if !column.top_rain_kg_m2_tile.is_finite() || column.top_rain_kg_m2_tile < 0.0 {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "covered top rain",
        ));
    }
    let mut incident_rain = column.top_rain_kg_m2_tile;
    let mut ground_stemflow = 0.0;
    let mut liquid_preparations = Vec::with_capacity(column.occupancies.len());
    let mut routed_liquid = Vec::with_capacity(column.occupancies.len());
    for (index, occupancy) in column.occupancies.iter().enumerate() {
        let block = &trial[index * 10..(index + 1) * 10];
        let preparation = prepare_covered_liquid(occupancy, incident_rain)?;
        let (wet_flux, _) = covered_wet_flux(
            column,
            occupancy,
            preparation,
            block[8],
            canopy_temperature,
            canopy_q,
            frozen,
        )?;
        let liquid = finalize_covered_liquid(
            preparation,
            wet_flux * column.interval_s,
            block[8],
            if caps.is_some() {
                CoveredLiquidPass::FixedAuthorizationFinal
            } else {
                CoveredLiquidPass::Potential
            },
        )?;
        incident_rain = liquid.throughfall_kg_m2_tile
            + liquid.initial_drainage_kg_m2_tile
            + liquid.second_drainage_kg_m2_tile;
        ground_stemflow += liquid.stemflow_kg_m2_tile;
        liquid_preparations.push(preparation);
        routed_liquid.push(liquid);
    }
    let longwave_layers: Vec<_> = column
        .occupancies
        .iter()
        .zip(&liquid_preparations)
        .enumerate()
        .map(|(index, (occupancy, liquid))| {
            let block = &trial[index * 10..(index + 1) * 10];
            let dry_sun = occupancy.sun.leaf_area_m2_m2_tile * (1.0 - liquid.wet_fraction);
            let dry_shade = occupancy.shade.leaf_area_m2_m2_tile * (1.0 - liquid.wet_fraction);
            let wet = liquid.wet_fraction
                * (occupancy.sun.leaf_area_m2_m2_tile
                    + occupancy.shade.leaf_area_m2_m2_tile
                    + occupancy.stem_area_m2_m2_tile);
            crate::physics::CanopyLongwaveLayer {
                clumping_index: occupancy.clumping_index,
                leaf_area_index: occupancy.lai,
                stem_area_index: occupancy.sai,
                component_areas: [
                    dry_sun,
                    dry_shade,
                    wet,
                    (1.0 - liquid.wet_fraction) * occupancy.stem_area_m2_m2_tile,
                ],
                component_temperatures_k: [block[6], block[7], block[8], block[9]],
            }
        })
        .collect();
    let longwave = crate::physics::reciprocal_longwave_column(
        column.atmospheric_downward_longwave_w_m2,
        stage3_boundary.map_or(ground_temperature, |boundary| boundary.snow_temperature_k),
        &longwave_layers,
    )?;
    let mut occupancy_results = Vec::with_capacity(column.occupancies.len());
    let mut raw = Vec::new();
    let mut tolerances = Vec::new();
    for (index, occupancy) in column.occupancies.iter().enumerate() {
        let context = CoveredOccupancyTrialContext {
            column,
            canopy_air_temperature_k: canopy_temperature,
            canopy_air_q: canopy_q,
            component_longwave_w_m2: longwave.component_net_w_m2[index],
            caps,
            frozen,
            liquid: liquid_preparations[index],
        };
        let result =
            evaluate_covered_occupancy(&context, occupancy, &trial[index * 10..(index + 1) * 10])?;
        if result.liquid != routed_liquid[index] {
            return Err(LandSurfaceEnergyError::OwnerEnvelope(
                "covered E04 routed/final evaluation mismatch",
            ));
        }
        raw.extend(result.residuals.iter().copied());
        tolerances.extend(result.tolerances.iter().copied());
        occupancy_results.push(result);
    }
    let rho = column.pressure_pa / (DRY_AIR_GAS_CONSTANT_J_KG_K * canopy_temperature);
    let ground_resistance = crate::physics::under_canopy_neutral_resistance(
        column.under_canopy_geometry,
        column.reference_wind_m_s,
    )?;
    let ground = &column.ground;
    let (ground_law, _) = if stage3_boundary.is_some() {
        (0.0, None)
    } else if ground.class == SurfaceClassKind::BareMineralSoil
        && ground.surface_liquid_kg_m2_tile == 0.0
    {
        let parameters = ground
            .bare_soil
            .ok_or(LandSurfaceEnergyError::ConstitutiveDomain(
                "missing_covered_bare_soil_parameters",
            ))?;
        let detail = bare_soil_vapor(BareSoilVaporOperands {
            top_layer_liquid_kg_m2: parameters.top_layer_liquid_kg_m2,
            top_layer_ice_kg_m2: parameters.top_layer_ice_kg_m2,
            top_layer_depth_m: ground.soil_nodes[0].depth_m,
            porosity: parameters.porosity,
            saturated_matric_potential_mm: parameters.saturated_matric_potential_mm,
            clapp_hornberger_b: parameters.clapp_hornberger_b,
            theta_initial: parameters.theta_initial,
            surface_temperature_k: ground_temperature,
            recipient_specific_humidity_kg_kg: canopy_q,
            pressure_pa: column.pressure_pa,
            aerodynamic_vapor_resistance_s_m: ground_resistance.resistance_s_m,
            moist_air_density_kg_m3: rho,
        })?;
        (detail.signed_flux_kg_m2_s, Some(detail))
    } else {
        let humidity = match ground.class {
            SurfaceClassKind::BareMineralSoil => 1.0,
            SurfaceClassKind::ForestLitter => litter_relative_humidity(
                ground.surface_liquid_kg_m2_tile,
                ground.litter_capacity_kg_m2_tile.ok_or(
                    LandSurfaceEnergyError::ConstitutiveDomain("covered_litter_capacity"),
                )?,
            )?,
        };
        let saturated = canopy_saturation_q(ground_temperature, column.pressure_pa)?;
        let surface_q = humidity * saturated + (1.0 - humidity) * canopy_q;
        (
            rho * (surface_q - canopy_q) / ground_resistance.resistance_s_m,
            None,
        )
    };
    let natural_ground_branch = if ground_law < 0.0 {
        WaterBranch::Condensation
    } else if caps.is_some_and(|value| value.ground.authorization_rate_kg_m2_tile_s <= ground_law) {
        WaterBranch::AuthorizationActiveOrTie
    } else {
        WaterBranch::ConstitutiveLaw
    };
    let ground_branch = frozen
        .and_then(|value| value.ground)
        .unwrap_or(natural_ground_branch);
    let final_ground_vapor = if stage3_boundary.is_some() {
        0.0
    } else if ground_branch == WaterBranch::AuthorizationActiveOrTie {
        caps.ok_or(LandSurfaceEnergyError::water_cardinality(
            "frozen_ground_cap_without_authorization",
        ))?
        .ground
        .authorization_rate_kg_m2_tile_s
    } else {
        ground_law
    };
    let ending_water = if stage3_boundary.is_none() {
        let uses_store = !(ground.class == SurfaceClassKind::BareMineralSoil
            && ground.surface_liquid_kg_m2_tile == 0.0);
        let ending_water = if uses_store {
            ground.surface_liquid_kg_m2_tile - final_ground_vapor.max(0.0) * column.interval_s
                + (-final_ground_vapor).max(0.0) * column.interval_s
        } else {
            ground.surface_liquid_kg_m2_tile
        };
        if ending_water < -1.0e-14 {
            return Err(LandSurfaceEnergyError::water_bound(
                "covered_surface_water_negative",
            ));
        }
        ending_water
    } else {
        0.0
    };
    let ground_sensible = if stage3_boundary.is_some() {
        0.0
    } else {
        rho * AIR_HEAT_CAPACITY_J_KG_K * (ground_temperature - canopy_temperature)
            / ground_resistance.resistance_s_m
    };
    let lower_boundary_sensible = stage3_boundary.map_or(ground_sensible, |boundary| {
        boundary.sensible_to_canopy_air_w_m2
    });
    let lower_boundary_vapor = stage3_boundary.map_or(final_ground_vapor, |boundary| {
        boundary.vapor_to_canopy_air_kg_m2_s
    });
    let reference_heat =
        rho * AIR_HEAT_CAPACITY_J_KG_K * (canopy_temperature - column.air_temperature_k)
            / column.canopy_to_atmosphere_heat_resistance_s_m;
    let reference_vapor = rho * (canopy_q - column.air_specific_humidity_kg_kg)
        / column.canopy_to_atmosphere_vapor_resistance_s_m;
    let canopy_sensible: f64 = occupancy_results
        .iter()
        .map(|value| value.canopy_sensible_w_m2)
        .sum();
    let canopy_vapor: f64 = occupancy_results
        .iter()
        .map(|value| value.canopy_vapor_kg_m2_s)
        .sum();
    let shared_heat = canopy_sensible + lower_boundary_sensible - reference_heat;
    let shared_vapor = canopy_vapor + lower_boundary_vapor - reference_vapor;
    let shared_heat_scale =
        (canopy_sensible.abs() + ground_sensible.abs() + reference_heat.abs()).max(1.0);
    let shared_vapor_scale = canopy_vapor
        .abs()
        .max(final_ground_vapor.abs())
        .max(reference_vapor.abs());
    let shared_heat_tolerance = crate::physics::energy_tolerance(shared_heat_scale);
    let shared_vapor_tolerance = crate::physics::water_tolerance(shared_vapor_scale);
    raw.extend([shared_heat, shared_vapor]);
    tolerances.extend([shared_heat_tolerance, shared_vapor_tolerance]);
    let (shortwave, ground_vapor_energy, ground_storage, ending_enthalpy, ground_heat) =
        if stage3_boundary.is_some() {
            (
                crate::physics::ShortwavePartition {
                    absorbed: BandDirectionalFluxes::default(),
                    reflected: BandDirectionalFluxes::default(),
                },
                0.0,
                0.0,
                ground.surface_enthalpy_j_m2_tile,
                vec![0.0; ground.soil_nodes.len()],
            )
        } else {
            let shortwave = partition_ground_shortwave(
                ground.terminal_shortwave_w_m2_tile,
                ground.surface_vis_albedo,
                ground.surface_nir_albedo,
            )?;
            let ground_vapor_energy = vapor_export_w_m2(final_ground_vapor, ground_temperature)?;
            let (ground_storage, ending_enthalpy, beginning_ground_temperature) = match ground
                .storage_branch
            {
                SurfaceStorageBranch::FiniteCapacity => {
                    let ending_capacity = ground.surface_dry_heat_capacity_j_m2_k
                        + ending_water.max(0.0) * WATER_HEAT_CAPACITY_J_KG_K;
                    let ending = ending_capacity * (ground_temperature - REFERENCE_TEMPERATURE_K);
                    let beginning_capacity = ground.surface_dry_heat_capacity_j_m2_k
                        + ground.surface_liquid_kg_m2_tile * WATER_HEAT_CAPACITY_J_KG_K;
                    (
                        (ending - ground.surface_enthalpy_j_m2_tile) / column.interval_s,
                        ending,
                        REFERENCE_TEMPERATURE_K
                            + ground.surface_enthalpy_j_m2_tile / beginning_capacity,
                    )
                }
                SurfaceStorageBranch::EquilibriumZero => (0.0, 0.0, ground_temperature),
            };
            let first = &ground.soil_nodes[0];
            let surface_conductance = harmonic_interface_conductance_w_m2_k(
                ground.surface_depth_m,
                ground.surface_conductivity_w_m_k,
                first.depth_m,
                first.conductivity_w_m_k,
            )?;
            let mut begin_fluxes = vec![
                surface_conductance
                    * (beginning_ground_temperature - first.beginning_temperature_k),
            ];
            let mut end_fluxes =
                vec![surface_conductance * (ground_temperature - soil_temperatures[0])];
            for index in 0..ground.soil_nodes.len().saturating_sub(1) {
                let upper = &ground.soil_nodes[index];
                let lower = &ground.soil_nodes[index + 1];
                let conductance = harmonic_interface_conductance_w_m2_k(
                    upper.depth_m,
                    upper.conductivity_w_m_k,
                    lower.depth_m,
                    lower.conductivity_w_m_k,
                )?;
                begin_fluxes.push(
                    conductance * (upper.beginning_temperature_k - lower.beginning_temperature_k),
                );
                end_fluxes
                    .push(conductance * (soil_temperatures[index] - soil_temperatures[index + 1]));
            }
            let ground_heat: Vec<f64> = begin_fluxes
                .iter()
                .zip(end_fluxes.iter())
                .map(|(begin, end)| 0.5 * (begin + end))
                .collect();
            (
                shortwave,
                ground_vapor_energy,
                ground_storage,
                ending_enthalpy,
                ground_heat,
            )
        };
    if let Some(boundary) = stage3_boundary {
        raw.push(ground_temperature - boundary.snow_temperature_k);
        tolerances.push(1.0e-9);
        for (temperature, node) in soil_temperatures.iter().zip(&ground.soil_nodes) {
            raw.push(temperature - node.beginning_temperature_k);
            tolerances.push(1.0e-9);
        }
    } else {
        let surface_operands = [
            shortwave.absorbed.total(),
            longwave.ground_net_w_m2,
            -ground_sensible,
            -ground_vapor_energy,
            -ground_heat[0],
            -ground_storage,
        ];
        let surface_residual: f64 = surface_operands.iter().sum();
        raw.push(surface_residual);
        tolerances.push(crate::physics::energy_tolerance(
            surface_operands.iter().map(|value| value.abs()).sum(),
        ));
        for (index, node) in ground.soil_nodes.iter().enumerate() {
            let incoming = ground_heat[index];
            let outgoing = ground_heat.get(index + 1).copied().unwrap_or(0.0);
            let storage = node.heat_capacity_j_m2_k
                * (soil_temperatures[index] - node.beginning_temperature_k)
                / column.interval_s;
            raw.push(incoming - outgoing - storage);
            tolerances.push(crate::physics::energy_tolerance(
                incoming.abs() + outgoing.abs() + storage.abs(),
            ));
        }
    }
    let normalized_residuals = raw
        .iter()
        .zip(tolerances.iter())
        .map(|(residual, tolerance)| residual / tolerance)
        .collect();
    let ground_authorization = caps.map(|value| {
        value.ground.authorization_rate_kg_m2_tile_s * column.tile_fraction * column.interval_s
    });
    let ground_finalized = if stage3_boundary.is_some() {
        0.0
    } else if ground_branch == WaterBranch::AuthorizationActiveOrTie {
        ground_authorization.ok_or(LandSurfaceEnergyError::water_cardinality(
            "missing_ground_authorization",
        ))?
    } else {
        final_ground_vapor.max(0.0) * column.tile_fraction * column.interval_s
    };
    Ok(CoveredColumnEvaluation {
        raw_residuals: raw,
        normalized_residuals,
        tolerances,
        occupancies: occupancy_results,
        canopy_air_temperature_k: canopy_temperature,
        canopy_air_specific_humidity_kg_kg: canopy_q,
        ground_temperature_k: ground_temperature,
        soil_temperature_k: soil_temperatures.to_vec(),
        ground_water: GroundWaterFlux {
            law_kg_m2_tile_s: ground_law,
            final_kg_m2_tile_s: final_ground_vapor,
            request_kg_m2_stand_ground: caps.map_or(ground_law.max(0.0), |value| {
                value.ground.request_rate_kg_m2_tile_s
            }) * column.tile_fraction
                * column.interval_s,
            authorization_kg_m2_stand_ground: ground_authorization,
            finalized_use_kg_m2_stand_ground: ground_finalized,
            condensation_credit_kg_m2_stand_ground: (-final_ground_vapor).max(0.0)
                * column.tile_fraction
                * column.interval_s,
            branch: ground_branch,
        },
        ground_heat_cn_w_m2_tile: ground_heat,
        ground_storage_w_m2_tile: ground_storage,
        ending_surface_enthalpy_j_m2_tile: ending_enthalpy,
        whole_column_longwave: longwave,
        ground_canopy_release_kg_m2_tile: incident_rain,
        ground_stemflow_kg_m2_tile: ground_stemflow,
        ground_sensible_to_canopy_air_w_m2: lower_boundary_sensible,
        lower_boundary_vapor_to_canopy_air_kg_m2_s: lower_boundary_vapor,
        canopy_sensible_w_m2: canopy_sensible,
        canopy_vapor_kg_m2_s: canopy_vapor,
        sensible_to_reference_air_w_m2: reference_heat,
        vapor_to_reference_air_kg_m2_s: reference_vapor,
        shared_heat_residual_w_m2: shared_heat,
        shared_heat_tolerance_w_m2: shared_heat_tolerance,
        shared_vapor_residual_kg_m2_s: shared_vapor,
        shared_vapor_tolerance_kg_m2_s: shared_vapor_tolerance,
    })
}

pub(crate) fn evaluate_covered_column_v3(
    column: &CoveredColumnInputs,
    trial: &[f64],
    caps: Option<&CoveredWaterCaps>,
    frozen: Option<&CoveredFrozenBranches>,
    context: crate::V3LitterResidualContext,
) -> Result<crate::V3PhaseFreeCoveredEvaluation, LandSurfaceEnergyError> {
    if column.ground.class != SurfaceClassKind::ForestLitter
        || column.ground.storage_branch != SurfaceStorageBranch::FiniteCapacity
        || column.stage3_lower_boundary.is_some()
        || column.ground.surface_liquid_kg_m2_tile.to_bits()
            != context.beginning.liquid_kg_m2_tile.to_bits()
        || column.ground.surface_enthalpy_j_m2_tile.to_bits()
            != context.beginning.sensible_energy_j_m2_tile.to_bits()
    {
        return Err(LandSurfaceEnergyError::FrozenLitterV3Identity(
            "V3 covered-column predecessor/state join",
        ));
    }
    crate::validate_litter_phase_configuration(context.configuration)?;
    crate::validate_beginning_litter_state(context.configuration, context.beginning)?;
    if column.interval_s.to_bits() != column.ground.interval_s.to_bits() {
        return Err(LandSurfaceEnergyError::FrozenLitterTransaction(
            "V3 column/ground support mismatch",
        ));
    }

    let mut detail = evaluate_covered_column(column, trial, caps, frozen)?;
    let rho = column.pressure_pa / (DRY_AIR_GAS_CONSTANT_J_KG_K * detail.canopy_air_temperature_k);
    let resistance = crate::physics::under_canopy_neutral_resistance(
        column.under_canopy_geometry,
        column.reference_wind_m_s,
    )?;
    let environment = crate::LitterVaporEnvironment {
        accepted_phase_free_temperature_k: detail.ground_temperature_k,
        air_density_kg_m3: rho,
        air_pressure_pa: column.pressure_pa,
        recipient_specific_humidity_kg_kg: detail.canopy_air_specific_humidity_kg_kg,
        litter_to_canopy_resistance_s_m: resistance.resistance_s_m,
    };
    let raw =
        crate::evaluate_raw_litter_vapor(context.configuration, context.beginning, environment)?;
    let locally_bounded = crate::FinalizedLitterVapor {
        liquid_signed_rate_kg_m2_s: if raw.raw_liquid_signed_rate_kg_m2_s > 0.0 {
            raw.raw_liquid_signed_rate_kg_m2_s
                .min(context.beginning.liquid_kg_m2_tile / column.interval_s)
        } else {
            raw.raw_liquid_signed_rate_kg_m2_s
        },
        ice_signed_rate_kg_m2_s: if raw.raw_ice_signed_rate_kg_m2_s > 0.0 {
            raw.raw_ice_signed_rate_kg_m2_s
                .min(context.beginning.ice_kg_m2_tile / column.interval_s)
        } else {
            raw.raw_ice_signed_rate_kg_m2_s
        },
    };
    let finalized = context
        .finalized_vapor
        .map_or(locally_bounded, |authorization| {
            crate::FinalizedLitterVapor {
                liquid_signed_rate_kg_m2_s: if raw.raw_liquid_signed_rate_kg_m2_s > 0.0 {
                    raw.raw_liquid_signed_rate_kg_m2_s
                        .min(authorization.liquid_signed_rate_kg_m2_s)
                        .min(context.beginning.liquid_kg_m2_tile / column.interval_s)
                } else {
                    raw.raw_liquid_signed_rate_kg_m2_s
                },
                ice_signed_rate_kg_m2_s: if raw.raw_ice_signed_rate_kg_m2_s > 0.0 {
                    raw.raw_ice_signed_rate_kg_m2_s
                        .min(authorization.ice_signed_rate_kg_m2_s)
                        .min(context.beginning.ice_kg_m2_tile / column.interval_s)
                } else {
                    raw.raw_ice_signed_rate_kg_m2_s
                },
            }
        });
    let vapor = crate::finalize_litter_vapor(
        raw,
        finalized,
        context.beginning,
        detail.ground_temperature_k,
        column.interval_s,
    )?;
    let post_vapor = crate::install_finalized_vapor(
        context.configuration,
        context.beginning,
        detail.ground_temperature_k,
        vapor,
    )?;
    let storage = (post_vapor.sensible_energy_j_m2_tile
        - context.beginning.sensible_energy_j_m2_tile)
        / column.interval_s;
    let liquid_vapor_energy = vapor.liquid_signed_energy_j_m2 / column.interval_s;
    let ice_vapor_energy = vapor.ice_signed_energy_j_m2 / column.interval_s;
    let beginning_capacity = context.configuration.dry_heat_capacity_j_m2_k
        + context.beginning.liquid_kg_m2_tile * WATER_HEAT_CAPACITY_J_KG_K
        + context.beginning.ice_kg_m2_tile * crate::LITTER_ICE_HEAT_CAPACITY_J_KG_K;
    let beginning_temperature =
        REFERENCE_TEMPERATURE_K + context.beginning.sensible_energy_j_m2_tile / beginning_capacity;
    let first = &column.ground.soil_nodes[0];
    let surface_conductance = harmonic_interface_conductance_w_m2_k(
        column.ground.surface_depth_m,
        column.ground.surface_conductivity_w_m_k,
        first.depth_m,
        first.conductivity_w_m_k,
    )?;
    let mut beginning_fluxes =
        vec![surface_conductance * (beginning_temperature - first.beginning_temperature_k)];
    let mut ending_fluxes =
        vec![surface_conductance * (detail.ground_temperature_k - detail.soil_temperature_k[0])];
    for index in 0..column.ground.soil_nodes.len().saturating_sub(1) {
        let upper = &column.ground.soil_nodes[index];
        let lower = &column.ground.soil_nodes[index + 1];
        let conductance = harmonic_interface_conductance_w_m2_k(
            upper.depth_m,
            upper.conductivity_w_m_k,
            lower.depth_m,
            lower.conductivity_w_m_k,
        )?;
        beginning_fluxes
            .push(conductance * (upper.beginning_temperature_k - lower.beginning_temperature_k));
        ending_fluxes.push(
            conductance * (detail.soil_temperature_k[index] - detail.soil_temperature_k[index + 1]),
        );
    }
    let ground_heat: Vec<f64> = beginning_fluxes
        .iter()
        .zip(&ending_fluxes)
        .map(|(beginning, ending)| 0.5 * (beginning + ending))
        .collect();

    let shortwave = partition_ground_shortwave(
        column.ground.terminal_shortwave_w_m2_tile,
        column.ground.surface_vis_albedo,
        column.ground.surface_nir_albedo,
    )?;
    let absorbed_shortwave = shortwave.absorbed.total();
    let net_longwave = detail.whole_column_longwave.ground_net_w_m2;
    let surface_components = [
        absorbed_shortwave,
        net_longwave,
        -detail.ground_sensible_to_canopy_air_w_m2,
        -liquid_vapor_energy,
        -ice_vapor_energy,
        -ground_heat[0],
        -storage,
    ];
    let surface_residual = surface_components.iter().sum::<f64>();
    let surface_tolerance = energy_tolerance(
        surface_components
            .iter()
            .map(|component| component.abs())
            .sum(),
    );
    let occupancy_count = column.occupancies.len();
    let shared_vapor_index = 10 * occupancy_count + 1;
    let surface_index = 10 * occupancy_count + 2;
    let total_vapor_rate = finalized.liquid_signed_rate_kg_m2_s + finalized.ice_signed_rate_kg_m2_s;
    let shared_vapor =
        detail.canopy_vapor_kg_m2_s + total_vapor_rate - detail.vapor_to_reference_air_kg_m2_s;
    let shared_vapor_tolerance = crate::physics::water_tolerance(
        detail
            .canopy_vapor_kg_m2_s
            .abs()
            .max(total_vapor_rate.abs())
            .max(detail.vapor_to_reference_air_kg_m2_s.abs()),
    );
    detail.raw_residuals[shared_vapor_index] = shared_vapor;
    detail.tolerances[shared_vapor_index] = shared_vapor_tolerance;
    detail.normalized_residuals[shared_vapor_index] = shared_vapor / shared_vapor_tolerance;
    detail.raw_residuals[surface_index] = surface_residual;
    detail.tolerances[surface_index] = surface_tolerance;
    detail.normalized_residuals[surface_index] = surface_residual / surface_tolerance;
    for (index, node) in column.ground.soil_nodes.iter().enumerate() {
        let incoming = ground_heat[index];
        let outgoing = ground_heat.get(index + 1).copied().unwrap_or(0.0);
        let node_storage = node.heat_capacity_j_m2_k
            * (detail.soil_temperature_k[index] - node.beginning_temperature_k)
            / column.interval_s;
        let residual = incoming - outgoing - node_storage;
        let tolerance = energy_tolerance(incoming.abs() + outgoing.abs() + node_storage.abs());
        let residual_index = surface_index + 1 + index;
        detail.raw_residuals[residual_index] = residual;
        detail.tolerances[residual_index] = tolerance;
        detail.normalized_residuals[residual_index] = residual / tolerance;
    }
    detail.ground_heat_cn_w_m2_tile = ground_heat;
    detail.ground_storage_w_m2_tile = storage;
    detail.ending_surface_enthalpy_j_m2_tile = post_vapor.sensible_energy_j_m2_tile;
    detail.lower_boundary_vapor_to_canopy_air_kg_m2_s = total_vapor_rate;
    detail.shared_vapor_residual_kg_m2_s = shared_vapor;
    detail.shared_vapor_tolerance_kg_m2_s = shared_vapor_tolerance;
    detail.ground_water = GroundWaterFlux {
        law_kg_m2_tile_s: raw.raw_liquid_signed_rate_kg_m2_s + raw.raw_ice_signed_rate_kg_m2_s,
        final_kg_m2_tile_s: total_vapor_rate,
        request_kg_m2_stand_ground: (raw.raw_liquid_signed_rate_kg_m2_s.max(0.0)
            + raw.raw_ice_signed_rate_kg_m2_s.max(0.0))
            * column.tile_fraction
            * column.interval_s,
        authorization_kg_m2_stand_ground: context.finalized_vapor.map(|authorized| {
            (authorized.liquid_signed_rate_kg_m2_s.max(0.0)
                + authorized.ice_signed_rate_kg_m2_s.max(0.0))
                * column.tile_fraction
                * column.interval_s
        }),
        finalized_use_kg_m2_stand_ground: (finalized.liquid_signed_rate_kg_m2_s.max(0.0)
            + finalized.ice_signed_rate_kg_m2_s.max(0.0))
            * column.tile_fraction
            * column.interval_s,
        condensation_credit_kg_m2_stand_ground: ((-finalized.liquid_signed_rate_kg_m2_s).max(0.0)
            + (-finalized.ice_signed_rate_kg_m2_s).max(0.0))
            * column.tile_fraction
            * column.interval_s,
        branch: if total_vapor_rate < 0.0 {
            WaterBranch::Condensation
        } else if context.finalized_vapor.is_some() {
            WaterBranch::AuthorizationActiveOrTie
        } else {
            WaterBranch::ConstitutiveLaw
        },
    };

    let ledger = crate::V3PhaseFreeSurfaceEnergyLedger {
        beginning_sensible_energy_j_m2: context.beginning.sensible_energy_j_m2_tile,
        ending_sensible_energy_j_m2: post_vapor.sensible_energy_j_m2_tile,
        absorbed_shortwave_w_m2: absorbed_shortwave,
        net_longwave_w_m2: net_longwave,
        sensible_to_canopy_air_w_m2: detail.ground_sensible_to_canopy_air_w_m2,
        liquid_vapor_energy_w_m2: liquid_vapor_energy,
        ice_vapor_energy_w_m2: ice_vapor_energy,
        ground_heat_w_m2: detail.ground_heat_cn_w_m2_tile[0],
        storage_w_m2: storage,
        reconstructed_energy_residual_w_m2: surface_residual,
    };
    Ok(crate::V3PhaseFreeCoveredEvaluation {
        predecessor: detail,
        vapor,
        post_vapor,
        surface_energy: ledger,
    })
}
