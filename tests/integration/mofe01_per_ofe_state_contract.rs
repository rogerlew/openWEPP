use std::fs;

fn read_repo_file(repo_root: &str, path: &str) -> String {
    fs::read_to_string(format!("{repo_root}/{path}")).unwrap_or_else(|err| {
        panic!("{path} should be readable from repository root {repo_root}: {err}")
    })
}

#[allow(clippy::too_many_lines)]
fn rust_code_without_comments_or_string_literals(source: &str) -> String {
    let bytes = source.as_bytes();
    let mut code = String::with_capacity(source.len());
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] == b'/' && bytes.get(index + 1) == Some(&b'/') {
            index += 2;
            while index < bytes.len() && bytes[index] != b'\n' {
                index += 1;
            }
            code.push('\n');
            index += usize::from(index < bytes.len());
        } else if bytes[index] == b'/' && bytes.get(index + 1) == Some(&b'*') {
            index += 2;
            let mut depth = 1usize;
            while index < bytes.len() && depth > 0 {
                if bytes[index] == b'/' && bytes.get(index + 1) == Some(&b'*') {
                    depth += 1;
                    index += 2;
                } else if bytes[index] == b'*' && bytes.get(index + 1) == Some(&b'/') {
                    depth -= 1;
                    index += 2;
                } else {
                    if bytes[index] == b'\n' {
                        code.push('\n');
                    }
                    index += 1;
                }
            }
            code.push(' ');
        } else if bytes[index] == b'r' {
            let mut delimiter = index + 1;
            while delimiter < bytes.len() && bytes[delimiter] == b'#' {
                delimiter += 1;
            }
            if delimiter < bytes.len() && bytes[delimiter] == b'"' {
                let hash_count = delimiter - index - 1;
                index = delimiter + 1;
                while index < bytes.len() {
                    if bytes[index] == b'"' {
                        let hash_end = index + 1 + hash_count;
                        if hash_end <= bytes.len()
                            && bytes[index + 1..hash_end].iter().all(|byte| *byte == b'#')
                        {
                            index = hash_end;
                            break;
                        }
                    }
                    index += 1;
                }
                code.push(' ');
            } else {
                code.push('r');
                index += 1;
            }
        } else if bytes[index] == b'\'' {
            let lifetime_like = bytes
                .get(index + 1)
                .is_some_and(|byte| byte.is_ascii_alphabetic() || *byte == b'_')
                && bytes.get(index + 2) != Some(&b'\'');
            if lifetime_like {
                code.push(' ');
                index += 1;
            } else {
                index += 1;
                while index < bytes.len() {
                    if bytes[index] == b'\\' {
                        index = (index + 2).min(bytes.len());
                    } else if bytes[index] == b'\'' {
                        index += 1;
                        break;
                    } else {
                        index += 1;
                    }
                }
                code.push(' ');
            }
        } else if bytes[index] == b'"' {
            index += 1;
            while index < bytes.len() {
                if bytes[index] == b'\\' {
                    index = (index + 2).min(bytes.len());
                } else if bytes[index] == b'"' {
                    index += 1;
                    break;
                } else {
                    index += 1;
                }
            }
            code.push(' ');
        } else {
            code.push(if bytes[index].is_ascii() {
                char::from(bytes[index])
            } else {
                ' '
            });
            index += 1;
        }
    }

    code
}

fn rust_tokens(source: &str) -> Vec<String> {
    source
        .split(|value: char| !(value.is_ascii_alphanumeric() || value == '_'))
        .filter(|token| !token.is_empty())
        .map(str::to_owned)
        .collect()
}

fn has_token(tokens: &[String], token: &str) -> bool {
    tokens.iter().any(|value| value == token)
}

fn has_item_definition(tokens: &[String], item_name: &str) -> bool {
    tokens.windows(2).any(|window| {
        matches!(window[0].as_str(), "struct" | "enum" | "type") && window[1] == item_name
    })
}

