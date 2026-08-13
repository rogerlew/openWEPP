use std::fs;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;

use serde_json::Value;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md";
const INDEX: &str = "docs/specifications/science-contracts/index.md";
const PACKAGE: &str =
    "docs/work-packages/20260808-vegetation-source-provenance-and-boundary-authority-001";
const COUPLED_PACKAGE: &str =
    "docs/work-packages/20260811-coupled-c3-forest-vegetation-model-stack-authority-001";
const V2_AUTHORITY_PACKAGE: &str =
    "docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001";
const V3_AUTHORITY_PACKAGE: &str =
    "docs/work-packages/20260812-c3-woody-potential-pass-authority-001";
const V4_AUTHORITY_PACKAGE: &str =
    "docs/work-packages/20260812-c3-woody-shared-state-authority-001";
const V6_AUTHORITY_PACKAGE: &str =
    "docs/work-packages/20260813-c3-woody-failure-diagnostic-portability-authority-001";
const V7_AUTHORITY_PACKAGE: &str =
    "docs/work-packages/20260813-c3-woody-storage-transfer-phenology-authority-001";

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

fn sha256_text(value: &str) -> String {
    let mut child = Command::new("sha256sum")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn sha256sum");
    child
        .stdin
        .take()
        .expect("sha256sum stdin")
        .write_all(value.as_bytes())
        .expect("write sha256 bytes");
    let output = child.wait_with_output().expect("wait for sha256sum");
    assert!(output.status.success());
    String::from_utf8(output.stdout)
        .expect("sha256 UTF-8")
        .split_whitespace()
        .next()
        .expect("sha256 digest")
        .to_owned()
}

fn json_number_after(text: &str, key: &str) -> f64 {
    let marker = format!("\"{key}\": ");
    let value = text
        .split_once(&marker)
        .unwrap_or_else(|| panic!("missing numeric JSON key {key}"))
        .1
        .split([',', '\n', '}'])
        .next()
        .expect("numeric JSON token")
        .trim();
    value
        .parse::<f64>()
        .unwrap_or_else(|error| panic!("invalid numeric JSON value for {key}: {error}"))
}

fn inclusive_section<'a>(text: &'a str, start: &str, end: &str) -> &'a str {
    let start_offset = text
        .find(start)
        .unwrap_or_else(|| panic!("missing {start}"));
    let end_offset = text[start_offset..]
        .find(end)
        .map_or_else(|| panic!("missing {end}"), |offset| start_offset + offset);
    &text[start_offset..end_offset]
}

fn assert_v3_section_digests(vegetation: &str, parsed: &Value) {
    for (key, start, end) in [
        (
            "vegetation_variables",
            "## Variables and Units Using Canonical Symbols First\n",
            "## Algorithm State Surfaces\n",
        ),
        (
            "vegetation_algorithm_and_equations",
            "## Algorithm Specification with Step Sequence\n",
            "## Branch and Guard Table\n",
        ),
        (
            "vegetation_invariants",
            "## Invariants and Invariant Guard Map\n",
            "### Invariant Guard Map\n",
        ),
        (
            "vegetation_schema",
            "## Constants and Parameters with Provenance Anchors\n",
            "## Unit-Governance Map\n",
        ),
        (
            "vegetation_numerics",
            "## Tolerance and Numeric Notes\n",
            "## Calibration and Identifiability\n",
        ),
    ] {
        assert_eq!(
            sha256_text(inclusive_section(vegetation, start, end)),
            parsed["canonical_section_sha256"][key]
                .as_str()
                .unwrap_or_else(|| panic!("missing V3 section digest {key}"))
        );
    }
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
        "contract_version: 11",
        "Version 7 admits the constitutive equations, topology inheritance, and V3",
        "Earlier-version statements limiting admission to",
        "source-derived formulas, constants, bounds, defaults, naming, or control",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
    let registry_row = index
        .lines()
        .find(|line| line.starts_with("| `SC-VEGETATION-001` |"))
        .expect("SC-VEGETATION-001 registry row");
    let lifecycle = if contract.contains("status: approved\nmaturity: active") {
        assert!(contract.contains("Status: `approved`\nMaturity: `active`"));
        "| `approved` | `active` |"
    } else {
        assert!(contract.contains("status: in_review\nmaturity: draft"));
        assert!(contract.contains("Status: `in_review`\nMaturity: `draft`"));
        "| `in_review` | `draft` |"
    };
    for field in [
        lifecycle,
        "| `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` |",
        "| `static` | `2026-08-13` |",
        "OPENWEPP_C3_WOODY_V7",
    ] {
        assert!(registry_row.contains(field), "registry row missing {field}");
    }
}

