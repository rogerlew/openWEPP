use super::*;
use crate::DirectSnowLayerState;

fn reseal(state: &mut DirectSnowStage3PersistentState) {
    state.fingerprint = Wb11HydrologyKernel::stage3_persistent_state_fingerprint(state);
}

fn literal_abs_rel_oracle(left: f64, right: f64, absolute: f64, relative: f64) -> bool {
    left.is_finite()
        && right.is_finite()
        && (left - right).abs() <= absolute + relative * left.abs().max(right.abs())
}

fn representable_literal_boundary(
    base: f64,
    direction: f64,
    absolute: f64,
    relative: f64,
) -> (f64, f64) {
    assert!(base > 0.0 && direction.abs() == 1.0);
    let mut span = 2.0 * (absolute + relative * base.abs());
    let mut outside = base + direction * span;
    while literal_abs_rel_oracle(base, outside, absolute, relative) {
        span *= 2.0;
        outside = base + direction * span;
    }

    let base_bits = base.to_bits();
    let outside_bits = outside.to_bits();
    let (mut inside_bits, mut rejected_bits) = if direction > 0.0 {
        (base_bits, outside_bits)
    } else {
        (outside_bits, base_bits)
    };
    while rejected_bits - inside_bits > 1 {
        let midpoint = inside_bits + (rejected_bits - inside_bits) / 2;
        if literal_abs_rel_oracle(base, f64::from_bits(midpoint), absolute, relative) {
            if direction > 0.0 {
                inside_bits = midpoint;
            } else {
                rejected_bits = midpoint;
            }
        } else if direction > 0.0 {
            rejected_bits = midpoint;
        } else {
            inside_bits = midpoint;
        }
    }
    let (inside, rejected) = if direction > 0.0 {
        (f64::from_bits(inside_bits), f64::from_bits(rejected_bits))
    } else {
        (f64::from_bits(rejected_bits), f64::from_bits(inside_bits))
    };
    assert!(literal_abs_rel_oracle(base, inside, absolute, relative));
    assert!(!literal_abs_rel_oracle(base, rejected, absolute, relative));
    (inside, rejected)
}