fn has_impl_block(tokens: &[String], item_name: &str) -> bool {
    tokens
        .windows(2)
        .any(|window| window[0] == "impl" && window[1] == item_name)
}

fn runtime_source_tokens(repo_root: &str) -> Vec<String> {
    let paths = [
        "crates/openwepp-hillslope-orchestrator/src/scheduler.rs",
        "crates/openwepp-hillslope-orchestrator/src/scheduler/water_balance.rs",
        "crates/openwepp-kernel-contract/src/lib_mod/core_types.rs",
        "crates/openwepp-kernel-contract/src/lib_mod/writeback.rs",
        "crates/openwepp-runner/src/hillslope/scheduler_trace/per_ofe_internal_wb13.rs",
        "crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs",
        "crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_publication.rs",
        "crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs",
        "crates/openwepp-hillslope-output/src/manifest.rs",
        "crates/openwepp-hillslope-output/src/hillslope_wat.rs",
        "crates/openwepp-hillslope-output/src/writers.rs",
    ];
    let stripped_code = paths
        .iter()
        .map(|path| rust_code_without_comments_or_string_literals(&read_repo_file(repo_root, path)))
        .collect::<Vec<_>>()
        .join("\n");

    rust_tokens(&stripped_code)
}

fn registry_row<'a>(index: &'a str, contract_id: &str) -> &'a str {
    index
        .lines()
        .find(|line| line.starts_with('|') && line.contains(&format!("`{contract_id}`")))
        .unwrap_or_else(|| panic!("science contract index must contain row for {contract_id}"))
}

fn assert_registry_row(index: &str, contract_id: &str, path: &str) {
    let row = registry_row(index, contract_id);
    assert!(
        row.contains("`in_review`")
            && row.contains("`draft`")
            && row.contains(path)
            && row.contains("`static`")
            && row.contains("Lifecycle-only row"),
        "science contract index row for {contract_id} must expose lifecycle fields and path"
    );
}

#[test]
fn mofe01_me0_contract_authority_is_present() {
    let repo_root = env!("CARGO_MANIFEST_DIR");

    let runoffpart = read_repo_file(
        repo_root,
        "docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md",
    );
    assert!(
        runoffpart.contains("INV-RUNOFFPART-029")
            && runoffpart.contains("MOFE01 M-E0 Per-OFE Runoff Lane-State Addendum")
            && runoffpart.contains("TransferInput")
            && runoffpart.contains("TransferOutput")
            && runoffpart.contains("TopologyGraph")
            && runoffpart.contains("single-OFE runoff behavior bit-identically")
            && runoffpart.contains("MOFE01 M-E0 amendment"),
        "SC-RUNOFFPART-001 must carry M-E0 per-OFE lane-state authority"
    );

    let watbal = read_repo_file(
        repo_root,
        "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
    );
    assert!(
        watbal.contains("INV-WATBAL-097")
            && watbal.contains("MOFE01 M-E0 Per-OFE Dynamic Water-Balance State Addendum")
            && watbal.contains("PerOfeDailyWaterBalanceRecord")
            && watbal.contains("publication policy is explicitly flipped")
            && watbal.contains("Single-OFE runs are one-record specialization")
            && watbal.contains("MOFE01 M-E0 amendment")
            && watbal.contains("MOFE01 M-E4-REDO Internal WB13 Identity Acceptance Addendum")
            && watbal.contains("TOL-WATBAL-007")
            && watbal.contains("<= 1e-11 mm")
            && watbal.contains("pre-day OFE dynamic storage snapshot")
            && watbal.contains("SoilWaterTotal == Total-Soil")
            && watbal.contains("current_transfer_output")
            && watbal.contains("upstream_transfer_input")
            && watbal.contains("all-exact-zero residual suite is a hold"),
        "SC-WATBAL-001 must carry M-E0 per-OFE dynamic water-balance authority"
    );
    assert!(
        watbal.contains("INV-WATBAL-100")
            && watbal.contains("MOFE01 M-I Hillslope-Total Closure Addendum")
            && watbal.contains("TOL-WATBAL-008")
            && watbal.contains("hillslope_total_identity_max_abs_mm")
            && watbal.contains("Published WAT rows may audit the result")
            && watbal.contains("persistent per-OFE scheduler lifecycle"),
        "SC-WATBAL-001 must carry M-I hillslope-total closure authority"
    );

    let system = read_repo_file(
        repo_root,
        "docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md",
    );
    assert!(
        system.contains("INV-SYSTEM-030")
            && system.contains("MOFE01 M-E0 Per-OFE Dynamic-State Publication Policy Addendum")
            && system.contains("per-ofe-dynamic-water-balance-state")
            && system.contains("per-ofe-dynamic-wb-state")
            && system.contains("aggregate rows are relabeled as per-OFE records")
            && system.contains("MOFE01 M-E0 amendment"),
        "SC-SYSTEM-001 must carry M-E0 per-OFE publication-policy authority"
    );

    let index = read_repo_file(repo_root, "docs/specifications/science-contracts/index.md");
    assert_registry_row(
        &index,
        "SC-RUNOFFPART-001",
        "docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md",
    );
    assert_registry_row(
        &index,
        "SC-WATBAL-001",
        "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
    );
    assert_registry_row(
        &index,
        "SC-SYSTEM-001",
        "docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md",
    );
}

