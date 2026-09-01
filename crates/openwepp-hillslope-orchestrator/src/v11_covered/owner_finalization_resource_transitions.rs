pub(crate) fn v11_shared_resource_transitions(
    envelope: &UncommittedCoveredV8OwnerEnvelope,
    input: &V11ImportedV10SegmentInput,
    debits: &[V11ResourceDebit],
    owners: &BTreeMap<String, V11OwnerEnvelope>,
    beginning_hydrology: &RealHydrologyShadowAdapter,
    ending_hydrology: &RealHydrologyShadowAdapter,
    beginning_bgc: &openwepp_biogeochemistry::BiogeochemistryState,
    ending_bgc: &openwepp_biogeochemistry::BiogeochemistryState,
    compositional: bool,
) -> Result<Vec<V11SharedResourceOwnerTransition>, DirectV11RealConsumerError> {
    let hydrology_digest = owners
        .get("hydrology")
        .ok_or(DirectV11RealConsumerError::Identity(
            "V11 hydrology candidate",
        ))?
        .state_sha256;
    let mut rows = v11_water_owner_transitions(
        envelope,
        input,
        debits,
        beginning_hydrology,
        ending_hydrology,
        hydrology_digest,
    )?;
    let bgc_digest = owners
        .get("bgc")
        .ok_or(DirectV11RealConsumerError::Identity("V11 BGC candidate"))?
        .state_sha256;
    let bgc_ofes = debits
        .iter()
        .filter(|debit| matches!(&debit.resource_key, V11ResourceKey::MineralNitrogen(_)))
        .map(|debit| debit.ofe_id.as_str())
        .collect::<BTreeSet<_>>();
    let ofe_id = bgc_ofes
        .iter()
        .next()
        .filter(|_| bgc_ofes.len() == 1)
        .copied();
    if envelope
        .biogeochemistry()
        .mineral_operands()
        .iter()
        .any(|operand| operand.finalized_use_kg_n_m2 > 0.0)
        && ofe_id.is_none()
    {
        return Err(DirectV11RealConsumerError::Identity(
            "V11 exact-one BGC transition OFE",
        ));
    }
    if let Some(ofe_id) = ofe_id {
        rows.extend(v11_bgc_owner_transitions(
            envelope,
            input,
            debits,
            beginning_bgc,
            ending_bgc,
            ofe_id,
            bgc_digest,
            compositional,
        )?);
    }
    Ok(rows)
}

pub(crate) fn v11_water_owner_transitions(
    envelope: &UncommittedCoveredV8OwnerEnvelope,
    input: &V11ImportedV10SegmentInput,
    debits: &[V11ResourceDebit],
    beginning: &RealHydrologyShadowAdapter,
    ending: &RealHydrologyShadowAdapter,
    owner_digest: Digest32,
) -> Result<Vec<V11SharedResourceOwnerTransition>, DirectV11RealConsumerError> {
    let mut rows = Vec::new();
    for (ofe_index, ofe) in envelope
        .hydrology()
        .receiver_closure_operands()
        .production_soil
        .iter()
        .enumerate()
    {
        for layer in &ofe.ordered_layers {
            let key = V11SharedResourceKey {
                resource: V11SharedResourceKind::Water,
                owner_id: "hydrology".into(),
                ofe_id: ofe.ofe_id.as_str().to_owned(),
                layer_id: layer.layer_id.as_str().to_owned(),
                source_id: "soil_water".into(),
                amount_basis: "kg_m2_stand_ground".into(),
            };
            let ids = v11_linked_debit_ids(debits, &key, true);
            if ids.is_empty() {
                continue;
            }
            let amount = |owner: &RealHydrologyShadowAdapter, message| {
                owner
                    .layer_facts()
                    .values()
                    .find(|fact| {
                        fact.source.ofe_lane.lane_index == ofe_index
                            && fact.source.layer_id == layer.layer_id
                    })
                    .map(|fact| fact.liquid_supply_kg_m2)
                    .ok_or(DirectV11RealConsumerError::Identity(message))
            };
            rows.push(v11_shared_transition(
                input,
                key,
                amount(beginning, "V11 beginning hydrology layer binding")?,
                amount(ending, "V11 ending hydrology layer binding")?,
                ids,
                owner_digest,
            )?);
        }
    }
    Ok(rows)
}