#[test]
fn production_convergence_does_not_compare_per_layer_density_across_maps() {
    struct EndpointProbe(CanonicalCoveredPhysicalEndpointV1);

    impl std::ops::Deref for EndpointProbe {
        type Target = CanonicalCoveredPhysicalEndpointV1;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let previous_state = authentic_stage3_owner(20.0, 0.0, 20.0, 0.0);
    let mut current_state = previous_state.clone();
    current_state.layers[0].density_kg_m3 = 275.0;
    reseal(&mut current_state);
    let density_model = crate::SnowDensityModel::PhysicsBulkDensityCompactionV1;
    let proposal = canonical_covered_lane_trial_v1(7, &current_state, density_model)
        .expect("current aggregate Stage-3 proposal");
    let coordinates = [
        proposal.surface_temperature_c + 273.15,
        271.0,
        0.0,
        0.0,
        proposal.ice_kg_m2 + proposal.liquid_kg_m2,
        proposal.cold_content_j_m2,
        proposal.snow_density_kg_m3,
        proposal.snow_depth_m,
        0.005,
    ];
    let endpoint = |state| {
        EndpointProbe(CanonicalCoveredPhysicalEndpointV1 {
            role: CanonicalCoveredMapRoleV1::Initial,
            ending_stage3: BTreeMap::from([(7, state)]),
            density_model_by_lane: BTreeMap::from([(7, density_model)]),
            lane_coordinates: BTreeMap::from([(7, coordinates)]),
            diagnostics: BTreeMap::new(),
        })
    };
    let previous = endpoint(previous_state.clone());
    let current = endpoint(current_state.clone());
    let convergence = canonical_covered_production_converged_v1(
        &previous,
        &current,
        &BTreeMap::from([(7, proposal.clone())]),
        CanonicalCoveredToleranceScaleV1::Nominal,
    )
    .expect("production convergence predicate");
    assert!(
        convergence.outer_coordinates,
        "per-layer density is not a previous-map/current-map tolerance coordinate",
    );
    assert!(convergence.dependent_carriers);

    let mut dependent_coordinates = coordinates;
    dependent_coordinates[1] = f64::from_bits(dependent_coordinates[1].to_bits() + 1_000_000);
    let previous_with_unstable_dependent = EndpointProbe(CanonicalCoveredPhysicalEndpointV1 {
        role: CanonicalCoveredMapRoleV1::Initial,
        ending_stage3: BTreeMap::from([(7, previous_state)]),
        density_model_by_lane: BTreeMap::from([(7, density_model)]),
        lane_coordinates: BTreeMap::from([(7, dependent_coordinates)]),
        diagnostics: BTreeMap::new(),
    });
    let convergence = canonical_covered_production_converged_v1(
        &previous_with_unstable_dependent,
        &current,
        &BTreeMap::from([(7, proposal.clone())]),
        CanonicalCoveredToleranceScaleV1::Nominal,
    )
    .expect("outer/dependent convergence split");
    assert!(convergence.outer_coordinates);
    assert!(
        !convergence.dependent_carriers,
        "dependent carrier stability must be adjudicated before the pending map's final disposition",
    );

    let mut wrong_chronology = current_state;
    wrong_chronology.layers[0].settle_day_count += 1.0;
    reseal(&mut wrong_chronology);
    assert!(
        !canonical_covered_production_converged_v1(
            &previous,
            &endpoint(wrong_chronology),
            &BTreeMap::from([(7, proposal)]),
            CanonicalCoveredToleranceScaleV1::Nominal,
        )
        .expect("production chronology predicate")
        .outer_coordinates,
        "settling chronology remains exact",
    );
}

#[test]
#[allow(clippy::too_many_lines)] // One table exercises every exact and continuous coordinate.
fn production_convergence_compares_fresh_layer_density_and_exact_template_to_proposal() {
    struct EndpointProbe(CanonicalCoveredPhysicalEndpointV1);

    impl std::ops::Deref for EndpointProbe {
        type Target = CanonicalCoveredPhysicalEndpointV1;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let density_model = crate::SnowDensityModel::PhysicsBulkDensityCompactionV1;
    let mapped_state = authentic_stage3_owner(20.0, 0.0, 20.0, 0.0);
    let base_proposal = canonical_covered_lane_trial_v1(7, &mapped_state, density_model)
        .expect("exact Stage-3 proposal");
    let coordinates = [
        base_proposal.surface_temperature_c + 273.15,
        271.0,
        0.0,
        0.0,
        base_proposal.ice_kg_m2 + base_proposal.liquid_kg_m2,
        base_proposal.cold_content_j_m2,
        base_proposal.snow_density_kg_m3,
        base_proposal.snow_depth_m,
        0.005,
    ];
    let endpoint = |state: DirectSnowStage3PersistentState| {
        EndpointProbe(CanonicalCoveredPhysicalEndpointV1 {
            role: CanonicalCoveredMapRoleV1::Initial,
            ending_stage3: BTreeMap::from([(7, state)]),
            density_model_by_lane: BTreeMap::from([(7, density_model)]),
            lane_coordinates: BTreeMap::from([(7, coordinates)]),
            diagnostics: BTreeMap::new(),
        })
    };
    let previous = endpoint(mapped_state.clone());
    let current = endpoint(mapped_state.clone());
    let density = mapped_state.layers[0].density_kg_m3;
    let (at_tolerance, above_tolerance) = representable_literal_boundary(density, 1.0, 1.0e-6, 0.0);
    let below_tolerance = f64::from_bits((density.to_bits() + at_tolerance.to_bits()) / 2);
    for (candidate, expected) in [
        (below_tolerance, true),
        (at_tolerance, true),
        (above_tolerance, false),
    ] {
        let mut proposal = base_proposal.clone();
        proposal.layer_density_kg_m3[0] = candidate;
        proposal.represented_layers[0].density_kg_m3 = candidate;
        assert_eq!(
            canonical_covered_production_converged_v1(
                &previous,
                &current,
                &BTreeMap::from([(7, proposal)]),
                CanonicalCoveredToleranceScaleV1::Nominal,
            )
            .expect("fresh density predicate")
            .outer_coordinates,
            expected,
            "per-layer density candidate {candidate:?}",
        );
    }

    let exact_rejects = |mutate: fn(&mut CoveredTerminalLaneTrialStateV2)| {
        let mut proposal = base_proposal.clone();
        mutate(&mut proposal);
        assert!(
            !canonical_covered_production_converged_v1(
                &previous,
                &current,
                &BTreeMap::from([(7, proposal)]),
                CanonicalCoveredToleranceScaleV1::Nominal,
            )
            .expect("exact proposal envelope predicate")
            .outer_coordinates,
        );
    };
    exact_rejects(|proposal| proposal.schema_version = proposal.schema_version.saturating_add(1));
    exact_rejects(|proposal| proposal.next_interval_index += 1);
    exact_rejects(|proposal| proposal.resolved_beginning = !proposal.resolved_beginning);
    exact_rejects(|proposal| {
        proposal.snow_density_model = crate::SnowDensityModel::PhysicsBulkMultilayerDensityV1;
    });
    exact_rejects(|proposal| {
        proposal.layer_settle_day_count[0] =
            f64::from_bits(proposal.layer_settle_day_count[0].to_bits() + 1);
        proposal.represented_layers[0].settle_day_count = proposal.layer_settle_day_count[0];
    });
    let mut continuous_mass = base_proposal.clone();
    continuous_mass.represented_layers[0].mass_swe_m =
        f64::from_bits(continuous_mass.represented_layers[0].mass_swe_m.to_bits() + 1);
    assert!(
        canonical_covered_production_converged_v1(
            &previous,
            &current,
            &BTreeMap::from([(7, continuous_mass)]),
            CanonicalCoveredToleranceScaleV1::Nominal,
        )
        .expect("continuous represented mass remains governed by the aggregate coordinate")
        .outer_coordinates,
        "represented mass is a continuous water coordinate, not an exact envelope field",
    );
    exact_rejects(|proposal| {
        proposal.represented_layers[0].liquid_water_m = f64::MIN_POSITIVE;
    });
    let mut continuous_refrozen_tracer = base_proposal.clone();
    continuous_refrozen_tracer.represented_layers[0].refrozen_liquid_m = f64::MIN_POSITIVE;
    assert!(
        canonical_covered_production_converged_v1(
            &previous,
            &current,
            &BTreeMap::from([(7, continuous_refrozen_tracer)]),
            CanonicalCoveredToleranceScaleV1::Nominal,
        )
        .expect("refrozen tracer is ledger evidence, not a constitutive selector")
        .outer_coordinates,
    );
    exact_rejects(|proposal| {
        proposal
            .represented_layers
            .push(proposal.represented_layers[0]);
    });

    let mut prior_only_poison = mapped_state;
    prior_only_poison.schema_version = prior_only_poison.schema_version.saturating_add(1);
    prior_only_poison.layers[0].settle_day_count =
        f64::from_bits(prior_only_poison.layers[0].settle_day_count.to_bits() + 1);
    reseal(&mut prior_only_poison);
    assert!(
        canonical_covered_production_converged_v1(
            &endpoint(prior_only_poison),
            &current,
            &BTreeMap::from([(7, base_proposal)]),
            CanonicalCoveredToleranceScaleV1::Nominal,
        )
        .expect("prior endpoint anti-substitution")
        .outer_coordinates,
        "prior F(x_(k-1)) exact-envelope differences cannot substitute for x_k",
    );
}

#[test]
fn stable_two_map_eligibility_uses_adjudication_candidate_vs_output_not_prior_equality() {
    struct EndpointProbe(CanonicalCoveredPhysicalEndpointV1);

    impl std::ops::Deref for EndpointProbe {
        type Target = CanonicalCoveredPhysicalEndpointV1;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let density_model = crate::SnowDensityModel::PhysicsBulkDensityCompactionV1;
    let mapped_state = authentic_stage3_owner(20.0, 0.0, 20.0, 0.0);
    let mapped_candidate = canonical_covered_lane_trial_v1(7, &mapped_state, density_model)
        .expect("initial charged candidate");
    let mut candidate = mapped_candidate.clone();
    candidate.cold_content_j_m2 += 1.0;
    let coordinates = [
        mapped_candidate.surface_temperature_c + 273.15,
        271.0,
        0.0,
        0.0,
        mapped_candidate.ice_kg_m2 + mapped_candidate.liquid_kg_m2,
        mapped_candidate.cold_content_j_m2,
        mapped_candidate.snow_density_kg_m3,
        mapped_candidate.snow_depth_m,
        0.005,
    ];
    let endpoint = EndpointProbe(CanonicalCoveredPhysicalEndpointV1 {
        role: CanonicalCoveredMapRoleV1::Initial,
        ending_stage3: BTreeMap::from([(7, mapped_state)]),
        density_model_by_lane: BTreeMap::from([(7, density_model)]),
        lane_coordinates: BTreeMap::from([(7, coordinates)]),
        diagnostics: BTreeMap::new(),
    });
    let convergence = canonical_covered_production_converged_v1(
        &endpoint,
        &endpoint,
        &BTreeMap::from([(7, candidate)]),
        CanonicalCoveredToleranceScaleV1::Nominal,
    )
    .expect("pending-adjudication stable-chronology eligibility");
    assert!(
        !convergence.outer_coordinates,
        "equal prior/current map outputs cannot substitute for candidate-vs-output closure",
    );
}

#[test]
fn canonical_tol_snowenergy_007_crosses_only_at_literal_binary64_boundary() {
    use CanonicalCoveredToleranceClassV1 as C;
    use CanonicalCoveredToleranceScaleV1 as S;

    let vectors = [
        (C::SnowTemperature, 263.0, 1.0e-5, 1.0e-9),
        (C::TopSoilTemperature, 271.0, 1.0e-8, 0.0),
        (C::HeatFlux, 100.0, 1.0e-5, 1.0e-8),
        (C::VaporFlux, 1.0e-5, 1.0e-10, 1.0e-6),
        (C::SnowWater, 20.0, 1.0e-6, 1.0e-9),
        (C::Energy, 1.0e5, 1.0e-6, 1.0e-10),
        (C::Density, 250.0, 1.0e-6, 0.0),
        (C::Thickness, 0.2, 1.0e-9, 1.0e-9),
        (C::SpecificHumidity, 0.01, 1.0e-12, 1.0e-8),
    ];

    for (class, base, absolute, relative) in vectors {
        for direction in [-1.0, 1.0] {
            for (scale, multiplier) in [
                (S::Tightened10x, 0.1),
                (S::Nominal, 1.0),
                (S::Loosened2x, 2.0),
            ] {
                let (boundary, next_outside) = representable_literal_boundary(
                    base,
                    direction,
                    absolute * multiplier,
                    relative * multiplier,
                );
                let midpoint_bits = (base.to_bits() + boundary.to_bits()) / 2;
                let midpoint = f64::from_bits(midpoint_bits);
                assert!(canonical_covered_outer_coordinate_converged_v1(
                    base, midpoint, class, scale
                ));
                assert!(canonical_covered_outer_coordinate_converged_v1(
                    base, boundary, class, scale
                ));
                assert!(!canonical_covered_outer_coordinate_converged_v1(
                    base,
                    next_outside,
                    class,
                    scale
                ));
            }
        }
        assert!(!canonical_covered_outer_coordinate_converged_v1(
            f64::NAN,
            base,
            class,
            S::Nominal
        ));
        assert!(!canonical_covered_outer_coordinate_converged_v1(
            base,
            f64::INFINITY,
            class,
            S::Nominal
        ));
    }

    assert!(!canonical_covered_outer_coordinate_converged_v1(
        271.0,
        271.0 + 0.5e-5,
        C::TopSoilTemperature,
        S::Nominal
    ));
    assert!(canonical_covered_outer_coordinate_converged_v1(
        100.0,
        100.0 + 0.5e-5,
        C::HeatFlux,
        S::Nominal
    ));
}

fn authentic_stage3_owner(
    ice_kg_m2: f64,
    liquid_kg_m2: f64,
    cold_content_j_m2: f64,
    refrozen_kg_m2: f64,
) -> DirectSnowStage3PersistentState {
    let mut owner = Wb11HydrologyKernel::initialize_stage3_persistent_state(7, Vec::new())
        .expect("authentic Stage-3 owner");
    if ice_kg_m2 > 0.0 || liquid_kg_m2 > 0.0 {
        owner.layers.push(
            DirectSnowLayerState::new(
                ice_kg_m2 / 1_000.0,
                (ice_kg_m2 / 250.0).max(1.0e-9),
                250.0,
                1.0,
            )
            .with_stage3_thermal_liquid_state(
                Wb11HydrologyKernel::stage3_temperature_from_cold_content_values(
                    ice_kg_m2 / 1_000.0,
                    cold_content_j_m2,
                ),
                liquid_kg_m2 / 1_000.0,
                cold_content_j_m2,
                refrozen_kg_m2 / 1_000.0,
            ),
        );
    }
    reseal(&mut owner);
    owner
}

fn rust_raw_literal_end(bytes: &[u8], start: usize) -> Option<usize> {
    let mut cursor = if bytes.get(start) == Some(&b'r') {
        start + 1
    } else if bytes.get(start) == Some(&b'b') && bytes.get(start + 1) == Some(&b'r') {
        start + 2
    } else {
        return None;
    };
    let mut hashes = 0_usize;
    while bytes.get(cursor) == Some(&b'#') {
        hashes += 1;
        cursor += 1;
    }
    if bytes.get(cursor) != Some(&b'"') {
        return None;
    }
    cursor += 1;
    while cursor < bytes.len() {
        if bytes[cursor] == b'"'
            && (0..hashes).all(|offset| bytes.get(cursor + 1 + offset) == Some(&b'#'))
        {
            return Some(cursor + 1 + hashes);
        }
        cursor += 1;
    }
    None
}

fn rust_ordinary_string_end(bytes: &[u8], quote: usize) -> Option<usize> {
    if bytes.get(quote) != Some(&b'"') {
        return None;
    }
    let mut cursor = quote + 1;
    let mut escaped = false;
    while cursor < bytes.len() {
        if escaped {
            escaped = false;
        } else if bytes[cursor] == b'\\' {
            escaped = true;
        } else if bytes[cursor] == b'"' {
            return Some(cursor + 1);
        }
        cursor += 1;
    }
    None
}

fn rust_character_literal_end(source: &str, quote: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    if bytes.get(quote) != Some(&b'\'') {
        return None;
    }
    let content = quote + 1;
    if bytes.get(content) == Some(&b'\\') {
        let mut cursor = content + 2;
        if bytes.get(content + 1) == Some(&b'u') && bytes.get(content + 2) == Some(&b'{') {
            cursor = content + 3;
            while cursor < bytes.len() && bytes[cursor] != b'}' {
                cursor += 1;
            }
            cursor += 1;
        } else if bytes.get(content + 1) == Some(&b'x') {
            let high = *bytes.get(content + 2)?;
            let low = *bytes.get(content + 3)?;
            if !high.is_ascii_hexdigit() || !low.is_ascii_hexdigit() {
                return None;
            }
            cursor = content + 4;
        }
        return (bytes.get(cursor) == Some(&b'\'')).then_some(cursor + 1);
    }
    let character = source.get(content..)?.chars().next()?;
    let close = content + character.len_utf8();
    (bytes.get(close) == Some(&b'\'')).then_some(close + 1)
}

fn blank_rust_span(output: &mut Vec<u8>, bytes: &[u8], start: usize, end: usize) {
    output.extend(
        bytes[start..end]
            .iter()
            .map(|byte| if *byte == b'\n' { b'\n' } else { b' ' }),
    );
}

fn rust_without_comments_or_literals(source: &str) -> String {
    let bytes = source.as_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0_usize;
    let mut block_depth = 0_usize;
    let mut in_line_comment = false;
    while index < bytes.len() {
        let byte = bytes[index];
        let next = bytes.get(index + 1).copied();
        if in_line_comment {
            output.push(if byte == b'\n' { b'\n' } else { b' ' });
            if byte == b'\n' {
                in_line_comment = false;
            }
            index += 1;
            continue;
        }
        if block_depth > 0 {
            if byte == b'/' && next == Some(b'*') {
                block_depth += 1;
                output.extend_from_slice(b"  ");
                index += 2;
            } else if byte == b'*' && next == Some(b'/') {
                block_depth -= 1;
                output.extend_from_slice(b"  ");
                index += 2;
            } else {
                output.push(if byte == b'\n' { b'\n' } else { b' ' });
                index += 1;
            }
            continue;
        }
        if byte == b'/' && next == Some(b'/') {
            in_line_comment = true;
            output.extend_from_slice(b"  ");
            index += 2;
            continue;
        }
        if byte == b'/' && next == Some(b'*') {
            block_depth = 1;
            output.extend_from_slice(b"  ");
            index += 2;
            continue;
        }
        if let Some(end) = rust_raw_literal_end(bytes, index) {
            blank_rust_span(&mut output, bytes, index, end);
            index = end;
            continue;
        }
        if byte == b'b' && next == Some(b'"') {
            let end = rust_ordinary_string_end(bytes, index + 1).expect("byte string terminator");
            blank_rust_span(&mut output, bytes, index, end);
            index = end;
            continue;
        }
        if byte == b'"' {
            let end = rust_ordinary_string_end(bytes, index).expect("string terminator");
            blank_rust_span(&mut output, bytes, index, end);
            index = end;
            continue;
        }
        let character_quote = if byte == b'b' && next == Some(b'\'') {
            Some(index + 1)
        } else if byte == b'\'' {
            Some(index)
        } else {
            None
        };
        if let Some(quote) = character_quote {
            if let Some(end) = rust_character_literal_end(source, quote) {
                blank_rust_span(&mut output, bytes, index, end);
                index = end;
                continue;
            }
        }
        output.push(byte);
        index += 1;
    }
    String::from_utf8(output).expect("Rust source remains UTF-8")
}

fn rust_tokens(source: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let bytes = source.as_bytes();
    let mut index = 0_usize;
    while index < bytes.len() {
        if bytes[index].is_ascii_alphabetic() || bytes[index] == b'_' {
            let start = index;
            index += 1;
            while index < bytes.len()
                && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_')
            {
                index += 1;
            }
            tokens.push(source[start..index].to_owned());
        } else if !bytes[index].is_ascii_whitespace() {
            tokens.push(char::from(bytes[index]).to_string());
            index += 1;
        } else {
            index += 1;
        }
    }
    tokens
}
fn rust_function_body(source: &str, signature: &str) -> String {
    let code = rust_without_comments_or_literals(source);
    let signature_start = code
        .find(signature)
        .unwrap_or_else(|| panic!("missing Rust function signature: {signature}"));
    let open = code[signature_start..]
        .find('{')
        .map(|offset| signature_start + offset)
        .expect("function opening brace");
    let mut depth = 0_usize;
    for (offset, byte) in code.as_bytes()[open..].iter().copied().enumerate() {
        if byte == b'{' {
            depth += 1;
        } else if byte == b'}' {
            depth -= 1;
            if depth == 0 {
                return code[open..=open + offset].to_owned();
            }
        }
    }
    panic!("unterminated Rust function body: {signature}");
}

#[derive(Debug, PartialEq, Eq)]
struct DirectControllerCallV1 {
    call_token: usize,
    call_close_parenthesis: usize,
    statement_end: usize,
    binding: String,
}

fn token_depths(tokens: &[String]) -> Vec<(isize, isize, isize)> {
    let mut depths = Vec::with_capacity(tokens.len());
    let (mut braces, mut parentheses, mut brackets) = (0_isize, 0_isize, 0_isize);
    for token in tokens {
        depths.push((braces, parentheses, brackets));
        match token.as_str() {
            "{" => braces += 1,
            "}" => braces -= 1,
            "(" => parentheses += 1,
            ")" => parentheses -= 1,
            "[" => brackets += 1,
            "]" => brackets -= 1,
            _ => {}
        }
    }
    depths
}

fn matching_close_parenthesis(tokens: &[String], open: usize) -> Result<usize, &'static str> {
    if tokens.get(open).map(String::as_str) != Some("(") {
        return Err("canonical call opening parenthesis");
    }
    let mut depth = 0_isize;
    for (offset, token) in tokens[open..].iter().enumerate() {
        if token == "(" {
            depth += 1;
        } else if token == ")" {
            depth -= 1;
            if depth == 0 {
                return Ok(open + offset);
            }
        }
    }
    Err("canonical call closing parenthesis")
}

fn simple_let_binding(prefix: &[String]) -> Result<String, &'static str> {
    if prefix.first().map(String::as_str) != Some("let") {
        return Err("canonical call must be let-bound");
    }
    let equals = prefix
        .iter()
        .position(|token| token == "=")
        .ok_or("canonical let binding equals")?;
    let pattern = &prefix[1..equals];
    let binding = match pattern {
        [binding] => binding,
        [mutable, binding] if mutable == "mut" => binding,
        _ => return Err("only identifier and mut identifier bindings are supported"),
    };
    let mut characters = binding.chars();
    if binding == "_"
        || !characters
            .next()
            .is_some_and(|first| first == '_' || first.is_ascii_alphabetic())
        || !characters.all(|character| character == '_' || character.is_ascii_alphanumeric())
    {
        return Err("canonical result binding identifier");
    }
    Ok(binding.clone())
}

fn exact_return_flow_after_call(tokens: &[String], after: usize, binding: &str) -> bool {
    let suffix = if tokens.last().map(String::as_str) == Some("}") {
        &tokens[after..tokens.len() - 1]
    } else {
        return false;
    };
    let direct = ["return", binding, ";"];
    let ok = ["return", "Ok", "(", binding, ")", ";"];
    let projected = [
        "return",
        "Ok",
        "(",
        binding,
        ".",
        "imported_v10_output",
        ")",
        ";",
    ];
    if suffix.iter().map(String::as_str).eq(direct)
        || suffix.iter().map(String::as_str).eq(ok)
        || suffix.iter().map(String::as_str).eq(projected)
    {
        return true;
    }

    let Some(output) = suffix.get(1).map(String::as_str) else {
        return false;
    };
    if suffix.first().map(String::as_str) != Some("let")
        || output == "_"
        || !output
            .chars()
            .next()
            .is_some_and(|first| first == '_' || first.is_ascii_alphabetic())
        || !output
            .chars()
            .all(|character| character == '_' || character.is_ascii_alphanumeric())
    {
        return false;
    }
    let projection_prefix = ["let", output, "=", binding, ".", "imported_v10_output", ";"];
    if suffix.len() <= projection_prefix.len()
        || !suffix[..projection_prefix.len()]
            .iter()
            .map(String::as_str)
            .eq(projection_prefix)
    {
        return false;
    }
    let returned = &suffix[projection_prefix.len()..];
    let direct_output = ["return", output, ";"];
    let ok_output = ["return", "Ok", "(", output, ")", ";"];
    returned.iter().map(String::as_str).eq(direct_output)
        || returned.iter().map(String::as_str).eq(ok_output)
}
fn direct_controller_call(
    tokens: &[String],
    function_name: &str,
) -> Result<DirectControllerCallV1, &'static str> {
    let calls = tokens
        .windows(2)
        .enumerate()
        .filter_map(|(index, pair)| (pair[0] == function_name && pair[1] == "(").then_some(index))
        .collect::<Vec<_>>();
    if calls.len() != 1 {
        return Err("exactly one structural controller call");
    }
    let call = calls[0];
    let depths = token_depths(tokens);
    if depths[call] != (1, 0, 0) {
        return Err("controller call must be a direct function-body edge");
    }

    let statement_start = tokens[..call]
        .iter()
        .enumerate()
        .filter(|(index, token)| {
            (token.as_str() == ";" && depths[*index] == (1, 0, 0))
                || (token.as_str() == "{" && depths[*index] == (0, 0, 0))
        })
        .next_back()
        .map_or(0, |(index, _)| index + 1);
    let prefix = &tokens[statement_start..call];
    if prefix.iter().any(|token| token == "|" || token == "#") {
        return Err("controller call cannot be closure/cfg guarded");
    }
    let binding = simple_let_binding(prefix)?;
    let equals = prefix
        .iter()
        .position(|token| token == "=")
        .ok_or("controller binding equals")?;
    if statement_start + equals + 1 != call {
        return Err("controller call must be the direct let initializer");
    }

    let close = matching_close_parenthesis(tokens, call + 1)?;
    let statement_end = (close + 1..tokens.len())
        .find(|candidate| tokens[*candidate] == ";" && depths[*candidate] == (1, 0, 0))
        .ok_or("canonical call statement terminator")?;
    let initializer_tail = &tokens[close + 1..statement_end];
    if !initializer_tail.is_empty() && !initializer_tail.iter().map(String::as_str).eq(["?"]) {
        return Err("canonical call initializer permits only direct or question-mark completion");
    }
    if !exact_return_flow_after_call(tokens, statement_end + 1, &binding) {
        return Err("canonical result must influence returned production output");
    }

    Ok(DirectControllerCallV1 {
        call_token: call,
        call_close_parenthesis: close,
        statement_end,
        binding,
    })
}

