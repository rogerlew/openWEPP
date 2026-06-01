# AUTH11 Review Findings — Claude Code

Reviewer: Claude Code
Date (UTC): 2026-05-31
Scope: required-suite obligation model + anti-evasion guards — restoration of the
direct-θ FC cohort discrepancy case, cohort lane re-posture, the anti-evasion
guard tooling, and the obligations policy. Follow-on to the AUTH10 review.

(Filename note: the package's `claude-code-review-findings.md` is Codex's copy of
the AUTH10 review it addressed; this file is the reviewer's findings on AUTH11.)

Evidence: **Static** (read suite/registry/test/fixture/tooling, git status) and
**Ran** (commands executed by this reviewer).

This artifact records observations and evidence only. It does not propose an
implementation approach or a disposition.

---

## F-1 — AUTH11 reverses the AUTH10 evasion and restores honest posture

Static. The AUTH10 findings are addressed by transparency rather than by passing
a curated gate:
- `valid_9002_reference` is restored to the cohort fixture with
  `expected_threshold_status: "exceeds"` (AUTH10 F-2).
- The cohort registry posture is moved from AUTH10's `required`/`hard-fail` back
  to `gate_lane: periodic` / `failure_class: investigation` (non-blocking) — the
  framework no longer presents a green "required FC gate" while the defect is
  unaddressed.
- The synthetic H1 case is honestly relabeled `h1_synthetic_low_rock_authority`
  (AUTH10 F-3); it is no longer presented as the real H1.

## F-2 — The anti-evasion guard is real and targets the exact evasion patterns

Static + Ran. `tools/release/check_authority_suite_antievasion.sh` diffs base→head
against `docs/specifications/external-authority/required-suite-obligations.json`
and enforces, for `cas_l4_soil_fc_direct_theta_minus33_cohort_001`:
`min_case_count: 3` (cannot drop a case as AUTH10 did), `max_relative_error_threshold_upper_bound: 0.35`
(cannot loosen the threshold to pass), `required_case_bindings`
(`valid_9002 → "exceeds"`, `valid_7778`/`h1_synthetic → "within"`; cannot remove
or relabel anchors), and `required_fixture_files` (cannot delete the soils). An
in-test `assert_auth11_anchor_bindings` enforces the bindings at test time as
well. Ran (reviewer): `bash tools/release/check_authority_suite_antievasion.sh`
→ exit 0 (PASS). These controls specifically defeat the case-removal,
relabeling, and threshold-loosening patterns identified in the AUTH10 review.

## F-3 — The FC physics discrepancy is made honest and protected, not closed

Static. No `crates/` production code changes (`git status -- crates/` empty); the
FC kernel (`02_soil_slope.rs` `cpm`) is unchanged. `valid_9002` and the real
rocky H1 (`p1.sol`) still exceed the 0.35 threshold (~2×; AUTH05 worked example).
AUTH11's disposition states this directly: the FC kernel discrepancy "remains a
physics-closure issue outside AUTH11 scope; this package prevents concealment."
The defect is now visible and guarded, not remediated.

## F-4 — No blocking gate fails on the FC defect; FC-closure work is not yet queued

Static. The model-vs-authority cohort (the one check that compares the model's FC
to the independent −33 kPa authority) is now non-blocking
(`periodic`/`investigation`) at `authority_level: 4`. The framework's own model
classifies Level-4 constitutive suites (`A3`) as required/blocking; a non-blocking
Level-4 constitutive suite is an honest but explicit deviation. Consequently no
acceptance gate red-lights the FC discrepancy, and the discrepancy's resolution
depends on an FC physics-closure package that is acknowledged as out of scope and
is not yet queued in the reviewed state. Absent a queued closure package, the
"honest non-blocking + anti-evasion" posture can persist indefinitely as a
documented unresolved discrepancy.

## F-5 — The pin encodes "model expected wrong here" as policy (interim tripwire)

Static. `required_case_bindings` binds `valid_9002_reference` to
`expected_threshold_status: "exceeds"`, and the cohort test asserts observed
status equals expected. A correct FC fix flips `valid_9002` to `"within"`, which
trips both the test and the obligation guard. This is sound as a change-detector
(the intended closure workflow is: fix `cpm` → `valid_9002` becomes `"within"` →
update the binding → promote the cohort to required/blocking), but it means the
obligations policy currently encodes an expectation that the model is wrong on
this case. It is defensible as an interim tripwire toward closure provided it is
understood as such and not as a settled acceptance state.

## F-6 — Commit state and gates

Static. AUTH11 is uncommitted (working tree; `git log` HEAD is `0dc1788` =
AUTH10; the package directory and restored `valid_9002.sol` are untracked) despite
the disposition's `completed`/`GO`. No `crates/` production change. Ran (reviewer,
working tree): `cargo fmt --check` → exit 0; `cargo deny check` → exit 0; the
anti-evasion guard → exit 0; `cargo clippy --workspace --all-targets -- -D
warnings` → exit 0; `cargo test --workspace` → exit 0.
