use openwepp_plant_phenology::{GsiDailyForcing, GsiDate, GsiParameters, GsiState};

fn forcing(day: u16) -> GsiDailyForcing {
    GsiDailyForcing {
        minimum_temperature_c: -4.0 + f64::from(day) * 0.35,
        vapor_pressure_deficit_pa: 650.0 + f64::from(day) * 83.0,
        latitude_degrees: 0.0,
        date: GsiDate {
            year: 2027,
            ordinal_day: day,
        },
    }
}

#[test]
fn anchored_restore_continues_bit_identically_to_uninterrupted_state() {
    let parameters = GsiParameters::generalized();
    let mut uninterrupted = GsiState::new();
    for day in 1_u16..=25 {
        uninterrupted
            .advance(parameters, forcing(day))
            .expect("valid uninterrupted forcing");
    }

    let retained_history = uninterrupted.history();
    assert!(
        retained_history
            .windows(2)
            .any(|pair| pair[0].to_bits() != pair[1].to_bits()),
        "restart vector must retain heterogeneous GSI values"
    );
    let mut restored = GsiState::try_from_history(&retained_history, uninterrupted.last_date())
        .expect("valid anchored restart");
    assert_eq!(restored, uninterrupted);

    let uninterrupted_result = uninterrupted
        .advance(parameters, forcing(26))
        .expect("valid uninterrupted continuation");
    let restored_result = restored
        .advance(parameters, forcing(26))
        .expect("valid restored continuation");

    assert_eq!(restored_result, uninterrupted_result);
    assert_eq!(restored, uninterrupted);
}
