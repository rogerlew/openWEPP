mod terminal_cumulative_cold_energy_order_tests {
    use super::*;

    #[test]
    fn discovery_prefix_and_accepted_trace_use_bit_exact_canonical_order() {
        let beginning = f64::from_bits(13_887_053_936_531_509_935);
        let first = TerminalLedger {
            cold_energy_change_j_m2: f64::from_bits(4_614_783_999_027_029_656),
            refrozen_kg_m2: f64::from_bits(4_529_196_899_309_098_716),
            ..TerminalLedger::default()
        };
        let second = TerminalLedger {
            cold_energy_change_j_m2: f64::from_bits(4_603_260_172_874_495_476),
            refrozen_kg_m2: f64::from_bits(4_521_386_001_815_403_664),
            ..TerminalLedger::default()
        };
        let prefix = first.add(second);
        let discovery =
            Wb11HydrologyKernel::terminal_cumulative_cold_energy_change_j_m2(beginning, prefix);
        let accepted_trace =
            Wb11HydrologyKernel::terminal_cumulative_cold_energy_change_j_m2(beginning, prefix);
        let historical_incremental =
            Wb11HydrologyKernel::terminal_cumulative_cold_energy_change_j_m2(beginning, first)
                + (second.cold_energy_change_j_m2
                    - STAGE3_LATENT_HEAT_FUSION_J_KG * second.refrozen_kg_m2);
        assert_eq!(discovery.to_bits(), 13_887_052_641_916_287_782);
        assert_eq!(accepted_trace.to_bits(), discovery.to_bits());
        assert_eq!(historical_incremental.to_bits(), 13_887_052_641_916_287_783);
        assert_ne!(historical_incremental.to_bits(), discovery.to_bits());

        let beginning_state = Wb11HydrologyKernel::initialize_stage3_persistent_state(
            1,
            vec![DirectSnowLayerState::new(0.08, 0.8, 100.0, 0.0)],
        )
        .expect("beginning Stage-3 state");
        let refrozen_steps = [
            f64::from_bits(4_558_201_905_820_510_167),
            f64::from_bits(4_576_513_483_090_770_651),
            f64::from_bits(4_573_993_328_041_080_417),
        ];
        let refrozen_prefix =
            refrozen_steps
                .into_iter()
                .fold(TerminalLedger::default(), |prefix, refrozen_kg_m2| {
                    prefix.add(TerminalLedger {
                        refrozen_kg_m2,
                        ..TerminalLedger::default()
                    })
                });
        let canonical_refrozen = Wb11HydrologyKernel::terminal_cumulative_refrozen_liquid_m(
            &beginning_state,
            refrozen_prefix,
        );
        let historical_incremental_refrozen = refrozen_steps
            .into_iter()
            .fold(0.0, |sum, value| sum + value / STAGE3_RHO_WATER_KG_M3);
        assert_eq!(canonical_refrozen.to_bits(), 4_535_407_366_274_033_979);
        assert_eq!(
            historical_incremental_refrozen.to_bits(),
            4_535_407_366_274_033_980,
        );
        assert_ne!(
            historical_incremental_refrozen.to_bits(),
            canonical_refrozen.to_bits(),
        );

        let cumulative_beginning = f64::from_bits(4_599_892_515_195_903_436);
        let cumulative_steps = [
            f64::from_bits(4_592_315_733_242_806_712),
            f64::from_bits(4_596_892_609_956_699_276),
            f64::from_bits(4_606_529_177_361_465_673),
        ];
        let cumulative_prefix = cumulative_steps.into_iter().sum::<f64>();
        let discovery_cumulative = Wb11HydrologyKernel::terminal_cumulative_quantity(
            cumulative_beginning,
            cumulative_prefix,
        );
        let composed_cumulative = Wb11HydrologyKernel::terminal_cumulative_quantity(
            cumulative_beginning,
            cumulative_prefix,
        );
        let historical_incremental_cumulative = cumulative_steps
            .into_iter()
            .fold(cumulative_beginning, |sum, value| sum + value);
        assert_eq!(discovery_cumulative.to_bits(), 4_609_854_802_644_739_013);
        assert_eq!(
            composed_cumulative.to_bits(),
            discovery_cumulative.to_bits(),
        );
        assert_eq!(
            historical_incremental_cumulative.to_bits(),
            4_609_854_802_644_739_012,
        );
        assert_ne!(
            historical_incremental_cumulative.to_bits(),
            discovery_cumulative.to_bits(),
        );
    }
}
