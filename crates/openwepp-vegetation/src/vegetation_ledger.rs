//! Independent vegetation-owner C, N, and dry-material reconstruction.
//!
//! These validators consume immutable owner operands. They do not call the
//! phenology, turnover, allocation, or material-transfer producer functions
//! and never accept a producer-supplied residual.

use openwepp_kernel_contract::{
    MaterialDonorClass, MaterialReceiverClass, StratumId, TransactionId,
};

use crate::VegetationError;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct VegetationLedgerIdentity {
    pub transaction_id: TransactionId,
    pub stratum_id: StratumId,
    pub beginning_state_sha256: String,
    pub ending_state_sha256: String,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct VegetationCarbonLedger {
    pub identity: VegetationLedgerIdentity,
    pub beginning_physical_vegetation_kg_c_m2: f64,
    pub beginning_xs_c_kg_c_m2: f64,
    pub gross_primary_production_kg_c_m2: f64,
    pub maintenance_respiration_kg_c_m2: f64,
    pub growth_respiration_kg_c_m2: f64,
    pub outgoing_material_kg_c_m2: f64,
    pub ending_physical_vegetation_kg_c_m2: f64,
    pub ending_xs_c_kg_c_m2: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct VegetationNitrogenLedger {
    pub identity: VegetationLedgerIdentity,
    pub beginning_vegetation_kg_n_m2: f64,
    pub finalized_external_mineral_n_kg_m2: f64,
    pub outgoing_material_kg_n_m2: f64,
    pub ending_vegetation_kg_n_m2: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct VegetationDryMaterialTransferOperand {
    pub proposal_id: u64,
    pub donor: MaterialDonorClass,
    pub receiver: MaterialReceiverClass,
    pub carbon_kg_m2: f64,
    pub nitrogen_kg_m2: f64,
    pub drymatter_carbon_fraction: f64,
    pub proposed_dry_matter_kg_m2: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct VegetationDryMaterialLedger {
    pub identity: VegetationLedgerIdentity,
    pub transfers: Vec<VegetationDryMaterialTransferOperand>,
    pub outgoing_dry_matter_kg_m2: f64,
}

pub(crate) fn validate_vegetation_ledgers(
    expected_strata: &std::collections::BTreeSet<StratumId>,
    expected_transaction_id: TransactionId,
    expected_beginning_state_sha256: &str,
    expected_ending_state_sha256: &str,
    carbon: &[VegetationCarbonLedger],
    nitrogen: &[VegetationNitrogenLedger],
    dry_material: &[VegetationDryMaterialLedger],
) -> Result<(), VegetationError> {
    if carbon.len() != expected_strata.len()
        || carbon.len() != nitrogen.len()
        || carbon.len() != dry_material.len()
    {
        return Err(VegetationError::V7Candidate("stratum cardinality"));
    }
    if expected_strata.is_empty() {
        return Ok(());
    }
    let first_identity = &carbon[0].identity;
    validate_identity(first_identity)?;
    if first_identity.transaction_id != expected_transaction_id
        || first_identity.beginning_state_sha256 != expected_beginning_state_sha256
        || first_identity.ending_state_sha256 != expected_ending_state_sha256
    {
        return Err(VegetationError::V7Candidate(
            "sealed transaction or state identity mismatch",
        ));
    }
    let mut actual_strata = std::collections::BTreeSet::new();
    let mut global_proposal_ids = std::collections::BTreeSet::new();
    for ((carbon, nitrogen), dry_material) in carbon.iter().zip(nitrogen).zip(dry_material) {
        if carbon.identity != nitrogen.identity || carbon.identity != dry_material.identity {
            return Err(VegetationError::V7Candidate(
                "cross-ledger identity mismatch",
            ));
        }
        validate_identity(&carbon.identity)?;
        if carbon.identity.transaction_id != first_identity.transaction_id
            || carbon.identity.beginning_state_sha256 != first_identity.beginning_state_sha256
            || carbon.identity.ending_state_sha256 != first_identity.ending_state_sha256
            || !actual_strata.insert(carbon.identity.stratum_id.clone())
        {
            return Err(VegetationError::V7Candidate(
                "mixed whole-state identity or duplicate stratum",
            ));
        }
        validate_nonnegative(
            &[
                carbon.beginning_physical_vegetation_kg_c_m2,
                carbon.gross_primary_production_kg_c_m2,
                carbon.maintenance_respiration_kg_c_m2,
                carbon.growth_respiration_kg_c_m2,
                carbon.outgoing_material_kg_c_m2,
                carbon.ending_physical_vegetation_kg_c_m2,
                nitrogen.beginning_vegetation_kg_n_m2,
                nitrogen.finalized_external_mineral_n_kg_m2,
                nitrogen.outgoing_material_kg_n_m2,
                nitrogen.ending_vegetation_kg_n_m2,
                dry_material.outgoing_dry_matter_kg_m2,
            ],
            "vegetation ledger operand",
        )?;
        if !carbon.beginning_xs_c_kg_c_m2.is_finite() || !carbon.ending_xs_c_kg_c_m2.is_finite() {
            return Err(VegetationError::V7Candidate(
                "signed maintenance reserve operand",
            ));
        }
        let carbon_residual = carbon.beginning_physical_vegetation_kg_c_m2
            + carbon.beginning_xs_c_kg_c_m2
            + carbon.gross_primary_production_kg_c_m2
            - carbon.maintenance_respiration_kg_c_m2
            - carbon.growth_respiration_kg_c_m2
            - carbon.outgoing_material_kg_c_m2
            - carbon.ending_physical_vegetation_kg_c_m2
            - carbon.ending_xs_c_kg_c_m2;
        require_closed("vegetation carbon", carbon_residual, carbon_scale(carbon))?;
        let nitrogen_residual = nitrogen.beginning_vegetation_kg_n_m2
            + nitrogen.finalized_external_mineral_n_kg_m2
            - nitrogen.outgoing_material_kg_n_m2
            - nitrogen.ending_vegetation_kg_n_m2;
        require_closed(
            "vegetation nitrogen",
            nitrogen_residual,
            nitrogen_scale(nitrogen),
        )?;
        let (proposal_carbon, proposal_nitrogen) =
            validate_dry_material(dry_material, &mut global_proposal_ids)?;
        if proposal_carbon.to_bits() != carbon.outgoing_material_kg_c_m2.to_bits()
            || proposal_nitrogen.to_bits() != nitrogen.outgoing_material_kg_n_m2.to_bits()
        {
            return Err(VegetationError::V7Candidate(
                "material proposal elemental aggregate mismatch",
            ));
        }
    }
    if &actual_strata != expected_strata {
        return Err(VegetationError::V7Candidate(
            "configured stratum set mismatch",
        ));
    }
    Ok(())
}

fn validate_identity(identity: &VegetationLedgerIdentity) -> Result<(), VegetationError> {
    let valid_sha256 = |value: &str| {
        value.len() == 64
            && value
                .as_bytes()
                .iter()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
    };
    if identity.transaction_id.0 == 0
        || !valid_sha256(&identity.beginning_state_sha256)
        || !valid_sha256(&identity.ending_state_sha256)
    {
        return Err(VegetationError::V7Candidate("owner identity"));
    }
    Ok(())
}

fn validate_dry_material(
    ledger: &VegetationDryMaterialLedger,
    global_proposal_ids: &mut std::collections::BTreeSet<u64>,
) -> Result<(f64, f64), VegetationError> {
    let mut proposal_ids = std::collections::BTreeSet::new();
    let mut reconstructed = 0.0;
    let mut carbon = 0.0;
    let mut nitrogen = 0.0;
    for transfer in &ledger.transfers {
        validate_nonnegative(
            &[
                transfer.carbon_kg_m2,
                transfer.nitrogen_kg_m2,
                transfer.proposed_dry_matter_kg_m2,
            ],
            "vegetation dry-material transfer",
        )?;
        if transfer.proposal_id == 0
            || !proposal_ids.insert(transfer.proposal_id)
            || !global_proposal_ids.insert(transfer.proposal_id)
            || !transfer.drymatter_carbon_fraction.is_finite()
            || transfer.drymatter_carbon_fraction <= 0.0
            || transfer.drymatter_carbon_fraction > 1.0
        {
            return Err(VegetationError::V7Candidate(
                "dry-material transfer identity",
            ));
        }
        let independently_reconstructed =
            transfer.carbon_kg_m2 / transfer.drymatter_carbon_fraction;
        if independently_reconstructed.to_bits() != transfer.proposed_dry_matter_kg_m2.to_bits() {
            return Err(VegetationError::V7Candidate(
                "dry-material proposal operand mismatch",
            ));
        }
        reconstructed += transfer.proposed_dry_matter_kg_m2;
        carbon += transfer.carbon_kg_m2;
        nitrogen += transfer.nitrogen_kg_m2;
    }
    if reconstructed.to_bits() != ledger.outgoing_dry_matter_kg_m2.to_bits() {
        return Err(VegetationError::V7Candidate(
            "dry-material aggregate mismatch",
        ));
    }
    Ok((carbon, nitrogen))
}

fn validate_nonnegative(values: &[f64], field: &'static str) -> Result<(), VegetationError> {
    if values
        .iter()
        .any(|value| !value.is_finite() || *value < 0.0)
    {
        return Err(VegetationError::V7Candidate(field));
    }
    Ok(())
}

fn require_closed(
    ledger: &'static str,
    residual: f64,
    operand_scale: f64,
) -> Result<(), VegetationError> {
    let tolerance = 1.0e-14 + 64.0 * f64::EPSILON * operand_scale;
    if !residual.is_finite() || residual.abs() > tolerance {
        return Err(VegetationError::V7Closure { ledger, residual });
    }
    Ok(())
}

fn carbon_scale(ledger: &VegetationCarbonLedger) -> f64 {
    ledger.beginning_physical_vegetation_kg_c_m2
        + ledger.beginning_xs_c_kg_c_m2.abs()
        + ledger.gross_primary_production_kg_c_m2
        + ledger.maintenance_respiration_kg_c_m2
        + ledger.growth_respiration_kg_c_m2
        + ledger.outgoing_material_kg_c_m2
        + ledger.ending_physical_vegetation_kg_c_m2
        + ledger.ending_xs_c_kg_c_m2.abs()
}

fn nitrogen_scale(ledger: &VegetationNitrogenLedger) -> f64 {
    ledger.beginning_vegetation_kg_n_m2
        + ledger.finalized_external_mineral_n_kg_m2
        + ledger.outgoing_material_kg_n_m2
        + ledger.ending_vegetation_kg_n_m2
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity() -> VegetationLedgerIdentity {
        VegetationLedgerIdentity {
            transaction_id: TransactionId(9),
            stratum_id: StratumId::try_new("canopy").expect("stratum"),
            beginning_state_sha256: "a".repeat(64),
            ending_state_sha256: "b".repeat(64),
        }
    }

    fn fixtures() -> (
        Vec<VegetationCarbonLedger>,
        Vec<VegetationNitrogenLedger>,
        Vec<VegetationDryMaterialLedger>,
    ) {
        let carbon = vec![VegetationCarbonLedger {
            identity: identity(),
            beginning_physical_vegetation_kg_c_m2: 10.0,
            beginning_xs_c_kg_c_m2: -2.0,
            gross_primary_production_kg_c_m2: 2.0,
            maintenance_respiration_kg_c_m2: 0.5,
            growth_respiration_kg_c_m2: 0.25,
            outgoing_material_kg_c_m2: 1.0,
            ending_physical_vegetation_kg_c_m2: 9.75,
            ending_xs_c_kg_c_m2: -1.5,
        }];
        let nitrogen = vec![VegetationNitrogenLedger {
            identity: identity(),
            beginning_vegetation_kg_n_m2: 1.0,
            finalized_external_mineral_n_kg_m2: 0.2,
            outgoing_material_kg_n_m2: 0.1,
            ending_vegetation_kg_n_m2: 1.1,
        }];
        let dry = vec![VegetationDryMaterialLedger {
            identity: identity(),
            transfers: vec![VegetationDryMaterialTransferOperand {
                proposal_id: 1,
                donor: MaterialDonorClass::Leaf,
                receiver: MaterialReceiverClass::Metabolic,
                carbon_kg_m2: 1.0,
                nitrogen_kg_m2: 0.1,
                drymatter_carbon_fraction: 0.5,
                proposed_dry_matter_kg_m2: 2.0,
            }],
            outgoing_dry_matter_kg_m2: 2.0,
        }];
        (carbon, nitrogen, dry)
    }

    fn expected_strata() -> std::collections::BTreeSet<StratumId> {
        std::collections::BTreeSet::from([identity().stratum_id])
    }

    fn validate(
        expected: &std::collections::BTreeSet<StratumId>,
        carbon: &[VegetationCarbonLedger],
        nitrogen: &[VegetationNitrogenLedger],
        dry: &[VegetationDryMaterialLedger],
    ) -> Result<(), VegetationError> {
        validate_vegetation_ledgers(
            expected,
            TransactionId(9),
            &"a".repeat(64),
            &"b".repeat(64),
            carbon,
            nitrogen,
            dry,
        )
    }

    #[test]
    fn reconstructs_all_three_vegetation_ledgers_without_residual_inputs() {
        let (carbon, nitrogen, dry) = fixtures();
        validate(&expected_strata(), &carbon, &nitrogen, &dry).expect("closed ledgers");
    }

    #[test]
    fn rejects_carbon_as_dry_matter_and_producer_aggregate_aliases() {
        let (carbon, nitrogen, mut dry) = fixtures();
        dry[0].transfers[0].proposed_dry_matter_kg_m2 = 1.0;
        assert!(validate(&expected_strata(), &carbon, &nitrogen, &dry).is_err());
        let (_, _, mut dry) = fixtures();
        dry[0].outgoing_dry_matter_kg_m2 = 1.0;
        assert!(validate(&expected_strata(), &carbon, &nitrogen, &dry).is_err());
    }

    #[test]
    fn rejects_wrong_elemental_closure_and_cross_ledger_identity() {
        let (mut carbon, nitrogen, dry) = fixtures();
        carbon[0].ending_physical_vegetation_kg_c_m2 += 1.0e-6;
        assert!(validate(&expected_strata(), &carbon, &nitrogen, &dry).is_err());
        let (carbon, mut nitrogen, dry) = fixtures();
        nitrogen[0].identity.transaction_id = TransactionId(10);
        assert!(validate(&expected_strata(), &carbon, &nitrogen, &dry).is_err());
    }

    #[test]
    fn rejects_residual_admitted_only_by_the_old_loose_envelope() {
        let (mut carbon, nitrogen, dry) = fixtures();
        carbon[0].ending_physical_vegetation_kg_c_m2 += 5.0e-13;
        assert!(matches!(
            validate(&expected_strata(), &carbon, &nitrogen, &dry),
            Err(VegetationError::V7Closure {
                ledger: "vegetation carbon",
                ..
            })
        ));
    }

    #[test]
    fn accepts_finite_signed_xs_even_when_physical_plus_reserve_is_negative() {
        let (mut carbon, nitrogen, dry) = fixtures();
        carbon[0].beginning_physical_vegetation_kg_c_m2 = 1.0;
        carbon[0].beginning_xs_c_kg_c_m2 = -20.0;
        carbon[0].ending_physical_vegetation_kg_c_m2 = 0.75;
        carbon[0].ending_xs_c_kg_c_m2 = -19.5;
        validate(&expected_strata(), &carbon, &nitrogen, &dry)
            .expect("signed XS is independently finite, not a nonnegative pool");
    }

    #[test]
    fn rejects_ending_xs_corruption_duplicate_strata_and_mixed_whole_state_identity() {
        let (mut carbon, nitrogen, dry) = fixtures();
        carbon[0].ending_xs_c_kg_c_m2 += 1.0e-6;
        assert!(validate(&expected_strata(), &carbon, &nitrogen, &dry).is_err());

        let (mut carbon, mut nitrogen, mut dry) = fixtures();
        carbon.push(carbon[0].clone());
        nitrogen.push(nitrogen[0].clone());
        dry.push(dry[0].clone());
        let expected = std::collections::BTreeSet::from([
            identity().stratum_id,
            StratumId::try_new("understory").expect("stratum"),
        ]);
        assert!(validate(&expected, &carbon, &nitrogen, &dry).is_err());

        let (mut carbon, mut nitrogen, mut dry) = fixtures();
        let second = StratumId::try_new("understory").expect("stratum");
        let mut c = carbon[0].clone();
        c.identity.stratum_id = second.clone();
        c.identity.ending_state_sha256 = "c".repeat(64);
        let mut n = nitrogen[0].clone();
        n.identity = c.identity.clone();
        let mut d = dry[0].clone();
        d.identity = c.identity.clone();
        d.transfers[0].proposal_id = 2;
        carbon.push(c);
        nitrogen.push(n);
        dry.push(d);
        let expected = std::collections::BTreeSet::from([identity().stratum_id, second]);
        assert!(validate(&expected, &carbon, &nitrogen, &dry).is_err());
    }

    #[test]
    fn rejects_globally_duplicate_proposal_identity_across_strata() {
        let (mut carbon, mut nitrogen, mut dry) = fixtures();
        let second = StratumId::try_new("understory").expect("stratum");
        let mut c = carbon[0].clone();
        c.identity.stratum_id = second.clone();
        let mut n = nitrogen[0].clone();
        n.identity.stratum_id = second.clone();
        let mut d = dry[0].clone();
        d.identity.stratum_id = second.clone();
        carbon.push(c);
        nitrogen.push(n);
        dry.push(d);
        let expected = std::collections::BTreeSet::from([identity().stratum_id, second]);
        assert!(matches!(
            validate(&expected, &carbon, &nitrogen, &dry),
            Err(VegetationError::V7Candidate(
                "dry-material transfer identity"
            ))
        ));
    }
}