pub(crate) fn v11_bgc_owner_transitions(
    envelope: &UncommittedCoveredV8OwnerEnvelope,
    input: &V11ImportedV10SegmentInput,
    debits: &[V11ResourceDebit],
    beginning_bgc: &openwepp_biogeochemistry::BiogeochemistryState,
    ending_bgc: &openwepp_biogeochemistry::BiogeochemistryState,
    ofe_id: &str,
    owner_digest: Digest32,
    compositional: bool,
) -> Result<Vec<V11SharedResourceOwnerTransition>, DirectV11RealConsumerError> {
    let mut rows = Vec::new();
    for operand in envelope.biogeochemistry().mineral_operands() {
        let source_id = match operand.key.species {
            MineralNitrogenSpecies::Ammonium => "nh4",
            MineralNitrogenSpecies::Nitrate => "no3",
        };
        let resource = match operand.key.species {
            MineralNitrogenSpecies::Ammonium => V11SharedResourceKind::Ammonium,
            MineralNitrogenSpecies::Nitrate => V11SharedResourceKind::Nitrate,
        };
        let key = V11SharedResourceKey {
            resource,
            owner_id: "bgc".into(),
            ofe_id: ofe_id.to_owned(),
            layer_id: operand.key.layer_id.as_str().to_owned(),
            source_id: source_id.into(),
            amount_basis: "kg_n_m2".into(),
        };
        let ids = v11_linked_debit_ids(debits, &key, true);
        if ids.is_empty() {
            if operand.finalized_use_kg_n_m2 > 0.0 {
                return Err(DirectV11RealConsumerError::Identity(
                    "V11 BGC debit omission",
                ));
            }
            continue;
        }
        let beginning_layer = beginning_bgc
            .layers
            .get(operand.key.layer_id.as_str())
            .ok_or(DirectV11RealConsumerError::Identity(
                "V11 beginning BGC layer binding",
            ))?;
        let beginning_amount = match operand.key.species {
            MineralNitrogenSpecies::Ammonium => beginning_layer.ammonium_n,
            MineralNitrogenSpecies::Nitrate => beginning_layer.nitrate_n,
        };
        let ending_layer = ending_bgc.layers.get(operand.key.layer_id.as_str()).ok_or(
            DirectV11RealConsumerError::Identity("V11 ending BGC layer binding"),
        )?;
        let ending_amount = match operand.key.species {
            MineralNitrogenSpecies::Ammonium => ending_layer.ammonium_n,
            MineralNitrogenSpecies::Nitrate => ending_layer.nitrate_n,
        };
        let linked_use = ids
            .iter()
            .map(|id| {
                debits.iter().find(|debit| debit.receipt_id == *id).ok_or(
                    DirectV11RealConsumerError::Identity("V11 BGC linked debit identity"),
                )
            })
            .try_fold(0.0_f64, |sum, debit| {
                let next = sum + debit?.final_use;
                next.is_finite()
                    .then_some(next)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "V11 BGC finalized-use sum",
                    ))
            })?;
        let reconstructed_ending = beginning_amount - linked_use;
        if (!compositional
            && (linked_use.to_bits() != operand.finalized_use_kg_n_m2.to_bits()
                || ending_amount.to_bits() != operand.ending_kg_n_m2.to_bits()))
            || reconstructed_ending.to_bits() != ending_amount.to_bits()
                && (!compositional
                    || !v11_compositional_pool_roundoff_within_one_ulp(
                        reconstructed_ending,
                        ending_amount,
                    ))
        {
            return Err(DirectV11RealConsumerError::Identity(
                "V11 BGC mineral-pool delta",
            ));
        }
        rows.push(v11_shared_transition(
            input,
            key,
            beginning_amount,
            ending_amount,
            ids,
            owner_digest,
        )?);
    }
    Ok(rows)
}

fn v11_compositional_pool_roundoff_within_one_ulp(reconstructed: f64, installed: f64) -> bool {
    reconstructed.is_finite()
        && installed.is_finite()
        && reconstructed >= 0.0
        && installed >= 0.0
        && reconstructed.to_bits().abs_diff(installed.to_bits()) <= 1
}

