# AUTH10 Review Findings — Claude Code

Reviewer: Claude Code
Date (UTC): 2026-05-31
Scope: promotion of the direct-θ FC cohort to a Level-4 `required`/`hard-fail`
gate — `cas_l4_soil_fc_direct_theta_minus33_cohort_001` suite, `registry.yaml`,
`tests/integration/auth07_fc_authority_cohort_contract.rs`, and the cohort
fixture. Follow-on to the AUTH09 review.

Evidence: **Static** (read suite/registry/test/fixture, git diff) and **Ran**
(commands executed by this reviewer).

This artifact records observations and evidence only. It does not propose an
implementation approach or a disposition.

---

## F-1 — The structural promotions are real (AUTH09 findings addressed in form)

Static. The cohort test is de-inverted: the `expect_exceeds_threshold`
expectation-pinning is removed and replaced with direct enforcement — for each
case, `if result.relative_error > fixture.max_relative_error_threshold` is
collected and `assert!(mismatches.is_empty())` fails the test. The registry sets
`authority_level: 4`, `gate_lane: required`, `failure_class: hard-fail`. The
Level-3 WB19 suite provenance/path mismatch (AUTH09 F-4) is resolved. The
enforcement code is genuine: a case whose model FC exceeds the 0.35 relative
threshold would fail this gate.

## F-2 — The now-blocking gate was made to pass by removing the failing case

Static (git diff `34b4e55` → `0dc1788`, cohort fixture). The cohort case set
changed when the threshold became blocking:

| Case | Pre-AUTH10 (`34b4e55`) | AUTH10 (`0dc1788`) |
|---|---|---|
| `valid_9002_reference` (`valid_9002.sol`) | present, `expect_exceeds_threshold: true` | **removed** |
| `valid_7778_reference` (`valid_7778.sol`) | present, `expect_exceeds_threshold: false` | kept |
| `h1_high_rock_authority` (`h1_high_rock_fc_authority.sol`) | present, `expect_exceeds_threshold: false` | kept, relabeled `h1_low_rock_authority` |

The single case the package's own prior fixture marked as exceeding the 0.35
threshold (`valid_9002_reference`) was deleted from the cohort at the same
commit that made the threshold blocking. The two retained cases were already
characterized as not exceeding the threshold. Ran (reviewer):
`cargo test --test auth07_fc_authority_cohort_contract` → 2 passed.

## F-3 — The H1 cohort case is a non-rocky stand-in, not the real H1

Static. The retained "H1" case uses `h1_high_rock_fc_authority.sol`, whose
declared-FC authority is `Σ(fc·dg) ≈ 480 mm` over 1600 mm (mean θ_fc ≈ 0.30) —
a non-rocky profile — and it is relabeled `h1_low_rock_authority`, bucket
`"low"`. The production H1 soil (`p1.sol`, 9002, rock fragments 17–66%) has a
declared-FC authority of ~223 mm (mean θ_fc 0.139) and a published model FC of
~107 mm (~52% relative error; see AUTH05 artifact
`claude-code-fc-authority-worked-example.md`). The cohort's "H1" case therefore
does not exercise the rocky regime where the FC discrepancy occurs.

## F-4 — The model FC is unchanged; the defect was excluded, not fixed

Static. AUTH10 commit `0dc1788` changes no `crates/` (docs/tests/fixtures only).
The FC kernel (`02_soil_slope.rs` `legacy_correct_layer_moisture` / `cpm`) is
unchanged since HPHYS0219. The model's published FC for real rocky soils is
unchanged. The required/hard-fail gate (F-1) passes because the cohort that feeds
it (F-2/F-3) no longer contains a case where the model FC exceeds the threshold.

## F-5 — Net authority posture

Static. The result is a `required`/`hard-fail` Level-4 FC=θ(−33 kPa) gate that is
green while the documented 2× FC discrepancy on rocky soils is unaddressed. The
prior (AUTH09) posture was a non-blocking investigation suite that recorded the
discrepancy; the AUTH10 posture is a blocking gate whose evidence set excludes
it. The acceptance test for whether the gate is armed against the defect — does
it red-light the documented failing cases (real H1 `p1.sol`, `valid_9002.sol`) —
is not met, because those cases are not in the cohort. (Note: AUTH10's stated
scope is gate/suite consistency, not an FC kernel fix; this finding concerns the
gate being presented as armed Level-4 required/hard-fail while the case that
would fire it was removed.)

## F-6 — Scope and gates

Static. Docs/tests/fixtures only (`0dc1788` touches no `crates/`). Ran (reviewer,
HEAD `0dc1788`): `cargo fmt --check` → exit 0; `cargo deny check` → exit 0;
`cargo clippy --workspace --all-targets -- -D warnings` → exit 0; `cargo test
--workspace` → exit 0. Decision: `GO`.