fn inspect_structural_entry(source: &str) -> Result<DirectControllerCallV1, &'static str> {
    let body = rust_function_body(source, "fn entry(");
    direct_controller_call(&rust_tokens(&body), "canonical_stage3_open_snow_execute_v1")
}

#[test]
fn structural_call_lexer_removes_every_literal_and_comment_decoy() {
    let source = r###"
        fn entry() {
            // canonical_stage3_open_snow_execute_v1();
            /* canonical_stage3_open_snow_execute_v1(); */
            let ordinary = "canonical_stage3_open_snow_execute_v1()";
            let raw = r#"canonical_stage3_open_snow_execute_v1()"#;
            let bytes = b"canonical_stage3_open_snow_execute_v1()";
            let raw_bytes = br##"canonical_stage3_open_snow_execute_v1()"##;
            let character = '(';
            let byte_character = b'(';
            let hex_character = '\x7b';
            let hex_byte_character = b'\x7d';
            consume(ordinary, raw, bytes, raw_bytes, character, byte_character, hex_character, hex_byte_character);
            let result = canonical_stage3_open_snow_execute_v1()?;
            return Ok(result);
        }
    "###;
    let body = rust_function_body(source, "fn entry(");
    let tokens = rust_tokens(&body);
    let call = direct_controller_call(&tokens, "canonical_stage3_open_snow_execute_v1")
        .expect("one direct returned controller call");
    assert_eq!(
        tokens[call.call_token..]
            .iter()
            .filter(|token| token.as_str() == "canonical_stage3_open_snow_execute_v1")
            .count(),
        1,
    );
    assert!(call.call_close_parenthesis < call.statement_end);
    assert_eq!(call.binding, "result");
}

