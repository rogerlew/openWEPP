use super::*;

#[test]
#[allow(clippy::too_many_lines)]
fn bgc_three_stratum_semantic_fold_accepts_restores_and_reordered_debits_rollback() {
    use openwepp_kernel_contract::{MineralNitrogenSpecies, OccupancyId, StratumId};

    let (mut configuration, mut state) = v10_fixture();
    let base_stratum = configuration.strata[0].clone();
    let (base_occupancy_id, base_occupancy) = state
        .0
        .occupancies
        .iter()
        .next()
        .map(|(id, value)| (id.clone(), value.clone()))
        .expect("base occupancy");
    let base_shared = state
        .0
        .strata
        .values()
        .next()
        .cloned()
        .expect("base stratum state");
    configuration.strata.clear();
    state.0.occupancies.clear();
    state.0.strata.clear();
    for (rank, id) in ["stratum-a", "stratum-b", "stratum-c"]
        .into_iter()
        .enumerate()
    {
        let stratum_id = StratumId::try_new(id).expect("stratum");
        let mut stratum = base_stratum.clone();
        stratum.stratum_id = stratum_id.clone();
        stratum.vertical_rank = u32::try_from(rank + 1).expect("rank");
        stratum.height_m = base_stratum.height_m - rank as f64;
        configuration.strata.push(stratum);
        state
            .0
            .strata
            .insert(stratum_id.clone(), base_shared.clone());
        state.0.occupancies.insert(
            OccupancyId {
                stratum_id,
                tile_id: base_occupancy_id.tile_id.clone(),
            },
            base_occupancy.clone(),
        );
    }
    configuration.configuration_sha256 = configuration.canonical_sha256().expect("config");
    state.0.configuration_sha256 = configuration.configuration_sha256.clone();
    state.0.state_sha256 = state.0.canonical_sha256();
    configuration.initial_state_sha256 = state.0.state_sha256.clone();
    let migrated = migrate_v10_runtime_to_v11(&configuration, &state).expect("migrate");
    let owners = complete_owners(&migrated.state);
    let (parent_id, receipts) = accepted_receipts(&owners, &[1_800_000_000_000]);
    let mut parent = V11ParentTransaction::new_with_complete_owners(
        &migrated.configuration,
        &migrated.state,
        parent_id,
        ModelTimeNs::new(0),
        owners.clone(),
    )
    .expect("parent");
    let receipt = &receipts[0];
    let layer = configuration.strata[0].root_layers[0].layer_id.clone();
    let amounts = [
        0.001_626_199_161_107_315_3,
        0.000_000_038_444_775_879_237_09,
        0.000_000_016_590_450_830_746_63,
    ];
    let mut debits = ["stratum-a", "stratum-b", "stratum-c"]
        .into_iter()
        .zip(amounts)
        .flat_map(|(stratum, amount)| {
            [
                (MineralNitrogenSpecies::Ammonium, "nh4", amount),
                (MineralNitrogenSpecies::Nitrate, "no3", 0.0),
            ]
            .into_iter()
            .map(|(species, source, amount)| {
                V11ResourceDebit::new(V11ResourceDebit {
                    receipt_id: Digest32::zero(),
                    parent_transaction_id: receipt.parent_transaction_id(),
                    segment_id: receipt.segment_id(),
                    accepted_slab_id: receipt.slab_id(),
                    support: receipt.support(),
                    owner_id: "bgc".into(),
                    resource_key: V11ResourceKey::MineralNitrogen(MineralNitrogenKey {
                        layer_id: layer.clone(),
                        species,
                    }),
                    ofe_id: "ofe-2".into(),
                    tile_id: "stratum_scoped".into(),
                    occupancy_id: stratum.into(),
                    layer_id: layer.as_str().into(),
                    source_id: source.into(),
                    amount_basis: "kg_n_m2".into(),
                    request: amount,
                    authorization: amount,
                    final_use: amount,
                })
                .expect("debit")
            })
        })
        .collect::<Vec<_>>();
    let beginning = 0.01_f64;
    let nh4_debits = debits
        .iter()
        .filter(|debit| debit.source_id == "nh4")
        .collect::<Vec<_>>();
    let used = nh4_debits
        .iter()
        .fold(0.0_f64, |sum, debit| sum + debit.final_use);
    let alternate_used = nh4_debits
        .iter()
        .rev()
        .fold(0.0_f64, |sum, debit| sum + debit.final_use);
    assert_ne!(used.to_bits(), alternate_used.to_bits());
    let ending = beginning - used;
    let transition = V11SharedResourceOwnerTransition::new(V11SharedResourceOwnerTransition {
        transition_id: Digest32::zero(),
        parent_transaction_id: receipt.parent_transaction_id(),
        segment_id: receipt.segment_id(),
        accepted_slab_id: receipt.slab_id(),
        support: receipt.support(),
        shared_resource_key: V11SharedResourceKey {
            resource: V11SharedResourceKind::Ammonium,
            owner_id: "bgc".into(),
            ofe_id: "ofe-2".into(),
            layer_id: layer.as_str().into(),
            source_id: "nh4".into(),
            amount_basis: "kg_n_m2".into(),
        },
        beginning_amount: beginning,
        ending_amount: ending,
        debit_receipt_ids: nh4_debits.iter().map(|debit| debit.receipt_id).collect(),
        admitted_flux_receipt_ids: vec![],
        owner_candidate_sha256: owners["bgc"].state_sha256,
    })
    .expect("transition");
    let nitrate_transition =
        V11SharedResourceOwnerTransition::new(V11SharedResourceOwnerTransition {
            transition_id: Digest32::zero(),
            parent_transaction_id: receipt.parent_transaction_id(),
            segment_id: receipt.segment_id(),
            accepted_slab_id: receipt.slab_id(),
            support: receipt.support(),
            shared_resource_key: V11SharedResourceKey {
                resource: V11SharedResourceKind::Nitrate,
                owner_id: "bgc".into(),
                ofe_id: "ofe-2".into(),
                layer_id: layer.as_str().into(),
                source_id: "no3".into(),
                amount_basis: "kg_n_m2".into(),
            },
            beginning_amount: beginning,
            ending_amount: beginning,
            debit_receipt_ids: debits
                .iter()
                .filter(|debit| debit.source_id == "no3")
                .map(|debit| debit.receipt_id)
                .collect(),
            admitted_flux_receipt_ids: vec![],
            owner_candidate_sha256: owners["bgc"].state_sha256,
        })
        .expect("nitrate transition");
    let scope = V11BgcDebitScope::try_new(BTreeMap::from([
        ("stratum-a".into(), "ofe-2".into()),
        ("stratum-b".into(), "ofe-2".into()),
        ("stratum-c".into(), "ofe-2".into()),
    ]))
    .expect("scope");
    let mut configured_wrong_stratum = debits.clone();
    configured_wrong_stratum[0].occupancy_id = "stratum-b".into();
    assert!(matches!(
        validate_bgc_debit_configuration(
            &migrated.configuration,
            Some(&scope),
            &configured_wrong_stratum,
        ),
        Err(V11Error::ResourceDebit)
    ));
    let mut bgc_water = debits.clone();
    bgc_water.push(V11ResourceDebit {
        resource_key: V11ResourceKey::Water(WaterResourceKey {
            occupancy_id: OccupancyId {
                stratum_id: StratumId::try_new("stratum-a").expect("stratum"),
                tile_id: base_occupancy_id.tile_id.clone(),
            },
            layer_id: layer.clone(),
        }),
        ..bgc_water[0].clone()
    });
    assert!(matches!(
        validate_bgc_debit_configuration(&migrated.configuration, Some(&scope), &bgc_water),
        Err(V11Error::ResourceDebit)
    ));
    let staged_state = parent.staged_state.clone();
    let ending_state = staged_state.clone();
    let ending_owners = owners.clone();
    let make_candidate = |ordered_debits: Vec<V11ResourceDebit>| {
        let transitions = vec![transition.clone(), nitrate_transition.clone()];
        V11AcceptedSegmentCandidate {
            accepted_slab_receipt: receipt.clone(),
            lse_support_receipt: test_lse_support_receipt(receipt),
            beginning_state_sha256: staged_state.state_sha256.clone(),
            ending_state: ending_state.clone(),
            resource_debits: ordered_debits,
            admitted_resource_fluxes: vec![],
            complete_owner_candidates: build_complete_owner_candidates(
                receipt,
                &ending_owners,
                &transitions,
            )
            .expect("candidates"),
            shared_resource_transitions: transitions,
            material_transfers: vec![],
            ending_resource_owners: ending_owners.clone(),
        }
    };
    let before = parent.checkpoint();
    let mut reversed_links = make_candidate(debits.clone());
    reversed_links.shared_resource_transitions[0]
        .debit_receipt_ids
        .reverse();
    reversed_links.shared_resource_transitions[0] = V11SharedResourceOwnerTransition::new(
        reversed_links.shared_resource_transitions[0].clone(),
    )
    .expect("resealed reversed BGC links");
    reversed_links.complete_owner_candidates = build_complete_owner_candidates(
        receipt,
        &ending_owners,
        &reversed_links.shared_resource_transitions,
    )
    .expect("reversed-link candidates");
    assert!(matches!(
        parent
            .accept_segment_with_bgc_scope(&migrated.configuration, reversed_links, Some(&scope),),
        Err(V11Error::ResourceCustody)
    ));
    assert_eq!(parent.checkpoint(), before);
    debits.reverse();
    assert!(matches!(
        parent.accept_segment_with_bgc_scope(
            &migrated.configuration,
            make_candidate(debits.clone()),
            Some(&scope),
        ),
        Err(V11Error::ResourceDebit)
    ));
    assert_eq!(parent.checkpoint(), before);
    debits.reverse();
    parent
        .accept_segment_with_bgc_scope(
            &migrated.configuration,
            make_candidate(debits),
            Some(&scope),
        )
        .expect("semantic-order acceptance");
    let checkpoint = parent.checkpoint();
    let mut reordered_checkpoint = checkpoint.clone();
    reordered_checkpoint.accepted_segments[0]
        .resource_debits
        .reverse();
    assert!(matches!(
        V11ParentTransaction::restore_with_bgc_scope(
            &migrated.configuration,
            reordered_checkpoint,
            Some(&scope),
        ),
        Err(V11Error::RestartCheckpoint)
    ));
    let restored = V11ParentTransaction::restore_with_bgc_scope(
        &migrated.configuration,
        checkpoint.clone(),
        Some(&scope),
    )
    .expect("semantic-order restore");
    assert_eq!(restored.checkpoint(), checkpoint);
}
