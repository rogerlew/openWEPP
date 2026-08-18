use std::fs;

const CONTRACT: &str =
    "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md";
const INDEX: &str = "docs/specifications/science-contracts/index.md";
const AUTHORITY_PACKAGE: &str =
    "docs/work-packages/20260814-snow-free-land-surface-energy-authority-001/artifacts";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

fn row<'a>(contract: &'a str, key: &str) -> &'a str {
    contract
        .lines()
        .find(|line| line.starts_with(&format!("| `{key}` |")))
        .unwrap_or_else(|| panic!("{CONTRACT} missing row {key}"))
}

#[test]
fn contract_binds_control_volume_closure_and_exact_one_custody() {
    let contract = read(CONTRACT);
    for required in [
        "contract_id: SC-LANDSURFACEENERGY-001",
        "E_s,1 - E_s,0 = dt * (R_sw + R_lw + H + LE + Q_p + Q_runon - Q_inf - Q_runoff + G)",
        "M_l,1 - M_l,0 = m_p + m_runon - m_evap - m_inf - m_runoff",
        "LE * dt = -L_v(T_s) * m_evap",
        "INV-LANDSURFACEENERGY-010",
        "INV-LANDSURFACEENERGY-011",
        "INV-LANDSURFACEENERGY-012",
        "INV-LANDSURFACEENERGY-013",
        "INV-LANDSURFACEENERGY-014",
        "INV-LANDSURFACEENERGY-015",
        "| `A` | `m^2` |",
        "| `dt` | `s` |",
        "| `T_s` | `K` |",
        "| `R_sw`, `R_lw` | `W m^-2` |",
        "TOL-LANDSURFACEENERGY-001",
        "TOL-LANDSURFACEENERGY-002",
        "`epsilon_E`, `epsilon_M` | `J m^-2`, `kg m^-2`",
        "`rho_E`, `rho_M` | `dimensionless`",
        "|epsilon_E| <= max(a_E, rho_E*sum_abs_energy_operands)",
        "|epsilon_M| <=\n  max(a_M, rho_M*sum_abs_mass_operands)",
        "validate before mutation and commit energy and\n  water state atomically",
        "surface records\n   `G`, while the sole soil/frost consumer records `-G`",
        "runon to `Q_runon`, infiltration to `Q_inf`, and\n   runoff to `Q_runoff`",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn contract_preserves_adjacent_owners_and_rejects_terminal_payload() {
    let contract = read(CONTRACT);
    for required in [
        "SC-CLIMATE-001",
        "SC-EVAP-001",
        "SC-WATBAL-001",
        "SC-RUNOFFPART-001",
        "SC-SOIL-001",
        "SC-SUBHYD-001",
        "INV-SNOWENERGY-034",
        "Branch priority is `snow_terminal` rejection, then `snow_present` delegation",
        "Schema-v8 terminal liquid, energy, and time are censored",
        "must not mutate ET,\nrunoff, infiltration, soil, or frost",
        "a real scheduler consumer must prove",
        "authoritative upstream state reports zero represented\nsnow at the interval start",
        "legacy `surtmp(hour)` / `Thra` | not an alias of `T_s` in v1",
        "future named `degC <-> K` conversion and atomic cutover required",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn current_version_releases_named_authority_without_production_claims() {
    let contract = read(CONTRACT);
    for required in [
        "contract_version: 4",
        "status: approved",
        "maturity: active",
        "OPENWEPP_SNOW_FREE_LSE_V1",
        "OPENWEPP_SNOW_FREE_LSE_V2",
        "OPENWEPP_C3_WOODY_V8",
        "AUTHORITY_ADMITTED",
        "IMPLEMENTATION_MISSING",
        "GAP-LANDSURFACEENERGY-001",
        "GAP-LANDSURFACEENERGY-006",
        "provisional, surrogate, heuristic, or comparator-targeted physics",
        "authorizes no production selector/default/output",
        "calibration, empirical validation or transferability",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn version_three_binds_surface_classes_reciprocal_coupling_and_water_custody() {
    let contract = read(CONTRACT);
    for required in [
        "bare_mineral_soil",
        "forest_litter",
        "Hydrology exclusively owns ponded, litter-held and soil-layer water mass",
        "Ldn_(i+1) = tau_i*Ldn_i + (1-tau_i)*E_i",
        "Lup_i     = tau_i*Lup_(i+1) + (1-tau_i)*E_i",
        "R_Tc = sum_j H_j + H_s - H_c->atm",
        "R_qc = sum_j v_j + v_s - v_c->atm",
        "h_ul     = 0.5*(1-cos(pi*W_l/W_l,max))",
        "h_l(T)=C_w*(T-T_ref)",
        "T_mix=T_ref+sum(m_i*h_i)/(C_w*sum(m_i))",
        "L_v(T)=2.501e6-2369*(T-T_ref) J kg^-1",
        "The water snapshot precedes all current-interval rain, runon, and canopy liquid",
        "No second\nauthorization",
        "No wind floor",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
#[allow(clippy::too_many_lines)] // One digest-bound authority record is audited as an indivisible fixture.
fn immutable_definitions_and_independent_vectors_are_digest_bound() {
    use sha2::{Digest, Sha256};

    let lse = read(&format!(
        "{AUTHORITY_PACKAGE}/openwepp_snow_free_lse_v1_definition.json"
    ));
    let vegetation = read(&format!(
        "{AUTHORITY_PACKAGE}/openwepp_c3_woody_v8_definition.json"
    ));
    let vectors = read(&format!(
        "{AUTHORITY_PACKAGE}/openwepp_snow_free_lse_v1_vectors.json"
    ));
    assert_eq!(
        format!("{:x}", Sha256::digest(lse.as_bytes())),
        "e1736b8c77d13d6fb12fb97a6f747e54eea877edf237817b6c6e8954cff8332f"
    );
    assert_eq!(
        format!("{:x}", Sha256::digest(vegetation.as_bytes())),
        "622bc900a08bd4c70e67c09e1fa113a9de24c48afce3b145a494bb76f6dcbe9b"
    );
    assert_eq!(
        format!("{:x}", Sha256::digest(vectors.as_bytes())),
        "3fb57d7c637abba20659a59e6eb1487f9f4130f909e17b61c8a6f2eb70f4c711"
    );

    let fixture: serde_json::Value = serde_json::from_str(&vectors).expect("LSE vectors");
    assert_eq!(
        fixture["schema"],
        "openwepp-snow-free-lse-v1-joint-authority-vectors-3"
    );
    assert_eq!(fixture["model"], "OPENWEPP_SNOW_FREE_LSE_V1");
    assert_eq!(
        fixture["model_definition_sha256"],
        "e1736b8c77d13d6fb12fb97a6f747e54eea877edf237817b6c6e8954cff8332f"
    );
    let invariants = &fixture["reconstructed_invariants"];
    assert_eq!(invariants["all_poisons_rejected"], true);
    assert_eq!(invariants["all_schema_instances_validated"], true);
    assert_eq!(invariants["all_validation_failures_rollback_exact"], true);
    assert_eq!(invariants["final_rebuilt_from_beginning"]["proved"], true);
    assert_eq!(
        invariants["single_immutable_authorization"]["call_count"],
        1
    );
    assert_eq!(invariants["single_immutable_authorization"]["proved"], true);
    assert!(
        invariants["post_ingress_energy_closure"]
            .as_f64()
            .expect("energy closure")
            .abs()
            < 1.0e-9
    );
    assert_eq!(invariants["post_ingress_mass_closure"], 0.0);

    let mandatory = fixture["mandatory_exact_scenario_vectors"]
        .as_object()
        .expect("mandatory exact scenarios");
    assert_eq!(mandatory.len(), 22);
    for required in [
        "open_bare_day",
        "open_bare_night",
        "covered_column",
        "dry_litter_covered",
        "wet_litter_covered",
        "supported_condensation",
        "partial_surface_cap",
        "partial_top_layer_cap",
        "alternate_starts",
        "storage",
        "ground_albedo_lower_boundary_feedback",
        "frozen_ground_cap_centered_probe",
    ] {
        assert!(mandatory.contains_key(required), "missing {required}");
    }

    let equilibrium = &fixture["equilibrium_zero_storage_branch"];
    assert_eq!(equilibrium["accepted"], true);
    assert_eq!(
        equilibrium["candidate"]["land_surface_energy"]["surface_enthalpy_j_m2_tile"],
        0.0
    );
    assert_eq!(equilibrium["components"]["surface_storage_w_m2_tile"], 0.0);
    assert_eq!(
        fixture["executed_poison_vectors"]
            .as_object()
            .expect("executed poison vectors")
            .len(),
        76
    );
    assert!(
        fixture["executed_poison_vectors"]
            .as_object()
            .expect("executed poison vectors")
            .values()
            .all(|value| value["accepted"] == false
                && value["candidate"].is_null()
                && value["typed_failure"].is_string())
    );
    let failures = fixture["executed_failure_vectors"]
        .as_object()
        .expect("executed failure vectors");
    assert_eq!(failures.len(), 11);
    assert!(failures.values().all(|value| value["accepted"] == false
        && value["candidate"].is_null()
        && value["rollback_exact"] == true));
    for (name, code, kind, typed_failure) in [
        ("singular", "LSEB-E-034", "singular_pivot", "singular"),
        (
            "iteration_limit",
            "LSEB-E-034",
            "backtracking_limit",
            "backtracking_limit",
        ),
        (
            "backtracking_limit",
            "LSEB-E-034",
            "backtracking_limit",
            "backtracking_limit",
        ),
        (
            "calm_wind",
            "LSEB-E-030",
            "unsupported_domain",
            "LSEB-E-030:calm_or_nonfinite_wind",
        ),
        (
            "nonneutral",
            "LSEB-E-030",
            "unsupported_domain",
            "LSEB-E-030:nonneutral_stability",
        ),
    ] {
        assert_eq!(failures[name]["diagnostics"]["failure_code"], code);
        assert_eq!(failures[name]["diagnostics"]["failure_kind"], kind);
        assert_eq!(failures[name]["typed_failure"], typed_failure);
    }
    assert_eq!(
        failures["iteration_limit"]["rust_expected_failure"],
        "iteration_limit"
    );
    assert_eq!(failures["iteration_limit"]["rust_expected_iterations"], 50);
    for required in ["singular", "iteration_limit", "backtracking_limit"] {
        assert!(failures.contains_key(required), "missing {required}");
        assert_eq!(
            failures[required]["owner_and_envelope_rollback_before"],
            failures[required]["owner_and_envelope_rollback_after"],
            "{required} owner envelope changed"
        );
        assert_eq!(
            failures[required]["owner_and_envelope_rollback_before"]
                .as_object()
                .expect("full owner rollback envelope")
                .len(),
            6
        );
    }

    assert_eq!(
        fixture["complete_water_transaction"]["potential"]["accepted"],
        true
    );
    assert_eq!(
        fixture["complete_water_transaction"]["final"]["accepted"],
        true
    );
    assert_eq!(
        fixture["shared_layer_root_ground_competition"]["arbitration_call_count"],
        1
    );
    assert_eq!(
        fixture["shared_layer_root_ground_competition"]["both_rebuilt_from_beginning"],
        true
    );
    let owner_candidates = fixture["post_ingress_owner_candidates"]["candidates"]
        .as_object()
        .expect("owner candidates");
    assert_eq!(owner_candidates.len(), 5);
    for owner in [
        "vegetation",
        "hydrology",
        "land_surface_energy",
        "biogeochemistry",
        "soil_thermal",
    ] {
        let candidate = &owner_candidates[owner];
        assert!(
            candidate["ending_state"].is_object(),
            "{owner} body missing"
        );
        assert!(
            !candidate["ending_state"]
                .as_object()
                .expect("candidate body")
                .is_empty(),
            "{owner} body empty"
        );
        assert!(candidate["beginning_state_sha256"].is_string());
        assert_eq!(candidate["transaction_id"], 20_260_814_001_u64);
    }
    let owner_validation = fixture["post_ingress_owner_candidates"]["owner_validation"]
        .as_object()
        .expect("independent owner validation");
    for join in [
        "owner_candidate_set_sha256",
        "water_protocol_sha256",
        "material_join_sha256",
        "ground_heat_join_sha256",
    ] {
        assert!(owner_validation[join].is_string(), "missing {join}");
    }
    let soil_beginning = &fixture["post_ingress_owner_candidates"]["beginning"]["soil_thermal"]["state"]
        ["temperatures_k"];
    let soil_candidate = &owner_candidates["soil_thermal"]["ending_state"];
    let soil_operands = &fixture["post_ingress_owner_candidates"]["joins"]["soil_thermal_operands"];
    assert_eq!(soil_candidate["temperatures_k"][0], 292.283_849_300_950_35);
    let beginning_t1 = soil_beginning[0].as_f64().expect("beginning soil T1");
    let tile_fraction = soil_operands["layers"][0]["tile_fraction"]
        .as_f64()
        .expect("soil tile fraction");
    let capacity = soil_operands["layers"][0]["areal_heat_capacity_j_m2_k"]
        .as_f64()
        .expect("soil capacity");
    let ground_heat = soil_operands["layers"][0]["ground_heat_receipt_j_m2_stand_ground"]
        .as_f64()
        .expect("ground heat receipt");
    let infiltration = soil_operands["infiltration_enthalpy_receipt_j_m2_stand_ground"]
        .as_f64()
        .expect("infiltration enthalpy receipt");
    let reconstructed_t1 = beginning_t1 + (ground_heat + infiltration) / tile_fraction / capacity;
    assert!(
        (soil_candidate["temperatures_k"][0]
            .as_f64()
            .expect("candidate soil T1")
            - reconstructed_t1)
            .abs()
            < 1.0e-12
    );
    assert_eq!(
        soil_operands["infiltration_receiver_layer_id"],
        soil_operands["layers"][0]["layer_id"]
    );
    for (poison, failure) in [
        (
            "infiltration_enthalpy_omitted_from_soil_node",
            "infiltration_enthalpy_receipt_join",
        ),
        (
            "infiltration_enthalpy_duplicated_in_soil_node",
            "infiltration_enthalpy_receipt_join",
        ),
        (
            "infiltration_enthalpy_wrong_soil_node",
            "infiltration_enthalpy_wrong_soil_node",
        ),
        (
            "infiltration_enthalpy_wrong_area_basis",
            "infiltration_enthalpy_wrong_area_basis",
        ),
    ] {
        let record = &fixture["executed_poison_vectors"][poison];
        assert_eq!(record["accepted"], false);
        assert!(record["candidate"].is_null());
        assert_eq!(record["typed_failure"], failure);
    }
    assert_eq!(
        fixture["post_ingress_owner_candidates"]["owner_receipts"]
            .as_object()
            .expect("owner receipts")
            .len(),
        5
    );
    assert!(
        fixture["post_ingress_owner_candidates"]["owner_receipts"]
            .as_object()
            .expect("owner receipts")
            .values()
            .all(|receipt| receipt["beginning_state_sha256"].is_string()
                && receipt["candidate_state_sha256"].is_string()
                && receipt["owner_id"].is_string()
                && receipt["owner_kind"].is_string()
                && receipt["transaction_id"] == 20_260_814_001_u64)
    );
    let ingress = &fixture["post_ingress_owner_candidates"]["ingress"];
    let ending = ingress["ending_surface_enthalpy_j_m2_tile_ground"]
        .as_f64()
        .expect("ending surface enthalpy");
    let dry = ingress["ending_dry_body_enthalpy_j_m2_tile_ground"]
        .as_f64()
        .expect("ending dry-body enthalpy");
    let liquid = ingress["ending_liquid_enthalpy_j_m2_tile_ground"]
        .as_f64()
        .expect("ending liquid enthalpy");
    assert!((ending - dry - liquid).abs() < 1.0e-9);
    assert_eq!(
        fixture["shared_layer_root_ground_competition"]["finalized_uses"]
            .as_array()
            .expect("shared finalized uses")
            .len(),
        19
    );
    assert_eq!(
        fixture["shared_layer_root_ground_competition"]["source_ending_store_ledger"]
            .as_array()
            .expect("source ending-store ledger")
            .len(),
        6
    );
    assert!(
        fixture["positive_condensation_owner_transaction"]["condensation_energy_credit"]
            ["amount_kg_m2_stand_ground"]
            .as_f64()
            .expect("positive condensation credit")
            > 0.0
    );
    assert_eq!(
        fixture["positive_condensation_owner_transaction"]["owner_receipts"]
            .as_object()
            .expect("condensation owner receipts")
            .len(),
        5
    );
    let route_join = &fixture["multi_ofe_routed_owner_vector"]["route_join"];
    assert_eq!(route_join["source_ofe_area_m2"], 120.0);
    assert_eq!(route_join["destination_ofe_area_m2"], 200.0);
    assert_eq!(route_join["upstream_mass_kg_m2"], 0.6);
    assert_eq!(route_join["downstream_mass_kg_m2"], 0.36);
    assert_eq!(route_join["extensive_mass_kg"], 72.0);
    assert_eq!(
        route_join["extensive_mass_kg"],
        route_join["downstream_reconstructed_extensive_mass_kg"]
    );
    assert_eq!(
        route_join["extensive_energy_j"],
        route_join["downstream_reconstructed_extensive_energy_j"]
    );
    assert_eq!(
        route_join["upstream_state_sha256"],
        route_join["downstream_source_state_sha256"]
    );
    assert_eq!(
        fixture["strict_schema_instances"]
            .as_object()
            .expect("schema instances")
            .len(),
        6
    );
    assert!(
        fixture["strict_schema_validation"]
            .as_object()
            .expect("schema validation")
            .values()
            .all(|value| value["validated"] == true)
    );

    for (path, expected) in [
        (
            "reference_calculator.py",
            "86aae7c5d3c435e88170bae7b7ef838644242d790e56348a58bc9b587dc07c0c",
        ),
        (
            "reference_joint_canopy_core.py",
            "c9555b2dd02a5d6f11d71eb923fb60bc882e9638ec20eb79accc96cec9018be5",
        ),
        (
            "reference_lse_v8_joint_canopy_core.py",
            "525538f32c91e2377f5d58f72fa4cfff2e81d46d5e12555e79792d92e1e81d6f",
        ),
        (
            "lse_v1_configuration_schema.json",
            "6499b98cc1e25f1379bc0ad6052a7536e20c4bfbb9335f9ba5c8de191ae2f009",
        ),
        (
            "lse_v1_coupled_transaction_schema.json",
            "02dfa522b7d070df9a7d3e904d4f538a7f734eb6c8315fcbf033b7628b28e07f",
        ),
        (
            "lse_v1_diagnostics_schema.json",
            "41fb7909d073b4fdf4e59c9fa7da26b9a965ad916688b7867a56525d1bf1460c",
        ),
        (
            "lse_v1_forcing_schema.json",
            "2138cfbfd69bb7561db6f8e8b995077cd87fa066b49387c18a0252abf820ab70",
        ),
        (
            "lse_v1_state_schema.json",
            "91243e4087fa2c4775cb3629fe14c64379def4977d3c54a72348ac56d5fa4ee8",
        ),
        (
            "lse_v1_water_protocol_schema.json",
            "2e5ade752deb0751bb31222da5d8fe3f6a1e5fbee407e20780fa26242a7afd07",
        ),
    ] {
        let bytes = fs::read(format!("{AUTHORITY_PACKAGE}/{path}"))
            .unwrap_or_else(|error| panic!("read {path}: {error}"));
        assert_eq!(format!("{:x}", Sha256::digest(bytes)), expected, "{path}");
    }
}

#[test]
fn typed_failures_state_surface_and_guard_map_are_complete() {
    let contract = read(CONTRACT);
    for required in [
        "Required future inputs are one state identity",
        "Required outputs are end state",
        "Mutated state is limited to `E_s` and `M_l`",
        "### Invariant Guard Map",
        "| Invariant ID | Enforcement path | Guard class | Failure behavior | Evidence artifact |",
        "LSEB-E-001",
        "LSEB-E-010",
        "LSEB-E-011",
        "LSEB-E-012",
        "LSEB-E-013",
        "LSEB-E-014",
        "LSEB-E-015",
        "LSEB-E-020",
        "LSEB-E-021",
        "`git show\ndac3c950d8b16cc73774bf5ce2e7e11f80baac70:<path>`",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }

    let guard_map = contract
        .split("### Invariant Guard Map")
        .nth(1)
        .expect("guard map section")
        .split("## Producer Obligations")
        .next()
        .expect("guard map terminator");
    for id in [
        "001", "002", "010", "011", "012", "013", "014", "015", "020", "021", "022", "030", "031",
        "032", "040", "041",
    ] {
        let invariant = format!("`INV-LANDSURFACEENERGY-{id}`");
        assert!(
            guard_map.contains(&format!("| {invariant} |")),
            "guard map missing {invariant}"
        );
    }

    for (symbol, units) in [
        ("A", "`m^2`"),
        ("dt", "`s`"),
        ("T_s", "`K`"),
        ("E_s,0`, `E_s,1", "`J m^-2`"),
        ("M_l,0`, `M_l,1", "`kg m^-2`"),
        ("R_sw`, `R_lw", "`W m^-2`"),
        ("H", "`W m^-2`"),
        ("LE", "`W m^-2`"),
        ("Q_p`, `Q_runon", "`W m^-2`"),
        ("Q_inf`, `Q_runoff", "`W m^-2`"),
        ("G", "`W m^-2`"),
        ("m_p`, `m_runon", "`kg m^-2`"),
        ("m_evap", "`kg m^-2`"),
        ("m_inf`, `m_runoff", "`kg m^-2`"),
    ] {
        assert!(
            row(&contract, symbol).contains(units),
            "{symbol} units changed"
        );
    }

    for mapping in [
        "non-finite/unit/domain failure | reject before mutation | `LSEB-E-001`",
        "duplicate/missing component lineage | reject | `LSEB-E-010`",
        "energy or water closure exceeds tolerance | reject atomically | `LSEB-E-011` / `LSEB-E-012`",
        "latent mass-energy mismatch | reject | `LSEB-E-013`",
        "ground-flux dual ownership | reject | `LSEB-E-014`",
        "negative end storage beyond tolerance | reject; no clamp/default | `LSEB-E-015`",
        "snow present | delegate exclusively to snow owner; no LSE mutation | `LSEB-E-020` on attempted dual evaluation",
        "snow terminal/censored payload present | reject; there is no v1 recipient | `LSEB-E-021`",
        "future branch selector plus poison vectors | runtime | `LSEB-E-020/021`; currently `HOLD`",
    ] {
        assert!(
            contract.contains(mapping),
            "typed mapping changed: {mapping}"
        );
    }
}

#[test]
fn schema_sections_test_vectors_and_registry_are_bound() {
    let contract = read(CONTRACT);
    let index = read(INDEX);
    for heading in [
        "## Purpose",
        "## Scientific Scope and Explicit Out-of-Scope Boundaries",
        "## Authority Anchors with Top-Down Citations",
        "## Variables and Units Using Canonical Symbols First",
        "## Algorithm State Surfaces",
        "## Algorithm Specification with Step Sequence",
        "## Branch and Guard Table",
        "## Invariants and Invariant Guard Map",
        "## Producer Obligations and Consumer Obligations",
        "## Symbol Alias Map",
        "## Constants and Parameters with Provenance Anchors",
        "## Unit-Governance Map",
        "## Tolerance and Numeric Notes",
        "## Calibration and Identifiability",
        "## Test-Vector Obligations",
        "## Binding Exposure Index",
        "## Gap Register and Promotability Labels",
        "## Change Log",
    ] {
        assert!(contract.contains(heading), "{CONTRACT} missing {heading}");
    }
    for poison in [
        "omit and duplicate precipitation water and",
        "runon water and heat",
        "infiltration water and heat",
        "runoff water and heat",
        "latent",
        "sensible",
        "shortwave",
        "longwave",
        "ground heat",
        "storage change",
        "evaporation",
        "infiltration",
        "runoff",
    ] {
        assert!(contract.contains(poison), "{CONTRACT} missing {poison}");
    }
    assert!(index.contains("| `SC-LANDSURFACEENERGY-001` |"));

    for expected in [
        "| all-zero flux | unchanged energy and water state with positive `dt` | `INV-010/011` |",
        "| terminal schema-v8 payload | reject with no mutation | `INV-021`, `LSEB-E-021` |",
        "| ground sign reversal | surface `G` equals soil/frost `-G` | `INV-013`, `LSEB-E-014` |",
        "| all-distinct operands | independently reconstructed `epsilon_E/epsilon_M` pass | `INV-010/011` |",
    ] {
        assert!(
            contract.contains(expected),
            "vector mapping changed: {expected}"
        );
    }

    for gap in ["001", "002", "003", "006"] {
        let line = row(&contract, &format!("GAP-LANDSURFACEENERGY-{gap}"));
        assert!(
            line.contains("AUTHORITY_ADMITTED") || line.contains("authority portion admitted"),
            "gap {gap} lacks admitted authority"
        );
    }
    assert!(row(&contract, "GAP-LANDSURFACEENERGY-004").contains("`IMPLEMENTATION_MISSING`"));
    assert!(row(&contract, "GAP-LANDSURFACEENERGY-005").contains("`AUTHORITY_MISSING`"));
}