#[test]
fn local_definition_acquisition_and_typed_schema_are_fail_closed() {
    let contract = read(CONTRACT);

    for required in [
        "The schema-form portion of `AUTH-RHEC-001` and all authority requirements of",
        "Version 5's exact consumed\nfield inventory is the canonical field list",
        "runtime names equal canonical\nsnake-case names, and no RHESSys spelling is a consumed alias",
        "(repository, immutable_commit,\n   repository_relative_path, sha256)",
        "caller-supplied local bytes",
        "HTTP, HTTPS, FTP, mutable branch names such as `master`",
        "Duplicate\n   keys are evidence and a resolution error",
        "No parser default fills an absent field",
        "Schema admission does not admit an empirical value",
        "Initial state may be caller-supplied site state",
        "cannot be created by averaging raw or resolved parameter records",
        "one-bit digest",
        "mismatch; mutable reference",
        "rejected cadence/unit alias",
        "INV-VEGETATION-053",
        "INV-VEGETATION-054",
        "INV-VEGETATION-055",
        "INV-VEGETATION-056",
        "INV-VEGETATION-057",
        "INV-VEGETATION-058",
        "AUTHORITY_ADMITTED`, `IMPLEMENTATION_MISSING",
        "CALLER_CONFIGURATION",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn native_forest_values_and_flux_components_have_the_correct_authority_boundary() {
    let contract = read(CONTRACT);

    for required in [
        "Site-specific parameter values are caller-supplied `external_configuration`",
        "compatible initial state may be caller-supplied",
        "`ASSUMED_FOR_EXECUTION` fixtures",
        "no calibration, validation, ecosystem applicability, or transferability claim",
        "Agricultural `Kcb`/LAI PMET partition",
        "canopy transpiration, wet-canopy evaporation, and forest-floor evaporation",
        "must not automatically reassign lost canopy demand to forest-floor evaporation",
        "layer-resolved root requests",
        "Penman-Monteith is neither required nor prohibited",
        "Stevens Canyon",
        "INV-VEGETATION-057",
        "INV-VEGETATION-058",
        "INV-VEGETATION-059",
        "INV-VEGETATION-060",
        "INV-VEGETATION-061",
        "CALLER_CONFIGURATION",
        "NATIVE_FOREST_PMET_PARTITION_PROHIBITED",
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
    let stage_b = contract.find("**Authorize water.**").expect("Stage B");
    let stage_c = contract
        .find("**Finalize water and carbon.**")
        .expect("Stage C");
    let commit = contract.find("**Close and commit.**").expect("commit");
    assert!(stage_a < stage_b && stage_b < stage_c && stage_c < commit);

    for required in [
        "Hydrology evaluates all same-`tau` demands",
        "0 <= A_W,s,l <= D_s,l",
        "sum_s A_W,s,l + W_comp,l <= A_l",
        "no individually valid request can overbook",
        "fully_supplied",
        "zero_demand",
        "liquid_storage_limit",
        "frozen_exclusion",
        "rooting_exclusion",
        "competing_demand",
        "T_s=sum_l F_W,s,l",
        "Vegetation never mutates soil-layer liquid/frozen state",
        "hydrology alone validates finalized use and forms the soil candidate",
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
    for symbol in [
        "P_liq,s",
        "E_int,s",
        "R_down,s",
        "D_s,l",
        "A_W,s,l`, `F_W,s,l",
        "T_s",
    ] {
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
        "versions 1-7 admit no canopy-snow constitutive law",
        "read-only, never feeds native state",
        "cannot support cutover without real downstream consumption",
        "science_implementation_status = NOT_IMPLEMENTED",
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

#[test]
fn coupled_c3_model_stack_and_biogeochemistry_boundary_are_admitted() {
    let vegetation = read(CONTRACT);
    let bgc = read("docs/specifications/science-contracts/contracts/SC-BIOGEOCHEM-001.md");
    let selection = read(&format!(
        "{COUPLED_PACKAGE}/artifacts/model-stack-selection.md"
    ));

    for required in [
        "contract_version: 11",
        "OPENWEPP_C3_WOODY_V1",
        "OPENWEPP_C3_WOODY_V2",
        "OPENWEPP_C3_WOODY_V3",
        "OPENWEPP_C3_WOODY_V4",
        "OPENWEPP_C3_WOODY_V5",
        "OPENWEPP_C3_WOODY_V6",
        "OPENWEPP_C3_WOODY_V7",
        "FvCB--Medlyn",
        "LAI=leaf_C*SLA",
        "INV-VEGETATION-062",
        "INV-VEGETATION-072",
        "science_implementation_status = NOT_IMPLEMENTED",
        "003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157",
    ] {
        assert!(
            vegetation.contains(required),
            "vegetation contract missing {required}"
        );
    }
    for required in [
        "contract_id: SC-BIOGEOCHEM-001",
        "INV-BIOGEOCHEM-001",
        "INV-BIOGEOCHEM-005",
        "BGC-E-040",
        "proportional",
    ] {
        assert!(bgc.contains(required), "BGC contract missing {required}");
    }
    assert!(selection.contains("one averaged canopy leaf is degenerate"));
    assert!(selection.contains("LUNA is not selected for v1"));
    assert_eq!(
        sha256(&format!(
            "{COUPLED_PACKAGE}/artifacts/openwepp_c3_woody_v1_definition.json"
        )),
        "003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157"
    );
    let definition = read(&format!(
        "{COUPLED_PACKAGE}/artifacts/openwepp_c3_woody_v1_definition.json"
    ));
    for expected in [
        "e41d67e578b44f8d80050277565cfb7b164cbc2bc93d0823fdffdede4fce893e",
        "240b29fa886752d98153e94fc2fb604745b31a46116254aca5d59bad2701dcfb",
        "c72baf1931b4ca85ec3e6a0333b86ff70ddeaa1a27d2a69e5d8a383e30e471e3",
        "1423808f4405e977112af40535b7f0659dc2b07ef4c6e57baa5c825c0ce6c57e",
        "6f1a363bf06b5fc7c91c87cdd9161cb570ec3545117f38895f3e29780a082323",
    ] {
        assert!(definition.contains(expected));
    }
    assert_eq!(
        sha256("docs/specifications/science-contracts/contracts/SC-BIOGEOCHEM-001.md"),
        "6cfd2143f9941613e6f6324d2790f88773c9b9eafa1ab8cad72e5a95df6794b4"
    );
    assert!(
        definition.contains("6cfd2143f9941613e6f6324d2790f88773c9b9eafa1ab8cad72e5a95df6794b4")
    );
}

#[test]
fn v2_tile_liquid_authority_is_digest_bound_and_v1_is_historical() {
    let vegetation = read(CONTRACT);
    let v2_path = format!("{V2_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v2_definition.json");
    let v2 = read(&v2_path);
    assert_eq!(
        sha256(&v2_path),
        "38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3"
    );
    assert_eq!(
        sha256(&format!(
            "{COUPLED_PACKAGE}/artifacts/openwepp_c3_woody_v1_definition.json"
        )),
        "003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157"
    );
    assert_eq!(
        sha256(&format!(
            "{COUPLED_PACKAGE}/artifacts/openwepp_c3_woody_v2_definition.json"
        )),
        sha256(&v2_path)
    );
    for expected in [
        "9dc6a1c86a82d4dbbcae560c85ef19be0401c64e636d36dc2ce7e09b0f1e170e",
        "c6f7870681c6d337166d07cdd468125a8547a72b65210669d61eda212a5b3fe5",
        "78dc4a30f6ec134500154eb3058719f3710931b92b988596c797a44967991386",
        "53adb89c1415a5b6e5026b981263cb6d43a78ba50e34651a60e92d866b0958ec",
        "acf9972a00dfcc0101e8dda47a4b2ade8b4dea8ad8168b264f1d1db21e808222",
    ] {
        assert!(v2.contains(expected));
    }
    for (path, expected) in [
        (
            "docs/specifications/science-contracts/contracts/SC-BIOGEOCHEM-001.md",
            "6cfd2143f9941613e6f6324d2790f88773c9b9eafa1ab8cad72e5a95df6794b4",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md",
            "7de4887f9d62202427552f7ef9a677ac9668811cca84fa5d816dd9dc45bf9f69",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
            "c30b7c243a36f7fc2aec316c3ba590c8f7629759d36bf1f91b60c0cf0c419188",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-VEGETATIONTRANSACTION-001.md",
            "c94d3c5745fd801b092f992b46fb6f5d4684b70acf24f198c4d4d6fdc42785c8",
        ),
    ] {
        assert_eq!(sha256(path), expected);
        assert!(v2.contains(expected));
    }
    for required in [
        "INV-VEGETATION-073",
        "INV-VEGETATION-079",
        "LAI_s,t=LAI_s/C_s",
        "stemflow bypasses foliage to same-tile ground",
        "D_W,s,t,l=f_t*D_tile,s,t,l",
        "nonzero V1 store over multiple tiles",
    ] {
        assert!(
            vegetation.contains(required),
            "missing V2 authority {required}"
        );
    }
}

#[test]
fn v3_potential_pass_authority_is_digest_bound_and_prior_models_are_immutable() {
    let vegetation = read(CONTRACT);
    let v3_path = format!("{V3_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v3_definition.json");
    let model_stack_copy =
        format!("{COUPLED_PACKAGE}/artifacts/openwepp_c3_woody_v3_definition.json");
    let definition = read(&v3_path);
    let parsed: Value = serde_json::from_str(&definition).expect("V3 definition JSON");

    assert_eq!(
        sha256(&v3_path),
        "7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852"
    );
    assert_eq!(read(&model_stack_copy), definition);
    assert_eq!(sha256(&model_stack_copy), sha256(&v3_path));
    assert_eq!(parsed["model_version"], "OPENWEPP_C3_WOODY_V3");
    assert_eq!(parsed["canonical_contract"], "SC-VEGETATION-001@7");
    assert_eq!(
        parsed["base_model_definition"]["sha256"],
        "38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3"
    );
    assert_eq!(
        parsed["independent_fixture"]["sha256"],
        "1210e41f13aeffd2e099f9c812b8c5da6109ee9e23c6f51f045af9684a7ae109"
    );
    assert_eq!(
        parsed["independent_fixture"]["generator_sha256"],
        "7b137c1aa9ed0912caf4d14c779eca1819014b4217156d36f98619f06daabd1a"
    );
    assert_eq!(
        parsed["leaf_respiration"]["immutable_source"],
        "ESCOMP/CTSM@8e1309ab0db671d884b80746cbae9bbaafbe78a7 src/biogeophys/PhotosynthesisMod.F90 sha256=e4c9ad718209af44fcfdfc1d591bd2729d345f9e422cf5d9c8a889525d6a1cdf lines 1318-1322,1441-1447"
    );

    assert_v3_section_digests(&vegetation, &parsed);

    for (path, key, expected) in [
        (
            "docs/specifications/science-contracts/contracts/SC-BIOGEOCHEM-001.md",
            "biogeochemistry_contract",
            "6cfd2143f9941613e6f6324d2790f88773c9b9eafa1ab8cad72e5a95df6794b4",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md",
            "land_surface_energy_contract",
            "7de4887f9d62202427552f7ef9a677ac9668811cca84fa5d816dd9dc45bf9f69",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
            "water_balance_contract",
            "c30b7c243a36f7fc2aec316c3ba590c8f7629759d36bf1f91b60c0cf0c419188",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-VEGETATIONTRANSACTION-001.md",
            "vegetation_transaction_contract",
            "c94d3c5745fd801b092f992b46fb6f5d4684b70acf24f198c4d4d6fdc42785c8",
        ),
    ] {
        assert_eq!(sha256(path), expected);
        assert_eq!(parsed["canonical_section_sha256"][key], expected);
    }

    assert_eq!(
        sha256(&format!(
            "{COUPLED_PACKAGE}/artifacts/openwepp_c3_woody_v1_definition.json"
        )),
        "003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157"
    );
    assert_eq!(
        sha256(&format!(
            "{V2_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v2_definition.json"
        )),
        "38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3"
    );
}

#[test]
fn v3_independent_oracle_is_deterministic_and_fixture_is_not_rust_generated() {
    let fixture_path =
        format!("{V3_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v3_vectors.json");
    let before = read(&fixture_path);
    let before_digest = sha256(&fixture_path);
    let output = Command::new(".venv/bin/python")
        .arg(format!(
            "{V3_AUTHORITY_PACKAGE}/artifacts/reference_calculator.py"
        ))
        .output()
        .expect("run independent V3 oracle");
    assert!(
        output.status.success(),
        "V3 oracle failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        read(&fixture_path),
        before,
        "oracle regeneration changed bytes"
    );
    assert_eq!(sha256(&fixture_path), before_digest);
    assert_eq!(
        before_digest,
        "1210e41f13aeffd2e099f9c812b8c5da6109ee9e23c6f51f045af9684a7ae109"
    );
    let parsed: Value = serde_json::from_str(&before).expect("V3 fixture JSON");
    assert_eq!(parsed["model_version"], "OPENWEPP_C3_WOODY_V3");
    assert_eq!(parsed["oracle_independence"]["calls_rust"], false);
    assert_eq!(
        parsed["oracle_independence"]["expected_values_generated_by_rust"],
        false
    );
    assert!(
        parsed["checks"]
            .as_object()
            .expect("V3 check object")
            .values()
            .all(|value| value.as_bool() == Some(true))
    );
}

fn assert_v3_radiation_and_aerodynamics(families: &Value) {
    for band in ["VIS", "NIR"] {
        for component in ["direct", "diffuse"] {
            let vector = &families["radiation"]["two_rank"][band][component];
            assert!(vector["closure_residual"].as_f64().unwrap().abs() <= 2.0e-8);
            for occupancy in vector["occupancies"].as_array().unwrap() {
                let result = &occupancy["results"];
                let owner_absorption = result["absorbed_leaf_sun"].as_f64().unwrap()
                    + result["absorbed_leaf_shade"].as_f64().unwrap()
                    + result["absorbed_stem"].as_f64().unwrap();
                assert!(
                    (owner_absorption - result["absorbed_plant"].as_f64().unwrap()).abs()
                        <= 2.0e-10
                );
            }
        }
    }
    let zero_direct = &families["radiation"]["zero_direct_exact_branch"];
    assert_eq!(zero_direct["directional_operands_evaluated"], false);
    assert!(zero_direct["beam_k_unclumped"].is_null());
    assert!(zero_direct["k_eff"].is_null());
    assert_eq!(zero_direct["terminal_direct"].as_f64(), Some(0.0));
    let resonance = &families["radiation"]["resonance_exact_integral_branch"];
    assert!(
        (resonance["source_offset"][0].as_f64().unwrap()
            - resonance["analytic_first_component"].as_f64().unwrap())
        .abs()
            <= 1.0e-12
    );

    let aerodynamic = &families["aerodynamics"];
    let u_star = aerodynamic["results"]["u_star_m_s"].as_f64().unwrap();
    for semantic in ["u_leaf_m_s", "u_wet_m_s", "u_stem_m_s"] {
        assert_eq!(
            aerodynamic["results"]["semantic_winds"][semantic].as_f64(),
            Some(u_star)
        );
    }
    let conductances = aerodynamic["results"]["conductances"].as_object().unwrap();
    assert_eq!(conductances.len(), 3);
    assert_ne!(conductances["gb_leaf_m_s"], conductances["gb_wet_m_s"]);
    assert_ne!(conductances["gb_wet_m_s"], conductances["gb_stem_m_s"]);
}

fn assert_v3_hydraulics_and_migration(families: &Value) {
    let hydraulic = &families["hydraulic_potential_pass"];
    let accepted = &hydraulic["accepted_uncapped_stage_a"];
    let beta = accepted["solution"]["beta_hyd"].as_f64().unwrap();
    let beta_sun = accepted["solution"]["beta_hyd_sun"].as_f64().unwrap();
    let beta_shade = accepted["solution"]["beta_hyd_shade"].as_f64().unwrap();
    assert!(beta > 0.0 && beta < 1.0);
    assert!(beta_sun > 0.0 && beta_sun < 1.0);
    assert!(beta_shade > 0.0 && beta_shade < 1.0);
    assert!((beta_sun - beta_shade).abs() > f64::EPSILON);
    let emax = &hydraulic["internal_maximum_evaluation"]["emax"];
    let emax_sun = emax["sun"].as_f64().unwrap();
    let emax_shade = emax["shade"].as_f64().unwrap();
    let reconstructed_beta =
        (emax_sun * beta_sun + emax_shade * beta_shade) / (emax_sun + emax_shade);
    assert!((beta - reconstructed_beta).abs() <= 2.0e-14);
    for residual in accepted["closures"].as_object().unwrap().values() {
        assert!(residual.as_f64().unwrap().abs() <= 2.0e-10);
    }
    let normalized = accepted["normalized_residuals"].as_array().unwrap();
    assert_eq!(normalized.len(), 6);
    for identity in [
        "sun_gas_minus_q1",
        "shade_gas_minus_q1",
        "sun_gas_minus_vulnerability_demand",
        "shade_gas_minus_vulnerability_demand",
        "q1_sum_minus_q2",
        "q3_sum_minus_q2",
    ] {
        let component = normalized
            .iter()
            .find(|component| component["identity"] == identity)
            .unwrap_or_else(|| panic!("missing coupled residual {identity}"));
        assert!(component["normalized"].as_f64().unwrap().abs() <= 1.0);
        assert!(component["tolerance"].as_f64().unwrap() > 0.0);
    }
    let energy = &accepted["fluxes"]["canopy_energy_state"];
    for field in [
        "canopy_air_specific_humidity_kg_kg",
        "canopy_air_temperature_k",
        "dry_stem_temperature_k",
        "wet_surface_temperature_k",
    ] {
        assert!(energy[field].as_f64().unwrap().is_finite());
    }
    assert_eq!(energy["wet_store_cap_active"], true);
    assert_eq!(energy["normalized_residuals"].as_array().unwrap().len(), 6);
    assert!(
        energy["normalized_residuals"]
            .as_array()
            .unwrap()
            .iter()
            .all(|residual| residual.as_f64().is_some_and(|value| value.abs() <= 1.0))
    );
    assert_eq!(
        hydraulic["internal_maximum_evaluation"]["accepted_state_or_request"],
        false
    );
    assert_eq!(accepted["fluxes"]["stem_path_length_m"], 12.5);
    assert_eq!(accepted["fluxes"]["stem_gravity_head_mm"], 12500.0);
    assert_v3_biochemistry_aerodynamics_and_root_paths(hydraulic, accepted);
    let fraction = hydraulic["operands"]["tile_fraction"].as_f64().unwrap();
    let interval = hydraulic["operands"]["dt_s"].as_f64().unwrap();
    for request in accepted["water_requests"].as_array().unwrap() {
        let layer = request["layer_id"].as_str().unwrap();
        let flux = accepted["fluxes"]["q3"]
            .as_array()
            .unwrap()
            .iter()
            .find(|row| row["layer_id"] == layer)
            .expect("request layer flux")["flux"]
            .as_f64()
            .unwrap();
        assert!(
            (request["amount_kg_h2o_m2_stand_ground"].as_f64().unwrap()
                - fraction * interval * flux)
                .abs()
                <= 2.0e-14
        );
        if matches!(layer, "soil-dry" | "soil-frozen") {
            assert_eq!(flux.to_bits(), 0.0_f64.to_bits());
        }
    }
    assert_eq!(
        hydraulic["singular_jacobian"]["failure"],
        "singular_jacobian"
    );
    assert_eq!(hydraulic["iteration_limit"]["failure"], "iteration_limit");
    assert_eq!(
        hydraulic["redistribution_poison"]["expected"],
        "VEG-E-063 hydraulic_redistribution_unsupported"
    );

    assert_v3_migration_and_zero_demand(families);
}

fn assert_v3_biochemistry_aerodynamics_and_root_paths(hydraulic: &Value, accepted: &Value) {
    let forcing = &hydraulic["operands"]["gas_energy"];
    assert_eq!(forcing["cp_air_j_kg_k"], 1004.64);
    assert_eq!(forcing["latent_heat_j_kg"], 2_501_000.0);
    let wind = &forcing["reference_wind_operands"];
    let kappa = wind["kappa"].as_f64().unwrap();
    let reference = wind["u_ref_m_s"].as_f64().unwrap();
    let height = wind["z_ref_m"].as_f64().unwrap() - wind["displacement_m"].as_f64().unwrap();
    let z0m = wind["z0m_m"].as_f64().unwrap();
    for (field, scalar) in [("rah_s_m", "z0h_m"), ("raw_s_m", "z0q_m")] {
        let reconstructed = (height / z0m).ln() * (height / wind[scalar].as_f64().unwrap()).ln()
            / (kappa * kappa * reference);
        assert!((forcing[field].as_f64().unwrap() - reconstructed).abs() <= 2.0e-14);
    }

    for class_name in ["sun_gas_energy_state", "shade_gas_energy_state"] {
        let state = &accepted["fluxes"][class_name];
        assert!(
            (state["ci_initial_bracket_pa"][0].as_f64().unwrap()
                - state["gamma_pa"].as_f64().unwrap())
            .abs()
                <= 2.0e-14
        );
        assert!(
            (state["ap"].as_f64().unwrap() - 3.0 * state["tp"].as_f64().unwrap()).abs() <= 1.0e-13
        );
        assert!(
            (state["an"].as_f64().unwrap()
                - (state["ag"].as_f64().unwrap() - state["rd"].as_f64().unwrap()))
            .abs()
                <= 1.0e-13
        );
        for field in [
            "vcmax",
            "jmax",
            "kc_pa",
            "ko_pa",
            "gamma_pa",
            "ipsii",
            "electron_transport",
            "ac",
            "aj",
            "ap",
            "ai",
            "ag",
        ] {
            assert!(state[field].as_f64().is_some_and(f64::is_finite));
        }
    }

    let parameters = &hydraulic["operands"]["parameters"];
    for row in accepted["fluxes"]["q3"].as_array().unwrap() {
        if row["flux"].as_f64().unwrap() == 0.0 {
            continue;
        }
        let layer = hydraulic["operands"]["layers"]
            .as_array()
            .unwrap()
            .iter()
            .find(|layer| layer["layer_id"] == row["layer_id"])
            .expect("layer operands");
        let kr = row["kr_m_s"].as_f64().unwrap();
        let ks = row["ks_m_s"].as_f64().unwrap();
        assert!(
            (kr - parameters["k3_max_m_s"].as_f64().unwrap() / layer["z3_m"].as_f64().unwrap()
                * row["soil_vulnerability"].as_f64().unwrap())
            .abs()
                <= 2.0e-20
        );
        assert!(
            (ks - layer["ksoil_m2_s"].as_f64().unwrap() / layer["dxroot_m"].as_f64().unwrap())
                .abs()
                <= 2.0e-20
        );
        assert!((row["k3_series_m_s"].as_f64().unwrap() - kr * ks / (kr + ks)).abs() <= 2.0e-20);
        let rai = (parameters["lai"].as_f64().unwrap() + parameters["sai"].as_f64().unwrap())
            * layer["root_fraction"].as_f64().unwrap()
            * parameters["root_to_leaf_area"].as_f64().unwrap();
        assert!((row["rai_m2_m2"].as_f64().unwrap() - rai).abs() <= 2.0e-14);
    }
}

fn assert_v3_migration_and_zero_demand(families: &Value) {
    let migration = &families["v2_to_v3_root_state_migration"];
    assert_eq!(
        migration["bitwise_identical"]["expected"]["status"],
        "complete"
    );
    assert_eq!(
        migration["numerically_equal_bitwise_distinct"]["expected"]["reason"],
        "ambiguous_v2_layer_root_warm_starts"
    );
    assert_eq!(
        migration["missing"]["expected"]["reason"],
        "ambiguous_v2_layer_root_warm_starts"
    );
    let zero_demand = &families["hydraulic_potential_pass"]["zero_maximum_demand_exact_branch"];
    assert_eq!(zero_demand["persisted_beta_hyd"], 1.0);
    assert_eq!(zero_demand["division_evaluated"], false);
}

fn assert_v3_respiration_failures_and_poisons(parsed: &Value) {
    let families = &parsed["families"];
    let respiration = &families["leaf_respiration"];
    let results = &respiration["results"];
    assert!(
        (respiration["operands"]["ag_sun_umol_co2_m2_leaf_s"]
            .as_f64()
            .unwrap()
            - results["rd_sun"].as_f64().unwrap()
            - results["an_sun"].as_f64().unwrap())
        .abs()
            <= 1.0e-14
    );
    let exact_once = results["leaf_maintenance_carbon_debit_exact_once"]
        .as_f64()
        .unwrap();
    let double_poison = results["double_debit_poison"].as_f64().unwrap();
    assert!((2.0 * exact_once - double_poison).abs() <= f64::EPSILON);
    assert_eq!(
        respiration["nonpositive_atkin_poison"]["expected"],
        "VEG-E-085 nonpositive_atkin_rd25"
    );
    assert_eq!(
        respiration["source_unit_conversion"]["atkin_result_units"],
        "umol CO2 m-2 leaf s-1"
    );
    assert_eq!(
        respiration["zero_leaf_area_exact_branch"]["leaf_n_division_evaluated"],
        false
    );

    assert_v3_failure_diagnostics(families);
    assert_v3_poison_inventory(parsed);
}

fn assert_v3_failure_diagnostics(families: &Value) {
    let mut failures = Vec::new();
    failures.extend(families["executed_ci_failures"].as_array().unwrap());
    failures.extend(
        families["hydraulic_potential_pass"]["executed_canopy_energy_failures"]
            .as_array()
            .unwrap(),
    );
    failures.push(&families["hydraulic_potential_pass"]["singular_jacobian"]);
    failures.push(&families["hydraulic_potential_pass"]["iteration_limit"]);
    for solve in [
        "sun_ci",
        "shade_ci",
        "canopy_energy",
        "hydraulic_system",
        "outer_gas_energy_hydraulic_coupling",
    ] {
        assert!(
            failures
                .iter()
                .any(|failure| failure["diagnostics"]["solve"] == solve)
        );
    }
    for failure in failures {
        assert!(failure["candidate"].is_null());
        assert!(failure["last_iterate"].is_null());
        let diagnostics = &failure["diagnostics"];
        for field in [
            "model_definition_sha256",
            "transaction_id",
            "occupancy_id",
            "pass",
            "solve",
            "iterations",
            "residual_norms",
            "step_norm",
            "backtracking_count",
            "active_bounds",
            "active_water_caps",
            "bracket",
            "pivot_magnitude",
            "matrix_norm",
        ] {
            assert!(
                diagnostics.get(field).is_some(),
                "failure payload missing {field}"
            );
        }
        assert_json_numbers_finite(diagnostics);
    }
    let precedence = &families["failure_precedence"];
    assert_eq!(
        precedence["order"],
        serde_json::json!([
            "identity_schema",
            "domain",
            "bracket",
            "singular",
            "iteration"
        ])
    );
    for (index, row) in precedence["competing_conditions"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
    {
        assert_eq!(row["selected"], precedence["order"][index]);
        assert!(row["candidate"].is_null() && row["last_iterate"].is_null());
    }
    for failure in families["executed_ci_failures"].as_array().unwrap() {
        assert_eq!(failure["evaluations"], 2);
        assert!(failure["diagnostics"]["bracket"].is_array());
        if failure["failure_kind"] != "domain" {
            assert_eq!(
                failure["diagnostics"]["residual_norms"]
                    .as_array()
                    .unwrap()
                    .len(),
                2
            );
        }
    }
}

fn assert_v3_poison_inventory(parsed: &Value) {
    let poisons = parsed["poisons"].as_object().unwrap();
    assert_eq!(poisons.len(), 40);
    assert!(
        poisons
            .values()
            .all(|poison| poison["executed"].as_bool() == Some(true))
    );
    for required_poison in V3_REQUIRED_POISONS {
        assert!(poisons.get(required_poison).is_some());
    }
    assert!(
        poisons
            .values()
            .filter(|poison| poison.get("typed_error").is_some())
            .all(|poison| poison["executed_by"] == "owning_validator")
    );
}

fn assert_json_numbers_finite(value: &Value) {
    match value {
        Value::Number(number) => assert!(number.as_f64().is_some_and(f64::is_finite)),
        Value::Array(values) => values.iter().for_each(assert_json_numbers_finite),
        Value::Object(values) => values.values().for_each(assert_json_numbers_finite),
        _ => {}
    }
}

const V3_REQUIRED_POISONS: [&str; 40] = [
    "aggregate_only_transpiration_equality",
    "area_only_absorption_partition",
    "arithmetic_mean_optics",
    "authorization_in_potential_pass",
    "average_v2_root_warm_starts",
    "clumping_applied_twice",
    "clumping_omitted",
    "crown_base_as_stem_path",
    "direct_diffuse_swap",
    "direct_summed_lower_reflection",
    "external_hydraulic_clamp",
    "first_v2_root_warm_start",
    "half_height_stem_path",
    "heat_roughness_in_momentum_log",
    "hidden_minimum_wind",
    "hydraulics_without_energy_resolve",
    "invalid_reference_height_geometry",
    "leaf_optics_for_all_plant_area",
    "legacy_rd_leaf_n_rate",
    "metres_as_mm_gravity",
    "missing_gravity",
    "nonpositive_friction_velocity",
    "nonpositive_height",
    "nonpositive_rd_clamp",
    "posthoc_scalar_stress",
    "publish_beta_one_emax_as_request",
    "rd_debited_twice",
    "reference_wind_as_leaf_wind",
    "root_weighted_v2_migration",
    "stem_absorption_in_fvcb_par",
    "stem_leaf_gravity",
    "stem_only_photosynthesis",
    "stem_optics_for_all_plant_area",
    "sun_shade_respiration_swap",
    "sunlit_plant_area_as_sunlit_leaf_area",
    "undocumented_wet_surface_wind",
    "vis_nir_swap",
    "whole_column_zero_lower_boundary",
    "wrong_gravity_sign",
    "wrong_rd_temperature_response",
];

#[test]
fn v3_vectors_close_radiation_hydraulics_respiration_and_failure_payloads() {
    let fixture = read(&format!(
        "{V3_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v3_vectors.json"
    ));
    let parsed: Value = serde_json::from_str(&fixture).expect("V3 fixture JSON");
    let families = &parsed["families"];
    assert_v3_radiation_and_aerodynamics(families);
    assert_v3_hydraulics_and_migration(families);
    assert_v3_respiration_failures_and_poisons(&parsed);
}

fn assert_v2_check_inventory(parsed: &Value) {
    let checks = parsed["checks"].as_object().expect("fixture checks object");
    assert_eq!(checks.len(), 31, "complete Stage-A check inventory");
    for required in [
        "aggregate_incident_poison",
        "aggregate_par_poison",
        "authorization_back_conversion",
        "average_wet_fraction_poison",
        "controlled_final_release_changes_lower",
        "condensation_second_drainage",
        "distinct_beginning_store",
        "distinct_tile_rain",
        "double_ft_poison",
        "duplicate_lane_rejected",
        "empty_tile",
        "heterogeneous_upper_columns",
        "homogeneous_two_tile_reduction",
        "local_and_stand_closure",
        "mineral_n_after_aggregation",
        "missing_lane_rejected",
        "omit_ft_poison",
        "omit_second_drainage_poison",
        "replicated_store_poison",
        "request_weighting",
        "rollback_exact_bytes",
        "shared_cn_once",
        "single_tile_reduction",
        "stemflow_bypass",
        "stemflow_through_foliage_poison",
        "tile_order_permutation",
        "two_rank_routing",
        "unequal_tile_fractions",
        "wrong_area_basis_poison",
        "wrong_authorization_poison",
        "wrong_tile_drainage_poison",
    ] {
        assert_eq!(
            checks.get(required).and_then(Value::as_bool),
            Some(true),
            "missing or failing Stage-A check {required}"
        );
    }
    assert!(
        checks.values().all(|value| value.as_bool() == Some(true)),
        "every Stage-A oracle check must pass"
    );
    assert_eq!(parsed["all_pass"].as_bool(), Some(true));
}

fn assert_v2_column_and_stand_closure(parsed: &Value) {
    let mut reconstructed_store = 0.0;
    let mut reconstructed_ground = 0.0;
    for tile in ["tile-a", "tile-b"] {
        let fraction = parsed["fractions"][tile].as_f64().unwrap();
        let column = &parsed["heterogeneous_columns"][tile];
        let mut column_store = 0.0;
        for row in column["occupancies"].as_array().unwrap() {
            let number = |key: &str| row[key].as_f64().unwrap();
            let residual = number("store0") + number("incident") + number("condensation")
                - number("store1")
                - number("wet_evaporation")
                - number("throughfall")
                - number("stemflow")
                - number("initial_drainage")
                - number("second_drainage");
            assert!(residual.abs() < 2e-14, "{tile} local closure");
            column_store += number("store1");
        }
        reconstructed_store += fraction * column_store;
        reconstructed_ground += fraction * column["ground_liquid"].as_f64().unwrap();
    }
    assert!(
        (reconstructed_store - parsed["stand"]["ending_store"].as_f64().unwrap()).abs() < 1e-14
    );
    assert!(
        (reconstructed_ground - parsed["stand"]["ground_liquid"].as_f64().unwrap()).abs() < 1e-14
    );
}

fn assert_v2_water_and_owner_transactions(parsed: &Value) {
    let tile_a = parsed["fractions"]["tile-a"].as_f64().unwrap();
    let water = &parsed["water_transaction"];
    assert!(
        (tile_a * water["demand_tile"].as_f64().unwrap() - water["demand_stand"].as_f64().unwrap())
            .abs()
            < 1e-14
    );
    assert!(
        (water["authorization_stand"].as_f64().unwrap() / tile_a
            - water["authorization_tile"].as_f64().unwrap())
        .abs()
            < 1e-14
    );
    assert!(
        (tile_a * water["final_tile"].as_f64().unwrap() - water["final_stand"].as_f64().unwrap())
            .abs()
            < 1e-14
    );
    let two_pass = &parsed["water_arbitration_and_routing_control"];
    let request_sum: f64 = two_pass["potential_requests"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item["amount"].as_f64().unwrap())
        .sum();
    let authorization_sum: f64 = two_pass["authorizations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item["amount"].as_f64().unwrap())
        .sum();
    assert!(request_sum > authorization_sum);
    assert!((authorization_sum - two_pass["supply"].as_f64().unwrap()).abs() < 1e-14);
    assert_ne!(
        two_pass["lower_potential_incident"],
        two_pass["lower_controlled_final_incident"]
    );
    assert_eq!(
        two_pass["claim_scope"],
        "topology_causality_only_exogenous_vapor_operands"
    );
    assert_eq!(
        two_pass["complete_coupled_acceptance_gate"],
        "STAGE_B_E11_E15_EXACT_ORACLE"
    );
    assert_eq!(
        parsed["owner_rollback"]["beginning_sha256"],
        parsed["owner_rollback"]["after_failure_sha256"]
    );
    assert_eq!(
        parsed["owner_rollback"]["owner_names"]
            .as_array()
            .unwrap()
            .len(),
        4
    );
}

fn assert_v2_mineral_n_and_nonlinear_poisons(parsed: &Value) {
    let mineral = &parsed["mineral_n_transaction"];
    let authorizations = mineral["authorizations"].as_array().unwrap();
    let finalized = mineral["finalized"].as_array().unwrap();
    for usage in finalized {
        let matching = authorizations
            .iter()
            .find(|authorization| authorization["key"] == usage["key"])
            .expect("finalized N key has exact authorization key");
        assert!(usage["amount"].as_f64().unwrap() <= matching["amount"].as_f64().unwrap());
    }
    assert_eq!(mineral["wrong_species_result"], "typed_identity_mismatch");
    assert_eq!(mineral["wrong_layer_result"], "typed_identity_mismatch");
    let nonlinear = &parsed["nonlinear_locality"];
    assert_ne!(nonlinear["weighted_fvbc"], nonlinear["aggregate_par_fvbc"]);
    assert_ne!(
        nonlinear["weighted_wet_response"],
        nonlinear["averaged_wet_response"]
    );
    let carbon = &parsed["shared_carbon_nitrogen"];
    assert_ne!(
        carbon["accepted_once"],
        carbon["duplicate_transition_poison"]
    );
    let state = &parsed["canonical_state"];
    let serialized = state["serialized"].as_str().unwrap();
    let reparsed: Value = serde_json::from_str(serialized).unwrap();
    assert_eq!(serde_json::to_string(&reparsed).unwrap(), serialized);
    assert_eq!(sha256_text(serialized), state["sha256"].as_str().unwrap());
    assert_eq!(state["wrong_unit_poison"], "typed_unit_mismatch:MPa");
}

#[test]
fn v2_committed_topology_vectors_are_independent_and_non_tautological() {
    let fixture_path =
        format!("{V2_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v2_topology_vectors.json");
    let fixture = read(&fixture_path);
    let parsed: Value = serde_json::from_str(&fixture).expect("V2 topology fixture is JSON");
    assert_v2_check_inventory(&parsed);
    assert_v2_column_and_stand_closure(&parsed);
    assert_v2_water_and_owner_transactions(&parsed);
    assert_v2_mineral_n_and_nonlinear_poisons(&parsed);
    assert!(fixture.contains("typed_unresolved_occupancy_lanes"));
    assert_eq!(
        sha256(&fixture_path),
        "c02e5e2a2287d84cfc584a6e3ec9c499cf7168160bc71f2577323f19dcb50bf1"
    );
}

#[test]
fn independent_coupled_reference_vectors_pass() {
    let output = Command::new(".venv/bin/python")
        .arg(format!(
            "{COUPLED_PACKAGE}/artifacts/reference_calculator.py"
        ))
        .output()
        .expect("run independent coupled vegetation reference calculator");
    assert!(
        output.status.success(),
        "oracle failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("oracle output UTF-8");
    for required in [
        "\"all_pass\": true",
        "\"zero_light\": true",
        "\"rubisco_limited\": true",
        "\"electron_limited\": true",
        "\"wet_canopy_closure\": true",
        "\"wet_stem_energy_owner\": true",
        "\"integrated_wet_dry_energy_water\": true",
        "\"wet_rate_amount_area_poisons\": true",
        "\"canopy_condensation_closure\": true",
        "\"subfreezing_liquid_rejected\": true",
        "\"radiation_removable_branches\": true",
        "\"sunlit_shaded_partition\": true",
        "\"root_profiles_distinct\": true",
        "\"hydraulic_four_node_closure\": true",
        "\"hydraulic_active_cap_resolve\": true",
        "\"hydraulic_rate_amount_poison\": true",
        "\"hydraulic_redistribution_rejected\": true",
        "\"hydraulic_dry_frozen_exclusion\": true",
        "\"coupled_hydraulic_fixed_point\": true",
        "\"nitrogen_competition\": true",
        "\"leaf_litter_cn_dm\": true",
        "\"floor_not_donation_target\": true",
        "\"deciduous_multistep\": true",
        "\"evergreen_turnover\": true",
        "\"root_wood_cwd_trajectory\": true",
        "\"wrong_root_c_dm_receipt_rejected\": true",
        "\"receiver_n_credit\": true",
        "\"wrong_competitor_debit_rejected\": true",
        "\"vertical_mixed_radiation\": true",
        "\"hydraulic_finalized_caps\": true",
        "\"cn_poison_rejected\": true",
        "\"rollback\": true",
        "\"radiation_absorbed\": 631.4550942161578",
        "\"leaf_temperature\": 295.4923277333952",
        "\"cn_leaf_growth\": 0.006442191726176829",
    ] {
        assert!(
            stdout.contains(required),
            "oracle output missing {required}: {stdout}"
        );
    }
    for closure in [
        "froot_c_closure",
        "froot_n_closure",
        "froot_dm_closure",
        "wood_c_closure",
        "wood_n_closure",
        "wood_dm_closure",
    ] {
        assert!(
            json_number_after(&stdout, closure).abs() < 1e-14,
            "nonclosing independent turnover ledger {closure}: {stdout}"
        );
    }
    assert!(json_number_after(&stdout, "wrong_n_remaining") < 0.01);
}

const V5_REQUIRED_POISONS: [&str; 27] = [
    "authorization_amount_as_rate",
    "authorization_as_finalized_debit",
    "borrow_unused_authorization",
    "cap_before_constitutive_law",
    "cap_tolerance_repairs_identity_or_basis",
    "continue_from_potential_candidate",
    "double_interval",
    "double_tile_fraction",
    "gas_energy_not_resolved",
    "omit_interval",
    "omit_tile_fraction",
    "partial_commit",
    "producer_supplied_zero_closure",
    "q_law_overwritten_by_q",
    "reauthorization_after_final_pass",
    "reselect_within_tie_perturbation",
    "scalar_ratio_all_layers",
    "sequential_clamp_potential_q",
    "sorted_layer_id_active_caps",
    "stale_generalized_branch",
    "stale_transaction",
    "stand_cap_used_inside_tile_law",
    "strict_less_than_tie",
    "wrong_basis",
    "wrong_layer",
    "wrong_occupancy",
    "wrong_tile_authorization",
];

fn v5_fixture() -> Value {
    serde_json::from_str(&read(&format!(
        "{V3_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v5_vectors.json"
    )))
    .expect("V5 fixture JSON")
}

#[test]
fn v5_capped_pass_authority_is_digest_bound_and_v1_through_v4_are_immutable() {
    let vegetation = read(CONTRACT);
    let definition_path =
        format!("{V3_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v5_definition.json");
    let definition: Value =
        serde_json::from_str(&read(&definition_path)).expect("V5 definition JSON");

    assert_eq!(
        sha256(&definition_path),
        "0ee6a50d5f72da0b9344d8bf1b77674e95a66ab196edc068851bb419eb7b36f3"
    );
    assert_eq!(definition["model_version"], "OPENWEPP_C3_WOODY_V5");
    assert_eq!(definition["canonical_contract"], "SC-VEGETATION-001@9");
    assert_eq!(
        definition["base_model_definition"]["sha256"],
        "8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437"
    );
    assert_eq!(
        definition["independent_fixture"]["sha256"],
        "6f5e9554fe7b91b6fcb76e777b027fbeafcf4c2873a6060bd158b6a578c37f6d"
    );
    assert_eq!(
        definition["independent_fixture"]["generator_sha256"],
        "4c3a1cfc18b2437dabd70e4aee03effa6af7aac893056c6248a896dd3a2b5775"
    );
    assert_eq!(
        sha256_text(inclusive_section(
            &vegetation,
            "## `OPENWEPP_C3_WOODY_V5` Fixed-Authorization Capped-Pass Amendment\n",
            "## Change Log\n",
        )),
        definition["canonical_section_sha256"]["v5_authorization_capped_pass_amendment"]
            .as_str()
            .expect("V5 section digest"),
    );

    for (path, digest) in [
        (
            format!("{COUPLED_PACKAGE}/artifacts/openwepp_c3_woody_v1_definition.json"),
            "003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157",
        ),
        (
            format!("{V2_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v2_definition.json"),
            "38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3",
        ),
        (
            format!("{V3_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v3_definition.json"),
            "7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852",
        ),
        (
            format!("{V4_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v4_definition.json"),
            "8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437",
        ),
    ] {
        assert_eq!(
            sha256(&path),
            digest,
            "historical model bytes changed: {path}"
        );
    }
    for required in [
        "INV-VEGETATION-093",
        "INV-VEGETATION-094",
        "INV-VEGETATION-095",
        "INV-VEGETATION-096",
        "INV-VEGETATION-097",
        "INV-VEGETATION-098",
        "INV-VEGETATION-099",
        "VEG-E-091",
        "VEG-E-092",
        "VEG-E-093",
        "VEG-E-094",
    ] {
        assert!(
            vegetation.contains(required),
            "missing V5 authority {required}"
        );
    }
}

#[test]
fn v5_independent_oracle_regenerates_exact_frozen_bytes() {
    let fixture_path =
        format!("{V3_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v5_vectors.json");
    let definition_path =
        format!("{V3_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v5_definition.json");
    let before_fixture = read(&fixture_path);
    let before_definition = read(&definition_path);
    let output = Command::new(".venv/bin/python")
        .arg(format!(
            "{V3_AUTHORITY_PACKAGE}/artifacts/reference_calculator_v5.py"
        ))
        .output()
        .expect("run independent V5 oracle");
    assert!(
        output.status.success(),
        "V5 oracle failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(read(&fixture_path), before_fixture);
    assert_eq!(read(&definition_path), before_definition);
    assert_eq!(
        sha256(&fixture_path),
        "6f5e9554fe7b91b6fcb76e777b027fbeafcf4c2873a6060bd158b6a578c37f6d"
    );
    assert_eq!(
        sha256(&format!(
            "{V3_AUTHORITY_PACKAGE}/artifacts/reference_calculator_v5.py"
        )),
        "4c3a1cfc18b2437dabd70e4aee03effa6af7aac893056c6248a896dd3a2b5775"
    );
}

#[test]
fn v5_vectors_bind_exact_cap_conversions_tie_rule_operands_and_poison_inventory() {
    let fixture = v5_fixture();
    assert_eq!(fixture["model_version"], "OPENWEPP_C3_WOODY_V5");
    assert_eq!(fixture["oracle_independence"]["calls_rust"], false);
    assert!(
        fixture["checks"]
            .as_object()
            .expect("V5 checks")
            .values()
            .all(|value| value == true)
    );

    let controlled = &fixture["families"]["controlled_layer_complementarity"];
    let fraction = controlled["tile_fraction"].as_f64().expect("tile fraction");
    let dt = controlled["dt_s"].as_f64().expect("interval");
    let configured = controlled["configured_layer_order"]
        .as_array()
        .expect("configured layers");
    let layers = controlled["layers"].as_array().expect("layer operands");
    assert_eq!(configured.len(), layers.len());
    for (identity, layer) in configured.iter().zip(layers) {
        assert_eq!(identity, &layer["layer_id"]);
        let authorization = layer["authorization_kg_m2_stand_ground"].as_f64().unwrap();
        let tile_amount = layer["authorization_kg_m2_tile_ground"].as_f64().unwrap();
        let cap_rate = layer["cap_rate_kg_m2_tile_s"].as_f64().unwrap();
        let q_law = layer["q_law_kg_m2_tile_s"].as_f64().unwrap();
        let q_final = layer["q_final_kg_m2_tile_s"].as_f64().unwrap();
        let finalized = layer["finalized_use_kg_m2_stand_ground"].as_f64().unwrap();
        assert_eq!(tile_amount.to_bits(), (authorization / fraction).to_bits());
        assert_eq!(
            cap_rate.to_bits(),
            (authorization / (fraction * dt)).to_bits()
        );
        assert_eq!(q_final.to_bits(), q_law.min(cap_rate).to_bits());
        assert_eq!(finalized.to_bits(), (fraction * q_final * dt).to_bits());
        let cap_active = cap_rate <= q_law;
        assert_eq!(
            layer["branch"],
            if cap_active {
                "authorization_active_or_tie"
            } else {
                "constitutive_law"
            }
        );
        let derivative = layer["dq_final_d_root_potential"].as_f64().unwrap();
        if cap_active {
            assert_eq!(derivative.to_bits(), 0.0_f64.to_bits());
        } else {
            assert!(
                derivative < 0.0,
                "law branch retains its constitutive derivative"
            );
        }
    }
    let tie_cases = fixture["families"]["exact_and_near_tie"]["cases"]
        .as_array()
        .unwrap();
    assert_eq!(tie_cases[1]["case"], "exact_tie");
    assert_eq!(tie_cases[1]["branch"], "authorization_active_or_tie");
    assert_eq!(
        tie_cases[1]["dq_final_d_root_potential"].as_f64(),
        Some(0.0)
    );

    let poisons = fixture["poisons"].as_object().expect("V5 poisons");
    assert_eq!(poisons.len(), V5_REQUIRED_POISONS.len());
    for name in V5_REQUIRED_POISONS {
        let poison = poisons
            .get(name)
            .unwrap_or_else(|| panic!("missing V5 poison {name}"));
        assert_eq!(poison["executed"], true, "unexecuted poison {name}");
        assert!(
            poison.get("typed_error").is_some() || poison["discriminates"] == true,
            "nondiscriminating poison {name}"
        );
    }
}

#[test]
fn v5_failure_payloads_rollback_and_migration_identity_are_complete() {
    let fixture = v5_fixture();
    for failure in fixture["families"]["capped_failures"]
        .as_array()
        .expect("capped failures")
    {
        assert!(failure["candidate"].is_null());
        assert_eq!(failure["rollback_byte_identical"], true);
        assert_eq!(
            failure["beginning_state_sha256_before"],
            failure["beginning_state_sha256_after"]
        );
        let diagnostics = &failure["diagnostics"];
        assert_eq!(diagnostics["pass"], "capped");
        for field in [
            "model_definition_sha256",
            "transaction_id",
            "occupancy_id",
            "solve",
            "iterations",
            "residual_norms",
            "backtracking_count",
            "active_bounds",
            "active_water_caps",
            "layer_operands_in_configuration_order",
            "fixed_authorization_identity",
            "step_norm",
            "bracket",
            "pivot_magnitude",
            "matrix_norm",
        ] {
            assert!(
                diagnostics.get(field).is_some(),
                "failure diagnostic missing {field}"
            );
        }
    }
    for injection in fixture["families"]["phase_rollback_injections"]
        .as_array()
        .unwrap()
    {
        assert_eq!(injection["candidate_survives"], false);
        assert_eq!(injection["beginning_owners_byte_identical"], true);
        assert_eq!(
            injection["beginning_owner_bytes_before_sha256"],
            injection["beginning_owner_bytes_after_sha256"]
        );
    }

    let migration = &fixture["families"]["v4_to_v5_identity_rebind"];
    assert_eq!(migration["payload_byte_identical"], true);
    assert_eq!(migration["field_migration_or_synthesis"], false);
    assert_eq!(
        migration["payload_sha256_before"],
        migration["payload_sha256_after"]
    );
    assert_ne!(
        migration["v4_identity"]["configuration_sha256"],
        migration["v5_identity"]["configuration_sha256"]
    );
    assert_eq!(
        migration["stale_v4_identity_poison"]["candidate"],
        Value::Null
    );
    assert_eq!(
        migration["stale_v4_identity_poison"]["typed_error"],
        "stale_v4_identity_in_v5_state"
    );
}

fn v6_fixture() -> Value {
    serde_json::from_str(&read(&format!(
        "{V6_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v6_vectors.json"
    )))
    .expect("V6 fixture JSON")
}

fn v6_portable_equal(reference: &Value, actual: &Value) -> bool {
    const EXACT_FIELDS: [&str; 21] = [
        "model_definition_sha256",
        "configuration_sha256",
        "transaction_id",
        "occupancy_id",
        "pass",
        "solve",
        "field",
        "typed_failure",
        "candidate",
        "unit",
        "basis",
        "present",
        "iterations",
        "backtracking_count",
        "residual_cardinality",
        "active_bounds",
        "active_water_caps",
        "branches",
        "rollback_sha256_before",
        "rollback_sha256_after",
        "accepted_value",
    ];
    // Scalar values are compared below; their encoded class is exact here.
    if EXACT_FIELDS
        .iter()
        .any(|field| reference[*field] != actual[*field])
    {
        return false;
    }
    if reference["model_definition_sha256"] != "BOUND_BY_V6_DEFINITION_NOT_ORACLE"
        || reference["pass"] != "capped"
        || reference["solve"] != "hydraulic_system"
        || reference["field"] != "step_norm"
        || reference["typed_failure"] != "backtracking_limit"
        || reference["present"] != true
        || !reference["candidate"].is_null()
        || reference["accepted_value"] != false
        || reference["rollback_sha256_before"] != reference["rollback_sha256_after"]
        || actual["rollback_sha256_before"] != actual["rollback_sha256_after"]
        || reference["scalar"]["class"] != "finite"
        || actual["scalar"]["class"] != "finite"
    {
        return false;
    }
    let Some(a) = reference["scalar"]["value"].as_f64() else {
        return false;
    };
    let Some(b) = actual["scalar"]["value"].as_f64() else {
        return false;
    };
    if !a.is_finite() || !b.is_finite() || a < 0.0 || b < 0.0 {
        return false;
    }
    let a_zero = a.abs().to_bits() == 0;
    let b_zero = b.abs().to_bits() == 0;
    if a_zero != b_zero || (!a_zero && a.is_sign_negative() != b.is_sign_negative()) {
        return false;
    }
    (a - b).abs() <= 3e-7 * a.abs().max(b.abs())
}

#[test]
fn v6_diagnostic_portability_authority_is_digest_bound_and_v1_through_v5_are_immutable() {
    let vegetation = read(CONTRACT);
    let definition_path =
        format!("{V6_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v6_definition.json");
    let model_stack_copy =
        format!("{COUPLED_PACKAGE}/artifacts/openwepp_c3_woody_v6_definition.json");
    let definition_bytes = read(&definition_path);
    let definition: Value = serde_json::from_str(&definition_bytes).expect("V6 definition JSON");

    assert_eq!(
        sha256(&definition_path),
        "a5a5ed77b4672b97b7c50103089067d70ade03bc1b5aff4e08ba6fdffc05d426"
    );
    assert_eq!(read(&model_stack_copy), definition_bytes);
    assert_eq!(sha256(&model_stack_copy), sha256(&definition_path));
    assert_eq!(definition["model_version"], "OPENWEPP_C3_WOODY_V6");
    assert_eq!(definition["canonical_contract"], "SC-VEGETATION-001@10");
    assert_eq!(
        definition["base_model_definition"]["sha256"],
        "0ee6a50d5f72da0b9344d8bf1b77674e95a66ab196edc068851bb419eb7b36f3"
    );
    assert_eq!(
        definition["independent_fixture"]["sha256"],
        "2e7005f88d788399e914b2034c0193fc6f08d1657532a349ec797b966432356b"
    );
    assert_eq!(
        definition["independent_fixture"]["generator_sha256"],
        "bfa805000a6e29b3c56a666ea97a4e4825f9262a3ef1f0daa5c3cfb5f2dd6532"
    );
    assert_eq!(
        sha256_text(inclusive_section(
            &vegetation,
            "## `OPENWEPP_C3_WOODY_V6` Rejected-Failure Diagnostic Portability Amendment\n",
            "## `OPENWEPP_C3_WOODY_V5` Fixed-Authorization Capped-Pass Amendment\n",
        )),
        definition["canonical_section_sha256"]
            ["v6_rejected_failure_diagnostic_portability_amendment"]
            .as_str()
            .expect("V6 section digest"),
    );

    for (path, digest) in [
        (
            format!("{COUPLED_PACKAGE}/artifacts/openwepp_c3_woody_v1_definition.json"),
            "003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157",
        ),
        (
            format!("{V2_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v2_definition.json"),
            "38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3",
        ),
        (
            format!("{V3_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v3_definition.json"),
            "7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852",
        ),
        (
            format!("{V4_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v4_definition.json"),
            "8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437",
        ),
        (
            format!("{V3_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v5_definition.json"),
            "0ee6a50d5f72da0b9344d8bf1b77674e95a66ab196edc068851bb419eb7b36f3",
        ),
    ] {
        assert_eq!(
            sha256(&path),
            digest,
            "historical model bytes changed: {path}"
        );
    }

    for required in [
        "INV-VEGETATION-100",
        "INV-VEGETATION-101",
        "INV-VEGETATION-102",
        "INV-VEGETATION-103",
        "VEG-E-095",
        "VEG-E-096",
        "abs(a-b) <= 3e-7*max(abs(a),abs(b))",
        "Portable diagnostic equality is evidence adjudication after a solve has\nalready rejected",
        "cannot alter solver acceptance",
        "byte-identical beginning-owner and transaction rollback evidence",
        "public vegetation transaction retains the V5 fail-closed\nposture",
    ] {
        assert!(
            vegetation.contains(required),
            "missing V6 authority {required}"
        );
    }
}

#[test]
fn v6_vectors_bind_comparison_boundary_identity_transition_and_acceptance_firewall() {
    let fixture_path =
        format!("{V6_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v6_vectors.json");
    let fixture = v6_fixture();

    assert_eq!(
        sha256(&fixture_path),
        "2e7005f88d788399e914b2034c0193fc6f08d1657532a349ec797b966432356b"
    );
    assert_eq!(
        sha256(&format!(
            "{V6_AUTHORITY_PACKAGE}/artifacts/reference_calculator_v6.py"
        )),
        "bfa805000a6e29b3c56a666ea97a4e4825f9262a3ef1f0daa5c3cfb5f2dd6532"
    );
    assert_eq!(fixture["model_version"], "OPENWEPP_C3_WOODY_V6");
    assert_eq!(
        fixture["base_model_sha256"],
        "0ee6a50d5f72da0b9344d8bf1b77674e95a66ab196edc068851bb419eb7b36f3"
    );
    assert_eq!(fixture["comparison"]["relative_tolerance"], 3e-7);
    assert_eq!(
        fixture["comparison"]["formula"],
        "abs(a-b) <= rtol*max(abs(a),abs(b))"
    );
    assert!(
        fixture["checks"]
            .as_object()
            .expect("V6 checks")
            .values()
            .all(|value| value == true)
    );

    let numeric_cases = fixture["families"]["numeric_boundary_cases"]
        .as_array()
        .expect("V6 numeric boundary cases");
    for required in [
        ("observed_cpython_rust_step_norm", true),
        ("exact_largest_representable_boundary", true),
        ("one_representable_value_inside", true),
        ("first_representable_value_outside", false),
        ("positive_zero_vs_negative_zero", true),
        ("zero_vs_minimum_positive_subnormal", false),
        ("sign_mismatch", false),
        ("reversed_observed_operands", true),
        ("lower_side_boundary", true),
        ("negative_step_norm", false),
    ] {
        let case = numeric_cases
            .iter()
            .find(|case| case["case"] == required.0)
            .unwrap_or_else(|| panic!("missing V6 numeric case {}", required.0));
        assert_eq!(case["expected_equal"], required.1);
        assert_eq!(case["observed_equal"], required.1);
        assert_eq!(
            v6_portable_equal(&case["reference"], &case["actual"]),
            required.1,
            "Rust V6 comparison disagreed for {}",
            required.0
        );
    }

    for family in ["eligibility_and_firewall_poisons", "nonfinite_rejections"] {
        for case in fixture["families"][family]
            .as_array()
            .unwrap_or_else(|| panic!("missing V6 family {family}"))
        {
            assert_eq!(case["expected_equal"], false, "accepted V6 poison {case}");
            assert_eq!(case["observed_equal"], false, "laundered V6 poison {case}");
            assert!(
                !v6_portable_equal(&case["reference"], &case["actual"]),
                "Rust comparison laundered V6 poison {case}"
            );
        }
    }

    let identity = &fixture["identity_transition"];
    assert_eq!(
        identity["non_identity_payload_bytes_sha256_before"],
        identity["non_identity_payload_bytes_sha256_after"]
    );
    assert_ne!(
        identity["source"]["configuration_sha256"],
        identity["target"]["configuration_sha256"]
    );
    assert_ne!(
        identity["source"]["state_sha256"],
        identity["target"]["state_sha256"]
    );
    assert_ne!(
        identity["source"]["diagnostic_sha256"],
        identity["target"]["diagnostic_sha256"]
    );
}

#[test]
fn v7_storage_transfer_authority_is_digest_bound_and_predecessors_are_immutable() {
    let vegetation = read(CONTRACT);
    let definition_path =
        format!("{V7_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v7_definition.json");
    let model_stack_copy =
        format!("{COUPLED_PACKAGE}/artifacts/openwepp_c3_woody_v7_definition.json");
    let definition_bytes = read(&definition_path);
    let definition: Value = serde_json::from_str(&definition_bytes).expect("V7 definition JSON");

    assert_eq!(
        sha256(&definition_path),
        "a78264d8cd24d2718e099420357e1632ac09f2ba18c4a42d21e7e5b282aa459f"
    );
    assert_eq!(read(&model_stack_copy), definition_bytes);
    assert_eq!(definition["model_version"], "OPENWEPP_C3_WOODY_V7");
    assert_eq!(definition["canonical_contract"], "SC-VEGETATION-001@11");
    assert_eq!(
        definition["base_model_definition_sha256"],
        "a5a5ed77b4672b97b7c50103089067d70ade03bc1b5aff4e08ba6fdffc05d426"
    );
    assert_eq!(
        sha256_text(inclusive_section(
            &vegetation,
            "## `OPENWEPP_C3_WOODY_V7` Storage-Transfer Phenology Amendment\n",
            "## `OPENWEPP_C3_WOODY_V4` Shared-State Authority Amendment\n",
        )),
        definition["canonical_section_sha256"]
            .as_str()
            .expect("V7 section digest"),
    );

    for (path, digest) in [
        (
            format!("{COUPLED_PACKAGE}/artifacts/openwepp_c3_woody_v1_definition.json"),
            "003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157",
        ),
        (
            format!("{V2_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v2_definition.json"),
            "38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3",
        ),
        (
            format!("{V3_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v3_definition.json"),
            "7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852",
        ),
        (
            format!("{V4_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v4_definition.json"),
            "8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437",
        ),
        (
            format!("{V3_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v5_definition.json"),
            "0ee6a50d5f72da0b9344d8bf1b77674e95a66ab196edc068851bb419eb7b36f3",
        ),
        (
            format!("{V6_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v6_definition.json"),
            "a5a5ed77b4672b97b7c50103089067d70ade03bc1b5aff4e08ba6fdffc05d426",
        ),
    ] {
        assert_eq!(
            sha256(&path),
            digest,
            "historical model bytes changed: {path}"
        );
    }

    for required in [
        "BEI-VEGETATION-011",
        "INV-VEGETATION-104",
        "INV-VEGETATION-105",
        "INV-VEGETATION-106",
        "INV-VEGETATION-107",
        "INV-VEGETATION-108",
        "INV-VEGETATION-109",
        "VEG-E-097",
        "VEG-E-098",
        "VEG-E-099",
        "VEG-E-100",
        "AUTHORITY_ADMITTED`, `IMPLEMENTATION_MISSING",
        "Current-interval E19 growth is never eligible",
    ] {
        assert!(
            vegetation.contains(required),
            "missing V7 authority {required}"
        );
    }
}

fn assert_v7_migration(fixture: &Value) {
    assert_eq!(
        fixture["migration"]["seasonal_nonidentity_fields_preserved"],
        true
    );
    assert_eq!(
        fixture["migration"]["source_nonidentity_sha256"],
        fixture["migration"]["migrated_nonidentity_sha256"]
    );
    assert_eq!(fixture["migration"]["preparation_not_executed"], true);
    assert_eq!(fixture["migration"]["evergreen_unresolved_field_count"], 25);
    let incomplete = &fixture["migration"]["evergreen_incomplete"];
    assert_eq!(incomplete["status"], "incomplete");
    assert!(incomplete["candidate"].is_null());
    let unresolved = incomplete["unresolved"]
        .as_array()
        .expect("evergreen unresolved fields");
    assert_eq!(unresolved.len(), 25);
    assert_eq!(unresolved[0]["field"], "current_growth_fraction");
    let mut expected_index = 1;
    for tissue in [
        "leaf",
        "fine_root",
        "live_stem",
        "dead_stem",
        "live_coarse_root",
        "dead_coarse_root",
    ] {
        for pool in ["storage", "transfer"] {
            for element in ["carbon", "nitrogen"] {
                assert_eq!(unresolved[expected_index]["stratum"], "stratum-1");
                assert_eq!(unresolved[expected_index]["tissue"], tissue);
                assert_eq!(unresolved[expected_index]["pool"], pool);
                assert_eq!(unresolved[expected_index]["element"], element);
                assert_eq!(
                    unresolved[expected_index]["reason"],
                    "evergreen_nonzero_pool"
                );
                expected_index += 1;
            }
        }
    }
}

fn assert_v7_poisons(fixture: &Value) {
    let poisons = fixture["poisons"].as_array().expect("V7 poisons");
    let poison_names: Vec<_> = poisons
        .iter()
        .map(|case| case["name"].as_str().expect("poison name"))
        .collect();
    for required in [
        "move_100_percent",
        "move_only_leaf",
        "move_leaf_and_fine_root_only",
        "overwrite_existing_transfer",
        "move_current_interval_storage",
        "prepare_every_onset_timestep",
        "move_c_without_n",
        "move_n_without_c",
        "recompute_n_from_c_over_cn",
        "double_growth_respiration",
        "request_mineral_n_for_stored_tissue",
        "active_with_nonleaf_transfer",
        "calendar_year_transfer_without_onset",
        "background_seasonal_transfer",
        "evergreen_storage_accumulation",
        "negative_pool",
        "nonfinite_pool",
    ] {
        assert!(
            poison_names.contains(&required),
            "missing V7 poison {required}"
        );
    }
    assert_eq!(poisons.len(), 17);
    assert!(poisons.iter().all(|case| {
        case["rejected"] == true
            && case["candidate"].is_null()
            && case["discriminator"]["alternative_executed"] == true
            && case["discriminator"]["differs"] == true
    }));
}

fn assert_v7_rollback(fixture: &Value) {
    let rollbacks = fixture["rollback_injections"]
        .as_array()
        .expect("rollback injections");
    let phases: Vec<_> = rollbacks
        .iter()
        .map(|case| case["phase"].as_str().expect("rollback phase"))
        .collect();
    assert_eq!(
        phases,
        [
            "after_preparation",
            "during_onset_deployment",
            "after_n_authorization",
            "during_allocation",
            "closure_validation",
            "before_commit",
        ]
    );
    assert!(rollbacks.iter().all(|case| {
        case["owners_byte_identical"] == true
            && case["before_sha256"] == case["after_sha256"]
            && case["candidate"].is_null()
    }));
    assert_eq!(fixture["rollback_leak_poison"]["accepted"], false);
    assert_ne!(
        fixture["rollback_leak_poison"]["before_sha256"],
        fixture["rollback_leak_poison"]["after_sha256"]
    );
}

#[test]
fn v7_vectors_bind_six_tissues_ordering_migration_poisons_and_rollback() {
    let fixture_path =
        format!("{V7_AUTHORITY_PACKAGE}/artifacts/openwepp_c3_woody_v7_vectors.json");
    let fixture: Value = serde_json::from_str(&read(&fixture_path)).expect("V7 vectors JSON");
    assert_eq!(
        sha256(&fixture_path),
        "d99288741f3cac16f017ffe5cd11620bfde2055e32f18b82e538eaf6d48ef411"
    );
    assert_eq!(
        sha256(&format!(
            "{V7_AUTHORITY_PACKAGE}/artifacts/reference_calculator_v7.py"
        )),
        "dfc7c586cb42f7729de09ac0660fa4b2f61d8132ccb3b24b570743bd1ba8a5dd"
    );
    assert_eq!(fixture["model_version"], "OPENWEPP_C3_WOODY_V7");
    assert_eq!(fixture["constants"]["f_stor_xfer"], 0.5);
    assert_eq!(
        fixture["six_tissue_vectors"]
            .as_array()
            .expect("six tissue vectors")
            .len(),
        6
    );
    for tissue in fixture["six_tissue_vectors"]
        .as_array()
        .expect("six tissue vectors")
    {
        assert_eq!(tissue["carbon_source_operand_independent"], true);
        assert_eq!(tissue["nitrogen_source_operand_independent"], true);
        assert_eq!(
            tissue["preparation_amount"]["carbon"].as_f64(),
            tissue["beginning"]["storage"]["carbon"]
                .as_f64()
                .map(|value| 0.5 * value)
        );
        assert_eq!(
            tissue["preparation_amount"]["nitrogen"].as_f64(),
            tissue["beginning"]["storage"]["nitrogen"]
                .as_f64()
                .map(|value| 0.5 * value)
        );
    }
    assert_eq!(fixture["multi_interval_onset"]["preparation_count"], 1);
    assert_eq!(
        fixture["multi_interval_onset"]["phase_before_exhaustion"],
        "onset"
    );
    assert_eq!(fixture["multi_interval_onset"]["final_phase"], "active");
    assert_eq!(fixture["terminal_remainder_branch"]["fraction"], 1.0);
    assert_eq!(
        fixture["terminal_remainder_branch"]["all_transfer_exact_zero"],
        true
    );
    assert_eq!(
        fixture["current_interval_allocation_exclusion"]["new_allocation_remains_in_storage"],
        true
    );
    assert_v7_migration(&fixture);
    assert_v7_poisons(&fixture);
    assert_v7_rollback(&fixture);
}
