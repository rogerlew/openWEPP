fn covered_private_q_lattice_witness_v1(
    root: &CoveredPhaseConsistentPhysicalEvaluationV1,
    budget: &mut CoveredPhysicalEvaluationBudgetV1,
    root_branch: &CoveredPhaseConsistentPhysicalBranchIdentityV1,
    mut evaluate: impl FnMut(
        &[f64],
        &mut CoveredPhysicalEvaluationBudgetV1,
    ) -> Result<
        CoveredPhaseConsistentPhysicalEvaluationV1,
        PhaseConsistentCoupledSolveErrorV1,
    >,
) -> Result<Option<CoveredPhaseConsistentPhysicalEvaluationV1>, PhaseConsistentCoupledSolveErrorV1>
{
    let lanes = root.residual.r_q_cn_j_m2.len();
    if lanes == 0 {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    if root.residual.physical_q_cn_j_m2.len() != lanes
        || root.residual.coordinates.len() < 4 * lanes
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    let unresolved = root
        .residual
        .r_q_cn_j_m2
        .iter()
        .enumerate()
        .filter(|(_, residual)| residual.to_bits() != 0.0_f64.to_bits())
        .map(|(lane, _)| lane)
        .collect::<Vec<_>>();
    if unresolved.len() != 1 {
        return Ok(None);
    }
    let lane = unresolved[0];
    let q_coordinate_index = 4 * lane + 3;
    let q_coordinate = root.residual.coordinates[q_coordinate_index];
    let q_out = root.residual.physical_q_cn_j_m2[lane];
    let r_q = root.residual.r_q_cn_j_m2[lane];
    if !q_coordinate.is_finite() || !q_out.is_finite() || !r_q.is_finite() {
        return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
    }
    if (q_coordinate - q_out).to_bits() != r_q.to_bits() {
        return Err(PhaseConsistentCoupledSolveErrorV1::PrivateQLatticeInterval);
    }
    if q_coordinate <= 0.0 || q_out <= 0.0 {
        return Ok(None);
    }
    let root_bits = q_coordinate.to_bits();
    let output_bits = q_out.to_bits();
    let candidate_count_u64 = root_bits.abs_diff(output_bits);
    if candidate_count_u64 == 0 {
        return Ok(None);
    }
    let Ok(candidate_count) = usize::try_from(candidate_count_u64) else {
        return Ok(None);
    };
    let Some(required) = candidate_count.checked_add(COVERED_ROOT_POLISH_RECEIPT_RESERVE_V1) else {
        return Ok(None);
    };
    if budget.maximum != COVERED_PHYSICAL_EVALUATION_LIMIT_V1 {
        return Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget);
    }
    if budget.maximum.saturating_sub(budget.used) < required {
        return Ok(None);
    }

    let unchanged_coordinates = root.residual.coordinates.clone();
    let ascending = output_bits > root_bits;
    let mut first_exact_witness = None;
    for offset in 1..=candidate_count_u64 {
        let candidate_bits = if ascending {
            root_bits
                .checked_add(offset)
                .ok_or(PhaseConsistentCoupledSolveErrorV1::PrivateQLatticeInterval)?
        } else {
            root_bits
                .checked_sub(offset)
                .ok_or(PhaseConsistentCoupledSolveErrorV1::PrivateQLatticeInterval)?
        };
        let mut coordinates = unchanged_coordinates.clone();
        coordinates[q_coordinate_index] = f64::from_bits(candidate_bits);
        let used_before = budget.used;
        let candidate = evaluate(&coordinates, budget)?;
        if budget.used
            != used_before
                .checked_add(1)
                .ok_or(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget)?
        {
            return Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget);
        }
        covered_phase_consistent_physical_evaluation_validate_v1(
            &coordinates,
            &candidate,
            Some(budget.used),
            Some(root_branch),
        )?;
        if candidate.residual.scaled_merit > 1.0
            || !candidate.residual.algebraic_side_constraints_satisfied
        {
            return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
        }
        for (index, (expected, actual)) in unchanged_coordinates
            .iter()
            .zip(&candidate.residual.coordinates)
            .enumerate()
        {
            if index != q_coordinate_index && expected.to_bits() != actual.to_bits() {
                return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
            }
        }
        let candidate_r_q = candidate.residual.r_q_cn_j_m2[lane];
        let candidate_q_out = candidate.residual.physical_q_cn_j_m2[lane];
        if candidate_r_q.to_bits() == 0.0_f64.to_bits()
            && candidate.residual.coordinates[q_coordinate_index].to_bits()
                == candidate_q_out.to_bits()
            && first_exact_witness.is_none()
        {
            first_exact_witness = Some(candidate);
        }
    }
    first_exact_witness
        .ok_or(PhaseConsistentCoupledSolveErrorV1::PrivateQLatticeNoWitness)
        .map(Some)
}
