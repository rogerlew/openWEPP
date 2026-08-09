use std::fs;
use std::process::Command;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md";
const INDEX: &str = "docs/specifications/science-contracts/index.md";
const PACKAGE: &str =
    "docs/work-packages/20260808-vegetation-source-provenance-and-boundary-authority-001";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

fn section<'a>(text: &'a str, start: &str, end: &str) -> &'a str {
    text.split_once(start)
        .unwrap_or_else(|| panic!("missing section start {start}"))
        .1
        .split_once(end)
        .unwrap_or_else(|| panic!("missing section end {end}"))
        .0
}

fn contains_table_row(table: &str, id: &str) -> bool {
    table.lines().any(|line| {
        line.starts_with(&format!("| {id} |")) || line.starts_with(&format!("| `{id}` |"))
    })
}

fn identifiers(text: &str, prefix: &str) -> Vec<String> {
    let mut rest = text;
    let mut found = Vec::new();
    while let Some(offset) = rest.find(prefix) {
        let candidate = &rest[offset..];
        let end = candidate
            .find(|character: char| {
                !(character.is_ascii_uppercase() || character.is_ascii_digit() || character == '-')
            })
            .unwrap_or(candidate.len());
        found.push(candidate[..end].to_owned());
        rest = &candidate[end..];
    }
    found.sort();
    found.dedup();
    found
}

fn sha256(path: &str) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .unwrap_or_else(|error| panic!("run sha256sum for {path}: {error}"));
    assert!(output.status.success(), "sha256sum failed for {path}");
    String::from_utf8(output.stdout)
        .expect("sha256sum output is UTF-8")
        .split_whitespace()
        .next()
        .expect("sha256sum digest")
        .to_owned()
}

