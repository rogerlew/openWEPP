#[cfg(test)]
mod nitrogen_protocol_cardinality_tests {
    use super::*;
    use crate::land_surface_energy_shadow::strict_v8_endpoint::endpoint_rollback_tests::{
        endpoint_fixture, two_ofe_routed_endpoint_fixture,
    };

    #[test]
    fn empty_protocol_is_admissible_and_every_partial_empty_protocol_rejects() {
        assert!(validate_v11_nitrogen_protocol_cardinality(0, 0, 0).expect("empty protocol"));
        for counts in [
            (0, 0, 1),
            (0, 1, 0),
            (1, 0, 0),
            (0, 1, 1),
            (1, 0, 1),
            (1, 1, 0),
        ] {
            assert!(
                validate_v11_nitrogen_protocol_cardinality(counts.0, counts.1, counts.2).is_err(),
                "partial-empty poison {counts:?}",
            );
        }
        assert!(!validate_v11_nitrogen_protocol_cardinality(1, 1, 1).expect("nonempty protocol"));
    }

    #[test]
    fn bgc_ofe_resolution_uses_explicit_vegetation_tile_mapping() {
        let mut fixture = two_ofe_routed_endpoint_fixture();
        for tile in &mut fixture.lse_configuration.ofes[0].tiles {
            tile.vegetation_tile_id = openwepp_kernel_contract::TileId::try_new(format!(
                "upper-open-{}",
                tile.tile_id.as_str()
            ))
            .expect("upper open vegetation tile");
        }
        let lower_forest = fixture.lse_configuration.ofes[1]
            .tiles
            .iter_mut()
            .find(|tile| tile.tile_id.as_str() == "lower-forest")
            .expect("lower forest tile");
        assert_ne!(lower_forest.tile_id, lower_forest.vegetation_tile_id);
        assert_eq!(
            v11_bgc_bearing_ofe(
                &fixture.vegetation_configuration.expected_occupancies(),
                &fixture.lse_configuration,
            )
            .expect("open first OFE and vegetated second OFE"),
            "ofe-2"
        );
    }

    #[test]
    fn bgc_ofe_resolution_admits_multi_tile_stratum_within_one_ofe() {
        let mut fixture = endpoint_fixture();
        let second_tile = openwepp_kernel_contract::TileId::try_new("open").expect("tile");
        fixture.vegetation_configuration.strata[0]
            .tile_ids
            .push(second_tile);
        assert_eq!(
            v11_bgc_bearing_ofe(
                &fixture.vegetation_configuration.expected_occupancies(),
                &fixture.lse_configuration,
            )
            .expect("one stratum on multiple vegetation tiles"),
            "ofe-1"
        );
    }

    #[test]
    fn bgc_ofe_resolution_rejects_two_covered_vegetated_ofes() {
        let fixture = two_ofe_routed_endpoint_fixture();
        assert!(
            v11_bgc_bearing_ofe(
                &fixture.vegetation_configuration.expected_occupancies(),
                &fixture.lse_configuration,
            )
            .is_err()
        );
    }

    #[test]
    fn repeated_local_lse_tile_ids_do_not_replace_vegetation_mapping() {
        let mut fixture = two_ofe_routed_endpoint_fixture();
        for tile in &mut fixture.lse_configuration.ofes[0].tiles {
            tile.vegetation_tile_id = openwepp_kernel_contract::TileId::try_new(format!(
                "upper-open-{}",
                tile.tile_id.as_str()
            ))
            .expect("upper open vegetation tile");
        }
        let repeated = fixture.lse_configuration.ofes[0].tiles[0].tile_id.clone();
        fixture.lse_configuration.ofes[1].tiles[0].tile_id = repeated;
        assert_eq!(
            v11_bgc_bearing_ofe(
                &fixture.vegetation_configuration.expected_occupancies(),
                &fixture.lse_configuration,
            )
            .expect("repeated local LSE IDs with unique vegetation mapping"),
            "ofe-2"
        );
    }

    #[test]
    fn bgc_linkage_uses_pre_hash_three_stratum_nonassociative_order() {
        use openwepp_coupled_time::{
            AcceptedSlabId, ModelTimeNs, ParentTransactionId, SegmentId, TimeSupport,
        };
        use openwepp_kernel_contract::{MineralNitrogenKey, SoilLayerId};

        let key = MineralNitrogenKey {
            layer_id: SoilLayerId::try_new("layer-1").expect("layer"),
            species: MineralNitrogenSpecies::Ammonium,
        };
        let shared = V11SharedResourceKey {
            resource: V11SharedResourceKind::Ammonium,
            owner_id: "bgc".into(),
            ofe_id: "ofe-2".into(),
            layer_id: "layer-1".into(),
            source_id: "nh4".into(),
            amount_basis: "kg_n_m2".into(),
        };
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(1)).expect("support");
        let values = [
            0.001_626_199_161_107_315_3,
            0.000_000_038_444_775_879_237_09,
            0.000_000_016_590_450_830_746_63,
        ];
        let make = |ordinal: u8, stratum: &str, amount: f64| V11ResourceDebit {
            receipt_id: Digest32::from_bytes([ordinal; 32]),
            parent_transaction_id: ParentTransactionId::from_digest(Digest32::from_bytes([9; 32])),
            segment_id: SegmentId::from_digest(Digest32::from_bytes([8; 32])),
            accepted_slab_id: AcceptedSlabId::from_digest(Digest32::from_bytes([7; 32])),
            support,
            owner_id: "bgc".into(),
            resource_key: V11ResourceKey::MineralNitrogen(key.clone()),
            ofe_id: "ofe-2".into(),
            tile_id: "stratum_scoped".into(),
            occupancy_id: stratum.into(),
            layer_id: "layer-1".into(),
            source_id: "nh4".into(),
            amount_basis: "kg_n_m2".into(),
            request: amount,
            authorization: amount,
            final_use: amount,
        };
        let debits = vec![
            make(2, "stratum-b", values[1]),
            make(1, "stratum-c", values[2]),
            make(3, "stratum-a", values[0]),
        ];
        let ids = v11_linked_debit_ids(&debits, &shared, true);
        assert_eq!(
            ids,
            vec![
                Digest32::from_bytes([3; 32]),
                Digest32::from_bytes([2; 32]),
                Digest32::from_bytes([1; 32])
            ]
        );
        let semantic = ids.iter().fold(0.0_f64, |sum, id| {
            sum + debits
                .iter()
                .find(|debit| debit.receipt_id == *id)
                .expect("linked")
                .final_use
        });
        let alternate_permutation = debits
            .iter()
            .map(|debit| debit.final_use)
            .fold(0.0_f64, |sum, value| sum + value);
        assert_eq!(
            semantic.to_bits(),
            0.001_626_254_196_334_025_4_f64.to_bits()
        );
        assert_eq!(
            alternate_permutation.to_bits(),
            0.001_626_254_196_334_025_1_f64.to_bits()
        );
        assert_ne!(semantic.to_bits(), alternate_permutation.to_bits());
    }
}
