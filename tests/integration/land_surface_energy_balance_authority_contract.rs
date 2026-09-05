use std::fs;

const CONTRACT: &str =
    "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md";
const INDEX: &str = "docs/specifications/science-contracts/index.md";
const AUTHORITY_PACKAGE: &str =
    "docs/work-packages/20260814-snow-free-land-surface-energy-authority-001/artifacts";
const LSE_V3_PRODUCTION_PATHS: &[&str] = &[
    "crates/openwepp-land-surface-energy/src/lib.rs",
    "crates/openwepp-land-surface-energy/src/v3_state.rs",
    "crates/openwepp-land-surface-energy/src/litter_phase.rs",
    "crates/openwepp-land-surface-energy/src/litter_phase_closure.rs",
    "crates/openwepp-land-surface-energy/src/litter_phase_output.rs",
    "crates/openwepp-land-surface-energy/src/solver_litter_phase.rs",
    "crates/openwepp-land-surface-energy/src/transaction_v3.rs",
];
const LSE_V3_REAL_CONSUMERS: &[&str] = &[
    "tests/integration/erosion_single_ofe_p61_sediment.rs",
    "tests/integration/dff_ws1_native_forest_cli.rs",
];
const EXACT_CARRY_PRODUCTION_PATHS: &[&str] = &[
    "crates/openwepp-land-surface-energy/src/lib.rs",
    "crates/openwepp-land-surface-energy/src/exact_dyadic_enthalpy.rs",
    "crates/openwepp-land-surface-energy/src/owner_envelope.rs",
    "crates/openwepp-land-surface-energy/src/transaction.rs",
];
const COMPONENT_DEPENDENCY_REPLAY_PRODUCTION_PATHS: &[&str] = &[
    "crates/openwepp-land-surface-energy/src/solver_covered_solve.rs",
    "crates/openwepp-land-surface-energy/src/solver_covered_evaluation.rs",
];

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

fn read_existing(paths: &[&str]) -> String {
    paths
        .iter()
        .filter_map(|path| fs::read_to_string(path).ok())
        .collect::<Vec<_>>()
        .join("\n")
}

fn rust_char_literal_end(bytes: &[u8], start: usize) -> Option<usize> {
    let quote = if bytes.get(start) == Some(&b'\'') {
        start
    } else if bytes.get(start) == Some(&b'b') && bytes.get(start + 1) == Some(&b'\'') {
        start + 1
    } else {
        return None;
    };
    let mut cursor = quote + 1;
    let first = *bytes.get(cursor)?;
    if first == b'\\' {
        cursor += 1;
        match *bytes.get(cursor)? {
            b'u' if bytes.get(cursor + 1) == Some(&b'{') => {
                cursor += 2;
                while bytes.get(cursor).is_some_and(|byte| *byte != b'}') {
                    cursor += 1;
                }
                cursor += usize::from(bytes.get(cursor) == Some(&b'}'));
            }
            b'x' => cursor += 3,
            _ => cursor += 1,
        }
    } else {
        if matches!(first, b'\'' | b'\n' | b'\r') {
            return None;
        }
        let scalar_length = match first {
            0x00..=0x7f => 1,
            0xc0..=0xdf => 2,
            0xe0..=0xef => 3,
            0xf0..=0xf7 => 4,
            _ => return None,
        };
        cursor += scalar_length;
    }
    (bytes.get(cursor) == Some(&b'\'')).then_some(cursor + 1)
}

fn rust_code_mask(source: &str) -> String {
    let bytes = source.as_bytes();
    let mut masked = bytes.to_vec();
    let mut index = 0;
    while index < bytes.len() {
        let raw_prefix_length = if bytes[index] == b'r' {
            Some(1)
        } else if index + 1 < bytes.len() && bytes[index] == b'b' && bytes[index + 1] == b'r' {
            Some(2)
        } else {
            None
        };
        let raw_delimiter = raw_prefix_length.and_then(|prefix_length| {
            let mut quote = index + prefix_length;
            while quote < bytes.len() && bytes[quote] == b'#' {
                quote += 1;
            }
            (quote < bytes.len() && bytes[quote] == b'"')
                .then_some((quote, quote - index - prefix_length))
        });
        if let Some((opening_quote, hash_count)) = raw_delimiter {
            let mut end = opening_quote + 1;
            while end < bytes.len() {
                if bytes[end] == b'"'
                    && end + 1 + hash_count <= bytes.len()
                    && bytes[end + 1..end + 1 + hash_count]
                        .iter()
                        .all(|byte| *byte == b'#')
                {
                    end += 1 + hash_count;
                    break;
                }
                end += 1;
            }
            while index < end {
                if bytes[index] != b'\n' {
                    masked[index] = b' ';
                }
                index += 1;
            }
        } else if index + 1 < bytes.len() && bytes[index] == b'/' && bytes[index + 1] == b'/' {
            while index < bytes.len() && bytes[index] != b'\n' {
                masked[index] = b' ';
                index += 1;
            }
        } else if index + 1 < bytes.len() && bytes[index] == b'/' && bytes[index + 1] == b'*' {
            let mut depth = 1_u32;
            masked[index] = b' ';
            masked[index + 1] = b' ';
            index += 2;
            while index < bytes.len() && depth > 0 {
                if index + 1 < bytes.len() && bytes[index] == b'/' && bytes[index + 1] == b'*' {
                    depth += 1;
                    masked[index] = b' ';
                    masked[index + 1] = b' ';
                    index += 2;
                } else if index + 1 < bytes.len()
                    && bytes[index] == b'*'
                    && bytes[index + 1] == b'/'
                {
                    depth -= 1;
                    masked[index] = b' ';
                    masked[index + 1] = b' ';
                    index += 2;
                } else {
                    if bytes[index] != b'\n' {
                        masked[index] = b' ';
                    }
                    index += 1;
                }
            }
        } else if let Some(end) = rust_char_literal_end(bytes, index) {
            while index < end {
                if bytes[index] != b'\n' {
                    masked[index] = b' ';
                }
                index += 1;
            }
        } else if bytes[index] == b'"' {
            masked[index] = b' ';
            index += 1;
            while index < bytes.len() {
                let byte = bytes[index];
                if byte != b'\n' {
                    masked[index] = b' ';
                }
                index += 1;
                if byte == b'\\' && index < bytes.len() {
                    if bytes[index] != b'\n' {
                        masked[index] = b' ';
                    }
                    index += 1;
                } else if byte == b'"' {
                    break;
                }
            }
        } else {
            index += 1;
        }
    }
    String::from_utf8(masked).expect("masked Rust remains UTF-8")
}

