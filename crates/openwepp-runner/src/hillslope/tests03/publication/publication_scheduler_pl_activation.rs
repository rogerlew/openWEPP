use super::super::*;

    #[test]
    fn hphys0250_scheduler_lifecycle_preserves_pl_runtime_sentinel_for_ep_lineage() {
        let source = include_str!("../../mod.rs");
        let sentinel = "pl_schedule_slot_count";
        let forbidden_fragment = ["symbol.as_str() != ", "\"", sentinel, "\""].concat();

        assert!(
            !source.contains(&forbidden_fragment),
            "runner scheduler lifecycle must not strip {sentinel}; PL growth must remain active so rtd can feed final Ep lineage"
        );
    }
    #[test]
    fn fq3dc_annual_preplant_skip_preserves_pl_sentinel_for_later_activation() {
        let mut runtime_surface = HillslopeWritebackSurface {
            state_surface: BTreeMap::new(),
            flux_surface: BTreeMap::new(),
        };
        for (symbol, value) in [
            ("pl_schedule_slot_count", 1.0),
            ("pl_schedule_rotation_years", 7.0),
            ("pl_schedule_rotation_repeats", 1.0),
            ("year", 1.0),
            ("day", 1.0),
            ("pl_schedule_slot_0001_ofe_index", 1.0),
            ("pl_schedule_slot_0001_year_in_rotation", 1.0),
            ("pl_schedule_slot_0001_rotation_index", 1.0),
            ("pl_schedule_slot_0001_crop_slots", 1.0),
            ("pl_schedule_slot_0001_crop_0001_imngmt", 1.0),
            ("pl_growth_slot_0001_crop_0001_jdplt", 130.0),
            ("pl_growth_slot_0001_crop_0001_jdharv", 288.0),
        ] {
            runtime_surface
                .state_surface
                .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
        }

        let sentinel_value = pl_runtime_activation_sentinel_value(&runtime_surface);
        prepare_pl_runtime_activation_for_scheduler(&mut runtime_surface)
            .expect("pre-plant annual day should be a day-local scheduler skip");
        assert!(
            !runtime_surface
                .state_surface
                .contains_key(&BoundarySymbol::from("pl_schedule_slot_count")),
            "pre-plant day should suppress PL phases for that day"
        );

        restore_pl_runtime_activation_sentinel_for_next_day(&mut runtime_surface, sentinel_value);
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("day"), BoundaryValue::scalar(153.0));

        prepare_pl_runtime_activation_for_scheduler(&mut runtime_surface)
            .expect("post-plant annual day should re-evaluate the carried PL schedule");
        assert!(
            runtime_surface
                .state_surface
                .contains_key(&BoundarySymbol::from("pl_schedule_slot_count")),
            "carried annual schedule sentinel must be available after jdplt so Corn growth can engage ET"
        );
    }
    #[test]
    fn fq3dc_scheduler_calendar_day_symbol_uses_julian_day_for_pl_activation() {
        let mut runtime_surface = HillslopeWritebackSurface {
            state_surface: BTreeMap::new(),
            flux_surface: BTreeMap::new(),
        };
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("day"), BoundaryValue::scalar(2.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("year"), BoundaryValue::scalar(1990.0));
        let calendar_day = ClimateDayProjection {
            year: 1990,
            month: 6,
            day_of_month: 2,
            julian_day: 153,
            precipitation_mm: 0.0,
            effective_temperature_c: 0.0,
        };
        let symbol_registry =
            SymbolRegistry::from_symbols(Vec::<BoundarySymbol>::new()).expect("registry builds");
        let hot_symbol_tables = build_hillslope_hot_symbol_tables(&symbol_registry);

        seed_scheduler_calendar_symbols(
            &mut runtime_surface,
            &SchedulerLifecycleContext {
                run_name: "calendar-probe",
                execution_lane: ExecutionLane::Hourly,
                publication_area_m2: 1.0,
                simulation_year: 1,
                sim_day_index: 153,
                calendar_day: &calendar_day,
                runtime_swe_before_m: 0.0,
                hphys0245_trace_config: None,
                symbol_registry: Some(&symbol_registry),
                hot_symbol_tables: Some(&hot_symbol_tables),
                indexed_scheduler_runtime_enabled: false,
            },
        );

        let day = require_runtime_surface_scalar(&runtime_surface, "day")
            .expect("scheduler day symbol should exist");
        assert!(
            (day - 153.0).abs() < f64::EPSILON,
            "PL activation must consume Julian day, not day-of-month"
        );
        let year = require_runtime_surface_scalar(&runtime_surface, "year")
            .expect("scheduler year symbol should exist");
        assert!(
            (year - 1.0).abs() < f64::EPSILON,
            "PL activation must consume simulation year within the rotation"
        );
    }
    #[test]
    fn hphys0250_pl_activation_keeps_zero_date_perennial_slots_active() {
        let mut runtime_surface = HillslopeWritebackSurface {
            state_surface: BTreeMap::new(),
            flux_surface: BTreeMap::new(),
        };
        for (symbol, value) in [
            ("pl_schedule_slot_count", 1.0),
            ("pl_schedule_rotation_years", 1.0),
            ("pl_schedule_rotation_repeats", 1.0),
            ("year", 1.0),
            ("day", 1.0),
            ("pl_schedule_slot_0001_ofe_index", 1.0),
            ("pl_schedule_slot_0001_year_in_rotation", 1.0),
            ("pl_schedule_slot_0001_rotation_index", 1.0),
            ("pl_schedule_slot_0001_crop_slots", 1.0),
            ("pl_schedule_slot_0001_crop_0001_imngmt", 2.0),
            ("pl_growth_slot_0001_crop_0001_jdplt", 0.0),
            ("pl_growth_slot_0001_crop_0001_jdharv", 250.0),
            ("pl_growth_slot_0001_crop_0001_jdstop", 0.0),
        ] {
            runtime_surface
                .state_surface
                .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
        }

        assert!(
            runtime_surface
                .state_surface
                .get(&BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdplt"))
                .is_some_and(|value| value.as_f64() == 0.0),
            "fixture should encode zero jdplt perennial runtime row"
        );

        prepare_pl_runtime_activation_for_scheduler(&mut runtime_surface)
            .expect("zero-date perennial pl slot should stay active for scheduler");
        assert!(
            runtime_surface
                .state_surface
                .contains_key(&BoundarySymbol::from("pl_schedule_slot_count")),
            "perennial zero-date slot should keep scheduler-active sentinel"
        );
    }