#[test]
fn canonical_schema_and_registry_entry_are_bound() {
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

    for required in [
        "contract_id: SC-VEGETATION-001",
        "Version 1 admits no vegetation-process numerical constant",
        "source-derived formulas, constants, bounds, defaults, naming, or control",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
    let registry_row = index
        .lines()
        .find(|line| line.starts_with("| `SC-VEGETATION-001` |"))
        .expect("SC-VEGETATION-001 registry row");
    for field in [
        "| `in_review` | `draft` |",
        "| `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` |",
        "| `static` | `2026-08-08` |",
        "Native stratum topology, Stage A/B/C custody, shared ledgers",
    ] {
        assert!(registry_row.contains(field), "registry row missing {field}");
    }
}

#[test]
fn local_definition_acquisition_and_typed_schema_are_fail_closed() {
    let contract = read(CONTRACT);

    for required in [
        "The schema-form portion of `AUTH-RHEC-001` and all authority requirements of",
        "complete selected\nconsumed-field manifest and aliases remain missing",
        "(repository, immutable_commit,\n   repository_relative_path, sha256)",
        "caller-supplied local bytes",
        "HTTP, HTTPS, FTP, mutable branch names such as `master`",
        "Duplicate\n   keys are evidence and a resolution error",
        "No parser default fills an absent field",
        "Schema admission does not admit an empirical value",
        "Initial state is a dated stand/plot observation",
        "cannot be created by averaging raw or resolved parameter records",
        "one-bit digest",
        "mismatch; mutable reference",
        "rejected cadence/unit alias",
        "INV-VEGETATION-053",
        "INV-VEGETATION-054",
        "INV-VEGETATION-055",
        "INV-VEGETATION-056",
        "AUTHORITY_ADMITTED`, `IMPLEMENTATION_MISSING",
        "INITIAL_STATE_AUTHORITY_MISSING",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn native_strata_topology_and_configuration_separation_are_explicit() {
    let contract = read(CONTRACT);

    for required in [
        "Configuration, parameter sets, initial state, and evolving state are distinct versioned objects",
        "Horizontal tiles are non-overlapping",
        "sum_t f_t = 1",
        "A tile contains at most one stratum at a given\n   vertical rank",
        "C_s = sum(f_t for tiles containing s)",
        "C_union = sum(f_t for tiles containing at least one stratum)",
        "cross-rank cover may sum above one without an\n   independence assumption",
        "Root participation is explicit by soil layer",
        "Sort vertical ranks top to bottom",
        "VEG-E-001/002/003",
        "GAP-VEGETATION-001/008",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn stage_transaction_has_single_mutators_bounded_receipts_and_atomic_commit() {
    let contract = read(CONTRACT);

    let stage_a = contract.find("**Assemble Stage A.**").expect("Stage A");
    let stage_b = contract.find("**Assemble Stage B.**").expect("Stage B");
    let stage_c = contract.find("**Assemble Stage C.**").expect("Stage C");
    let commit = contract.find("**Close and commit.**").expect("commit");
    assert!(stage_a < stage_b && stage_b < stage_c && stage_c < commit);

    for required in [
        "Hydrology evaluates all same-`tau` demands",
        "0 <= U_s,l <= D_s,l",
        "sum_s U_s,l + W_comp,l <= A_l",
        "no individually valid request can overbook",
        "fully_supplied",
        "zero_demand",
        "liquid_storage_limit",
        "frozen_exclusion",
        "rooting_exclusion",
        "competing_demand",
        "T_s = sum_l U_s,l",
        "Vegetation never mutates soil-layer liquid/frozen state",
        "hydrology is sole Stage B mutator",
        "atomically commit",
        "errors leave\nevery owner state byte-identical",
        "VEG-E-020/021",
        "VEG-E-022",
        "VEG-E-030",
        "VEG-E-032/041",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn shared_water_energy_and_elemental_operands_cannot_alias() {
    let contract = read(CONTRACT);
    let variables = section(
        &contract,
        "## Variables and Units Using Canonical Symbols First",
        "## Algorithm State Surfaces",
    );

    for required in [
        "Q_T,s = -h_v*T_s",
        "Actual transpiration and latent energy share one transaction/stratum/lineage",
        "Canopy, ground, litter, snow, soil, ponded-water, and atmospheric radiation/latent terms remain distinct",
        "start + incident - evaporation - every release = end",
        "canopy/ground/litter/snow/soil poison aliases",
        "dry matter/C/N transfer",
        "donor and receiver reconstruct same three distinct operands",
        "VEG-E-011/032",
        "VEG-E-031",
        "no partial mutation or publication",
        "Every transfer above is an amount integrated over `tau`",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
    assert!(!variables.contains("kg m^-2 interval^-1"));
    assert!(!variables.contains("J m^-2 interval^-1"));
    for symbol in ["P_liq,s", "E_int,s", "R_down,s", "D_s,l", "U_s,l", "T_s"] {
        let row = variables
            .lines()
            .find(|line| line.starts_with(&format!("| `{symbol}` |")))
            .unwrap_or_else(|| panic!("variables table missing {symbol}"));
        assert!(
            row.contains("`kg m^-2`"),
            "{symbol} is not an interval amount"
        );
    }
    let latent = variables
        .lines()
        .find(|line| line.starts_with("| `Q_T,s` |"))
        .expect("latent energy row");
    assert!(latent.contains("`J m^-2`"));
}

#[test]
fn canonical_authority_and_test_vector_references_resolve() {
    let contract = read(CONTRACT);
    let anchors = section(
        &contract,
        "## Authority Anchors with Top-Down Citations",
        "## Variables and Units Using Canonical Symbols First",
    );
    let invariants = section(
        &contract,
        "## Invariants and Invariant Guard Map",
        "### Invariant Guard Map",
    );
    let vectors = section(
        &contract,
        "## Test-Vector Obligations",
        "## Binding Exposure Index",
    );

    for reference in identifiers(invariants, "REF-") {
        assert!(
            contains_table_row(anchors, &reference),
            "unresolved invariant authority {reference}"
        );
    }
    for invariant in identifiers(vectors, "INV-") {
        assert!(
            contains_table_row(invariants, &invariant),
            "unresolved test-vector invariant {invariant}"
        );
    }
}

#[test]
fn canopy_snow_compatibility_calibration_and_gaps_remain_non_promotable() {
    let contract = read(CONTRACT);
    let canopy = read(&format!(
        "{PACKAGE}/artifacts/canopy-snow-ownership-disposition.md"
    ));
    let compliance = read(&format!("{PACKAGE}/artifacts/compliance-review.md"));

    for required in [
        "Vegetation owns intercepted canopy snow; snow/frost owns ground snow",
        "v1 admits no canopy-snow constitutive law",
        "read-only, never feeds native state",
        "cannot support cutover without real downstream consumption",
        "science_implementation_status = AUTHORITY_MISSING",
        "calibration_evidence_status = NOT_CALIBRATION_READY",
        "identifiability_status = NOT_ASSESSED",
        "DIRECT_TRANSLATION_PROHIBITED",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
    for gap in 1..=10 {
        let gap = format!("GAP-VEGETATION-{gap:03}");
        assert!(contract.contains(&gap), "{CONTRACT} missing {gap}");
    }

    assert!(canopy.contains("NON_PROMOTABLE"));
    assert!(canopy.contains("does not admit a formula"));
    assert!(compliance.contains("Status: PASS"));
    let approved = format!("{PACKAGE}/artifacts/approved-spec.md");
    let approved_digest = sha256(&approved);
    assert_eq!(
        approved_digest,
        "afd6044612f15ec0838bafd1c3ed63a5e06f912b0dc3224c5249eb656a6e988b"
    );
    assert!(compliance.contains(&approved_digest));
}

#[test]
fn adjacent_contracts_retain_current_owners_until_real_consumer_cutover() {
    for (path, invariant, invariant_start, invariant_end, guard_start, guard_end) in [
        (
            "docs/specifications/science-contracts/contracts/SC-PLANT-001.md",
            "INV-PLANT-040",
            "## Invariants",
            "## Allowed Degenerate States",
            "## Guard Map",
            "## Symbol Alias Map",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-EVAP-001.md",
            "INV-EVAP-028",
            "## Invariants",
            "## Invariant Guard Map",
            "## Invariant Guard Map",
            "## Symbol Alias Map",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md",
            "INV-RESIDUE-023",
            "## Invariants",
            "## Invariant Guard Map",
            "## Invariant Guard Map",
            "## Symbol Alias Map",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
            "INV-WATBAL-101",
            "## Invariants",
            "## Binding Exposure Index",
            "## Invariant Guard Map",
            "## Symbol Alias Map",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md",
            "INV-LANDSURFACEENERGY-042",
            "## Invariants and Invariant Guard Map",
            "### Invariant Guard Map",
            "### Invariant Guard Map",
            "## Producer Obligations and Consumer Obligations",
        ),
    ] {
        let adjacent = read(path);
        let invariant_table = section(&adjacent, invariant_start, invariant_end);
        let guard_table = section(&adjacent, guard_start, guard_end);
        assert!(
            contains_table_row(invariant_table, invariant),
            "{path} invariant table missing {invariant}"
        );
        assert!(
            contains_table_row(guard_table, invariant),
            "{path} guard map missing {invariant}"
        );
        assert!(
            adjacent.contains("SC-VEGETATION-001"),
            "{path} missing vegetation boundary link"
        );
    }

    let energy =
        read("docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md");
    let energy_invariants = section(
        &energy,
        "## Invariants and Invariant Guard Map",
        "### Invariant Guard Map",
    );
    let energy_guards = section(
        &energy,
        "### Invariant Guard Map",
        "## Producer Obligations and Consumer Obligations",
    );
    assert!(contains_table_row(
        energy_invariants,
        "INV-LANDSURFACEENERGY-043"
    ));
    assert!(contains_table_row(
        energy_guards,
        "INV-LANDSURFACEENERGY-043"
    ));
}

#[test]
fn assurance_receipts_form_the_recorded_generation_chain() {
    let initial_plant = read(
        "assurance/v2/transactions/3208ab181e5eb9261a51bb3d8ea63d25c133244b8cf25b6949b4f4eb3a26cc1f.json",
    );
    let initial_residue = read(
        "assurance/v2/transactions/b8f0e7ed62428b2e3bfd9a5fb603ca45e9698dd9d4a63b3a6c435f925d81458b.json",
    );
    let review_plant = read(
        "assurance/v2/transactions/bb0bd503a7db0b2211c1810994f464165f98a87cf71bb8fc964cddffda0d4c7e.json",
    );
    let review_residue = read(
        "assurance/v2/transactions/642d054e638f3a6a3301e9af61f1dee6aa6bbadcbd63108145bc9762c980212f.json",
    );
    let terminal_plant = read(
        "assurance/v2/transactions/c1d9ad5502198d10faef62be994525bf23cb69375114a27275028c0d14d1bcaa.json",
    );
    let terminal_residue = read(
        "assurance/v2/transactions/df95b74417166de4ef891f20db27f3b1cad1c0d89be907b7fa582323a21363c6.json",
    );
    let identity = read("assurance/v2/identity.lock.json");
    let impact = read(&format!("{PACKAGE}/artifacts/assurance-impact.md"));

    for required in [
        "docs/specifications/science-contracts/contracts/SC-PLANT-001.md",
        "92aeea68348b5acb69bc281a495a56b21e1323ba430e4fe4b3d39ef613bf9f07",
    ] {
        assert!(initial_plant.contains(required));
    }
    for required in [
        "92aeea68348b5acb69bc281a495a56b21e1323ba430e4fe4b3d39ef613bf9f07",
        "docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md",
        "78d0a933bc07d82ef29f085b3e02485c0c165a78f0be7eaf291f1c79cd93f750",
    ] {
        assert!(initial_residue.contains(required));
    }
    for required in [
        "78d0a933bc07d82ef29f085b3e02485c0c165a78f0be7eaf291f1c79cd93f750",
        "docs/specifications/science-contracts/contracts/SC-PLANT-001.md",
        "100681154b587d34d29b5c83328016c5d3ed0828290a50eb5714eea9cea57b63",
    ] {
        assert!(review_plant.contains(required));
    }
    for required in [
        "100681154b587d34d29b5c83328016c5d3ed0828290a50eb5714eea9cea57b63",
        "docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md",
        "f509db04ab810d00a2eded24d32e87189e6ee0c3a2c9650fa28eb80e980cb11f",
    ] {
        assert!(review_residue.contains(required));
    }
    for required in [
        "f509db04ab810d00a2eded24d32e87189e6ee0c3a2c9650fa28eb80e980cb11f",
        "docs/specifications/science-contracts/contracts/SC-PLANT-001.md",
        "22b4fda37bc176ae4fedc04fdeb13900304a13f72eb6c920194e422627af6817",
    ] {
        assert!(terminal_plant.contains(required));
    }
    for required in [
        "22b4fda37bc176ae4fedc04fdeb13900304a13f72eb6c920194e422627af6817",
        "docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md",
        "90313e7b476cb5366605a1a708c29b5c2eeb68ecac36f90b00b9160b882c4fd8",
    ] {
        assert!(terminal_residue.contains(required));
    }
    assert!(identity.contains("90313e7b476cb5366605a1a708c29b5c2eeb68ecac36f90b00b9160b882c4fd8"));
    assert!(impact.contains(
        "initial `SC-PLANT-001.md` | `3208ab181e5eb9261a51bb3d8ea63d25c133244b8cf25b6949b4f4eb3a26cc1f`"
    ));
    assert!(impact.contains(
        "initial `SC-RESIDUE-001.md` | `b8f0e7ed62428b2e3bfd9a5fb603ca45e9698dd9d4a63b3a6c435f925d81458b`"
    ));
    assert!(impact.contains(
        "review remediation `SC-PLANT-001.md` | `bb0bd503a7db0b2211c1810994f464165f98a87cf71bb8fc964cddffda0d4c7e`"
    ));
    assert!(impact.contains(
        "review remediation `SC-RESIDUE-001.md` | `642d054e638f3a6a3301e9af61f1dee6aa6bbadcbd63108145bc9762c980212f`"
    ));
    assert!(impact.contains(
        "terminal addendum cleanup `SC-PLANT-001.md` | `c1d9ad5502198d10faef62be994525bf23cb69375114a27275028c0d14d1bcaa`"
    ));
    assert!(impact.contains(
        "terminal addendum cleanup `SC-RESIDUE-001.md` | `df95b74417166de4ef891f20db27f3b1cad1c0d89be907b7fa582323a21363c6`"
    ));
}
