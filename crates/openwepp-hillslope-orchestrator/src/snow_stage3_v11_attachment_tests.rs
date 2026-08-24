#[cfg(test)]
mod tests {
    use super::*;

    fn lifecycle_state(
        mass_swe_m: Option<f64>,
        layer_liquid_m: f64,
        detached_liquid_kg_m2: f64,
    ) -> DirectSnowStage3PersistentState {
        let layers = mass_swe_m.map_or_else(Vec::new, |mass| {
            let mut layer = crate::winter_column::DirectSnowLayerState::new(
                mass,
                mass.max(0.001) * 2.0,
                500.0,
                0.0,
            );
            layer.liquid_water_m = layer_liquid_m;
            vec![layer]
        });
        DirectSnowStage3PersistentState {
            schema_version: 1,
            terminal_event_model: None,
            fingerprint: 0,
            lane_id: 1,
            next_interval_index: 0,
            layers,
            detached_retained_liquid_kg_m2: detached_liquid_kg_m2,
            initial_ice_kg_m2: 0.0,
            initial_retained_liquid_kg_m2: 0.0,
            cumulative_snowfall_kg_m2: 0.0,
            cumulative_external_liquid_kg_m2: 0.0,
            cumulative_deposition_kg_m2: 0.0,
            cumulative_sublimation_kg_m2: 0.0,
            cumulative_melt_kg_m2: 0.0,
            cumulative_unresolved_liquid_kg_m2: 0.0,
            cumulative_complete_energy_j_m2: 0.0,
            cumulative_cold_energy_change_j_m2: 0.0,
            cumulative_terminal_unallocated_energy_j_m2: 0.0,
        }
    }

    #[test]
    fn lane_lifecycle_uses_owner_represented_and_terminal_predicates() {
        assert_eq!(
            stage3_lane_lifecycle(&lifecycle_state(None, 0.0, 0.0), 0.0),
            Stage3LaneLifecycleV1::SnowFree
        );
        assert_eq!(
            stage3_lane_lifecycle(&lifecycle_state(Some(0.001_000_000_000_1), 0.0, 0.0), 0.0),
            Stage3LaneLifecycleV1::ResolvedSnow
        );
        assert_eq!(
            stage3_lane_lifecycle(&lifecycle_state(Some(0.001), 0.0, 0.0), 0.0),
            Stage3LaneLifecycleV1::TerminalPending
        );
        assert_eq!(
            stage3_lane_lifecycle(&lifecycle_state(Some(0.0), 0.001, 0.0), 0.0),
            Stage3LaneLifecycleV1::TerminalPending
        );
        assert_eq!(
            stage3_lane_lifecycle(&lifecycle_state(None, 0.0, 1.0), 0.0),
            Stage3LaneLifecycleV1::TerminalPending
        );
        assert_eq!(
            stage3_lane_lifecycle(&lifecycle_state(None, 0.0, 0.0), 0.001),
            Stage3LaneLifecycleV1::SolidPrecipitationPending
        );
    }

    fn support_identity(ofe_id: &str, tile_id: &str) -> PreparedStage3V11SupportIdentityV1 {
        PreparedStage3V11SupportIdentityV1::new(
            ofe_id.to_owned(),
            tile_id.to_owned(),
            "a".repeat(64),
            Digest32::zero(),
            Vec::new(),
            Digest32::zero(),
        )
    }

    #[test]
    fn parent_support_cadence_is_exactly_1_800_seconds() {
        assert_eq!(STAGE3_V11_PARENT_SUPPORT_NS, 1_800_000_000_000);
        let support = TimeSupport::new(
            ModelTimeNs::new(0),
            ModelTimeNs::new(STAGE3_V11_PARENT_SUPPORT_NS),
        )
        .expect("valid parent support");
        assert_eq!(support.duration_ns(), 1_800_000_000_000);
        assert_eq!(support.duration_s_bits(), 1_800.0_f64.to_bits());
        assert_eq!(
            STAGE3_V11_PARENT_SUPPORT_NS * STAGE3_V11_PARENT_SUPPORT_COUNT as u128,
            86_400_000_000_000
        );
        assert!(validate_parent_support_duration(1_800_000_000).is_err());
        assert!(validate_parent_support_duration(STAGE3_V11_PARENT_SUPPORT_NS + 1).is_err());
        validate_parent_support_duration(STAGE3_V11_PARENT_SUPPORT_NS)
            .expect("1,800-second support accepted");
    }

