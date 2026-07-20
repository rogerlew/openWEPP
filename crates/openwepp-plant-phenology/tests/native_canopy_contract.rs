use openwepp_plant_phenology::{
    ForestCanopyDailyResult, ForestCanopyParameters, ForestCanopyState, GsiDailyForcing, GsiDate,
    GsiParameters, realize_forest_canopy,
};

const YEAR_DAYS: u16 = 365;
const HALF_YEAR_SHIFT: u16 = 182;

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

fn seasonal_forcing(
    source_day: u16,
    latitude_degrees: f64,
    year: i32,
    date_day: u16,
) -> GsiDailyForcing {
    let seasonal = ((2.0 * std::f64::consts::PI * (f64::from(source_day) - 172.0)) / 365.0).cos();
    GsiDailyForcing {
        minimum_temperature_c: 8.0 + 10.0 * seasonal,
        vapor_pressure_deficit_pa: 1_500.0 - 400.0 * seasonal,
        latitude_degrees,
        date: GsiDate {
            year,
            ordinal_day: date_day,
        },
    }
}

fn shift_day(day: u16) -> u16 {
    ((day - 1 + HALF_YEAR_SHIFT) % YEAR_DAYS) + 1
}

fn unshift_day(day: u16) -> u16 {
    ((day - 1 + YEAR_DAYS - HALF_YEAR_SHIFT) % YEAR_DAYS) + 1
}

fn circular_day_distance(left: u16, right: u16) -> u16 {
    let direct = left.abs_diff(right);
    direct.min(YEAR_DAYS - direct)
}

#[test]
fn deciduous_mixed_and_evergreen_endpoints_are_explicit() {
    let deciduous_winter =
        realize_forest_canopy(parameters(0.0), 0.0, 0.8).expect("deciduous winter endpoint");
    assert_close(deciduous_winter.live_foliar_biomass_kg_m2, 0.0, 1.0e-15);
    assert_close(deciduous_winter.leaf_area_index, 0.0, 1.0e-15);
    assert_close(deciduous_winter.canopy_cover_fraction, 0.2, 1.0e-15);
    assert_close(deciduous_winter.leaf_off_litter_kg_m2, 0.8, 1.0e-15);

    let mixed_winter =
        realize_forest_canopy(parameters(0.25), 0.0, 0.8).expect("mixed winter endpoint");
    assert_close(mixed_winter.live_foliar_biomass_kg_m2, 0.2, 1.0e-15);
    assert_close(mixed_winter.evergreen_foliar_biomass_kg_m2, 0.2, 1.0e-15);
    assert_close(mixed_winter.deciduous_foliar_biomass_kg_m2, 0.0, 1.0e-15);
    assert_close(mixed_winter.leaf_area_index, 1.25, 1.0e-15);

    let evergreen_winter =
        realize_forest_canopy(parameters(1.0), 0.0, 0.8).expect("evergreen winter endpoint");
    let evergreen_summer =
        realize_forest_canopy(parameters(1.0), 1.0, 0.8).expect("evergreen summer endpoint");
    assert_close(
        evergreen_winter.live_foliar_biomass_kg_m2,
        evergreen_summer.live_foliar_biomass_kg_m2,
        0.0,
    );
    assert_close(
        evergreen_winter.leaf_area_index,
        evergreen_summer.leaf_area_index,
        0.0,
    );
    assert_close(
        evergreen_winter.canopy_cover_fraction,
        evergreen_summer.canopy_cover_fraction,
        0.0,
    );
    assert_close(evergreen_winter.live_foliar_biomass_kg_m2, 0.8, 1.0e-15);
    assert_close(evergreen_winter.leaf_area_index, 5.0, 1.0e-15);
}

#[test]
fn first_realization_has_no_fabricated_transfer_or_aggregate_seed() {
    let mut state = ForestCanopyState::new_uninitialized();
    let result = state
        .advance(parameters(0.25), seasonal_forcing(150, 45.0, 2027, 1))
        .expect("first native realization");
    assert_close(
        result.canopy.previous_foliar_biomass_kg_m2,
        result.canopy.live_foliar_biomass_kg_m2,
        0.0,
    );
    assert_close(result.canopy.leaf_on_allocation_kg_m2, 0.0, 0.0);
    assert_close(result.canopy.leaf_off_litter_kg_m2, 0.0, 0.0);
}

#[test]
fn leaf_on_and_leaf_off_close_the_daily_foliar_mass_ledger() {
    let spring = realize_forest_canopy(parameters(0.25), 0.6, 0.2).expect("spring transition");
    assert!(spring.leaf_on_allocation_kg_m2 > 0.0);
    assert_close(spring.leaf_off_litter_kg_m2, 0.0, 0.0);
    assert_close(
        spring.previous_foliar_biomass_kg_m2 + spring.leaf_on_allocation_kg_m2
            - spring.leaf_off_litter_kg_m2,
        spring.live_foliar_biomass_kg_m2,
        1.0e-15,
    );

    let autumn = realize_forest_canopy(parameters(0.25), 0.1, spring.live_foliar_biomass_kg_m2)
        .expect("autumn transition");
    assert_close(autumn.leaf_on_allocation_kg_m2, 0.0, 0.0);
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

    invalid = parameters(0.5);
    invalid.canopy_cover_coefficient_m2_kg = 0.0;
    assert!(realize_forest_canopy(invalid, 0.5, 0.4).is_err());

    assert!(realize_forest_canopy(parameters(0.5), 1.1, 0.4).is_err());
}

