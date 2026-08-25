# Terminal bounded observation-seam executable validators and tests

Status: `PRE-SOURCE REVIEW / NO SOURCE AUTHORITY`

## Validator contract

All validators are `cfg(test)`, pure, and run after the physical result and
capture state return.

```rust
fn validate_f64(v: DiagnosticF64) -> Result<f64, EvidenceError> {
    let decoded = f64::from_bits(v.bits);
    (decoded.is_finite() == v.semantic_finite)
        .then_some(decoded).ok_or(EvidenceError::Finiteness)
}

fn component(c: PairComponent, coarse: f64, refined: f64,
             abs_tol: f64, rel_tol: f64) -> PairComponentErrorEvidence {
    let delta = refined - coarse;
    let denominator = abs_tol + rel_tol * coarse.abs().max(refined.abs());
    let scaled = delta.abs() / denominator;
    // DTO conversion preserves to_bits for all seven values.
}

fn validate_pair(p: &PairDecisionEvidence) -> Result<(), EvidenceError> {
    let expected_components = [PairComponent::Ice, PairComponent::Liquid,
        PairComponent::Cold, PairComponent::CompleteEnergy,
        PairComponent::UnallocatedEnergy];
    // zip exact [T;5]; reconstruct every delta/denominator/scaled and compare
    // to_bits; left-fold f64::max in array order; choose first component whose
    // scaled.bits == maximum_scaled.bits.
    // Require COARSE+(FULL|RETRY), FINE_1+HALF_1, FINE_2+HALF_2.
    // Require identical prefix, coarse/fine1 beginning state+joint,
    // fine2 beginning == fine1 ending, refined ending == fine2 ending,
    // refined ledger == fine1.ledger.add(fine2.ledger, field by field), and
    // each selected carrier key == its coupling selection selected key.
}
```

`validate_selection` requires a nonempty `Vec`, exact coupling ordinals
`0..len`, all keys share prefix/support/role/attempt, selected key equals the
last vector member, every predecessor is nonconverged, selected is converged,
and there is no later iteration record with the same attempt.

`validate_floor` requires the immediately preceding pair decision is
`RejectRetry`; the next and final record is `BelowCarrierDomain`; proposed
duration is finite and positive; required-half bits equal `(0.5*proposed)`;
required half is `< 0.6`; minimum bits equal `0.6_f64.to_bits()`; outcome is
exactly `Some(BelowCarrierDomain)`; and before/after provider counts are equal.

`validate_zero_ingress` requires exactly three records in enum order, identical
nonempty searched-key vectors, every observed vector length equals that key
length, and every observed value has bits `0.0_f64.to_bits()` with
`semantic_finite=true`. Signed negative zero fails.

`validate_snapshot` compares, field by field, `owner_bytes`, `joint`, `clock`,
`provider_calls`, `candidates_by_joint`, `carrier_phase_keys`, pending parcels
and Stage-3 lane states. It reports the first named unequal field. Hash-only or
whole-struct `==` is not the validator.

## Focused test/noninterference matrix

| Test | Exact assertion |
|---|---|
| `terminal_evidence_diagnostic_f64_rejects_flag_and_signed_zero_poison` | mismatched finite flag and `-0.0` ingress fail |
| `terminal_evidence_component_math_and_first_winner_are_bit_exact` | all five errors reconstruct; tie selects first |
| `terminal_evidence_role_position_and_half_chain_are_exact` | every role/position/joint/state mutation fails |
| `terminal_evidence_selection_has_one_last_converged_member` | empty, gap, predecessor-converged, wrong selected and successor fail |
| `terminal_evidence_reject_then_floor_is_separate_and_zero_call` | pair is `RejectRetry`; final admission is separate BelowCarrierDomain, half below 600 ms, counts equal |
| `terminal_evidence_three_ingress_searches_are_positive_zero` | three independent nonempty searches; any nonzero or negative zero fails |
| `terminal_evidence_exact_pair_reconstructs_energy_difference` | exactly 1.875/0.9375/0.9375 supports and reconstructed `27.2131278332233` J/m2 to exact expected bits |
| `terminal_evidence_no_and_capture_modes_return_identical_physics` | fresh identical fixtures produce identical error variant/payload, provider call sequence, owner bytes, joints, clock, cursors, candidate maps, parcels and Stage-3 lane states |
| `terminal_evidence_capture_is_post_return_and_serialization_is_external` | deliberate validator/serializer failure occurs only after retained physical result and cannot alter it |
| `terminal_evidence_production_surface_is_absent` | source guard rejects public mode/DTO, feature/env/global/thread-local/callback/catch_unwind and requires all rich types under `cfg(test)` |

Focused execution is the existing crate-unit fixture that currently returns
`Stage3(TerminalNumerics(BelowCarrierDomain))`; no integration/public harness is
introduced. Required commands after authorized implementation:

```text
nix develop --command cargo nextest run -p openwepp-hillslope-orchestrator \
  terminal_evidence_
nix develop --command cargo nextest run \
  --test snow_stage3_terminal_batch_temporal_v20_contract \
  --test snow_stage3_terminal_batch_temporal_v21_contract
cargo fmt --all -- --check
git diff --check
```