pub(crate) fn v11_linked_debit_ids(
    debits: &[V11ResourceDebit],
    key: &V11SharedResourceKey,
    bind_amount_basis: bool,
) -> Vec<Digest32> {
    let mut linked = debits
        .iter()
        .filter(|debit| {
            debit.owner_id == key.owner_id
                && debit.ofe_id == key.ofe_id
                && debit.layer_id == key.layer_id
                && debit.source_id == key.source_id
                && (!bind_amount_basis || debit.amount_basis == key.amount_basis)
        })
        .collect::<Vec<_>>();
    if key.owner_id == "bgc"
        && matches!(
            key.resource,
            V11SharedResourceKind::Ammonium | V11SharedResourceKind::Nitrate
        )
    {
        linked.sort_by(|left, right| {
            left.occupancy_id
                .cmp(&right.occupancy_id)
                .then_with(|| left.layer_id.cmp(&right.layer_id))
                .then_with(|| left.resource_key.cmp(&right.resource_key))
        });
    } else {
        linked.sort_by_key(|debit| debit.receipt_id);
    }
    linked.into_iter().map(|debit| debit.receipt_id).collect()
}

pub(crate) fn v11_shared_transition(
    input: &V11ImportedV10SegmentInput,
    key: V11SharedResourceKey,
    beginning_amount: f64,
    ending_amount: f64,
    debit_receipt_ids: Vec<Digest32>,
    owner_candidate_sha256: Digest32,
) -> Result<V11SharedResourceOwnerTransition, DirectV11RealConsumerError> {
    Ok(V11SharedResourceOwnerTransition::new(
        V11SharedResourceOwnerTransition {
            transition_id: Digest32::zero(),
            parent_transaction_id: input.parent_transaction_id,
            segment_id: input.accepted_slab_receipt.segment_id(),
            accepted_slab_id: input.accepted_slab_receipt.slab_id(),
            support: input.support,
            shared_resource_key: key,
            beginning_amount,
            ending_amount,
            debit_receipt_ids,
            admitted_flux_receipt_ids: Vec::new(),
            owner_candidate_sha256,
        },
    )?)
}

pub(crate) fn v11_water_resource_debits(
    envelope: &UncommittedCoveredV8OwnerEnvelope,
    configuration: &VegetationConfiguration,
    input: &V11ImportedV10SegmentInput,
) -> Result<Vec<V11ResourceDebit>, DirectV11RealConsumerError> {
    let occupancies = configuration
        .expected_occupancies()
        .into_iter()
        .map(|id| {
            (
                format!("{}::{}", id.stratum_id.as_str(), id.tile_id.as_str()),
                id,
            )
        })
        .collect::<BTreeMap<_, _>>();
    let requests = &envelope.hydrology().arbitration().requests;
    let authorizations = &envelope.hydrology().arbitration().authorizations;
    envelope
        .hydrology()
        .finalized_uses()
        .iter()
        .filter_map(|value| {
            let component = value.key.occupancy_id.as_ref()?;
            let layer = value.key.soil_layer_id.as_ref()?;
            Some((value, component.as_str(), layer))
        })
        .map(|(value, component, layer)| {
            let occupancy =
                occupancies
                    .get(component)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "V11 water occupancy binding",
                    ))?;
            let request = requests.iter().find(|row| row.key == value.key).ok_or(
                DirectV11RealConsumerError::Identity("V11 water request binding"),
            )?;
            let authorization = authorizations
                .iter()
                .find(|row| row.key == value.key)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "V11 water authorization binding",
                ))?;
            V11ResourceDebit::new(V11ResourceDebit {
                receipt_id: Digest32::zero(),
                parent_transaction_id: input.parent_transaction_id,
                segment_id: input.accepted_slab_receipt.segment_id(),
                accepted_slab_id: input.accepted_slab_receipt.slab_id(),
                support: input.support,
                owner_id: "hydrology".to_owned(),
                resource_key: V11ResourceKey::Water(openwepp_kernel_contract::WaterResourceKey {
                    occupancy_id: occupancy.clone(),
                    layer_id: layer.clone(),
                }),
                ofe_id: value.key.ofe_id.as_str().to_owned(),
                tile_id: occupancy.tile_id.as_str().to_owned(),
                occupancy_id: component.to_owned(),
                layer_id: layer.as_str().to_owned(),
                source_id: "soil_water".to_owned(),
                amount_basis: "kg_m2_stand_ground".to_owned(),
                request: request.amount_kg_m2_stand_ground,
                authorization: authorization.amount_kg_m2_stand_ground,
                final_use: value.amount_kg_m2_stand_ground,
            })
            .map_err(|_| DirectV11RealConsumerError::Identity("V11 water debit"))
        })
        .collect()
}

#[cfg(test)]
mod owner_join_tests {
    use super::*;
    use openwepp_coupled_time::{ModelTimeNs, TimeSupport};

