#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_inputs::SnowPhasePartitionModel;

    fn precipitation_test_digest(label: &[u8]) -> Digest32 {
        digest_bytes(label)
    }

    fn precipitation_test_set() -> Stage3PrecipitationPhaseParcelSetV1 {
        let ofe_id = OfeId::try_new("ofe-precip").expect("OFE");
        let open_tile = TileId::try_new("open").expect("tile");
        let covered_tile = TileId::try_new("covered").expect("tile");
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(1_800_000_000_000))
            .expect("support");
        let destinations = vec![
            Stage3PrecipitationDestinationV1 {
                topology_index: 0,
                ofe_id: ofe_id.clone(),
                tile_id: open_tile.clone(),
                fraction_of_ofe: 0.25,
                canopy_covered: false,
                destination_identity_sha256: precipitation_test_digest(b"open-destination"),
            },
            Stage3PrecipitationDestinationV1 {
                topology_index: 1,
                ofe_id: ofe_id.clone(),
                tile_id: covered_tile.clone(),
                fraction_of_ofe: 0.75,
                canopy_covered: true,
                destination_identity_sha256: precipitation_test_digest(b"covered-destination"),
            },
        ];
        let parcel = |destination_topology_index,
                      destination_tile_id,
                      phase,
                      source,
                      mass,
                      _receipt: &'static [u8]| {
            Stage3PrecipitationPhaseParcelV1 {
                support,
                lane_id: 7,
                destination_topology_index,
                destination_ofe_id: ofe_id.clone(),
                destination_tile_id,
                phase,
                source,
                semantic_receipt_ordinal: 0,
                mass_kg_m2_tile_ground: mass,
                enthalpy_provider: Stage3PrecipitationEnthalpyProviderV1::SpecificEnthalpy {
                    specific_enthalpy_j_kg: 100.0,
                    provider_receipt_sha256: precipitation_test_digest(b"enthalpy-provider"),
                },
                source_identity_sha256: precipitation_test_digest(b"source"),
                producer_beginning_state_sha256: precipitation_test_digest(b"producer-beginning"),
                receipt_sha256: Digest32::zero(),
            }
            .seal()
            .expect("sealed parcel")
        };
        Stage3PrecipitationPhaseParcelSetV1 {
            schema_version: 1,
            support,
            lane_id: 7,
            ofe_id: ofe_id.clone(),
            ofe_ground_basis: true,
            beginning_snow_state_sha256: precipitation_test_digest(b"beginning-snow"),
            topology_identity_sha256: precipitation_test_digest(b"topology"),
            destinations,
            parcels: vec![
                parcel(
                    0,
                    open_tile.clone(),
                    Stage3PrecipitationPhaseV1::Solid,
                    Stage3PrecipitationSourceV1::AtmosphericGroundSnow,
                    2.0,
                    b"open-solid",
                ),
                parcel(
                    0,
                    open_tile,
                    Stage3PrecipitationPhaseV1::Liquid,
                    Stage3PrecipitationSourceV1::OpenRawRain,
                    4.0,
                    b"open-liquid",
                ),
                parcel(
                    1,
                    covered_tile.clone(),
                    Stage3PrecipitationPhaseV1::Solid,
                    Stage3PrecipitationSourceV1::AtmosphericGroundSnow,
                    2.0,
                    b"covered-solid",
                ),
                parcel(
                    1,
                    covered_tile.clone(),
                    Stage3PrecipitationPhaseV1::Liquid,
                    Stage3PrecipitationSourceV1::VegetationTerminalThroughfall,
                    3.0,
                    b"covered-throughfall",
                ),
                parcel(
                    1,
                    covered_tile.clone(),
                    Stage3PrecipitationPhaseV1::Liquid,
                    Stage3PrecipitationSourceV1::VegetationTerminalInitialDrainage,
                    0.5,
                    b"covered-initial-drainage",
                ),
                parcel(
                    1,
                    covered_tile.clone(),
                    Stage3PrecipitationPhaseV1::Liquid,
                    Stage3PrecipitationSourceV1::VegetationTerminalSecondDrainage,
                    0.5,
                    b"covered-second-drainage",
                ),
                parcel(
                    1,
                    covered_tile,
                    Stage3PrecipitationPhaseV1::Liquid,
                    Stage3PrecipitationSourceV1::VegetationTerminalStemflow,
                    1.0,
                    b"covered-stemflow",
                ),
            ],
            receipt_sha256: Digest32::zero(),
        }
        .seal()
        .expect("sealed precipitation set")
    }

    #[test]
    fn precipitation_phase_parcel_set_closes_mass_and_advection_from_same_ordered_set() {
        let set = precipitation_test_set();
        validate_precipitation_phase_parcel_set(&set).expect("valid set");
        let (mass, heat) =
            reconstruct_precipitation_mass_and_advected_heat(&set).expect("reconstruction");
        assert_eq!(mass.to_bits(), 6.75_f64.to_bits());
        assert_eq!(heat.to_bits(), 675.0_f64.to_bits());

        let mut empty = set.clone();
        empty.parcels.clear();
        empty.receipt_sha256 = Digest32::zero();
        let empty = empty.seal().expect("complete empty set");
        assert_eq!(
            reconstruct_precipitation_mass_and_advected_heat(&empty).expect("empty reconstruction"),
            (0.0, 0.0)
        );
    }

    #[test]
    fn precipitation_phase_parcel_set_rejects_exclusivity_order_and_identity_poisons() {
        let valid = precipitation_test_set();

        let mut covered_raw_rain = valid.clone();
        covered_raw_rain.parcels[3].source = Stage3PrecipitationSourceV1::OpenRawRain;
        covered_raw_rain.receipt_sha256 =
            precipitation_parcel_set_digest(&covered_raw_rain).expect("digest");
        assert!(matches!(
            validate_precipitation_phase_parcel_set(&covered_raw_rain),
            Err(DirectSnowStage3V11AttachmentError::Precipitation(_))
        ));

        let mut intercepted_solid = valid.clone();
        intercepted_solid.parcels[2].source =
            Stage3PrecipitationSourceV1::VegetationTerminalThroughfall;
        intercepted_solid.receipt_sha256 =
            precipitation_parcel_set_digest(&intercepted_solid).expect("digest");
        assert!(matches!(
            validate_precipitation_phase_parcel_set(&intercepted_solid),
            Err(DirectSnowStage3V11AttachmentError::Precipitation(_))
        ));

        let mut reordered = valid.clone();
        reordered.parcels.swap(0, 1);
        reordered.receipt_sha256 = precipitation_parcel_set_digest(&reordered).expect("digest");
        assert!(matches!(
            validate_precipitation_phase_parcel_set(&reordered),
            Err(DirectSnowStage3V11AttachmentError::Precipitation(_))
        ));

        let mut wrong_beginning = valid;
        wrong_beginning.beginning_snow_state_sha256 = Digest32::zero();
        wrong_beginning.receipt_sha256 =
            precipitation_parcel_set_digest(&wrong_beginning).expect("digest");
        assert!(matches!(
            validate_precipitation_phase_parcel_set(&wrong_beginning),
            Err(DirectSnowStage3V11AttachmentError::Precipitation(_))
        ));

        let mut nonfinite_mass = precipitation_test_set();
        nonfinite_mass.parcels[0].mass_kg_m2_tile_ground = f64::NAN;
        nonfinite_mass.receipt_sha256 =
            precipitation_parcel_set_digest(&nonfinite_mass).expect("digest");
        assert!(matches!(
            validate_precipitation_phase_parcel_set(&nonfinite_mass),
            Err(DirectSnowStage3V11AttachmentError::Precipitation(_))
        ));

        let mut substituted_receipt = precipitation_test_set();
        substituted_receipt.parcels[0].receipt_sha256 = precipitation_test_digest(b"substitute");
        assert!(matches!(
            validate_precipitation_phase_parcel_set(&substituted_receipt),
            Err(DirectSnowStage3V11AttachmentError::Precipitation(_))
        ));

        let mut omitted_route = precipitation_test_set();
        omitted_route.parcels.remove(4);
        assert!(matches!(
            validate_precipitation_phase_parcel_set(&omitted_route),
            Err(DirectSnowStage3V11AttachmentError::Precipitation(_))
        ));

        let mut duplicate_route = precipitation_test_set();
        let mut duplicate = duplicate_route.parcels[3].clone();
        duplicate.mass_kg_m2_tile_ground = 0.25;
        duplicate.source_identity_sha256 = precipitation_test_digest(b"duplicate-source");
        duplicate = duplicate.seal().expect("duplicate parcel seal");
        duplicate_route.parcels.push(duplicate);
        duplicate_route
            .parcels
            .sort_by_key(precipitation_parcel_key);
        duplicate_route.receipt_sha256 =
            precipitation_parcel_set_digest(&duplicate_route).expect("digest");
        assert!(matches!(
            validate_precipitation_phase_parcel_set(&duplicate_route),
            Err(DirectSnowStage3V11AttachmentError::Precipitation(_))
        ));

        let mut repeated_raw = precipitation_test_set();
        let first_raw = repeated_raw.parcels[1].clone();
        let mut second_raw = first_raw.clone();
        second_raw.semantic_receipt_ordinal = 1;
        second_raw.mass_kg_m2_tile_ground = 0.125;
        let second_raw = (0_u32..10_000)
            .find_map(|nonce| {
                let mut candidate = second_raw.clone();
                candidate.source_identity_sha256 = digest_bytes(&nonce.to_be_bytes());
                let candidate = candidate.seal().ok()?;
                (candidate.receipt_sha256 < first_raw.receipt_sha256).then_some(candidate)
            })
            .expect("opposite hash order fixture");
        assert!(second_raw.receipt_sha256 < first_raw.receipt_sha256);
        repeated_raw.parcels.push(second_raw);
        repeated_raw.parcels.sort_by_key(precipitation_parcel_key);
        repeated_raw.receipt_sha256 =
            precipitation_parcel_set_digest(&repeated_raw).expect("digest");
        validate_precipitation_phase_parcel_set(&repeated_raw)
            .expect("semantic order ignores receipt hash order");

        let mut wrong_support = precipitation_test_set();
        wrong_support.parcels[0].support =
            TimeSupport::new(ModelTimeNs::new(1), ModelTimeNs::new(1_800_000_000_001))
                .expect("shifted support");
        wrong_support.parcels[0] = wrong_support.parcels[0]
            .clone()
            .seal()
            .expect("resealed parcel");
        wrong_support.receipt_sha256 =
            precipitation_parcel_set_digest(&wrong_support).expect("digest");
        assert!(matches!(
            validate_precipitation_phase_parcel_set(&wrong_support),
            Err(DirectSnowStage3V11AttachmentError::Precipitation(_))
        ));

        let mut sealed_zero_mass = precipitation_test_set();
        let mut zero_parcel = sealed_zero_mass.parcels[0].clone();
        zero_parcel.mass_kg_m2_tile_ground = 0.0;
        sealed_zero_mass.parcels = vec![zero_parcel.seal().expect("sealed zero-mass parcel")];
        sealed_zero_mass.receipt_sha256 =
            precipitation_parcel_set_digest(&sealed_zero_mass).expect("set digest");
        assert!(matches!(
            validate_precipitation_phase_parcel_set(&sealed_zero_mass),
            Err(DirectSnowStage3V11AttachmentError::Precipitation(_))
        ));

        let mut negative_zero = precipitation_test_set();
        negative_zero.parcels[0].mass_kg_m2_tile_ground = -0.0;
        negative_zero.parcels[0] = negative_zero.parcels[0]
            .clone()
            .seal()
            .expect("sealed negative-zero parcel");
        negative_zero.receipt_sha256 =
            precipitation_parcel_set_digest(&negative_zero).expect("set digest");
        assert!(matches!(
            validate_precipitation_phase_parcel_set(&negative_zero),
            Err(DirectSnowStage3V11AttachmentError::Precipitation(_))
        ));
    }

    fn snow_soil_test_receipt() -> SnowSoilHeatReceiptV1 {
        let (beginning, ending, accepted) =
            snow_soil_heat_w_m2_ofe_ground(0.1, 0.2, 0.2, 0.4, 274.0, 272.0, 273.0, 272.0)
                .expect("snow-soil heat");
        SnowSoilHeatReceiptV1 {
            schema_version: 1,
            model_identity_sha256: digest_bytes(b"snow-soil-model"),
            support: TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(1_800_000_000_000))
                .expect("support"),
            support_duration_ns: 1_800_000_000_000,
            lane_id: 7,
            ofe_id: OfeId::try_new("ofe-soil-heat").expect("OFE"),
            ofe_ground_basis: true,
            topology_identity_sha256: digest_bytes(b"soil-heat-topology"),
            configuration_identity_sha256: digest_bytes(b"soil-heat-configuration"),
            beginning_snow_owner_identity_sha256: digest_bytes(b"beginning-snow-owner"),
            beginning_soil_owner_identity_sha256: digest_bytes(b"beginning-soil-owner"),
            bottom_snow_layer_id: 1,
            first_soil_layer_id: SoilLayerId::try_new("thermal-top").expect("soil layer"),
            bottom_snow_half_thickness_m: 0.1,
            bottom_snow_conductivity_w_m_k: 0.2,
            top_soil_half_thickness_m: 0.2,
            top_soil_conductivity_w_m_k: 0.4,
            beginning_bottom_snow_temperature_k: 274.0,
            beginning_top_soil_temperature_k: 272.0,
            ending_bottom_snow_temperature_k: 273.0,
            ending_top_soil_temperature_k: 272.0,
            beginning_heat_flux_w_m2_ofe_ground: beginning,
            ending_heat_flux_w_m2_ofe_ground: ending,
            accepted_heat_flux_w_m2_ofe_ground: accepted,
            accepted_heat_j_m2_ofe_ground: accepted * 1_800.0,
            snow_candidate_heat_j_m2_ofe_ground: -accepted * 1_800.0,
            soil_candidate_heat_j_m2_ofe_ground: accepted * 1_800.0,
            snow_candidate_ending_identity_sha256: digest_bytes(b"ending-snow-candidate"),
            soil_candidate_ending_identity_sha256: digest_bytes(b"ending-soil-candidate"),
            receipt_sha256: Digest32::zero(),
        }
        .seal()
        .expect("sealed snow-soil receipt")
    }

    #[test]
    fn snow_soil_heat_reconstructs_positive_zero_and_negative_cn_flux() {
        let receipt = snow_soil_test_receipt();
        validate_snow_soil_heat_receipt(&receipt).expect("valid receipt");
        assert_eq!(
            receipt.beginning_heat_flux_w_m2_ofe_ground.to_bits(),
            2.0_f64.to_bits()
        );
        assert_eq!(
            receipt.ending_heat_flux_w_m2_ofe_ground.to_bits(),
            1.0_f64.to_bits()
        );
        assert_eq!(
            receipt.accepted_heat_flux_w_m2_ofe_ground.to_bits(),
            1.5_f64.to_bits()
        );
        assert_eq!(
            receipt.accepted_heat_j_m2_ofe_ground.to_bits(),
            2_700.0_f64.to_bits()
        );
        assert_eq!(
            receipt.snow_candidate_heat_j_m2_ofe_ground.to_bits(),
            (-2_700.0_f64).to_bits()
        );
        assert_eq!(
            receipt.soil_candidate_heat_j_m2_ofe_ground.to_bits(),
            2_700.0_f64.to_bits()
        );

        let zero = snow_soil_heat_w_m2_ofe_ground(0.1, 0.2, 0.2, 0.4, 273.0, 273.0, 272.0, 272.0)
            .expect("zero heat");
        assert_eq!(zero, (0.0, 0.0, 0.0));

        let upward = snow_soil_heat_w_m2_ofe_ground(0.1, 0.2, 0.2, 0.4, 271.0, 273.0, 272.0, 273.0)
            .expect("upward heat");
        assert_eq!(upward.0.to_bits(), (-2.0_f64).to_bits());
        assert_eq!(upward.1.to_bits(), (-1.0_f64).to_bits());
        assert_eq!(upward.2.to_bits(), (-1.5_f64).to_bits());
    }

    #[test]
    fn snow_soil_heat_rejects_substitution_nonfinite_and_one_bit_poisons() {
        let valid = snow_soil_test_receipt();

        let mut substituted_owner = valid.clone();
        substituted_owner.beginning_soil_owner_identity_sha256 = digest_bytes(b"substitute");
        assert!(matches!(
            validate_snow_soil_heat_receipt(&substituted_owner),
            Err(DirectSnowStage3V11AttachmentError::SnowSoilHeat(_))
        ));

        let valid = snow_soil_test_receipt();
        validate_snow_soil_heat_receipt_installed_join(
            &valid,
            &valid.first_soil_layer_id,
            valid.snow_candidate_ending_identity_sha256,
            valid.soil_candidate_ending_identity_sha256,
        )
        .expect("exact installed join");
        let wrong_node = SoilLayerId::try_new("adjacent-layer").expect("wrong node");
        assert!(matches!(
            validate_snow_soil_heat_receipt_installed_join(
                &valid,
                &wrong_node,
                valid.snow_candidate_ending_identity_sha256,
                valid.soil_candidate_ending_identity_sha256,
            ),
            Err(DirectSnowStage3V11AttachmentError::SnowSoilHeat(
                "typed node or canonical installed candidate identity"
            ))
        ));
        let mut one_bit_snow = *valid.snow_candidate_ending_identity_sha256.as_bytes();
        one_bit_snow[0] ^= 1;
        assert!(
            validate_snow_soil_heat_receipt_installed_join(
                &valid,
                &valid.first_soil_layer_id,
                Digest32::from_bytes(one_bit_snow),
                valid.soil_candidate_ending_identity_sha256,
            )
            .is_err()
        );
        let stale_soil = digest_bytes(b"stale installed soil OFE");
        assert!(
            validate_snow_soil_heat_receipt_installed_join(
                &valid,
                &valid.first_soil_layer_id,
                valid.snow_candidate_ending_identity_sha256,
                stale_soil,
            )
            .is_err()
        );

        let mut wrong_sign = valid.clone();
        wrong_sign.accepted_heat_flux_w_m2_ofe_ground =
            -wrong_sign.accepted_heat_flux_w_m2_ofe_ground;
        wrong_sign.receipt_sha256 = snow_soil_heat_receipt_digest(&wrong_sign).expect("digest");
        assert!(matches!(
            validate_snow_soil_heat_receipt(&wrong_sign),
            Err(DirectSnowStage3V11AttachmentError::SnowSoilHeat(_))
        ));

        let mut wrong_candidate_debit = valid.clone();
        wrong_candidate_debit.snow_candidate_heat_j_m2_ofe_ground =
            wrong_candidate_debit.accepted_heat_j_m2_ofe_ground;
        wrong_candidate_debit.receipt_sha256 =
            snow_soil_heat_receipt_digest(&wrong_candidate_debit).expect("digest");
        assert!(matches!(
            validate_snow_soil_heat_receipt(&wrong_candidate_debit),
            Err(DirectSnowStage3V11AttachmentError::SnowSoilHeat(_))
        ));

        let mut nonfinite = valid.clone();
        nonfinite.ending_top_soil_temperature_k = f64::NAN;
        nonfinite.receipt_sha256 = snow_soil_heat_receipt_digest(&nonfinite).expect("digest");
        assert!(matches!(
            validate_snow_soil_heat_receipt(&nonfinite),
            Err(DirectSnowStage3V11AttachmentError::SnowSoilHeat(_))
        ));

        let mut one_bit_operand = valid.clone();
        one_bit_operand.bottom_snow_half_thickness_m =
            f64::from_bits(one_bit_operand.bottom_snow_half_thickness_m.to_bits() ^ 1);
        assert!(matches!(
            validate_snow_soil_heat_receipt(&one_bit_operand),
            Err(DirectSnowStage3V11AttachmentError::SnowSoilHeat(_))
        ));

        let mut one_bit_seal = valid;
        let mut bytes = *one_bit_seal.receipt_sha256.as_bytes();
        bytes[31] ^= 1;
        one_bit_seal.receipt_sha256 = Digest32::from_bytes(bytes);
        assert!(matches!(
            validate_snow_soil_heat_receipt(&one_bit_seal),
            Err(DirectSnowStage3V11AttachmentError::SnowSoilHeat(_))
        ));

        assert!(matches!(
            snow_soil_heat_w_m2_ofe_ground(0.1, 0.2, 0.2, 0.4, f64::INFINITY, 273.0, 273.0, 273.0,),
            Err(DirectSnowStage3V11AttachmentError::SnowSoilHeat(_))
        ));
    }

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

    #[test]
    fn sequential_state_selects_meltout_then_solid_reappearance_without_future_regime_input() {
        let persistent = lifecycle_state(Some(0.002), 0.0, 0.0);
        let terminal = lifecycle_state(Some(0.000_5), 0.0, 0.0);
        let snow_free = lifecycle_state(None, 0.0, 0.0);
        let reappearing = lifecycle_state(None, 0.0, 0.0);
        let represented_again = lifecycle_state(Some(0.002), 0.0, 0.0);
        assert_eq!(
            [
                stage3_lane_lifecycle(&persistent, 0.0),
                stage3_lane_lifecycle(&terminal, 0.0),
                stage3_lane_lifecycle(&snow_free, 0.0),
                stage3_lane_lifecycle(&reappearing, 0.001),
                stage3_lane_lifecycle(&represented_again, 0.0),
            ],
            [
                Stage3LaneLifecycleV1::ResolvedSnow,
                Stage3LaneLifecycleV1::TerminalPending,
                Stage3LaneLifecycleV1::SnowFree,
                Stage3LaneLifecycleV1::SolidPrecipitationPending,
                Stage3LaneLifecycleV1::ResolvedSnow,
            ]
        );
    }

    #[test]
    fn exact_terminal_domain_crosses_parent_without_parcel_or_stale_state() {
        let mut terminal = lifecycle_state(Some(0.000_5), 0.001, 0.0);
        terminal.schema_version = 2;
        terminal.terminal_event_model =
            Some(crate::hydrology::DirectSnowTerminalEventModel::EnthalpyEventV1);
        assert!(terminal_domain_can_cross_parent_support(&terminal, false));
        assert!(!terminal_domain_can_cross_parent_support(&terminal, true));

        let mut stale = terminal.clone();
        stale.schema_version = 1;
        stale.terminal_event_model = None;
        assert!(!terminal_domain_can_cross_parent_support(&stale, false));

        let mut liquid_only = terminal;
        liquid_only.layers.clear();
        liquid_only.detached_retained_liquid_kg_m2 = 1.0;
        assert!(!terminal_domain_can_cross_parent_support(
            &liquid_only,
            false
        ));
    }

    #[test]
    fn multi_lane_covered_lifecycle_is_lane_keyed_without_candidate_mutation() {
        let first = lifecycle_state(Some(0.002), 0.0, 0.0);
        let mut second = first.clone();
        second.lane_id = 2;
        let lanes = BTreeMap::from([(1, first), (2, second)]);
        let frozen = lanes.clone();

        assert_eq!(
            lanes
                .values()
                .filter(|state| stage3_is_resolved_thermal_domain(state))
                .count(),
            2
        );
        assert_eq!(lanes, frozen, "admission must not mutate candidate owners");
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

    fn terminal_ledger_fixture() -> Stage3V11TerminalPhysicalLedgerV1 {
        Stage3V11TerminalPhysicalLedgerV1 {
            support: TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(60_000_000_000))
                .expect("terminal support"),
            event_result_set_sha256: digest_bytes(b"events"),
            proposal_core_sha256: digest_bytes(b"proposal"),
            accepted_event_receipt_sha256: digest_bytes(b"accepted"),
            accepted_event_ledger_sha256: digest_bytes(b"accepted-ledger"),
            produced_unconsumed_parcel_set_sha256: digest_bytes(b"parcels"),
            beginning_owner_set_sha256: digest_bytes(b"begin-owners"),
            ending_owner_set_sha256: digest_bytes(b"end-owners"),
            ending_snow_owner_sha256: digest_bytes(b"end-snow"),
            evaluated_seconds: 60.0,
            snow_soil_heat_j_m2: -125.0,
            receipt_sha256: Digest32::zero(),
        }
        .seal()
        .expect("terminal ledger seal")
    }

    #[test]
    fn terminal_physical_ledger_is_nonempty_and_self_reconstructing() {
        let ledger = terminal_ledger_fixture();
        assert_ne!(ledger.receipt_sha256, Digest32::zero());
        ledger.validate().expect("terminal ledger validation");
    }

    #[test]
    fn terminal_physical_ledger_rejects_owner_parcel_and_heat_poisons() {
        for poison in 0..3 {
            let mut ledger = terminal_ledger_fixture();
            match poison {
                0 => ledger.ending_owner_set_sha256 = digest_bytes(b"wrong-owner"),
                1 => ledger.produced_unconsumed_parcel_set_sha256 = digest_bytes(b"wrong-parcel"),
                _ => ledger.snow_soil_heat_j_m2 = 125.0,
            }
            assert!(ledger.validate().is_err());
        }
    }

    #[test]
    fn snow_free_successor_retains_authoritative_terminal_v4_owner() {
        let source = include_str!("snow_stage3_v11_attachment.rs");
        assert!(source.contains("snow-free successor terminal V4 owner"));
        assert!(source.contains("snow-free successor changed pending terminal V4 custody"));
    }

    #[test]
    fn solid_reappearance_phase_debit_seals_rain_only_successor_forcing() {
        let mut mixed = DirectSnowHourlyForcing {
            active_precipitation_m: 0.003,
            rain_m: 0.001,
            snowfall_m: 0.02,
            radiation_mj_m2: 4.0,
            air_temperature_c: -1.0,
            cloud_fraction: 0.5,
            phase_model: SnowPhasePartitionModel::LegacyRst,
            rain_fraction: 1.0 / 3.0,
            snow_fraction: 2.0 / 3.0,
            hydrometeor_temperature_c: Some(-0.5),
        };
        debit_solid_reappearance_phase_v1(&mut mixed)
            .expect("canonical solid reappearance phase debit");
        assert_eq!(mixed.active_precipitation_m.to_bits(), 0.001_f64.to_bits());
        assert_eq!(mixed.rain_m.to_bits(), 0.001_f64.to_bits());
        assert_eq!(mixed.snowfall_m.to_bits(), 0.0_f64.to_bits());
        assert_eq!(mixed.rain_fraction.to_bits(), 1.0_f64.to_bits());
        assert_eq!(mixed.snow_fraction.to_bits(), 0.0_f64.to_bits());
        assert_eq!(mixed.hydrometeor_temperature_c, Some(-0.5));

        let mut all_solid = DirectSnowHourlyForcing {
            active_precipitation_m: 0.002,
            rain_m: 0.0,
            snowfall_m: 0.02,
            radiation_mj_m2: 4.0,
            air_temperature_c: -1.0,
            cloud_fraction: 0.5,
            phase_model: SnowPhasePartitionModel::LegacyRst,
            rain_fraction: 0.0,
            snow_fraction: 1.0,
            hydrometeor_temperature_c: Some(-1.0),
        };
        debit_solid_reappearance_phase_v1(&mut all_solid)
            .expect("all-solid reappearance phase debit");
        assert_eq!(
            all_solid.active_precipitation_m.to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(all_solid.rain_fraction.to_bits(), 0.0_f64.to_bits());
        assert_eq!(all_solid.snow_fraction.to_bits(), 0.0_f64.to_bits());
        assert_eq!(all_solid.hydrometeor_temperature_c, None);
    }

    #[test]
    fn solid_reappearance_phase_debit_rejects_omission_and_substitution_without_mutation() {
        let canonical = DirectSnowHourlyForcing {
            active_precipitation_m: 0.003,
            rain_m: 0.001,
            snowfall_m: 0.02,
            radiation_mj_m2: 4.0,
            air_temperature_c: -1.0,
            cloud_fraction: 0.5,
            phase_model: SnowPhasePartitionModel::LegacyRst,
            rain_fraction: 1.0 / 3.0,
            snow_fraction: 2.0 / 3.0,
            hydrometeor_temperature_c: Some(-0.5),
        };
        let mut poisons = Vec::new();
        let mut missing_solid = canonical;
        missing_solid.snowfall_m = 0.0;
        poisons.push(missing_solid);
        let mut substituted_fraction = canonical;
        substituted_fraction.snow_fraction += 1.0e-6;
        poisons.push(substituted_fraction);
        let mut substituted_rain = canonical;
        substituted_rain.rain_m += 1.0e-6;
        poisons.push(substituted_rain);
        let mut nonfinite_phase = canonical;
        nonfinite_phase.snow_fraction = f64::NAN;
        poisons.push(nonfinite_phase);

        for original in poisons {
            let mut candidate = original;
            assert!(debit_solid_reappearance_phase_v1(&mut candidate).is_err());
            assert_eq!(
                candidate.active_precipitation_m.to_bits(),
                original.active_precipitation_m.to_bits()
            );
            assert_eq!(candidate.rain_m.to_bits(), original.rain_m.to_bits());
            assert_eq!(
                candidate.snowfall_m.to_bits(),
                original.snowfall_m.to_bits()
            );
            assert_eq!(
                candidate.rain_fraction.to_bits(),
                original.rain_fraction.to_bits()
            );
            assert_eq!(
                candidate.snow_fraction.to_bits(),
                original.snow_fraction.to_bits()
            );
            assert_eq!(
                candidate.hydrometeor_temperature_c.map(f64::to_bits),
                original.hydrometeor_temperature_c.map(f64::to_bits),
                "rejected debit must roll back exactly"
            );
        }
    }
}