#[test]
fn mofe01_me0_current_architecture_requires_structural_per_ofe_state_collection() {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let tokens = runtime_source_tokens(repo_root);

    let has_collection = has_item_definition(&tokens, "PerOfeDailyWaterBalanceCollection")
        && has_item_definition(&tokens, "PerOfeDailyWaterBalanceRecord")
        && has_impl_block(&tokens, "PerOfeDailyWaterBalanceCollection")
        && [
            "records",
            "ofe_id",
            "upstream_transfer_input",
            "current_transfer_output",
        ]
        .iter()
        .all(|required_token| has_token(&tokens, required_token));

    assert!(
        has_collection,
        "M-E0 red gate: current aggregate architecture lacks a structural PerOfeDailyWaterBalanceCollection, PerOfeDailyWaterBalanceRecord, and typed OFE-keyed transfer fields"
    );
}

#[test]
fn mofe01_me0_current_architecture_requires_structural_transfer_payloads() {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let tokens = runtime_source_tokens(repo_root);

    let has_transfer_payloads = has_item_definition(&tokens, "TransferInput")
        && has_item_definition(&tokens, "TransferOutput")
        && [
            "source_ofe_id",
            "recipient_ofe_id",
            "surface_carry",
            "lateral_carry",
        ]
        .iter()
        .all(|required_token| has_token(&tokens, required_token));

    assert!(
        has_transfer_payloads,
        "M-E0 red gate: current architecture lacks structural TransferInput/TransferOutput payloads with OFE identity and separated surface/lateral carry"
    );
}

#[test]
fn mofe01_me0_current_architecture_requires_publication_policy_manifest_gate() {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let tokens = runtime_source_tokens(repo_root);

    let has_publication_gate = [
        "publication_ofe_policy",
        "per_ofe_dynamic_water_balance_state",
        "contributor_ofe_count",
        "per_ofe_record_count",
        "per_ofe_state_policy",
        "transfer_identity_status",
        "per_element_identity_status",
        "aggregate_identity_status",
        "storage_lineage_policy",
        "per_ofe_dynamic_wb_state",
    ]
    .iter()
    .all(|required_token| has_token(&tokens, required_token));

    assert!(
        has_publication_gate,
        "M-E0 red gate: current publication path lacks manifest-gated per-OFE policy, record cardinality, identity statuses, and per-OFE storage lineage"
    );
}

