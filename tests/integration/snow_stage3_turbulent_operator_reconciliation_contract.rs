use std::fs;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const INDEX: &str = "docs/specifications/science-contracts/index.md";
const PACKAGE: &str = "docs/work-packages/20260806-snow-stage3-turbulent-carrier-lineage-and-operator-reconciliation-001/package.md";
const PROTOCOL: &str = "docs/work-packages/20260806-snow-stage3-turbulent-carrier-lineage-and-operator-reconciliation-001/artifacts/protocol-freeze.json";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

fn section<'a>(document: &'a str, start: &str, end: &str) -> &'a str {
    let start_index = document
        .find(start)
        .unwrap_or_else(|| panic!("missing section start {start}"));
    let remainder = &document[start_index..];
    let end_index = remainder
        .find(end)
        .unwrap_or_else(|| panic!("missing section end {end}"));
    &remainder[..end_index]
}

#[test]
fn v129_admits_only_additive_default_off_operator_reconciliation() {
    let contract = read(CONTRACT);

    for required in [
        "contract_version: 129",
        "REF-SNOWFREEZE-STAGE3-OPERATOR-RECONCILIATION",
        "INV-SNOWFREEZE-096",
        "OBL-SNOWFREEZE-P-069",
        "OBL-SNOWFREEZE-C-011",
        "one boxed, enabled-only reconciliation companion",
        "internal schema-v6 row",
        "share one private Monin-Obukhov solver",
        "duplicated formulas, coefficients, convergence logic, or arithmetic order are prohibited",
        "Historical schema-v5 meaning remains unchanged",
        "disabled/default rows remain exact schema v4 with no companion allocation",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn v129_binds_exact_tuple_lineage_and_independent_reconstruction() {
    let contract = read(CONTRACT);

    for required in [
        "ordered duration-tagged tuples",
        "typed applicability instead of numeric-zero N/A",
        "`z_T/z_q/z_u/z_0,aero`",
        "solver options/status, stability length/corrections/log factors",
        "explicit albedo source/model/state",
        "active/lower cold-content change",
        "mixed sequential complete = comparable external subset + active internal conduction",
        "`F-S` projection and `Q-F` evolution effects",
        "without importing producer calculation helpers",
        "No production or public consumer may read schema-v6 evidence as state or",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn v129_retains_production_and_claim_holds() {
    let contract = read(CONTRACT);
    let index = read(INDEX);

    for required in [
        "no result may change equations, constants, thresholds, defaults, ownership, state, ledgers, fixtures, observations, or public outputs",
        "claim seasonal physical chronology, terminal closure, validation, promotion, CoE retirement, or cutover",
        "CoE ownership",
        "seasonal/terminal/cutover holds remain unchanged",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
    for required in [
        "v129",
        "schema-v6 turbulent/state lineage",
        "historical schema-v5 semantics",
        "CoE ownership",
    ] {
        assert!(index.contains(required), "{INDEX} missing {required}");
    }
}

#[test]
fn admitted_protocol_freezes_support_bridge_and_decision_rules() {
    let package = read(PACKAGE);
    let protocol = read(PROTOCOL);

    for required in [
        "result-blind protocol admitted PASS/PASS",
        "same_state_paired_carrier_v1",
        "sequential_resolved_shadow_v1",
        "frozen_active_projection_reference",
        "CoE remains the sole authoritative production melt",
    ] {
        assert!(package.contains(required), "{PACKAGE} missing {required}");
    }
    for required in [
        "frozen_result_blind_admitted_pass_pass",
        "common_support_integration",
        "legacy_sequential_bridge",
        "ordered_three_way_support",
        "delta_projection=F-S and delta_evolution=Q-F",
        "INITIAL_CONTROL_VOLUME_PROJECTION_RECONCILES_SIGN_CONTRADICTION",
        "STATE_EVOLUTION_RECONCILES_SIGN_CONTRADICTION",
        "not promotion or cutover",
        "zero_alias_prohibited",
    ] {
        assert!(protocol.contains(required), "{PROTOCOL} missing {required}");
    }
}

#[test]
fn v129_canonical_addendum_pins_exact_algorithm_units_and_failures() {
    let contract = read(CONTRACT);
    let addendum = section(
        &contract,
        "## Stage 3 Turbulent Carrier And Operator Reconciliation Addendum",
        "## Wet-Compaction Operand Authority And Duplicate-Alias Closure Addendum",
    );
    let addendum = addendum.split_whitespace().collect::<Vec<_>>().join(" ");

    for required in [
        "SC-SNOWENERGY-001#INV-SNOWENERGY-029",
        "SC-SNOWENERGY-001#INV-SNOWENERGY-030",
        "aerodynamic_roughness_length_m",
        "bare `z_0` remains a rejected alias",
        "snow_albedo_source_id",
        "stage3_default_snow_albedo_0p82",
        "24 * 60 = 1,440",
        "post_substep_no_resolved_surface",
        "total_ice_mass_after_kg_m2",
        "- lower_cold_energy_change_j_m2 - cold_content_export_j_m2",
        "legacy_sequential_complete_j_m2",
        "iterative_zero_buoyancy",
        "iterative_invalid_obukhov",
        "did_not_converge",
        "fails without authoritative mutation",
        "frozen_active_projection_reference",
        "min(S_evaluated_seconds, 3600, Q_evaluated_seconds)",
        "delta_projection = F - S",
        "delta_evolution = Q - F",
        "`S < -tol`, `F > +tol`, and `Q > +tol`",
        "`S < -tol`, `F < -tol`, and `Q > +tol`",
        "+170.2536089 MJ m^-2",
        "SUPPORT_CENSORING_MATERIALLY_CONTRIBUTES",
        "A zero denominator makes the ratio N/A",
    ] {
        assert!(addendum.contains(required), "canonical addendum missing {required}");
    }

    let aliases = section(&contract, "## Symbol Alias Map", "## Allowed Degenerate States");
    let aliases = aliases.split_whitespace().collect::<Vec<_>>().join(" ");
    for required in [
        "snow_stage3_operator_reconciliation_tuple",
        "scalar/vector diagnostic exception",
        "HillslopeProductionStateSymbol",
        "hourly radiation `MJ m^-2 * 10^6 / 3600 -> W m^-2`",
    ] {
        assert!(aliases.contains(required), "alias map missing {required}");
    }

    let tolerances = section(
        &contract,
        "## Tolerance and Numeric Notes",
        "## Stage 3 Evaluation Shadow Authority Addendum",
    );
    let tolerances = tolerances.split_whitespace().collect::<Vec<_>>().join(" ");
    for required in [
        "TOL-SNOWFREEZE-018",
        "max(1e-10 W m^-2, 1e-12 * sum_abs_operands)",
        "max(1e-12 kg m^-2, 1e-12 * sum_abs_operands)",
        "TOL-SNOWFREEZE-019",
        "1e-7 MJ m^-2",
        "support materiality ratio threshold `0.05`",
    ] {
        assert!(tolerances.contains(required), "tolerance table missing {required}");
    }
}

#[test]
fn v129_binding_surfaces_each_retain_inv_096() {
    let contract = read(CONTRACT);
    let guard = section(&contract, "## Invariant Guard Map", "## Symbol Alias Map");
    let boundary = section(
        &contract,
        "## Boundary Disposition",
        "## Tolerance and Numeric Notes",
    );
    let exposure = section(&contract, "## Binding Exposure Index", "## Known Gaps");

    for (name, scoped) in [
        ("guard map", guard),
        ("boundary disposition", boundary),
        ("binding exposure", exposure),
    ] {
        assert!(scoped.contains("INV-SNOWFREEZE-096"), "{name} lost INV-SNOWFREEZE-096");
    }
    for required in [
        "OBL-SNOWFREEZE-P-069",
        "OBL-SNOWFREEZE-C-011",
        "TOL-SNOWFREEZE-018",
        "TOL-SNOWFREEZE-019",
        "`flagged-binding-addition`",
    ] {
        assert!(exposure.contains(required), "binding exposure missing {required}");
    }
}
