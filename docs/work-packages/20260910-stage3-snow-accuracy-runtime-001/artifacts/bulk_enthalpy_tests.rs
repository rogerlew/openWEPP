// Independent analytical vectors for EXP-SNOW-ACCURACY-20260910-B.
#[path = "bulk_enthalpy.rs"]
mod bulk;
use bulk::*;

fn forcing() -> Forcing {
    Forcing {
        seconds: 600.0, solid: 0.0, rain: 0.0, vapor: 0.0,
        shortwave: 0.0, longwave: 0.0, sensible: 0.0, latent: 0.0,
        precipitation_sensible: 0.0, fresh_density: 100.0,
        conductance: 1.0, soil_temperature: 273.15, soil_capacity: 120_000.0,
    }
}

#[test]
fn phase_melt_and_real_release_pay_fusion() {
    let s = State { water: 1.0, enthalpy: 0.0, depth: 0.01 };
    let r = advance(s, Forcing { shortwave: 83_400.0, ..forcing() }).unwrap();
    assert_eq!(r.ending.water, 0.75);
    assert_eq!(r.liquid_released, 0.25);
    assert_eq!(r.sensible_released, 0.0);
    assert_eq!(r.soil_heat, 0.0);
    assert_eq!(r.ending.enthalpy, 0.0);
    assert_eq!(r.ending.depth, 0.01);
}

#[test]
fn cold_rain_refreezes_and_storage_accounts_energy() {
    let s = State { water: 1.0, enthalpy: -33_360.0, depth: 0.01 };
    let f = Forcing { rain: 0.1, soil_temperature: 273.15, ..forcing() };
    let r = advance(s, f).unwrap();
    assert!((r.ending.water - 1.1).abs() < 1e-12);
    assert!(r.liquid_released.abs() < 1e-12);
    assert!(r.ending.enthalpy.abs() < 1e-6);
}

#[test]
fn cold_two_capacity_exchange_matches_analytic_solution() {
    let s = State { water: 1.0, enthalpy: -21_000.0, depth: 0.01 };
    let f = Forcing { soil_temperature: 253.15, ..forcing() };
    let r = advance(s, f).unwrap();
    let q = 600.0 * 10.0 / (1.0 + 600.0 * (1.0/2100.0 + 1.0/120_000.0));
    assert!((r.soil_heat-q).abs() < 1e-9);
    assert_eq!(r.snow_heat.to_bits(),(-r.soil_heat).to_bits());
    assert!((r.ending.enthalpy - s.enthalpy + q).abs() < 1e-9);
}

#[test]
fn vapor_carries_latent_and_material_enthalpy_once() {
    let s = State { water: 1.0, enthalpy: -21_000.0, depth: 0.01 };
    let r = advance(s, Forcing { vapor:-0.01, latent:-28_340.0, soil_temperature:263.15, ..forcing() }).unwrap();
    assert!((r.vapor_material - 210.0).abs()<1e-9);
    assert!((r.ending.water - 0.99).abs()<1e-12);
    let external = -28_340.0+210.0-r.soil_heat;
    assert!((r.ending.enthalpy-s.enthalpy-external).abs()<1e-6);
}

#[test]
fn exhaustion_nonfinite_and_zero_mass_energy_fail_without_mutation() {
    let s = State { water: 0.1, enthalpy: 0.0, depth: 0.001 };
    assert_eq!(advance(s,Forcing { vapor:-0.2, latent:-566_800.0,..forcing() }),Err(Error::VaporCapacity));
    assert_eq!(advance(s,Forcing { seconds:f64::NAN,..forcing() }),Err(Error::Domain));
    assert_eq!(advance(State {water:0.0,enthalpy:0.0,depth:0.0},forcing()),Err(Error::NoRepresentedIce));
    assert_eq!(s.water,0.1);
}

#[test]
fn ending_snow_below_absolute_zero_is_typed_failure() {
    let s=State {water:1.0,enthalpy:0.0,depth:0.01};
    assert_eq!(advance(s,Forcing {sensible:-1_000_000.0,conductance:1e-6,..forcing()}),Err(Error::Domain));
    assert_eq!(s.enthalpy,0.0);
}

#[test]
fn conversion_and_hysteresis_preserve_inventory() {
    let layers = [Layer {ice:0.03,liquid:0.01,cold:100.0,depth:0.001}, Layer {ice:0.02,liquid:0.0,cold:200.0,depth:0.002}];
    let s=from_layers(&layers).unwrap();
    assert!((s.water-0.06).abs()<1e-15);
    assert!((s.enthalpy-(3336.0-300.0)).abs()<1e-10);
    assert_eq!(s.depth,0.003);
    assert!(select_bulk(false,0.1,0.1).unwrap());
    assert!(select_bulk(true,0.11,0.1).unwrap());
    assert!(!select_bulk(false,0.11,0.1).unwrap());
    assert!(!select_bulk(true,0.121,0.1).unwrap());
    assert_eq!(select_bulk(true,f64::NAN,0.1),Err(Error::Domain));
}