#[test]
fn mofe01_me4_redo_current_architecture_requires_non_tautological_internal_wb13_checks() {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let tokens = runtime_source_tokens(repo_root);

    let has_non_tautological_internal_wb13_checks = [
        "ME4_INTERNAL_WB13_IDENTITY_TOLERANCE_MM",
        "previous_storage_total_mm",
        "internal_wb13_storage_total_mm_from_surface",
        "per_element_water_balance_terms",
        "adjacent_transfer_residual_mm",
        "internal_wb13_frost_internal_adjustment_m",
        "frost_internal_adjustment_mm",
        "frost_upper_overflow_mm",
        "scaled_transfer_sum_mm",
        "current_transfer_output",
        "upstream_transfer_input",
        "frozwt",
    ]
    .iter()
    .all(|required_token| has_token(&tokens, required_token));

    assert!(
        has_non_tautological_internal_wb13_checks,
        "M-E4-REDO red gate: internal WB13 identity checks must use pre-day storage and adjacent sent/received transfer operands, not row aliases or self-built transfer inputs"
    );
}

#[test]
fn mofe01_mi_current_architecture_requires_independent_hillslope_total_identity() {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let tokens = runtime_source_tokens(repo_root);

    let has_hillslope_total_identity = [
        "MI_HILLSLOPE_TOTAL_IDENTITY_TOLERANCE_MM",
        "area_m2",
        "hillslope_total_identity_residual_mm",
        "hillslope_total_identity_max_abs_mm",
        "with_upstream_area_ratio",
        "upstream_area_ratio",
        "local_liquid_input_mm",
        "storage_delta_mm",
        "interception_mm",
        "physical_q_mm",
        "latqcc_mm",
        "frost_internal_adjustment_mm",
        "frost_upper_overflow_mm",
    ]
    .iter()
    .all(|required_token| has_token(&tokens, required_token));

    assert!(
        has_hillslope_total_identity,
        "M-I red gate: internal WB13 checks must publish an area-weighted hillslope-total identity from internal operands, not public WAT aliases"
    );
}

#[test]
fn mofe01_mi_multiofe_runner_lifecycle_is_mutually_exclusive_with_single_ofe_aggregate_path() {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let runner_source = rust_code_without_comments_or_string_literals(&read_repo_file(
        repo_root,
        "crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs",
    ));
    let persistent_call = runner_source
        .find("execute_persistent_scheduler_kernel_lifecycle")
        .expect("runner must call the persistent per-OFE scheduler lifecycle");
    let aggregate_call = runner_source
        .find("execute_scheduler_kernel_lifecycle")
        .expect("runner must keep the single-OFE aggregate scheduler lifecycle");
    let branch_open = runner_source[..persistent_call]
        .rfind("if let Some")
        .expect("persistent scheduler call must be under the multi-OFE branch");

    assert!(
        persistent_call < aggregate_call
            && runner_source[persistent_call..aggregate_call].contains("} else {")
            && runner_source[branch_open..persistent_call].contains("persistent_lane_state"),
        "M-I gate: multi-OFE persistent scheduler and single-OFE aggregate scheduler must remain mutually exclusive, not sequential double execution"
    );
}

#[test]
fn mofe01_mf_current_architecture_requires_public_per_ofe_wat_publication() {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let tokens = runtime_source_tokens(repo_root);

    let has_public_per_ofe_wat_publication = [
        "PersistentDailyExecutionResult",
        "append_publication_rows_to",
        "outlet_row",
        "MF_PUBLICATION_OFE_POLICY",
        "MF_STORAGE_LINEAGE_POLICY",
        "MF_PER_OFE_STATE_POLICY",
        "MF_IDENTITY_STATUS",
        "per_ofe_dynamic_water_balance_state",
        "per_ofe_dynamic_wb_state",
        "per_ofe_record_count",
        "per_ofe_expected_record_count",
        "row_count",
        "sim_day_index_monotonic",
    ]
    .iter()
    .all(|required_token| has_token(&tokens, required_token));

    assert!(
        has_public_per_ofe_wat_publication,
        "M-F red gate: current publication path must publish internal per-OFE WB13 records into public WAT/WB13 rows with per-OFE policy, storage lineage, and row-cardinality manifest guards"
    );
}
