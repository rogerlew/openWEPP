use super::*;

#[test]
#[allow(clippy::too_many_lines)]
fn closed_multi_occupancy_water_and_mineral_n_custody_rejects_aliases() {
    use openwepp_kernel_contract::{MineralNitrogenSpecies, OccupancyId, StratumId, TileId};
    let (mut v10_configuration, mut v10_state) = v10_fixture();
    v10_configuration.strata[0].root_layers.truncate(1);
    v10_configuration.configuration_sha256 = v10_configuration.canonical_sha256().expect("config");
    v10_state.0.configuration_sha256 = v10_configuration.configuration_sha256.clone();
    v10_state.0.state_sha256 = v10_state.0.canonical_sha256();
    v10_configuration.initial_state_sha256 = v10_state.0.state_sha256.clone();
    let migrated = migrate_v10_runtime_to_v11(&v10_configuration, &v10_state).expect("migrate");
    let owners = complete_owners(&migrated.state);
    let (_, receipts) = accepted_receipts(&owners, &[1_800_000_000_000]);
    let receipt = &receipts[0];
    let configured_stratum = migrated
        .configuration
        .imported_v10
        .expected_occupancies()
        .into_iter()
        .next()
        .expect("configured occupancy")
        .stratum_id
        .as_str()
        .to_owned();
    let bgc_scope = V11BgcDebitScope::try_new(BTreeMap::from([(
        configured_stratum.clone(),
        "ofe-1".into(),
    )]))
    .expect("BGC scope");
    let layer = migrated.configuration.imported_v10.strata[0].root_layers[0]
        .layer_id
        .clone();
    let layer_id = layer.as_str().to_owned();
    let water_key = V11ResourceKey::Water(WaterResourceKey {
        occupancy_id: OccupancyId {
            stratum_id: StratumId::try_new("s1").expect("s"),
            tile_id: TileId::try_new("t1").expect("t"),
        },
        layer_id: layer.clone(),
    });
    let nh4 = V11ResourceKey::MineralNitrogen(MineralNitrogenKey {
        layer_id: layer.clone(),
        species: MineralNitrogenSpecies::Ammonium,
    });
    let no3 = V11ResourceKey::MineralNitrogen(MineralNitrogenKey {
        layer_id: layer,
        species: MineralNitrogenSpecies::Nitrate,
    });
    let make = |occupancy: &str, key: V11ResourceKey, owner: &str, source: &str, amount: f64| {
        V11ResourceDebit::new(V11ResourceDebit {
            receipt_id: Digest32::zero(),
            parent_transaction_id: receipt.parent_transaction_id(),
            segment_id: receipt.segment_id(),
            accepted_slab_id: receipt.slab_id(),
            support: receipt.support(),
            owner_id: owner.into(),
            resource_key: key,
            ofe_id: "ofe-1".into(),
            tile_id: if owner == "bgc" {
                "stratum_scoped".into()
            } else {
                format!("tile-{occupancy}")
            },
            occupancy_id: occupancy.into(),
            layer_id: layer_id.clone(),
            source_id: source.into(),
            amount_basis: if owner == "bgc" {
                "kg_n_m2".into()
            } else {
                "kg_m2".into()
            },
            request: amount,
            authorization: amount,
            final_use: amount,
        })
        .expect("debit")
    };
    let mut debits = vec![
        make("a", water_key.clone(), "hydrology", "soil_water", 4.0),
        make("b", water_key, "hydrology", "soil_water", 4.0),
        make(&configured_stratum, nh4.clone(), "bgc", "nh4", 0.1),
        make(&configured_stratum, no3.clone(), "bgc", "no3", 0.2),
    ];
    debits.sort_by(|left, right| {
        left.owner_id.cmp(&right.owner_id).then_with(|| {
            if left.owner_id == "bgc" {
                left.occupancy_id
                    .cmp(&right.occupancy_id)
                    .then_with(|| left.layer_id.cmp(&right.layer_id))
                    .then_with(|| left.resource_key.cmp(&right.resource_key))
            } else {
                left.receipt_id.cmp(&right.receipt_id)
            }
        })
    });
    let transition = |owner: &str,
                      key: V11ResourceKey,
                      source: &str,
                      begin: f64,
                      end: f64,
                      ids: Vec<Digest32>| {
        let resource = match key {
            V11ResourceKey::Water(_) => V11SharedResourceKind::Water,
            V11ResourceKey::MineralNitrogen(key) => match key.species {
                MineralNitrogenSpecies::Ammonium => V11SharedResourceKind::Ammonium,
                MineralNitrogenSpecies::Nitrate => V11SharedResourceKind::Nitrate,
            },
        };
        V11SharedResourceOwnerTransition::new(V11SharedResourceOwnerTransition {
            transition_id: Digest32::zero(),
            parent_transaction_id: receipt.parent_transaction_id(),
            segment_id: receipt.segment_id(),
            accepted_slab_id: receipt.slab_id(),
            support: receipt.support(),
            shared_resource_key: V11SharedResourceKey {
                resource,
                owner_id: owner.into(),
                ofe_id: "ofe-1".into(),
                layer_id: layer_id.clone(),
                source_id: source.into(),
                amount_basis: if owner == "bgc" {
                    "kg_n_m2".into()
                } else {
                    "kg_m2".into()
                },
            },
            beginning_amount: begin,
            ending_amount: end,
            debit_receipt_ids: ids,
            admitted_flux_receipt_ids: vec![],
            owner_candidate_sha256: owners[owner].state_sha256,
        })
        .expect("transition")
    };
    let water_ids = debits
        .iter()
        .filter(|d| d.owner_id == "hydrology")
        .map(|d| d.receipt_id)
        .collect();
    let transitions = vec![
        transition(
            "bgc",
            nh4,
            "nh4",
            1.0,
            0.9,
            vec![
                debits
                    .iter()
                    .find(|d| d.source_id == "nh4")
                    .unwrap()
                    .receipt_id,
            ],
        ),
        transition(
            "bgc",
            no3,
            "no3",
            2.0,
            1.8,
            vec![
                debits
                    .iter()
                    .find(|d| d.source_id == "no3")
                    .unwrap()
                    .receipt_id,
            ],
        ),
        transition(
            "hydrology",
            debits
                .iter()
                .find(|d| d.owner_id == "hydrology")
                .unwrap()
                .resource_key
                .clone(),
            "soil_water",
            10.0,
            2.0,
            water_ids,
        ),
    ];
    let candidates =
        build_complete_owner_candidates(receipt, &owners, &transitions).expect("candidates");
    validate_resource_custody(
        &migrated.configuration,
        Some(&bgc_scope),
        receipt.parent_transaction_id(),
        receipt.segment_id(),
        receipt.slab_id(),
        receipt.slab_ordinal(),
        receipt.support(),
        &debits,
        &[],
        &transitions,
        &candidates,
        None,
    )
    .expect("valid custody");

    let assert_poison =
        |poisoned_debits: &[V11ResourceDebit],
         poisoned_transitions: &[V11SharedResourceOwnerTransition]| {
            let poisoned_candidates =
                build_complete_owner_candidates(receipt, &owners, poisoned_transitions)
                    .expect("poison candidates");
            assert!(
                validate_resource_custody(
                    &migrated.configuration,
                    Some(&bgc_scope),
                    receipt.parent_transaction_id(),
                    receipt.segment_id(),
                    receipt.slab_id(),
                    receipt.slab_ordinal(),
                    receipt.support(),
                    poisoned_debits,
                    &[],
                    poisoned_transitions,
                    &poisoned_candidates,
                    None,
                )
                .is_err()
            );
        };

    let mut wrong_delta = transitions.clone();
    wrong_delta[0].ending_amount = 0.8;
    wrong_delta[0] =
        V11SharedResourceOwnerTransition::new(wrong_delta[0].clone()).expect("rebind wrong delta");
    assert_poison(&debits, &wrong_delta);

    let omitted = transitions[1..].to_vec();
    assert_poison(&debits, &omitted);

    let mut substituted = debits.clone();
    let nitrogen = substituted
        .iter_mut()
        .find(|debit| debit.source_id == "nh4")
        .expect("nitrogen debit");
    nitrogen.ofe_id = "ofe-2".into();
    *nitrogen = V11ResourceDebit::new(nitrogen.clone()).expect("rebind substituted OFE");
    substituted.sort_by_key(|debit| debit.receipt_id);
    assert_poison(&substituted, &transitions);

    let assert_resealed_bgc_scope_poison = |mutate: fn(&mut V11ResourceDebit)| {
        let mut poisoned_debits = debits.clone();
        let debit = poisoned_debits
            .iter_mut()
            .find(|debit| debit.source_id == "nh4")
            .expect("NH4 debit");
        let old_id = debit.receipt_id;
        mutate(debit);
        *debit = V11ResourceDebit::new(debit.clone()).expect("resealed debit");
        let new_id = debit.receipt_id;
        let mut poisoned_transitions = transitions.clone();
        let transition = poisoned_transitions
            .iter_mut()
            .find(|transition| transition.debit_receipt_ids.contains(&old_id))
            .expect("linked transition");
        transition.debit_receipt_ids = vec![new_id];
        *transition =
            V11SharedResourceOwnerTransition::new(transition.clone()).expect("resealed transition");
        assert_poison(&poisoned_debits, &poisoned_transitions);
    };
    assert_resealed_bgc_scope_poison(|debit| debit.tile_id = "occupancy_scoped".into());
    assert_resealed_bgc_scope_poison(|debit| debit.occupancy_id = "unknown-stratum".into());
    assert_resealed_bgc_scope_poison(|debit| debit.source_id = "no3".into());
    assert_resealed_bgc_scope_poison(|debit| debit.layer_id = "wrong-layer".into());
    assert_resealed_bgc_scope_poison(|debit| debit.amount_basis = "kg_m2".into());

    let mut reversed = transitions.clone();
    let water = reversed
        .iter_mut()
        .find(|transition| transition.shared_resource_key.owner_id == "hydrology")
        .expect("water transition");
    water.debit_receipt_ids.reverse();
    *water = V11SharedResourceOwnerTransition::new(water.clone()).expect("rebind order");
    assert_poison(&debits, &reversed);

    let mut duplicate = transitions.clone();
    let nh4_transition = duplicate
        .iter_mut()
        .find(|transition| {
            transition.shared_resource_key.resource == V11SharedResourceKind::Ammonium
        })
        .expect("NH4 transition");
    nh4_transition
        .debit_receipt_ids
        .push(nh4_transition.debit_receipt_ids[0]);
    *nh4_transition =
        V11SharedResourceOwnerTransition::new(nh4_transition.clone()).expect("rebind duplicate");
    assert_poison(&debits, &duplicate);

    let mut overbook = debits.clone();
    for d in overbook.iter_mut().filter(|d| d.owner_id == "hydrology") {
        d.request = 6.;
        d.authorization = 6.;
        d.final_use = 6.;
        *d = V11ResourceDebit::new(d.clone()).expect("rebind");
    }
    overbook.sort_by_key(|d| d.receipt_id);
    assert!(
        validate_resource_custody(
            &migrated.configuration,
            Some(&bgc_scope),
            receipt.parent_transaction_id(),
            receipt.segment_id(),
            receipt.slab_id(),
            receipt.slab_ordinal(),
            receipt.support(),
            &overbook,
            &[],
            &transitions,
            &candidates,
            None
        )
        .is_err()
    );
}