#[test]
#[allow(clippy::needless_raw_string_hashes, clippy::too_many_lines)] // Raw fixtures form one structural anti-evasion table.
fn structural_call_result_flow_rejects_argument_and_dead_use_decoys() {
    let mutable_returned = r#"
        fn entry() {
            let mut result = canonical_stage3_open_snow_execute_v1()?;
            return Ok(result);
        }
    "#;
    assert_eq!(
        inspect_structural_entry(mutable_returned)
            .expect("mut identifier binding with returned result")
            .binding,
        "result",
    );

    let returned = r#"
        fn entry() {
            let result = canonical_stage3_open_snow_execute_v1(nested(argument()))?;
            return Ok(result.imported_v10_output);
        }
    "#;
    let returned_call =
        inspect_structural_entry(returned).expect("result consumed after complete nested call");
    assert!(returned_call.call_close_parenthesis < returned_call.statement_end);

    let exact_return = r#"
        fn entry() {
            let result = canonical_stage3_open_snow_execute_v1()?;
            return result;
        }
    "#;
    assert!(inspect_structural_entry(exact_return).is_ok());

    let exact_ok_return = r#"
        fn entry() {
            let result = canonical_stage3_open_snow_execute_v1()?;
            return Ok(result);
        }
    "#;
    assert!(inspect_structural_entry(exact_ok_return).is_ok());

    let direct_initializer_suffix = r#"
        fn entry() {
            let result = canonical_stage3_open_snow_execute_v1();
            return result;
        }
    "#;
    assert!(inspect_structural_entry(direct_initializer_suffix).is_ok());

    let question_mark_initializer_suffix = r#"
        fn entry() {
            let result = canonical_stage3_open_snow_execute_v1()?;
            return Ok(result);
        }
    "#;
    assert!(inspect_structural_entry(question_mark_initializer_suffix).is_ok());

    let one_projection = r#"
        fn entry() {
            let result = canonical_stage3_open_snow_execute_v1()?;
            let output = result.imported_v10_output;
            return Ok(output);
        }
    "#;
    assert!(inspect_structural_entry(one_projection).is_ok());

    let destructured = r#"
        fn entry() {
            let (result, receipt) = canonical_stage3_open_snow_execute_v1()?;
            return Ok(result);
        }
    "#;
    assert!(inspect_structural_entry(destructured).is_err());

    let argument_only = r#"
        fn entry() {
            let result = canonical_stage3_open_snow_execute_v1(result)?;
            return Ok(other);
        }
    "#;
    assert!(inspect_structural_entry(argument_only).is_err());

    let dead_consumption = r#"
        fn entry() {
            let result = canonical_stage3_open_snow_execute_v1()?;
            if false {
                return Ok(result);
            }
            return Ok(other);
        }
    "#;
    assert!(inspect_structural_entry(dead_consumption).is_err());

    let returned_block_decoy = r#"
        fn entry() {
            let result = canonical_stage3_open_snow_execute_v1()?;
            return { drop(result); Ok(other) };
        }
    "#;
    assert!(inspect_structural_entry(returned_block_decoy).is_err());

    let mapped_block_decoy = r#"
        fn entry() {
            let result = canonical_stage3_open_snow_execute_v1()?;
            let output = { drop(result); other };
            return output;
        }
    "#;
    assert!(inspect_structural_entry(mapped_block_decoy).is_err());

    let arbitrary_mapping_call = r#"
        fn entry() {
            let result = canonical_stage3_open_snow_execute_v1()?;
            let output = ProductionOutput::from(result);
            return Ok(output);
        }
    "#;
    assert!(inspect_structural_entry(arbitrary_mapping_call).is_err());

    let initializer_map_replacement = r#"
        fn entry() {
            let result = canonical_stage3_open_snow_execute_v1().map(|_| other)?;
            return Ok(result);
        }
    "#;
    assert!(inspect_structural_entry(initializer_map_replacement).is_err());

    let initializer_and_then_replacement = r#"
        fn entry() {
            let result = canonical_stage3_open_snow_execute_v1()
                .and_then(|_| Ok(other))?;
            return Ok(result);
        }
    "#;
    assert!(inspect_structural_entry(initializer_and_then_replacement).is_err());

    let initializer_closure_block_replacement = r#"
        fn entry() {
            let result = canonical_stage3_open_snow_execute_v1()
                .map(|canonical| { drop(canonical); other })?;
            return Ok(result);
        }
    "#;
    assert!(inspect_structural_entry(initializer_closure_block_replacement).is_err());

    let two_hops = r#"
        fn entry() {
            let result = canonical_stage3_open_snow_execute_v1()?;
            let output = result.imported_v10_output;
            let final_output = output;
            return Ok(final_output);
        }
    "#;
    assert!(inspect_structural_entry(two_hops).is_err());

    let unused = r#"
        fn entry() {
            let result = canonical_stage3_open_snow_execute_v1()?;
            consume_unrelated(other);
            return Ok(output);
        }
    "#;
    assert!(inspect_structural_entry(unused).is_err());
}