    #[test]
    fn compositional_pool_roundoff_accepts_only_one_ulp() {
        let value = 1.0_f64;
        assert!(v11_compositional_pool_roundoff_within_one_ulp(value, value));
        assert!(v11_compositional_pool_roundoff_within_one_ulp(
            value,
            f64::from_bits(value.to_bits() + 1),
        ));
        assert!(!v11_compositional_pool_roundoff_within_one_ulp(
            value,
            f64::from_bits(value.to_bits() + 2),
        ));
        assert!(!v11_compositional_pool_roundoff_within_one_ulp(
            f64::NAN,
            value,
        ));
        assert!(!v11_compositional_pool_roundoff_within_one_ulp(
            -value, value,
        ));
    }

    #[test]
    fn final_owner_join_seal_rejects_each_owner_digest_substitution() {
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(1_800_000_000_000))
            .expect("support");
        let mut receipt = CoveredParentOwnerJoinReceiptV1 {
            run_identity: Digest32::from_bytes([21; 32]),
            parent_interval_sha256: Digest32::from_bytes([20; 32]),
            parent_transaction_sha256: Digest32::from_bytes([22; 32]),
            segment_sha256: Digest32::from_bytes([23; 32]),
            accepted_slab_sha256: Digest32::from_bytes([24; 32]),
            forcing_receipt_sha256: Digest32::from_bytes([25; 32]),
            beginning_complete_owner_set_sha256: Digest32::from_bytes([26; 32]),
            ending_complete_owner_set_sha256: Digest32::from_bytes([27; 32]),
            support,
            final_boundary_receipt_set_sha256: Digest32::from_bytes([1; 32]),
            final_lane_boundary_receipt_set_sha256: Digest32::from_bytes([18; 32]),
            component_carrier_receipt_set_sha256: Digest32::from_bytes([2; 32]),
            snow_soil_heat_receipt_set_sha256: Digest32::from_bytes([19; 32]),
            terminal_snow_soil_heat_receipt_set_sha256: Digest32::from_bytes([30; 32]),
            physical_outcome_ledger_set_sha256: Digest32::from_bytes([31; 32]),
            wb14_child_receipt_set_sha256: Digest32::from_bytes([29; 32]),
            wb14_parent_receipt_set_sha256: None,
            stage3_physical_state_sha256: Digest32::from_bytes([3; 32]),
            vegetation_owner_sha256: Digest32::from_bytes([4; 32]),
            snow_owner_sha256: Digest32::from_bytes([5; 32]),
            land_surface_energy_owner_sha256: Digest32::from_bytes([6; 32]),
            hydrology_owner_sha256: Digest32::from_bytes([7; 32]),
            biogeochemistry_owner_sha256: Digest32::from_bytes([8; 32]),
            soil_thermal_owner_sha256: Digest32::from_bytes([9; 32]),
            surface_liquid_owner_sha256: Digest32::from_bytes([10; 32]),
            receipt_sha256: Digest32::zero(),
        };
        receipt.receipt_sha256 = receipt.reconstructed_digest().expect("join digest");
        receipt.validate_seal().expect("valid join seal");
        for mutate in [
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.ending_complete_owner_set_sha256 = Digest32::from_bytes([28; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.final_lane_boundary_receipt_set_sha256 = Digest32::from_bytes([19; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.vegetation_owner_sha256 = Digest32::from_bytes([11; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.snow_owner_sha256 = Digest32::from_bytes([12; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.land_surface_energy_owner_sha256 = Digest32::from_bytes([13; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.hydrology_owner_sha256 = Digest32::from_bytes([14; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.biogeochemistry_owner_sha256 = Digest32::from_bytes([15; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.soil_thermal_owner_sha256 = Digest32::from_bytes([16; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.surface_liquid_owner_sha256 = Digest32::from_bytes([17; 32]);
            },
        ] {
            let mut poisoned = receipt.clone();
            mutate(&mut poisoned);
            assert!(poisoned.validate_seal().is_err());
        }
    }

    #[test]
    fn valid_alternate_snow_owner_rejects_against_unchanged_physical_bytes() {
        let expected = b"canonical-stage3-and-boundaries";
        let expected_owner =
            V11OwnerEnvelope::try_new("snow".into(), expected.to_vec()).expect("snow owner");
        validate_exact_snow_owner_bytes(expected, &expected_owner).expect("exact snow join");

        let alternate = V11OwnerEnvelope::try_new(
            "snow".into(),
            b"different-valid-canonical-snow-owner".to_vec(),
        )
        .expect("alternate valid snow owner");
        assert!(validate_exact_snow_owner_bytes(expected, &alternate).is_err());
    }
}