    #[test]
    fn run_relative_day_supports_are_contiguous_across_midnight() {
        assert_eq!(day_start_ns(0).expect("day zero start"), 0);
        assert_eq!(day_start_ns(1).expect("day one start"), STAGE3_V11_DAY_NS);
        let day_zero_last = TimeSupport::new(
            ModelTimeNs::new(
                STAGE3_V11_DAY_NS
                    .checked_sub(STAGE3_V11_PARENT_SUPPORT_NS)
                    .expect("day zero last support start"),
            ),
            ModelTimeNs::new(STAGE3_V11_DAY_NS),
        )
        .expect("day zero last support");
        let day_one_first = TimeSupport::new(
            ModelTimeNs::new(STAGE3_V11_DAY_NS),
            ModelTimeNs::new(
                STAGE3_V11_DAY_NS
                    .checked_add(STAGE3_V11_PARENT_SUPPORT_NS)
                    .expect("day one first support end"),
            ),
        )
        .expect("day one first support");
        assert_eq!(day_zero_last.end_ns(), day_one_first.start_ns());
        assert_eq!(day_one_first.start_ns().get(), 86_400_000_000_000);
    }

    #[test]
    fn parent_forcing_digest_binds_interval_receipt_identity() {
        let support = TimeSupport::new(
            ModelTimeNs::new(0),
            ModelTimeNs::new(STAGE3_V11_PARENT_SUPPORT_NS),
        )
        .expect("support");
        let mut identities = BTreeMap::from([(7, vec![support_identity("ofe-1", "tile-1")])]);
        let first = canonical_parent_forcing_digest_from_parts(
            0,
            0,
            Digest32::from_bytes([1; 32]),
            support,
            "b".repeat(64).as_str(),
            &identities,
        )
        .expect("first forcing digest");
        identities.get_mut(&7).expect("lane")[0].forcing_receipt_digest =
            Digest32::from_bytes([2; 32]);
        let second = canonical_parent_forcing_digest_from_parts(
            0,
            0,
            Digest32::from_bytes([1; 32]),
            support,
            "b".repeat(64).as_str(),
            &identities,
        )
        .expect("second forcing digest");
        assert_ne!(first, second);
    }

    #[test]
    fn parent_interval_identity_binds_run_relative_support() {
        let first_support = TimeSupport::new(
            ModelTimeNs::new(0),
            ModelTimeNs::new(STAGE3_V11_PARENT_SUPPORT_NS),
        )
        .expect("first support");
        let second_support = TimeSupport::new(
            ModelTimeNs::new(STAGE3_V11_PARENT_SUPPORT_NS),
            ModelTimeNs::new(STAGE3_V11_DAY_NS),
        )
        .expect("second support");
        let first = ParentIntervalId::derive(
            Digest32::from_bytes([1; 32]),
            Digest32::from_bytes([2; 32]),
            Digest32::from_bytes([3; 32]),
            first_support,
        )
        .expect("first parent interval");
        let second = ParentIntervalId::derive(
            Digest32::from_bytes([1; 32]),
            Digest32::from_bytes([2; 32]),
            Digest32::from_bytes([3; 32]),
            second_support,
        )
        .expect("second parent interval");
        assert_ne!(first, second);
    }

    #[test]
    fn lane_destination_permutation_fails_exact_lane_ofe_join() {
        let mut provider_destinations_by_ofe = BTreeMap::new();
        provider_destinations_by_ofe.insert(
            "ofe-1".to_owned(),
            BTreeSet::from([("ofe-1".to_owned(), "tile-1".to_owned())]),
        );
        provider_destinations_by_ofe.insert(
            "ofe-2".to_owned(),
            BTreeSet::from([("ofe-2".to_owned(), "tile-2".to_owned())]),
        );
        let lane_one_identities = vec![support_identity("ofe-2", "tile-2")];
        let lane_two_identities = vec![support_identity("ofe-1", "tile-1")];

        assert!(
            validate_lane_destination_set(
                "ofe-1",
                &lane_one_identities,
                provider_destinations_by_ofe
                    .get("ofe-1")
                    .expect("lane one OFE destinations"),
            )
            .is_err()
        );
        assert!(
            validate_lane_destination_set(
                "ofe-2",
                &lane_two_identities,
                provider_destinations_by_ofe
                    .get("ofe-2")
                    .expect("lane two OFE destinations"),
            )
            .is_err()
        );
    }
}