#[test]
fn real_open_snow_source_calls_outer_canonical_execution_without_admission_shim() {
    let source = include_str!("open_snow.rs");
    let body = rust_function_body(source, "fn execute_imported_v10_stack(");
    let tokens = rust_tokens(&body);
    let call = direct_controller_call(&tokens, "canonical_stage3_open_snow_execute_v1")
        .expect("one direct controller call whose result reaches explicit production return");
    assert!(call.call_close_parenthesis < call.statement_end);
    assert!(body.contains("evaluate_stage3"));
    assert!(body.contains("open_snow_boundaries_by_destination"));
    for forbidden_parallel_controller in [
        "canonical_stage3_production_admit_and_finalize_v1(",
        "for iteration in 0..COVERED_FIXED_POINT_POLICY.max_iterations",
        "phase_consistent_coupled_physical_solve_v1(",
        "covered_frozen_temperature_primary_solve_v1(",
        "phase_consistent_coupled_root_polish_v1(",
        "covered_private_q_lattice_witness_v1(",
        "covered_authentic_receipt_stabilize_or_cycle_v1(",
        "covered_stable_monotone_",
    ] {
        assert!(
            !body.contains(forbidden_parallel_controller),
            "real open_snow retains parallel controller `{forbidden_parallel_controller}`",
        );
    }
    for forbidden_caller_assertion in [
        "active_set_scenario:",
        "all_physical_ledgers_close:",
        "all_constitutive_domains_valid:",
        "exact_discrete_custody_matches:",
    ] {
        assert!(
            !body.contains(forbidden_caller_assertion),
            "real open_snow must derive `{forbidden_caller_assertion}` from authentic physics",
        );
    }
}

