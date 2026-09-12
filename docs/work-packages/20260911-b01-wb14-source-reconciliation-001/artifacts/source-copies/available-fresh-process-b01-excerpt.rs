#[test]
#[ignore = "fresh process B01 policy identity"]
fn actual_day_four_thin_snow_publication_preserves_owner() {
    openwepp_land_surface_energy::snow_accuracy_policy::Policy::B01.install().expect("B01");
    publication_projection_regime_vectors_preserve_state();
    let ending: DirectSnowStage3PersistentState = serde_json::from_str(r###"{"schema_version": 2, "experimental_accuracy": [14, false], "terminal_event_model": "EnthalpyEventV1", "fingerprint": 4430680846865068613, "lane_id": 1, "next_interval_index": 192, "layers": [{"mass_swe_m": 0.0004300877158356576, "thickness_m": 0.004300877158356585, "density_kg_m3": 99.9999999999998, "settle_day_count": 0.0, "temperature_c": 0.0, "liquid_water_m": 0.0007144978421999323, "cold_content_j_m2": 0.0, "refrozen_liquid_m": 0.0003326108662195986}], "detached_retained_liquid_kg_m2": 0.0, "initial_ice_kg_m2": -0.0, "initial_retained_liquid_kg_m2": 0.0, "cumulative_snowfall_kg_m2": 11.999999718902094, "cumulative_external_liquid_kg_m2": 2.8109790785872166e-07, "cumulative_deposition_kg_m2": 0.6033821552684834, "cumulative_sublimation_kg_m2": 0.5901930165073069, "cumulative_melt_kg_m2": 11.915712267531866, "cumulative_unresolved_liquid_kg_m2": 10.868603580725626, "cumulative_complete_energy_j_m2": 3859427.129069834, "cumulative_cold_energy_change_j_m2": -115654.48337879096, "cumulative_terminal_unallocated_energy_j_m2": 0.0}"###).expect("authentic day four snow state");
    let before = serde_json::to_vec(&ending).expect("owner bytes");
    assert!(crate::hydrology::stage3_is_terminal_event_domain(&ending));
    assert!(Wb11HydrologyKernel::project_stage3_surface_state_v1(&ending).is_err());
    let expected = Wb11HydrologyKernel::project_stage3_terminal_surface_state_v1(&ending)
        .expect("valid canonical thin snow").surface_temperature_k - 273.15;
    let actual = publication_surface_temperature_c(&ending).expect("publish valid terminal snow").expect("represented snow");
    assert_eq!(actual.to_bits(), expected.to_bits());
    assert_eq!(actual.to_bits(), 0.0_f64.to_bits());
    assert_eq!(serde_json::to_vec(&ending).expect("after bytes"), before);
}

// Scratch-only test body; not compiled/run. Place in committed-publication tests.
// Invoke in the existing fresh-process B01 test AFTER installing its policy.
fn publication_projection_regime_vectors_preserve_state() {