fn rust_item_body<'a>(source: &'a str, marker: &str) -> Option<&'a str> {
    let masked = rust_code_mask(source);
    let start = masked.find(marker)?;
    let open = masked[start..].find('{')? + start;
    let mut depth = 0_u32;
    for (offset, byte) in masked.as_bytes()[open..].iter().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&source[open..=open + offset]);
                }
            }
            _ => {}
        }
    }
    None
}

fn rust_item_attribute_cursor(masked: &str, marker_start: usize) -> usize {
    let bytes = masked.as_bytes();
    let mut cursor = marker_start;
    loop {
        while cursor > 0 && bytes[cursor - 1].is_ascii_whitespace() {
            cursor -= 1;
        }
        let modifier_end = cursor;

        if modifier_end > 0 && bytes[modifier_end - 1] == b')' {
            let mut depth = 0_u32;
            let mut open = None;
            for index in (0..modifier_end).rev() {
                match bytes[index] {
                    b')' => depth += 1,
                    b'(' => {
                        depth -= 1;
                        if depth == 0 {
                            open = Some(index);
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if let Some(open) = open {
                let mut word_end = open;
                while word_end > 0 && bytes[word_end - 1].is_ascii_whitespace() {
                    word_end -= 1;
                }
                let mut word_start = word_end;
                while word_start > 0
                    && (bytes[word_start - 1].is_ascii_alphanumeric()
                        || bytes[word_start - 1] == b'_')
                {
                    word_start -= 1;
                }
                if &masked[word_start..word_end] == "pub" {
                    cursor = word_start;
                    continue;
                }
            }
        }

        let mut word_start = modifier_end;
        while word_start > 0
            && (bytes[word_start - 1].is_ascii_alphanumeric() || bytes[word_start - 1] == b'_')
        {
            word_start -= 1;
        }
        if matches!(
            &masked[word_start..modifier_end],
            "pub" | "async" | "const" | "unsafe" | "extern" | "default"
        ) {
            cursor = word_start;
            continue;
        }
        return cursor;
    }
}

fn rust_item_cfg_attributes(source: &str, marker: &str) -> Option<String> {
    let masked = rust_code_mask(source);
    let mut cursor = rust_item_attribute_cursor(&masked, masked.find(marker)?);
    let bytes = masked.as_bytes();
    let mut attributes = Vec::new();
    loop {
        while cursor > 0 && bytes[cursor - 1].is_ascii_whitespace() {
            cursor -= 1;
        }
        if cursor == 0 || bytes[cursor - 1] != b']' {
            break;
        }
        let end = cursor;
        let mut depth = 0_u32;
        let mut open = None;
        for index in (0..cursor).rev() {
            match bytes[index] {
                b']' => depth += 1,
                b'[' => {
                    depth -= 1;
                    if depth == 0 {
                        open = Some(index);
                        break;
                    }
                }
                _ => {}
            }
        }
        let Some(open) = open else {
            return None;
        };
        if open == 0 || bytes[open - 1] != b'#' {
            break;
        }
        let attribute = &masked[open - 1..end];
        if attribute
            .split(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
            .any(|token| token == "cfg" || token == "cfg_attr")
        {
            attributes.push(attribute.to_owned());
        }
        cursor = open - 1;
    }
    Some(attributes.join("\n"))
}

fn rust_item_is_cfg_gated(source: &str, marker: &str) -> bool {
    rust_item_cfg_attributes(source, marker).is_some_and(|attributes| !attributes.is_empty())
}

fn rust_item_is_top_level(source: &str, marker: &str) -> bool {
    let masked = rust_code_mask(source);
    let Some(start) = masked.find(marker) else {
        return false;
    };
    let mut depth = 0_i32;
    for byte in masked.as_bytes()[..start].iter() {
        match byte {
            b'{' => depth += 1,
            b'}' => depth -= 1,
            _ => {}
        }
    }
    depth == 0
}

fn rust_item_is_unconditional_top_level(source: &str, marker: &str) -> bool {
    rust_item_body(source, marker).is_some()
        && !rust_item_is_cfg_gated(source, marker)
        && rust_item_is_top_level(source, marker)
}

#[test]
fn rust_structural_item_parser_rejects_cfg_disabled_and_test_only_decoys() {
    let fixture = r#####"
        // struct CommentDecoy { marker: bool }
        const STRING_DECOY: &str = "fn StringDecoy() {}";
        const RAW_STRING_DECOY: &str = r#"embedded "quote" before
            #[cfg(not(any()))]
            pub(crate) struct RawStringDecoy { marker: bool }
        "#;
        const RAW_BYTE_STRING_DECOY: &[u8] = br###"
            fn RawByteStringDecoy() {}
        "###;

        #[cfg(any())]
        pub(crate) struct DisabledDecoy {
            marker: bool,
        }

        #[allow(dead_code)]
        #[cfg(any(
            test,
            feature = "structural-decoy",
        ))]
        pub(in crate) async fn StackedTestDecoy() {
            let _marker = true;
        }

        #[cfg_attr(test, cfg(any()))]
        pub struct ConditionalCfgDecoy {
            marker: bool,
        }

        #[allow(dead_code)]
        pub(super) struct LiveProductionItem {
            marker: bool,
        }
    "#####;

    assert!(rust_item_body(fixture, "struct CommentDecoy").is_none());
    assert!(rust_item_body(fixture, "fn StringDecoy").is_none());
    assert!(rust_item_body(fixture, "struct RawStringDecoy").is_none());
    assert!(rust_item_body(fixture, "fn RawByteStringDecoy").is_none());
    for marker in [
        "struct DisabledDecoy",
        "fn StackedTestDecoy",
        "struct ConditionalCfgDecoy",
    ] {
        assert!(rust_item_body(fixture, marker).is_some());
        assert!(rust_item_is_cfg_gated(fixture, marker), "{marker}");
        assert!(rust_item_is_top_level(fixture, marker), "{marker}");
    }
    assert!(!rust_item_is_cfg_gated(
        fixture,
        "struct LiveProductionItem"
    ));
    assert!(rust_item_is_top_level(fixture, "struct LiveProductionItem"));
}

#[test]
fn rust_structural_item_parser_masks_char_literals_and_rejects_nested_required_decoys() {
    let fixture = r#####"
        fn lifetime_control<'a>(value: &'a str, other: &'static str) -> &'a str {
            let _placeholder: &'_ str = other;
            value
        }

        fn nested_decoy_container() {
            let _closing_char = '}';
            let _escaped_quote = '\'';

            #[cfg(test)]
            struct CoveredComponentTemperatureDependencyGraph {}
            #[cfg(any(test, feature = "structural-decoy"))]
            struct ValidatedCoveredComponentReplaySweepBase {}
            #[cfg(any())]
            struct ValidatedCoveredComponentProbeReplay {}
            #[cfg_attr(test, cfg(any()))]
            struct CoveredComponentDependencyReplayAudit {}
            #[cfg(test)]
            fn covered_component_temperature_probe_residuals() {}
            #[cfg(any())]
            fn begin_covered_component_dependency_replay_audit() {}
            #[cfg(any(test, feature = "structural-decoy"))]
            fn take_covered_component_dependency_replay_audit() {}

            let _closing_byte = b'}';
            let _escaped_byte_quote = b'\'';
        }

        fn covered_jacobian_probe_residuals() {
            let _closing_char = '}';
            let _escaped_quote = '\'';
            #[cfg(test)]
            {
                covered_component_temperature_probe_residuals();
            }
            let covered_component_temperature_probe_residuals = false;
            let _bare_reference = covered_component_temperature_probe_residuals;
            if false {
                covered_component_temperature_probe_residuals();
            }
            let _closing_byte = b'}';
            let _escaped_byte_quote = b'\'';
        }
    "#####;

    let lifetime_mask = rust_code_mask(fixture);
    assert!(lifetime_mask.contains("<'a>"));
    assert!(lifetime_mask.contains("&'static str"));
    assert!(lifetime_mask.contains("&'_ str"));
    assert!(rust_item_body(fixture, "fn lifetime_control").is_some());

    for marker in [
        "struct CoveredComponentTemperatureDependencyGraph",
        "struct ValidatedCoveredComponentReplaySweepBase",
        "struct ValidatedCoveredComponentProbeReplay",
        "struct CoveredComponentDependencyReplayAudit",
        "fn covered_component_temperature_probe_residuals",
        "fn begin_covered_component_dependency_replay_audit",
        "fn take_covered_component_dependency_replay_audit",
    ] {
        assert!(rust_item_body(fixture, marker).is_some(), "{marker}");
        assert!(rust_item_is_cfg_gated(fixture, marker), "{marker}");
        assert!(!rust_item_is_top_level(fixture, marker), "{marker}");
        assert!(!rust_item_is_unconditional_top_level(fixture, marker));
    }
    let dispatcher = rust_item_body(fixture, "fn covered_jacobian_probe_residuals")
        .expect("adversarial dispatcher fixture");
    assert!(dispatcher.contains("#[cfg(test)]"));
    assert!(
        dispatcher
            .matches("covered_component_temperature_probe_residuals")
            .count()
            >= 4,
        "cfg-only calls, local bindings, bare references, and dead calls show why tokens cannot prove connectivity"
    );
}

fn row<'a>(contract: &'a str, key: &str) -> &'a str {
    contract
        .lines()
        .find(|line| line.starts_with(&format!("| `{key}` |")))
        .unwrap_or_else(|| panic!("{CONTRACT} missing row {key}"))
}