#[test]
fn canonical_covered_multisecant_refuses_predictor_replay_histories_before_a_map_charge() {
    for (dot, norm, expected_detail) in [
        (0.0, 0.0, "canonical covered degenerate multisecant history"),
        (
            0.0,
            1.0,
            "canonical covered nonadvancing multisecant history",
        ),
        (
            f64::NAN,
            1.0,
            "canonical covered degenerate multisecant history",
        ),
    ] {
        // Alpha selection precedes construction/evaluation of the next
        // authentic map in execute_canonical_covered_production_v1. A typed
        // refusal here therefore consumes no MultisecantAdjudication charge.
        match canonical_covered_multisecant_alpha_v1(dot, norm) {
            Err(DirectV11RealConsumerError::AdaptiveRefinement(detail)) => {
                assert_eq!(detail, expected_detail);
            }
            other => panic!("expected typed multisecant refinement, observed {other:?}"),
        }
    }
    assert_eq!(
        canonical_covered_multisecant_alpha_v1(-2.0, 1.0)
            .expect("positive safeguarded multisecant step")
            .to_bits(),
        1.0_f64.to_bits()
    );
}

#[test]
fn canonical_covered_multisecant_admits_oscillatory_contraction_and_guards_endpoints() {
    assert_eq!(
        canonical_covered_multisecant_alpha_v1(0.75, 2.25).expect("slope -0.5 contraction"),
        -1.0 / 3.0,
    );
    let lower = -0.75_f64;
    let below_lower = f64::from_bits(lower.to_bits() + 1);
    let above_lower = f64::from_bits(lower.to_bits() - 1);
    for raw in [lower, below_lower, -1.0, -2.0, -f64::MAX] {
        assert_eq!(
            canonical_covered_multisecant_alpha_v1(-raw, 1.0)
                .expect("finite negative history uses the canonical lower safeguard")
                .to_bits(),
            lower.to_bits(),
        );
    }
    assert_eq!(
        canonical_covered_multisecant_alpha_v1(-above_lower, 1.0)
            .expect("negative history above the lower safeguard")
            .to_bits(),
        above_lower.to_bits(),
    );
    assert!(canonical_covered_multisecant_alpha_v1(0.0, 1.0).is_err());
    assert!(canonical_covered_multisecant_alpha_v1(f64::INFINITY, 1.0).is_err());
    assert!(canonical_covered_multisecant_alpha_v1(1.0, f64::INFINITY).is_err());

    let previous = 2.0;
    let current = 10.0;
    assert_eq!(
        canonical_covered_multisecant_coordinate_v1(previous, current, lower).to_bits(),
        (0.25 * current + 0.75 * previous).to_bits(),
    );
    assert_ne!(
        canonical_covered_multisecant_coordinate_v1(previous, current, lower).to_bits(),
        previous.to_bits(),
    );
    assert_ne!(
        canonical_covered_multisecant_coordinate_v1(previous, current, lower).to_bits(),
        current.to_bits(),
    );

    let captured_dot = 362_742.313_896_017_56;
    let captured_norm = 325_841.810_884_757_96;
    let model_delta = 2.0 * lower * captured_dot + lower * lower * captured_norm;
    assert!(
        model_delta < 0.0,
        "saturated capture must reduce residual norm"
    );
}

