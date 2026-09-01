const COVERED_FROZEN_SNOW_ICE_HEAT_CAPACITY_J_KG_K_V1: f64 = 2_100.0;
const COVERED_FROZEN_EXTERNAL_LIQUID_ELIGIBILITY_MAX_KG_M2_V1: f64 = 1.0e-12;

fn covered_frozen_external_liquid_eligibility_neutral_v1(external_liquid_kg_m2: f64) -> bool {
    external_liquid_kg_m2.is_finite()
        && external_liquid_kg_m2 >= 0.0
        && external_liquid_kg_m2
            <= COVERED_FROZEN_EXTERNAL_LIQUID_ELIGIBILITY_MAX_KG_M2_V1
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CoveredPhaseConsistentCoordinatePostureV1 {
    EnthalpyPrimaryWithCnHeat,
    FrozenTemperaturePrimary,
}

impl CoveredPhaseConsistentCoordinatePostureV1 {
    const fn snow_stride(self) -> usize {
        match self {
            Self::EnthalpyPrimaryWithCnHeat => 4,
            Self::FrozenTemperaturePrimary => 3,
        }
    }

    fn soil_coordinate_offset(
        self,
        lane_count: usize,
    ) -> Result<usize, PhaseConsistentCoupledSolveErrorV1> {
        self.snow_stride()
            .checked_mul(lane_count)
            .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredFrozenTemperaturePrimaryEligibilityV1 {
    seed_coordinates: Vec<f64>,
    lane_count: usize,
    soil_count: usize,
    publication_eligible: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredFrozenTemperaturePrimaryLaneEnthalpyV1 {
    lane_id: u32,
    enthalpy_hi_j_m2: f64,
    enthalpy_carry: openwepp_land_surface_energy::ExactDyadicEnthalpy,
    snow_temperature_k: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredFrozenTemperaturePrimaryResidualInputsV1 {
    coordinates: Vec<f64>,
    lane_ids: Vec<u32>,
    beginning_snow_water_kg_m2: Vec<f64>,
    beginning_snow_enthalpy_hi_j_m2: Vec<f64>,
    beginning_snow_enthalpy_carry: Vec<openwepp_land_surface_energy::ExactDyadicEnthalpy>,
    physical_delta_water_kg_m2: Vec<f64>,
    ordered_physical_energy_operands_j_m2: Vec<Vec<f64>>,
    physical_ice_kg_m2: Vec<f64>,
    physical_density_kg_m3: Vec<f64>,
    physical_thickness_m: Vec<f64>,
    exact_density_settling_branch_satisfied: Vec<bool>,
    beginning_soil_enthalpy_j_m2: Vec<f64>,
    physical_soil_delta_energy_j_m2: Vec<f64>,
    owner_soil_temperature_k: Vec<f64>,
    absolute_tolerances: Vec<f64>,
    algebraic_side_constraints_satisfied: bool,
}

fn covered_v56_digest_domain_v1(domain: &[u8], payload: &[u8]) -> openwepp_coupled_time::Digest32 {
    let mut bytes = Vec::with_capacity(domain.len() + payload.len());
    bytes.extend_from_slice(domain);
    bytes.extend_from_slice(payload);
    openwepp_coupled_time::digest_bytes(&bytes)
}

fn covered_frozen_temperature_primary_beginning_carries_v1(
    base_material_owner: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    lane_ids: &[u32],
    authenticated_beginning: Option<
        &crate::snow_stage3_v11_snow_enthalpy_carry::AuthenticatedCoveredSnowMaterialOwnerV1,
    >,
) -> Result<
    Vec<crate::snow_stage3_v11_snow_enthalpy_carry::CoveredSnowEnthalpyCarryStateV1>,
    PhaseConsistentCoupledSolveErrorV1,
> {
    if let Some(owner) = authenticated_beginning {
        owner
            .validate()
            .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
        if owner.base_material_owner() != base_material_owner
            || owner
                .carries()
                .iter()
                .map(|carry| carry.lane_id())
                .ne(lane_ids.iter().copied())
        {
            return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
        }
        return Ok(owner.carries().to_vec());
    }
    lane_ids
        .iter()
        .map(|lane_id| {
            let layer = base_material_owner
                .get(lane_id)
                .and_then(|state| state.layers.first())
                .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
            crate::snow_stage3_v11_snow_enthalpy_carry::CoveredSnowEnthalpyCarryStateV1::zero_carry(
                *lane_id,
                0,
                -layer.cold_content_j_m2,
                layer.temperature_c + 273.15,
            )
            .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn covered_frozen_temperature_primary_compound_owner_v1(
    support: TimeSupport,
    transaction_id: TransactionId,
    lane_ids: &[u32],
    beginning_base_material_owner: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    ending_base_material_owner: BTreeMap<u32, DirectSnowStage3PersistentState>,
    ending_enthalpies: &[CoveredFrozenTemperaturePrimaryLaneEnthalpyV1],
    support_images: &BTreeMap<u32, CoveredExactFloorTerminalPhaseSupportImageV1>,
    physical_snow_soil_receipts: &BTreeMap<u32, SnowSoilHeatReceiptV1>,
    branch_identity: &CoveredPhaseConsistentPhysicalBranchIdentityV1,
    authenticated_beginning: Option<
        &crate::snow_stage3_v11_snow_enthalpy_carry::AuthenticatedCoveredSnowMaterialOwnerV1,
    >,
) -> Result<
    crate::snow_stage3_v11_snow_enthalpy_carry::AuthenticatedCoveredSnowMaterialOwnerV1,
    PhaseConsistentCoupledSolveErrorV1,
> {
    use crate::snow_stage3_v11_snow_enthalpy_carry::{
        covered_snow_base_material_owner_sha256, covered_snow_material_candidate_sha256,
        AuthenticatedCoveredSnowMaterialOwnerV1, CoveredSnowEnthalpyCarryReceiptInputsV1,
        CoveredSnowEnthalpyCarryReceiptV1, CoveredSnowEnthalpyCarryStateV1,
        CoveredSnowEnthalpyEnergyOperandKindV1, CoveredSnowEnthalpyEnergyOperandV1,
    };
    use openwepp_coupled_time::ParentTransactionId;

    if ending_enthalpies.len() != lane_ids.len()
        || support_images.keys().copied().ne(lane_ids.iter().copied())
        || physical_snow_soil_receipts
            .keys()
            .copied()
            .ne(lane_ids.iter().copied())
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    let beginning_carries = covered_frozen_temperature_primary_beginning_carries_v1(
        beginning_base_material_owner,
        lane_ids,
        authenticated_beginning,
    )?;
    let ending_carries = ending_enthalpies
        .iter()
        .map(|enthalpy| {
            CoveredSnowEnthalpyCarryStateV1::new(
                enthalpy.lane_id,
                0,
                enthalpy.enthalpy_hi_j_m2,
                enthalpy.enthalpy_carry.clone(),
                enthalpy.snow_temperature_k,
            )
            .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut ordered_energy_operands = Vec::with_capacity(lane_ids.len() * 8);
    for lane_id in lane_ids {
        let image = support_images
            .get(lane_id)
            .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
        for (component, energy) in image.ordered_energy_components_j_m2.iter().enumerate() {
            let kind = match component {
                3 => CoveredSnowEnthalpyEnergyOperandKindV1::LatentMassTransfer,
                5 => CoveredSnowEnthalpyEnergyOperandKindV1::SnowSoilCrankNicolson,
                6 => CoveredSnowEnthalpyEnergyOperandKindV1::InternalConduction,
                _ => CoveredSnowEnthalpyEnergyOperandKindV1::ExternalSurface,
            };
            ordered_energy_operands.push(
                CoveredSnowEnthalpyEnergyOperandV1::new(
                    u32::try_from(ordered_energy_operands.len())
                        .map_err(|_| PhaseConsistentCoupledSolveErrorV1::Structure)?,
                    kind,
                    *energy,
                )
                .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?,
            );
        }
        ordered_energy_operands.push(
            CoveredSnowEnthalpyEnergyOperandV1::new(
                u32::try_from(ordered_energy_operands.len())
                    .map_err(|_| PhaseConsistentCoupledSolveErrorV1::Structure)?,
                CoveredSnowEnthalpyEnergyOperandKindV1::MeltRefreeze,
                image.cold_content_export_j_m2,
            )
            .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?,
        );
    }
    let candidate_sha256 =
        covered_snow_material_candidate_sha256(&ending_base_material_owner, &ending_carries)
            .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
    let beginning_compound_owner_sha256 = authenticated_beginning
        .map(AuthenticatedCoveredSnowMaterialOwnerV1::compound_owner_sha256)
        .unwrap_or_else(|| {
            covered_snow_material_candidate_sha256(
                beginning_base_material_owner,
                &beginning_carries,
            )
            .unwrap_or_else(|_| openwepp_coupled_time::Digest32::zero())
        });
    if beginning_compound_owner_sha256 == openwepp_coupled_time::Digest32::zero() {
        return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
    }
    let branch_bytes = serde_json::to_vec(&(
        &branch_identity.phase_branch,
        &branch_identity.density_model_branch,
    ))
    .map_err(|_| PhaseConsistentCoupledSolveErrorV1::Structure)?;
    let mut topology_bytes = Vec::with_capacity(32 * physical_snow_soil_receipts.len());
    let mut configuration_bytes = Vec::with_capacity(32 * physical_snow_soil_receipts.len());
    let mut custody_bytes = Vec::new();
    for receipt in physical_snow_soil_receipts.values() {
        crate::snow_stage3_v11_attachment::validate_snow_soil_heat_receipt(receipt)
            .map_err(|_| PhaseConsistentCoupledSolveErrorV1::ReplayMismatch)?;
        topology_bytes.extend_from_slice(receipt.topology_identity_sha256.as_bytes());
        configuration_bytes.extend_from_slice(receipt.configuration_identity_sha256.as_bytes());
        custody_bytes.extend_from_slice(receipt.model_identity_sha256.as_bytes());
        custody_bytes.extend_from_slice(&receipt.lane_id.to_be_bytes());
        custody_bytes.extend_from_slice(receipt.ofe_id.as_str().as_bytes());
        custody_bytes.extend_from_slice(&receipt.bottom_snow_layer_id.to_be_bytes());
        custody_bytes.extend_from_slice(receipt.first_soil_layer_id.as_str().as_bytes());
    }
    let transaction_digest = covered_v56_digest_domain_v1(
        b"OPENWEPP_V56_SNOW_TRANSACTION_V1",
        &transaction_id.0.to_be_bytes(),
    );
    let receipt =
        CoveredSnowEnthalpyCarryReceiptV1::seal(CoveredSnowEnthalpyCarryReceiptInputsV1 {
            support,
            transaction_id: ParentTransactionId::from_digest(transaction_digest),
            predecessor_transaction_id: authenticated_beginning
                .map(|owner| owner.receipt().transaction_id()),
            beginning_carries,
            ending_carries: ending_carries.clone(),
            ordered_energy_operands,
            base_material_owner_sha256: covered_snow_base_material_owner_sha256(
                &ending_base_material_owner,
            )
            .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?,
            beginning_compound_owner_sha256,
            predecessor_receipt_chain_sha256: authenticated_beginning
                .map(|owner| owner.receipt().receipt_sha256())
                .unwrap_or_else(openwepp_coupled_time::Digest32::zero),
            branch_identity_sha256: covered_v56_digest_domain_v1(
                b"OPENWEPP_V56_BRANCH_V1",
                &branch_bytes,
            ),
            topology_identity_sha256: covered_v56_digest_domain_v1(
                b"OPENWEPP_V56_TOPOLOGY_V1",
                &topology_bytes,
            ),
            configuration_identity_sha256: covered_v56_digest_domain_v1(
                b"OPENWEPP_V56_CONFIGURATION_V1",
                &configuration_bytes,
            ),
            custody_identity_sha256: covered_v56_digest_domain_v1(
                b"OPENWEPP_V56_CUSTODY_V1",
                &custody_bytes,
            ),
            candidate_sha256,
        })
        .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
    AuthenticatedCoveredSnowMaterialOwnerV1::seal(
        ending_base_material_owner,
        ending_carries,
        receipt,
    )
    .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)
}

fn covered_frozen_temperature_primary_eligibility_v1(
    lane_ids: &[u32],
    beginning_states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    endpoint_states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    support_images: &BTreeMap<u32, CoveredExactFloorTerminalPhaseSupportImageV1>,
    legacy_seed_coordinates: &[f64],
    soil_count: usize,
    budget: &CoveredPhysicalEvaluationBudgetV1,
    minimum_solver_reserve: usize,
) -> Result<Option<CoveredFrozenTemperaturePrimaryEligibilityV1>, PhaseConsistentCoupledSolveErrorV1>
{
    let legacy_expected = 3_usize
        .checked_mul(lane_ids.len())
        .and_then(|snow| {
            soil_count
                .checked_mul(2)
                .and_then(|soil| snow.checked_add(soil))
        })
        .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
    if lane_ids.is_empty()
        || beginning_states.len() != lane_ids.len()
        || endpoint_states.len() != lane_ids.len()
        || support_images.len() != lane_ids.len()
        || legacy_seed_coordinates.len() != legacy_expected
        || beginning_states.keys().ne(lane_ids.iter())
        || endpoint_states.keys().ne(lane_ids.iter())
        || support_images.keys().ne(lane_ids.iter())
        || budget.maximum != COVERED_PHYSICAL_EVALUATION_LIMIT_V1
        || budget.used > budget.maximum
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    if budget.maximum - budget.used < minimum_solver_reserve {
        return Ok(None);
    }

    let mut seed_coordinates = Vec::with_capacity(legacy_expected);
    for (lane_index, lane_id) in lane_ids.iter().enumerate() {
        let beginning = beginning_states
            .get(lane_id)
            .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
        let state = endpoint_states
            .get(lane_id)
            .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
        let support = support_images
            .get(lane_id)
            .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
        support
            .validate()
            .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
        if beginning.layers.len() != 1
            || state.layers.len() != 1
            || beginning.detached_retained_liquid_kg_m2.to_bits() != 0.0_f64.to_bits()
            || state.detached_retained_liquid_kg_m2.to_bits() != 0.0_f64.to_bits()
            || !covered_frozen_external_liquid_eligibility_neutral_v1(
                support.external_liquid_kg_m2,
            )
            || state.terminal_event_model != beginning.terminal_event_model
            || (state.cumulative_melt_kg_m2 - beginning.cumulative_melt_kg_m2).to_bits()
                != 0.0_f64.to_bits()
        {
            return Ok(None);
        }
        let beginning_layer = &beginning.layers[0];
        let layer = &state.layers[0];
        let temperature_k = layer.temperature_c + 273.15;
        let beginning_temperature_k = beginning_layer.temperature_c + 273.15;
        if beginning_layer.liquid_water_m.to_bits() != 0.0_f64.to_bits()
            || layer.liquid_water_m.to_bits() != 0.0_f64.to_bits()
            || !beginning_temperature_k.is_finite()
            || !(0.0..273.15).contains(&beginning_temperature_k)
            || !layer.mass_swe_m.is_finite()
            || layer.mass_swe_m <= 0.0
            || !layer.density_kg_m3.is_finite()
            || layer.density_kg_m3 <= 0.0
            || !temperature_k.is_finite()
            || !(0.0..273.15).contains(&temperature_k)
        {
            return Ok(None);
        }
        let legacy = 3 * lane_index;
        let phase = phase_consistent_canonical_phase_projection_v1(
            legacy_seed_coordinates[legacy],
            legacy_seed_coordinates[legacy + 1],
            legacy_seed_coordinates[legacy + 2],
        )?;
        if phase.liquid_kg_m2.to_bits() != 0.0_f64.to_bits()
            || !(0.0..273.15).contains(&phase.snow_temperature_k)
        {
            return Ok(None);
        }
        seed_coordinates.extend([
            phase.water_kg_m2,
            phase.snow_temperature_k,
            phase.density_kg_m3,
        ]);
    }
    seed_coordinates.extend_from_slice(&legacy_seed_coordinates[3 * lane_ids.len()..]);
    Ok(Some(CoveredFrozenTemperaturePrimaryEligibilityV1 {
        seed_coordinates,
        lane_count: lane_ids.len(),
        soil_count,
        publication_eligible: false,
    }))
}

#[allow(clippy::too_many_arguments)]
fn covered_frozen_temperature_primary_post_root_transition_v1(
    legacy_root: &CoveredPhaseConsistentPhysicalEvaluationV1,
    lane_ids: &[u32],
    beginning_states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    soil_count: usize,
    budget: &CoveredPhysicalEvaluationBudgetV1,
    minimum_solver_reserve: usize,
) -> Result<Option<CoveredFrozenTemperaturePrimaryEligibilityV1>, PhaseConsistentCoupledSolveErrorV1>
{
    if legacy_root.coordinate_posture
        != CoveredPhaseConsistentCoordinatePostureV1::EnthalpyPrimaryWithCnHeat
        || !legacy_root.residual.scaled_merit.is_finite()
        || legacy_root.residual.scaled_merit > 1.0
        || !legacy_root
            .residual
            .algebraic_side_constraints_satisfied
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
    }
    let legacy_soil_offset = CoveredPhaseConsistentCoordinatePostureV1::EnthalpyPrimaryWithCnHeat
        .soil_coordinate_offset(lane_ids.len())?;
    let expected = legacy_soil_offset
        .checked_add(2 * soil_count)
        .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
    if legacy_root.residual.coordinates.len() != expected {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    let mut legacy_temperature_transition_coordinates =
        Vec::with_capacity(3 * lane_ids.len() + 2 * soil_count);
    for lane in 0..lane_ids.len() {
        legacy_temperature_transition_coordinates
            .extend_from_slice(&legacy_root.residual.coordinates[4 * lane..4 * lane + 3]);
    }
    legacy_temperature_transition_coordinates
        .extend_from_slice(&legacy_root.residual.coordinates[legacy_soil_offset..]);
    let used_before = budget.used;
    let eligibility = covered_frozen_temperature_primary_eligibility_v1(
        lane_ids,
        beginning_states,
        &legacy_root.artifacts.stage3_candidate,
        &legacy_root.artifacts.stage3_support_images,
        &legacy_temperature_transition_coordinates,
        soil_count,
        budget,
        minimum_solver_reserve,
    )?;
    if budget.used != used_before {
        return Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget);
    }
    Ok(eligibility)
}

fn covered_frozen_temperature_primary_project_stage3_coordinates_v1(
    baseline: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    lane_ids: &[u32],
    coordinates: &[f64],
) -> Result<
    (
        BTreeMap<u32, DirectSnowStage3PersistentState>,
        Vec<CoveredFrozenTemperaturePrimaryLaneEnthalpyV1>,
    ),
    PhaseConsistentCoupledSolveErrorV1,
> {
    if baseline.len() != lane_ids.len()
        || coordinates.len() < 3 * lane_ids.len()
        || baseline.keys().ne(lane_ids.iter())
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    let mut legacy_coordinates = Vec::with_capacity(coordinates.len());
    let mut enthalpies = Vec::with_capacity(lane_ids.len());
    for (lane_index, lane_id) in lane_ids.iter().enumerate() {
        let coordinate = 3 * lane_index;
        let water = coordinates[coordinate];
        let snow_temperature_k = coordinates[coordinate + 1];
        let density = coordinates[coordinate + 2];
        let (enthalpy_hi_j_m2, enthalpy_carry) =
            openwepp_land_surface_energy::frozen_snow_enthalpy_high_and_carry(
                water,
                COVERED_FROZEN_SNOW_ICE_HEAT_CAPACITY_J_KG_K_V1,
                snow_temperature_k,
            )
            .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
        legacy_coordinates.extend([water, enthalpy_hi_j_m2, density]);
        enthalpies.push(CoveredFrozenTemperaturePrimaryLaneEnthalpyV1 {
            lane_id: *lane_id,
            enthalpy_hi_j_m2,
            enthalpy_carry,
            snow_temperature_k,
        });
    }
    legacy_coordinates.extend_from_slice(&coordinates[3 * lane_ids.len()..]);
    let (mut projected, phases) = covered_phase_consistent_project_stage3_coordinates_v1(
        baseline,
        lane_ids,
        &legacy_coordinates,
    )?;
    for enthalpy in &enthalpies {
        let phase = phases
            .get(&enthalpy.lane_id)
            .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
        if phase.liquid_kg_m2.to_bits() != 0.0_f64.to_bits()
            || phase.enthalpy_j_m2.to_bits() != enthalpy.enthalpy_hi_j_m2.to_bits()
        {
            return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
        }
        let state = projected
            .get_mut(&enthalpy.lane_id)
            .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
        let layer = state
            .layers
            .first_mut()
            .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
        layer.temperature_c = enthalpy.snow_temperature_k - 273.15;
        state.fingerprint = Wb11HydrologyKernel::stage3_persistent_state_fingerprint(state);
        Wb11HydrologyKernel::validate_stage3_persistent_state(state)
            .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
    }
    Ok((projected, enthalpies))
}

fn covered_frozen_temperature_primary_residual_assemble_v1(
    inputs: CoveredFrozenTemperaturePrimaryResidualInputsV1,
) -> Result<CoveredPhaseConsistentResidualEvaluationV1, PhaseConsistentCoupledSolveErrorV1> {
    let lanes = inputs.lane_ids.len();
    let soils = inputs.beginning_soil_enthalpy_j_m2.len();
    let expected = 3 * lanes + 2 * soils;
    if lanes == 0
        || inputs.coordinates.len() != expected
        || inputs.beginning_snow_water_kg_m2.len() != lanes
        || inputs.beginning_snow_enthalpy_hi_j_m2.len() != lanes
        || inputs.beginning_snow_enthalpy_carry.len() != lanes
        || inputs.physical_delta_water_kg_m2.len() != lanes
        || inputs.ordered_physical_energy_operands_j_m2.len() != lanes
        || inputs.physical_ice_kg_m2.len() != lanes
        || inputs.physical_density_kg_m3.len() != lanes
        || inputs.physical_thickness_m.len() != lanes
        || inputs.exact_density_settling_branch_satisfied.len() != lanes
        || inputs.physical_soil_delta_energy_j_m2.len() != soils
        || inputs.owner_soil_temperature_k.len() != soils
        || inputs.absolute_tolerances.len() != expected
        || !inputs.algebraic_side_constraints_satisfied
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    if inputs
        .coordinates
        .iter()
        .chain(&inputs.beginning_snow_water_kg_m2)
        .chain(&inputs.beginning_snow_enthalpy_hi_j_m2)
        .chain(&inputs.physical_delta_water_kg_m2)
        .chain(&inputs.physical_ice_kg_m2)
        .chain(&inputs.physical_density_kg_m3)
        .chain(&inputs.physical_thickness_m)
        .chain(&inputs.beginning_soil_enthalpy_j_m2)
        .chain(&inputs.physical_soil_delta_energy_j_m2)
        .chain(&inputs.owner_soil_temperature_k)
        .chain(&inputs.absolute_tolerances)
        .chain(
            inputs
                .ordered_physical_energy_operands_j_m2
                .iter()
                .flatten(),
        )
        .any(|value| !value.is_finite())
        || inputs.absolute_tolerances.iter().any(|value| *value <= 0.0)
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
    }

    let mut r_w_kg_m2 = Vec::with_capacity(lanes);
    let mut r_h_j_m2 = Vec::with_capacity(lanes);
    let mut r_rho_kg_m3 = Vec::with_capacity(lanes);
    let mut derived_thickness_closures = Vec::with_capacity(lanes);
    let mut exact_energy_closed = true;
    for lane in 0..lanes {
        let coordinate = 3 * lane;
        let water = inputs.coordinates[coordinate];
        let snow_temperature_k = inputs.coordinates[coordinate + 1];
        let density = inputs.coordinates[coordinate + 2];
        let (ending_hi, ending_carry) =
            openwepp_land_surface_energy::frozen_snow_enthalpy_high_and_carry(
                water,
                COVERED_FROZEN_SNOW_ICE_HEAT_CAPACITY_J_KG_K_V1,
                snow_temperature_k,
            )
            .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
        let expected_exact = openwepp_land_surface_energy::ExactDyadicEnthalpy::exact_sum_binary64(
            inputs.beginning_snow_enthalpy_hi_j_m2[lane],
            &inputs.beginning_snow_enthalpy_carry[lane],
            &inputs.ordered_physical_energy_operands_j_m2[lane],
        )
        .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
        let (expected_hi, expected_carry) = expected_exact
            .rounded_high_and_remainder()
            .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
        let within =
            openwepp_land_surface_energy::exact_reconstructed_enthalpy_within_abs_tolerance(
                ending_hi,
                &ending_carry,
                expected_hi,
                &expected_carry,
                COVERED_FIXED_POINT_POLICY.energy_abs_j_m2,
            )
            .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
        exact_energy_closed &= within;
        r_w_kg_m2.push(
            water
                - inputs.beginning_snow_water_kg_m2[lane]
                - inputs.physical_delta_water_kg_m2[lane],
        );
        r_h_j_m2.push(
            ending_hi
                - expected_exact
                    .round_to_f64()
                    .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?,
        );
        let geometry = CoveredTerminalDensityGeometryCoordinateV1 {
            rho_1_kg_m3: density,
            ice_1_kg_m2: water,
            z_1_m: water / density,
        };
        r_rho_kg_m3.push(covered_terminal_density_geometry_residual_evaluate_v1(
            geometry,
            inputs.physical_density_kg_m3[lane],
            inputs.exact_density_settling_branch_satisfied[lane],
        )?);
        derived_thickness_closures.push(covered_derived_thickness_closure_evaluate_v1(
            geometry,
            inputs.physical_ice_kg_m2[lane],
            inputs.physical_density_kg_m3[lane],
            inputs.physical_thickness_m[lane],
        )?);
    }

    let mut r_e_j_m2 = Vec::with_capacity(soils);
    let mut r_t_k = Vec::with_capacity(soils);
    for soil in 0..soils {
        let coordinate = 3 * lanes + 2 * soil;
        r_e_j_m2.push(
            inputs.coordinates[coordinate]
                - inputs.beginning_soil_enthalpy_j_m2[soil]
                - inputs.physical_soil_delta_energy_j_m2[soil],
        );
        r_t_k.push(inputs.coordinates[coordinate + 1] - inputs.owner_soil_temperature_k[soil]);
    }
    let mut residuals = Vec::with_capacity(expected);
    for lane in 0..lanes {
        residuals.extend([r_w_kg_m2[lane], r_h_j_m2[lane], r_rho_kg_m3[lane]]);
    }
    for soil in 0..soils {
        residuals.extend([r_e_j_m2[soil], r_t_k[soil]]);
    }
    let derived_constraints_scaled_merit = derived_thickness_closures
        .iter()
        .map(|closure| closure.scaled_merit)
        .fold(0.0_f64, f64::max);
    let scaled_merit = residuals
        .iter()
        .zip(&inputs.absolute_tolerances)
        .map(|(residual, tolerance)| residual.abs() / tolerance)
        .fold(derived_constraints_scaled_merit, f64::max);
    if !scaled_merit.is_finite() || residuals.iter().any(|value| !value.is_finite()) {
        return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
    }
    Ok(CoveredPhaseConsistentResidualEvaluationV1 {
        coordinates: inputs.coordinates,
        residuals,
        absolute_tolerances: inputs.absolute_tolerances,
        r_w_kg_m2,
        r_h_j_m2,
        r_rho_kg_m3,
        r_q_cn_j_m2: Vec::new(),
        physical_q_cn_j_m2: Vec::new(),
        derived_thickness_closures,
        r_e_j_m2,
        r_t_k,
        scaled_merit,
        derived_constraints_scaled_merit,
        algebraic_side_constraints_satisfied: exact_energy_closed,
    })
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredFrozenTemperaturePrimarySolveV1 {
    root: CoveredPhaseConsistentPhysicalEvaluationV1,
    evaluations_used: usize,
    trust_radius: f64,
    publication_eligible: bool,
}

fn covered_frozen_temperature_primary_solve_v1<F>(
    initial_coordinates: Vec<f64>,
    budget: &mut CoveredPhysicalEvaluationBudgetV1,
    evaluate: F,
) -> Result<CoveredFrozenTemperaturePrimarySolveV1, PhaseConsistentCoupledSolveErrorV1>
where
    F: FnMut(
        &[f64],
        &mut CoveredPhysicalEvaluationBudgetV1,
    ) -> Result<
        CoveredPhaseConsistentPhysicalEvaluationV1,
        PhaseConsistentCoupledSolveErrorV1,
    >,
{
    let state = phase_consistent_coupled_solve_engine_v1(
        initial_coordinates,
        budget,
        Some(COVERED_PRIVATE_SOLVE_POST_ROOT_RESERVE_V1),
        evaluate,
    )?;
    if state.root.coordinate_posture
        != CoveredPhaseConsistentCoordinatePostureV1::FrozenTemperaturePrimary
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
    }
    Ok(CoveredFrozenTemperaturePrimarySolveV1 {
        root: state.root,
        evaluations_used: state.evaluations_used,
        trust_radius: state.trust_radius,
        publication_eligible: false,
    })
}