#[test]
fn full_wrapped_nh_climate_phase_flip_preserves_sh_canopy_and_limb_order() {
    let mut north = ForestCanopyState::new_uninitialized();
    let mut south = ForestCanopyState::new_uninitialized();
    let canopy = parameters(0.25);
    let mut north_final_year = Vec::<ForestCanopyDailyResult>::new();
    let mut south_final_year = Vec::<ForestCanopyDailyResult>::new();

    for year in 2027..=2028 {
        for day in 1..=YEAR_DAYS {
            let north_result = north
                .advance(canopy, seasonal_forcing(day, 45.0, year, day))
                .expect("valid northern forcing");
            let south_result = south
                .advance(canopy, seasonal_forcing(unshift_day(day), -45.0, year, day))
                .expect("valid phase-flipped southern forcing");
            if year == 2028 {
                north_final_year.push(north_result);
                south_final_year.push(south_result);
            }
        }
    }

    for north_day in 1..=YEAR_DAYS {
        let south_day = shift_day(north_day);
        let north_result = north_final_year[usize::from(north_day - 1)];
        let south_result = south_final_year[usize::from(south_day - 1)];
        assert_close(
            north_result.gsi.growing_season_index,
            south_result.gsi.growing_season_index,
            0.025,
        );
        assert_close(
            north_result.canopy.live_foliar_biomass_kg_m2,
            south_result.canopy.live_foliar_biomass_kg_m2,
            0.02,
        );
        assert_close(
            north_result.canopy.canopy_cover_fraction,
            south_result.canopy.canopy_cover_fraction,
            0.02,
        );
    }

    for (north_events, south_events) in [
        (
            north_final_year
                .iter()
                .enumerate()
                .filter(|(_, result)| result.canopy.leaf_on_allocation_kg_m2 > 1.0e-12)
                .map(|(index, _)| u16::try_from(index + 1).expect("day index"))
                .collect::<Vec<_>>(),
            south_final_year
                .iter()
                .enumerate()
                .filter(|(_, result)| result.canopy.leaf_on_allocation_kg_m2 > 1.0e-12)
                .map(|(index, _)| u16::try_from(index + 1).expect("day index"))
                .collect::<Vec<_>>(),
        ),
        (
            north_final_year
                .iter()
                .enumerate()
                .filter(|(_, result)| result.canopy.leaf_off_litter_kg_m2 > 1.0e-12)
                .map(|(index, _)| u16::try_from(index + 1).expect("day index"))
                .collect::<Vec<_>>(),
            south_final_year
                .iter()
                .enumerate()
                .filter(|(_, result)| result.canopy.leaf_off_litter_kg_m2 > 1.0e-12)
                .map(|(index, _)| u16::try_from(index + 1).expect("day index"))
                .collect::<Vec<_>>(),
        ),
    ] {
        assert!(!north_events.is_empty());
        assert!(!south_events.is_empty());
        for north_day in north_events {
            let expected_south_day = shift_day(north_day);
            assert!(
                south_events
                    .iter()
                    .copied()
                    .any(|south_day| { circular_day_distance(expected_south_day, south_day) <= 1 })
            );
        }
    }
}

#[test]
fn repeated_annual_forcing_has_bit_identical_foliar_state_and_transfers() {
    let mut state = ForestCanopyState::new_uninitialized();
    let canopy = parameters(0.25);
    let mut endpoints = Vec::new();
    let mut annual_transfers = Vec::new();
    for year in 2025..=2027 {
        let mut allocation = 0.0_f64;
        let mut litter = 0.0_f64;
        let mut final_canopy = None;
        for day in 1..=YEAR_DAYS {
            let result = state
                .advance(canopy, seasonal_forcing(day, 45.0, year, day))
                .expect("valid cyclic forcing");
            allocation += result.canopy.leaf_on_allocation_kg_m2;
            litter += result.canopy.leaf_off_litter_kg_m2;
            final_canopy = Some(result.canopy.canopy_cover_fraction);
        }
        endpoints.push((
            state
                .previous_foliar_biomass_kg_m2()
                .expect("realized state"),
            final_canopy.expect("annual canopy endpoint"),
        ));
        annual_transfers.push((allocation, litter));
    }

    assert_eq!(endpoints[1].0.to_bits(), endpoints[2].0.to_bits());
    assert_eq!(endpoints[1].1.to_bits(), endpoints[2].1.to_bits());
    assert_eq!(
        annual_transfers[1].0.to_bits(),
        annual_transfers[2].0.to_bits()
    );
    assert_eq!(
        annual_transfers[1].1.to_bits(),
        annual_transfers[2].1.to_bits()
    );
}