#[test]
fn canonical_covered_multisecant_reconstructs_the_v22_cold_phase_at_snow_reappearance() {
    let proposed_total_water = 1.834_731_610_098_15;
    let foreign_current_ice = 1.835_069_966_575_340_5;
    let (ice, liquid) = canonical_covered_phase_split_v1(
        proposed_total_water,
        6_000.139_049_912_76,
        foreign_current_ice,
    )
    .expect("positive-cold-content V22 branch");
    assert_eq!(ice.to_bits(), proposed_total_water.to_bits());
    assert_eq!(liquid.to_bits(), 0.0_f64.to_bits());

    assert!(canonical_covered_phase_split_v1(1.0, 0.0, 1.0 + f64::EPSILON).is_err());
    let (ice, liquid) = canonical_covered_phase_split_v1(1.25, 0.0, 1.0)
        .expect("zero-cold-content current authentic phase split");
    assert_eq!(ice.to_bits(), 1.0_f64.to_bits());
    assert_eq!(liquid.to_bits(), 0.25_f64.to_bits());
    for poison in [f64::NAN, f64::INFINITY, -1.0] {
        assert!(canonical_covered_phase_split_v1(poison, 1.0, 0.0).is_err());
    }
}

#[test]
fn canonical_covered_stopping_uses_the_charged_candidate_not_prior_map_output() {
    let scale = CanonicalCoveredToleranceScaleV1::Nominal;
    let class = CanonicalCoveredToleranceClassV1::Energy;

    // F_k and F_(k-1) can be identical while the authentic map remains far
    // from its charged candidate x_k. Cross-history stability cannot admit it.
    assert!(!canonical_covered_outer_coordinate_converged_v1(
        10.0, 10.1, class, scale,
    ));

    // Conversely, the authentic map is converged against x_k even when an
    // earlier output was arbitrarily far away.
    assert!(canonical_covered_outer_coordinate_converged_v1(
        10.0,
        10.0 + 0.5e-6,
        class,
        scale,
    ));
}