fn assert_lse_registry_lifecycle(index: &str) {
    let lifecycle = row(index, "SC-LANDSURFACEENERGY-001");
    assert!(lifecycle.starts_with(
        "| `SC-LANDSURFACEENERGY-001` | Land-Surface Energy-Balance Process Contract | `approved` | `active` |"
    ));
    assert!(
        lifecycle.contains(
            "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md"
        )
    );
    assert!(lifecycle.contains("| `2026-09-04` |"));
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
        "contract_version: 31",
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
        "INV-LANDSURFACEENERGY-152",
        "OBL-LANDSURFACEENERGY-C-007",
        "INV-LANDSURFACEENERGY-153",
        "ParentLocalPartial",
        "PersistentParentFinal",
        "INV-LANDSURFACEENERGY-154",
        "OBL-LANDSURFACEENERGY-C-009",
        "LSE-STAGE3-NATIVE-CROSS-REGIME",
        "OPENWEPP_SNOW_FREE_LSE_V3`\nremains exclusively snow-free",
        "CoveredLowerBoundaryEnergyOperands::Stage3SnowCovered",
        "A second inner legacy LSE/hydrology envelope is forbidden",
        "frozen-litter\nV3 physical owner",
        "V4 exact-energy companion",
        "only a positive\nsnow-free successor may invoke the snow-free V3/V4 litter path",
        "INV-LANDSURFACEENERGY-155",
        "OBL-LANDSURFACEENERGY-C-010",
        "LSE-V2-UNPUBLISHED-CANDIDATE-BEGINNING",
        "SoilThermalUnpublishedPhysicalBeginningV2",
        "borrowed constitutive read surface",
        "no `SoilThermalOwnerEnvelopeV2`, restart, checkpoint",
        "complete original prepared owner plus every canonically accumulated",
        "Support rebinding,\nowner-byte synthesis, private-trial promotion, intermediate or dual acceptance",
        "INV-LANDSURFACEENERGY-156",
        "OBL-LANDSURFACEENERGY-C-011",
        "LSE-V3-LITTER-PHASE-CAPACITY-SPILL",
        "LitterPhaseCapacitySpillV1",
        "LitterPhaseCapacitySpillEnergy",
        "The second subtraction is the authoritative remainder",
        "`LitterPhaseOverflow` parcel",
        "It cannot be caller synthesized or labeled as a\ncondensation credit",
        "INV-LANDSURFACEENERGY-157",
        "OBL-LANDSURFACEENERGY-C-012",
        "LSE-V3-HETEROGENEOUS-SURFACE-RESOURCE-JOIN",
        "SurfaceLiquidV2HeterogeneousResourceJoinV1",
        "Every finalized row is accounted exactly once",
        "INV-LANDSURFACEENERGY-158",
        "OBL-LANDSURFACEENERGY-C-013",
        "LSE-V16-TOPOLOGY-RANKED-EXACT-OWNER",
        "OFE IDs are opaque",
        "Bare envelope validation may prove schema, digest, uniqueness, and lineage",
        "INV-LANDSURFACEENERGY-159",
        "OBL-LANDSURFACEENERGY-C-014",
        "LSE-V24-VALIDATED-IN-MEMORY-HANDOFF",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn version_twenty_four_binds_private_validated_handoffs_and_full_boundary_validation() {
    let contract = read(CONTRACT);
    for required in [
        "## Validated In-Memory LSE Custody Handoff Amendment",
        "INV-LANDSURFACEENERGY-159",
        "OBL-LANDSURFACEENERGY-C-014",
        "LSE-V24-VALIDATED-IN-MEMORY-HANDOFF",
        "private nonserializable typed handoff",
        "exact prefix count",
        "first receipt, last receipt, and chain digest",
        "append operation validates the new support",
        "does not revalidate the immutable prefix",
        "repeatedly serializing the same three nested owner envelopes",
        "Every\nmutation or replacement consumes the proof",
        "Restart/checkpoint\nrestore, external bytes, durable publication, and untrusted executor outputs",
        "full prefix replay",
        "O(1)-with-history direct install",
        "complete rollback",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
    assert_lse_registry_lifecycle(&read(INDEX));
    assert!(
        read("docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md")
            .contains("INV-SURFACELIQUID-031")
    );
    assert!(
        read("docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md")
            .contains("INV-SNOWENERGY-083")
    );
}

#[test]
fn version_thirty_binds_parent_static_and_same_map_validation_once_to_existing_invariant() {
    let contract = read(CONTRACT);
    for required in [
        "contract_version: 31",
        "## Carrier Parent-Static and Same-Map Validation-Once Amendment",
        "extends the already admitted private validation-once custody of\n`INV-LANDSURFACEENERGY-159`; it creates no new invariant",
        "private non-Clone, non-wire,\ngeneration-bound structural plan",
        "Plan construction is lazy at the first structural validation",
        "existing forcing-validation position before V8",
        "bound to the live forcing allocation",
        "Equal digest with a different allocation is not authority",
        "V8 neither receives nor attests to the\n   distinct native resident's V3 LSE or V2 surface objects",
        "ValidatedFrozenLitterV3ResidentRevisionV1",
        "borrowed, pointer-, revision-, parent-generation-, and map-bound",
        "surface_beginning.canonical_bytes(surface_configuration)",
        "Ordinary maps mint and consume no\n   resident proof",
        "direct before composed, and composed\nretains Half1 before Half2",
        "ValidatedV8RuntimeInputProjection",
        "projected column, solver-ready tile",
        "hydrology snapshot, physical result",
        "must not\nuse `Arc<DirectV10...>`",
        "existing `Clone` implementation is authorized only as an inseparable",
        "OBL-LANDSURFACEENERGY-C-019",
        "exactly one\nparent-static validation, 52 exact normalized-forcing validations, and 52 fresh",
        "Initial/history/final and\ndirect/Half1/Half2 role/path",
        "same digest/different allocation, ingress schedule, resident revision",
        "dynamic\nvegetation/surface/soil-hydrology state, native solver/residual, and output",
        "through\ndynamic validation, solver/residual, and output validation",
        "fabricated outcomes, manually incremented fixture counters, or source scanning\nalone cannot satisfy",
        "LSE-V30-CARRIER-PARENT-STATIC-VALIDATION-ONCE",
        "`maps-to-existing-INV`",
        "CALIBRATION_NOT_APPLICABLE",
        "no process physics, solver, tolerance, output, publication, or wire change",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }

    let invariant_rows = contract
        .lines()
        .filter(|line| line.starts_with("| `INV-LANDSURFACEENERGY-"))
        .collect::<Vec<_>>();
    assert!(invariant_rows.iter().any(|line| {
        line.starts_with("| `INV-LANDSURFACEENERGY-159` |")
            && line.contains("parent-static structural plan")
            && line.contains("resident-revision-sourced native proof")
    }));
    assert_eq!(
        invariant_rows
            .iter()
            .filter(|line| line.starts_with("| `INV-LANDSURFACEENERGY-164` |"))
            .count(),
        2,
        "v31 must register INV-164 once in the authority table and once in the producer guard map",
    );

    let exposure = contract
        .split("## Binding Exposure Index")
        .nth(1)
        .expect("binding exposure section")
        .split("## Gap Register")
        .next()
        .expect("binding exposure body");
    let v30_rows = exposure
        .lines()
        .filter(|line| line.starts_with("| `LSE-V30-CARRIER-PARENT-STATIC-VALIDATION-ONCE` |"))
        .collect::<Vec<_>>();
    assert_eq!(v30_rows.len(), 1);
    assert!(v30_rows[0].contains("INV-LANDSURFACEENERGY-159"));
    assert!(v30_rows[0].contains("OBL-LANDSURFACEENERGY-C-019"));

    assert_lse_registry_lifecycle(&read(INDEX));
}

#[test]
fn version_thirty_one_binds_component_temperature_dependency_replay() {
    let contract = read(CONTRACT);
    for required in [
        "contract_version: 31",
        "## Component-Temperature Jacobian Dependency-Replay Amendment",
        "INV-LANDSURFACEENERGY-164",
        "OBL-LANDSURFACEENERGY-C-020",
        "CoveredComponentTemperatureDependencyGraph",
        "ValidatedCoveredComponentReplaySweepBase",
        "ValidatedCoveredComponentProbeReplay",
        "topology-generic static transitive dependency graph",
        "`covered-component-temperature-dependency-v1`",
        "Expanded node and edge records are\nsorted lexically",
        "unrecognized\nevaluator node/read or missing edge",
        "`route.wet[o]` | first/source-distinct wet-flux evaluation",
        "`occ.wet[o]` | second/source-distinct wet-flux evaluation",
        "`occ.route_match[o]`",
        "`result.ground_release`, `result.ground_stemflow`, `result.output`",
        "`probe[o,wet]` | `route.wet[o]`, `route.finalize[o]`",
        "`route.prepare[o]` | `route.wet/finalize[o]`, `longwave.layer[o]`, `occ.wet[o]`",
        "the column feeds all component energy/tolerance nodes, every `occ.output[o]`",
        "`lower.ground_output` | `shared.heat`, `shared.vapor`, `shared.tolerance`",
        "`shared.heat` | `shared.tolerance`",
        "`shared.vapor` | `shared.tolerance`",
        "`result.ground_release`, `result.ground_stemflow` | `result.output`",
        "no adjacent-only truncation",
        "sun current, shade\ncurrent, sun maximum, shade maximum",
        "covered_component_dependency_replay_integrity",
        "never run the complete evaluator or another solver afterward",
        "### Normative fallibility and canonical-crossability matrix",
        "fallible and canonically crossable",
        "fallible but not currently established crossable",
        "fallible but noncrossable from an admitted replay",
        "infallible computations or assembly under their already validated predecessors",
        "currently that is\n`occ.leaf.current`",
        "`occ.leaf.maximum` joins this class only after an\nauthentic counterexample",
        "successful-base-plus-admitted-probe implication proof",
        "whenever any unmodified canonical\ninput in the branch/bound corpus naturally makes replay or forced-complete",
        "must not add mutation seams, fault-injection hooks, synthetic error branches",
        "one shared canonical\nimplementation for every common node and evaluator tail",
        "compares every record and the golden schema hash",
        "Custody is exact, not a costly proxy check",
        "A `Debug` string, digest length, allocation-independent hash",
        "caps, frozen branches, graph, trial,\ncoordinate, sign, perturbation, probe and stencil",
        "Map, solve, Newton-iteration, and sweep identities are independent authentic",
        "One ordinal\ncopied into another field",
        "if no such path exists, the variant, counter and claimed population are absent",
        "`RejectedBeforeProbe` is a per-column\nstencil/admission outcome and is not a sweep short-circuit",
        "Every hydraulic-potential, beta, shared-canopy-air, non-Stage-3",
        "No analytic or automatic\nderivative, graph coloring",
        "sparse Jacobian or LU",
        "cross-sweep/iteration/map/retry cache",
        "hardcoded two-occupancy/six-soil logic",
        "structural expected-red has one deliberately narrow claim",
        "It cannot establish\ndispatcher invocation, control-flow reachability, graph/evidence consumption",
        "Empty, skeleton, token-only,\ndead-code, or disconnected declarations",
        "exercise the real dispatcher, observe authentic sealed sweep/run counters",
        "`2*(10*N+3+S)` ordered logical probes",
        "`58 = 14 + 16 + 28`",
        "eight hydraulic, four beta, and two shared-canopy-air columns",
        "never a release-run total",
        "`Centered|InwardLower|InwardUpper|RejectedBeforeProbe`",
        "`logical=anchor+replay+complete`",
        "potential and fixed-final\nsolves",
        "same first typed error",
        "physical_phase_wall_us.potential=353431",
        "`run_wall_us <= 4803570`",
        "`physical_phase_wall_us.potential <= 253431`",
        "source `0.8488061229561478`, outlet `0.8471105124736579`, storage\n`0.0016956104824910018`, clamp `0`",
        "exact `48/56/20/32/4` workload",
        "JSON `rss_kib <= 65536`",
        "fully revert the\nrevision-31 production increment",
        "LSE-V31-COMPONENT-TEMPERATURE-DEPENDENCY-REPLAY",
        "`maps-to-existing-INV`",
        "CALIBRATION_NOT_APPLICABLE",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
    assert!(
        !contract.contains("Paired physical poisons at every ordered node boundary"),
        "v31 feasibility correction must not mandate impossible every-node poisons",
    );
    assert!(
        !contract.contains("completed/failed/short-circuited"),
        "v31 audit authority must not require a solver lifecycle state that cannot occur",
    );

    let invariant = row(&contract, "INV-LANDSURFACEENERGY-164");
    assert!(invariant.contains("same-iteration node results"));
    assert!(invariant.contains("forced-complete oracle"));

    let exposure = row(&contract, "LSE-V31-COMPONENT-TEMPERATURE-DEPENDENCY-REPLAY");
    assert!(exposure.contains("INV-LANDSURFACEENERGY-164"));
    assert!(exposure.contains("OBL-LANDSURFACEENERGY-C-020"));
    assert!(exposure.contains("| `maps-to-existing-INV` |"));
    assert!(exposure.contains("new IDs `INV-164/C-020`"));
    assert_lse_registry_lifecycle(&read(INDEX));
}

#[test]
fn revision_31_component_temperature_dependency_replay_structural_seam_is_expected_red() {
    let solve = read(COMPONENT_DEPENDENCY_REPLAY_PRODUCTION_PATHS[0]);
    let required = [
        (&solve, "struct CoveredComponentTemperatureDependencyGraph"),
        (&solve, "struct ValidatedCoveredComponentReplaySweepBase"),
        (&solve, "struct ValidatedCoveredComponentProbeReplay"),
        (&solve, "struct CoveredComponentDependencyReplayAudit"),
        (&solve, "fn covered_component_temperature_probe_residuals"),
        (&solve, "fn begin_covered_component_dependency_replay_audit"),
        (&solve, "fn take_covered_component_dependency_replay_audit"),
    ];
    let missing = required
        .iter()
        .filter_map(|(source, marker)| {
            (!rust_item_is_unconditional_top_level(source, marker)).then_some(*marker)
        })
        .collect::<Vec<_>>();

    assert!(
        missing.is_empty(),
        "revision 31 expected-red: production dependency-replay seam is absent: {missing:?}"
    );
}

#[test]
fn version_twenty_three_binds_exact_surface_order_to_authenticated_topology() {
    let contract = read(CONTRACT);
    for required in [
        "## Topology-Ranked V16 Exact-Surface Owner Amendment",
        "INV-LANDSURFACEENERGY-158",
        "OBL-LANDSURFACEENERGY-C-013",
        "LSE-V16-TOPOLOGY-RANKED-EXACT-OWNER",
        "surface_key.ofe_id",
        "ofe-9 -> ofe-10",
        "OfeId` remains an opaque identity",
        "context-free\nvalidator therefore proves schema",
        "does not impose lexical OFE order",
        "authenticated frozen-\nparent/configuration join is the canonical-order authority",
        "Duplicate, omitted,\nsubstituted, foreign, topology-relative reordered",
        "stale configuration/digest",
        "complete rollback",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
    assert!(
        read("docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md")
            .contains("INV-SURFACELIQUID-030")
    );
    assert_lse_registry_lifecycle(&read(INDEX));
}

#[test]
fn version_twenty_one_binds_exact_litter_phase_capacity_spill() {
    let contract = read(CONTRACT);
    for required in [
        "## Exact V3 Litter-Phase Capacity-Spill Amendment",
        "W_raw = W_l,* - m_freeze + m_melt",
        "m_spill,tile = W_raw - W_l,max",
        "h_spill       = C_w*(T_raw-T_ref)",
        "U_retained    = U_raw - Q_spill,tile",
        "m_spill,ofe=f_t*m_spill,tile",
        "same transaction and full accepted child",
        "one named negative\n`LitterPhaseCapacitySpillEnergy` operand",
        "does not invoke a second vapor/phase evaluation",
        "preserves the complete LSE,\nsurface-liquid, exact-enthalpy, soil, WB14 parent",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }

    let liquid_capacity = 6.0_f64;
    let raw_liquid = 6.125_f64;
    let raw_temperature = 276.0_f64;
    let water_heat_capacity = 4218.0_f64;
    let spill = raw_liquid - liquid_capacity;
    let specific_enthalpy = water_heat_capacity * (raw_temperature - 273.15);
    let spill_energy = spill * specific_enthalpy;
    let retained_liquid = raw_liquid - spill;
    let raw_energy = 72_500.0_f64;
    let retained_energy = raw_energy - spill_energy;
    assert_eq!(retained_liquid.to_bits(), liquid_capacity.to_bits());
    assert_eq!((retained_liquid + spill).to_bits(), raw_liquid.to_bits());
    assert_eq!(
        (retained_energy + spill_energy).to_bits(),
        raw_energy.to_bits()
    );
    assert!(spill > 0.0 && spill_energy > 0.0);
}

#[test]
fn version_twenty_two_binds_exact_heterogeneous_surface_resource_join() {
    let contract = read(CONTRACT);
    for required in [
        "## Exact Heterogeneous V3 Surface-Resource Join Amendment",
        "Native litter vapor rows already consumed by the phase receipt",
        "The complete unmatched surface set is the ordinary set",
        "apply the existing checked `F/f_t` debit exactly once",
        "Zero ordinary rows are the identity join",
        "retains native litter ice,\nsurface enthalpy high mirrors and exact carry/receipt bytes",
        "spill remains the separate internal\n`LitterPhaseOverflow` ingress parcel",
        "supplies no new parcel and no second enthalpy, latent,\nfusion, or exact-surface operand",
        "Every finalized row is accounted exactly once",
        "one resource candidate and ingress",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }

    let phase_adjusted_liquid = 2.5_f64;
    let tile_fraction = 0.4_f64;
    let ordinary_finalized_ofe = 0.2_f64;
    let ordinary_tile_debit = ordinary_finalized_ofe / tile_fraction;
    let ending = phase_adjusted_liquid - ordinary_tile_debit;
    assert_eq!(ordinary_tile_debit.to_bits(), 0.5_f64.to_bits());
    assert_eq!(ending.to_bits(), 2.0_f64.to_bits());
    assert_eq!(
        (ending + ordinary_tile_debit).to_bits(),
        phase_adjusted_liquid.to_bits()
    );
}

#[test]
fn version_fourteen_binds_frozen_litter_phase_vapor_and_atomic_chronology() {
    let contract = read(CONTRACT);
    for required in [
        "OPENWEPP_SNOW_FREE_LSE_V3",
        "OPENWEPP_FOREST_LITTER_PHASE_RECEIPT_V1",
        "phase-free: no freeze, melt",
        "q_sat_liquid(T_l,p)",
        "Q_v,l = v_l*[C_w*(T_l-T_ref) + L_v(T_l)]",
        "Q_v,i = v_i*[C_i*(T_l-T_ref) + L_s(T_l)]",
        "tau_ice  = 3300 s",
        "L_f      = 333700 J kg^-1",
        "W_i,max  = 0.85*rho_w*dz_l kg m^-2 tile-ground",
        "m_phase  = m_freeze - m_melt",
        "W_l,phase = W_l,* - m_freeze + m_melt",
        "W_i,phase = W_i,* + m_freeze - m_melt",
        "U_phase   = U_* + L_f*m_phase",
        "T_phase   = T_ref + U_phase/C_phase",
        "`H_phase = U - L_f*W_i` is invariant exactly",
        "current precipitation,\nrunon, throughfall, canopy drainage, stemflow, and litter overflow",
        "existing WB14 partition with liquid-only\navailability",
        "same-support flux, fixed-point, water-authorization, or\nNewton re-solve",
        "SC-EVAP-001 remains the owner of daily WB17",
        "exact\n`60000000000 ns` physical fallback floor",
        "steps substantially larger than 60 seconds",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }

    for invariant in 140..=149 {
        let id = format!("INV-LANDSURFACEENERGY-{invariant}");
        assert!(
            contract.contains(&id),
            "{CONTRACT} missing successor invariant {id}"
        );
    }

    for (source, digest) in [
        (
            "gmd-10-1621-2017-isba-meb-litter.pdf",
            "2a8c14d912651457bf9205a4a963b78dd12f1aa7f243bccb025e4b81ce99716d",
        ),
        (
            "isba_meb.F90.source.html",
            "0a300739b5dc660b61d29db144dd92f886e8fdf9934eac8facc022585992087a",
        ),
        (
            "isba_fluxes_meb.F90.source.html",
            "e0378bc89ee0d52cffe14841aac56de1d8d379edf18ad29f24cfdb9ea0dfdbbc",
        ),
        (
            "ini_csts.F90.source.html",
            "f39840df4d851efc70044f9e3ad62822371ed743c3c3a8055a4c940e2f86d73a",
        ),
    ] {
        assert!(contract.contains(source), "{CONTRACT} missing {source}");
        assert!(contract.contains(digest), "{CONTRACT} missing {digest}");
    }

    for refusal in [
        "wrong A4 sign",
        "`273.16 K`",
        "`rho_i=917`",
        "saturation over ice",
        "instant\nequilibrium",
        "ice-as-WB14 supply",
        "`zertol` ice deletion",
        "`xwgmin` regularization",
        "soil\ncompensation",
        "producer-residual closure",
    ] {
        assert!(contract.contains(refusal), "{CONTRACT} missing {refusal}");
    }
}

#[test]
fn version_fourteen_requires_successor_production_identity_and_typed_guards() {
    let production = read_existing(LSE_V3_PRODUCTION_PATHS);
    for required in [
        "OPENWEPP_SNOW_FREE_LSE_V3",
        "OPENWEPP_FOREST_LITTER_PHASE_RECEIPT_V1",
        "LSEB-E-045",
        "LSEB-E-046",
        "LSEB-E-047",
        "LSEB-E-048",
    ] {
        assert!(
            production.contains(required),
            "unchanged production is missing required V3 binding {required}"
        );
    }
}

#[test]
fn version_fourteen_requires_p61_and_native_real_consumer_adoption() {
    for path in LSE_V3_REAL_CONSUMERS {
        let consumer = read(path);
        for required in [
            "OPENWEPP_SNOW_FREE_LSE_V3",
            "OPENWEPP_FOREST_LITTER_PHASE_RECEIPT_V1",
        ] {
            assert!(
                consumer.contains(required),
                "unchanged real consumer {path} is missing V3 binding {required}"
            );
        }
    }
}

#[test]
fn version_fifteen_binds_receiver_owned_exact_soil_enthalpy_carry() {
    let contract = read(CONTRACT);
    for required in [
        "INV-LANDSURFACEENERGY-150",
        "E_k = exact(H_hi,k) + R_k",
        "ExactDyadicEnthalpy",
        "value = sign * coefficient * 2^exponent2 J m^-2 OFE-ground",
        "Zero is uniquely `(0,\"0\",0)`",
        "no\nnew high-term canonical-zero rule",
        "positive odd coefficient",
        "round-to-nearest,\n   ties-to-even",
        "E_candidate,k=E_begin,k+sum(Q_soil,k)+sum(Q_top,k)+sum(Q_inf,k)",
        "SoilThermalOwnerEnvelopeV2",
        "SoilThermalEnergyCreditReceiptV2",
        "SoilThermalOwnerRestartV2",
        "SoilThermalOwnerCheckpointV2",
        "Production downgrade is always\nrejected",
        "-34315.42154113602 J m^-2",
        "-8.0670339832330148e-19 J m^-2",
        "1.10875e-7` ULP",
        "(sign=-1,coefficient_hex=\"1dc319224e55f\",exponent2=-109)",
        "exact-halfway even-low and odd-low ties",
        "minimum positive/negative subnormal operands",
        "largest-finite rounding boundary",
        "receipt omission/duplication/reorder/\nsubstitution",
        "wrong schema/definition/parent/\nconfiguration/state/version/owner/transaction/predecessor/support/OFE/layer/",
        "Every poison proves byte-exact rollback",
        "Restart tests split\nbefore and after a nonzero credit",
        "canonical WAT5 transaction plus\nunchanged `p61` and native-forest successor consumers",
        "`nextafter`, forced-ULP",
        "exact `60000000000 ns` fallback floor are unchanged",
        "LSEB-E-049",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }

    let wat5_credit = -8.067_033_983_233_015e-19_f64;
    assert_eq!(
        wat5_credit.to_bits(),
        (-8.067_033_983_233_015e-19_f64).to_bits()
    );
    assert_eq!(wat5_credit.to_bits() & (1_u64 << 63), 1_u64 << 63);
    assert_eq!(f64::from_bits(1).to_bits(), 1);
    assert_eq!(
        f64::from_bits((1_u64 << 63) | 1).to_bits(),
        (1_u64 << 63) | 1
    );
    assert_eq!(wat5_credit.to_bits(), 0xbc2d_c319_224e_55f0);

    assert_lse_registry_lifecycle(&read(INDEX));
}

#[test]
fn version_fifteen_requires_exact_carry_production_identity() {
    let production = read_existing(EXACT_CARRY_PRODUCTION_PATHS);
    for required in [
        "pub struct ExactDyadicEnthalpy",
        "SoilThermalOwnerEnvelopeV2",
        "pub struct SoilThermalEnergyCreditReceiptV2",
        "pub struct SoilThermalOwnerRestartV2",
        "pub struct SoilThermalOwnerCheckpointV2",
        "LSEB-E-049",
    ] {
        assert!(
            production.contains(required),
            "unchanged production is missing required v15 exact-carry binding {required}"
        );
    }
}

#[test]
fn version_sixteen_binds_exact_lse_surface_enthalpy_carry() {
    let contract = read(CONTRACT);
    for required in [
        "## Version 16 LSE Surface-Enthalpy Exact-Carry Amendment",
        "INV-LANDSURFACEENERGY-151",
        "U = exact(U_hi) + R_U",
        "LseSurfaceEnthalpyOwnerEnvelopeV1",
        "LseSurfaceEnthalpyEnergyCreditReceiptV1",
        "LseSurfaceEnthalpyOwnerRestartV1",
        "LseSurfaceEnthalpyOwnerCheckpointV1",
        "SurfaceLiquidCompleteOwnerProjectionV4",
        "nonauthoritative high mirror",
        "retained_ingress_tile_credit",
        "U_candidate=U_begin+sum(Q_surface,j)",
        "binary64 nearest-even",
        "R_U,candidate=U_candidate-exact(U_hi,candidate)",
        "176400000000000..178200000000000 ns",
        "did not preserve the exact\nbeginning high bits or retained tile-credit operands",
        "exact `60000000000 ns` fallback floor remains unchanged",
        "LSEB-E-050",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }

    assert_lse_registry_lifecycle(&read(INDEX));
}

#[test]
fn version_sixteen_requires_exact_surface_owner_receipt_and_projection_symbols() {
    let production = read_existing(&[
        "crates/openwepp-land-surface-energy/src/lib.rs",
        "crates/openwepp-land-surface-energy/src/exact_dyadic_enthalpy.rs",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner/v3_exact_enthalpy.rs",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner/v4_projection.rs",
        "crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs",
        "crates/openwepp-persisted-restart-v1/src/lib.rs",
    ]);
    for required in [
        "pub struct LseSurfaceEnthalpyOwnerEnvelopeV1",
        "pub struct LseSurfaceEnthalpyEnergyCreditReceiptV1",
        "pub struct LseSurfaceEnthalpyOwnerRestartV1",
        "pub struct LseSurfaceEnthalpyOwnerCheckpointV1",
        "SurfaceLiquidCompleteOwnerProjectionV4",
        "LSEB-E-050",
    ] {
        assert!(
            production.contains(required),
            "unchanged production is missing required v16 exact-surface binding {required}"
        );
    }
}

#[test]
fn version_eleven_binds_inactive_liquid_vapor_coordinates_without_physical_interference() {
    let contract = read(CONTRACT);
    for required in [
        "INV-LANDSURFACEENERGY-131",
        "T_inactive_liquid_vapor - max(T_canopy, T_ref) = 0",
        "zero-area component contributes no physical",
        "Active components,\nphysical residual equations, tolerances, ledgers, receipts, events, the exact\n60-second raw fallback",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn version_twelve_binds_exact_closed_bound_derivatives_without_numeric_fallbacks() {
    let contract = read(CONTRACT);
    for required in [
        "INV-LANDSURFACEENERGY-138",
        "For every covered-column authority and for both owner-uncapped potential and\nfixed-authorization final solves",
        "J[:,i] = (R(x + delta_i e_i) - R(x - delta_i e_i)) / (2 delta_i)",
        "lower bound: J[:,i] = (R(x + delta_i e_i) - R(x)) / delta_i",
        "upper bound: J[:,i] = (R(x) - R(x - delta_i e_i)) / delta_i",
        "it does not shrink `delta_i`, clamp a probe, infer a\nderivative, or continue",
        "diagonal coordinate scaling admitted by `INV-LANDSURFACEENERGY-112` remains\nexclusive",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }

    let index = read(INDEX);
    let index_row = row(&index, "SC-LANDSURFACEENERGY-001");
    assert!(index_row.starts_with(
        "| `SC-LANDSURFACEENERGY-001` | Land-Surface Energy-Balance Process Contract | `approved` | `active` |"
    ));
    assert_lse_registry_lifecycle(&index);
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
