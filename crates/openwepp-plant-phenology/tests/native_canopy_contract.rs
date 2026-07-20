use openwepp_plant_phenology::{
    ForestCanopyParameters, ForestCanopyState, GsiDailyForcing, GsiDate, GsiParameters, GsiState,
    realize_forest_canopy,
};

fn parameters(evergreen_fraction: f64) -> ForestCanopyParameters {
    ForestCanopyParameters {
        gsi: GsiParameters::generalized(),
        summer_foliar_biomass_kg_m2: 0.8,
        maximum_leaf_area_index: 5.0,
        evergreen_fraction,
        structural_canopy_cover_fraction: 0.2,
        structural_biomass_kg_m2: 6.0,
        canopy_cover_coefficient_m2_kg: 2.0,
    }
}

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "actual={actual}, expected={expected}, tolerance={tolerance}"
    );
}

#[test]
fn deciduous_mixed_and_evergreen_endpoints_are_explicit() {
    let deciduous_winter =
        realize_forest_canopy(parameters(0.0), 0.0, 0.8).expect("deciduous winter endpoint");
    assert_eq!(deciduous_winter.live_foliar_biomass_kg_m2, 0.0);
    assert_eq!(deciduous_winter.leaf_area_index, 0.0);
    assert_eq!(deciduous_winter.canopy_cover_fraction, 0.2);
    assert_eq!(deciduous_winter.leaf_off_litter_kg_m2, 0.8);

    let mixed_winter =
        realize_forest_canopy(parameters(0.25), 0.0, 0.8).expect("mixed winter endpoint");
    assert_close(mixed_winter.live_foliar_biomass_kg_m2, 0.2, 1.0e-15);
    assert_close(mixed_winter.evergreen_foliar_biomass_kg_m2, 0.2, 1.0e-15);
    assert_eq!(mixed_winter.deciduous_foliar_biomass_kg_m2, 0.0);
    assert_close(mixed_winter.leaf_area_index, 1.25, 1.0e-15);

    let evergreen_winter =
        realize_forest_canopy(parameters(1.0), 0.0, 0.8).expect("evergreen winter endpoint");
    let evergreen_summer =
        realize_forest_canopy(parameters(1.0), 1.0, 0.8).expect("evergreen summer endpoint");
    assert_eq!(
        evergreen_winter.live_foliar_biomass_kg_m2,
        evergreen_summer.live_foliar_biomass_kg_m2
    );
    assert_eq!(
        evergreen_winter.leaf_area_index,
        evergreen_summer.leaf_area_index
    );
    assert_eq!(
        evergreen_winter.canopy_cover_fraction,
        evergreen_summer.canopy_cover_fraction
    );
    assert_eq!(evergreen_winter.live_foliar_biomass_kg_m2, 0.8);
    assert_eq!(evergreen_winter.leaf_area_index, 5.0);
}

#[test]
fn leaf_on_and_leaf_off_close_the_daily_foliar_mass_ledger() {
    let spring = realize_forest_canopy(parameters(0.25), 0.6, 0.2).expect("spring transition");
    assert!(spring.leaf_on_allocation_kg_m2 > 0.0);
    assert_eq!(spring.leaf_off_litter_kg_m2, 0.0);
    assert_close(
        spring.previous_foliar_biomass_kg_m2 + spring.leaf_on_allocation_kg_m2
            - spring.leaf_off_litter_kg_m2,
        spring.live_foliar_biomass_kg_m2,
        1.0e-15,
    );

    let autumn = realize_forest_canopy(parameters(0.25), 0.1, spring.live_foliar_biomass_kg_m2)
        .expect("autumn transition");
    assert_eq!(autumn.leaf_on_allocation_kg_m2, 0.0);
    assert!(autumn.leaf_off_litter_kg_m2 > 0.0);
    assert_close(
        autumn.previous_foliar_biomass_kg_m2 + autumn.leaf_on_allocation_kg_m2
            - autumn.leaf_off_litter_kg_m2,
        autumn.live_foliar_biomass_kg_m2,
        1.0e-15,
    );
}

#[test]
fn invalid_canopy_authority_fails_closed() {
    let mut invalid = parameters(0.5);
    invalid.structural_canopy_cover_fraction = 1.0;
    assert!(realize_forest_canopy(invalid, 0.5, 0.4).is_err());

    invalid = parameters(0.5);
    invalid.summer_foliar_biomass_kg_m2 = f64::NAN;
    assert!(realize_forest_canopy(invalid, 0.5, 0.4).is_err());

    assert!(realize_forest_canopy(parameters(0.5), 1.1, 0.4).is_err());
}

#[test]
fn negated_latitude_and_half_year_forcing_shift_preserve_seasonal_phase() {
    let mut north = GsiState::new();
    let mut south = GsiState::new();
    let gsi = GsiParameters::generalized();

    for north_day in 1_u16..=183 {
        let south_day = north_day + 182;
        let seasonal =
            ((2.0 * std::f64::consts::PI * (f64::from(north_day) - 172.0)) / 365.0).cos();
        let minimum_temperature_c = 8.0 + 10.0 * seasonal;
        let vapor_pressure_deficit_pa = 1_500.0 - 400.0 * seasonal;
        let north_result = north
            .advance(
                gsi,
                GsiDailyForcing {
                    minimum_temperature_c,
                    vapor_pressure_deficit_pa,
                    latitude_degrees: 45.0,
                    date: GsiDate {
                        year: 2027,
                        ordinal_day: north_day,
                    },
                },
            )
            .expect("valid northern forcing");
        let south_result = south
            .advance(
                gsi,
                GsiDailyForcing {
                    minimum_temperature_c,
                    vapor_pressure_deficit_pa,
                    latitude_degrees: -45.0,
                    date: GsiDate {
                        year: 2027,
                        ordinal_day: south_day,
                    },
                },
            )
            .expect("valid phase-shifted southern forcing");

        assert_close(
            north_result.growing_season_index,
            south_result.growing_season_index,
            0.025,
        );
    }
}

#[test]
fn repeated_annual_forcing_has_no_foliar_state_drift() {
    let parameters = parameters(0.25);
    let mut state = ForestCanopyState::new(0.2).expect("valid initial foliar mass");
    let mut endpoints = Vec::new();
    let mut annual_transfers = Vec::new();
    for year in [2025, 2026] {
        let mut allocation = 0.0;
        let mut litter = 0.0;
        for ordinal_day in 1_u16..=365 {
            let seasonal =
                ((2.0 * std::f64::consts::PI * (f64::from(ordinal_day) - 172.0)) / 365.0).cos();
            let result = state
                .advance(
                    parameters,
                    GsiDailyForcing {
                        minimum_temperature_c: 8.0 + 10.0 * seasonal,
                        vapor_pressure_deficit_pa: 1_500.0 - 400.0 * seasonal,
                        latitude_degrees: 45.0,
                        date: GsiDate { year, ordinal_day },
                    },
                )
                .expect("valid cyclic forcing");
            allocation += result.canopy.leaf_on_allocation_kg_m2;
            litter += result.canopy.leaf_off_litter_kg_m2;
        }
        endpoints.push(state.previous_foliar_biomass_kg_m2());
        annual_transfers.push((allocation, litter));
    }

    assert_close(endpoints[0], endpoints[1], 1.0e-15);
    assert_close(annual_transfers[0].0, annual_transfers[1].0, 1.0e-15);
    assert_close(annual_transfers[0].1, annual_transfers[1].1, 1.0e-15);
}
