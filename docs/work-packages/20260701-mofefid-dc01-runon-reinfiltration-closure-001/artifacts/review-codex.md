# Codex Review — MOFEFID-DC01 Runon Re-Infiltration Closure

Review date: 2026-07-01
Reviewer: Codex
Branch/worktree: `worktree-mofefid-dc01` at `61c24f18`

## Evidence Classes

Static:
- Read `package.md`, especially Progress/Surprises/Decision Log.
- Read contract diffs for `SC-RUNOFFPART-001.md` and `SC-WATBAL-001.md`.
- Read the changed runtime surfaces in `runoff.rs`, `03_executor.rs`,
  `04_audit_error_helpers.rs`, `erosion.rs`, runner day-input plumbing, and
  changed tests.
- Searched for DC01/dry-runon/two-OFE coverage and surviving A02/DC01 env
  hooks.

Ran:
- `cargo fmt --check` — pass.
- `cargo deny check` — pass.
- `cargo clippy -p openwepp-hillslope-orchestrator -p openwepp-runner --all-targets -- -D warnings` — fail.
- `cargo test -p openwepp-hillslope-orchestrator dc01 -- --nocapture` — pass, 3 tests.
- `cargo test -p openwepp-hillslope-orchestrator r4j_runon_carry_consumes_dynamic_transfer_arrays_and_feeds_total_runon -- --nocapture` — pass, 1 test.
- `cargo test -p openwepp-hillslope-orchestrator r4k_wb14_producer_feeds_runoff_percolation_and_et_lineage -- --nocapture` — pass, 1 test.
- `cargo test -p openwepp-runner direct_production_manifest_reports_direct_runtime_counters -- --nocapture` — ran 0 tests; no evidence.
- `cargo test -p openwepp-hillslope-orchestrator r4j_runon_carry_consumes_dynamic_transfer_arrays_and_feeds_total_runon r4k_wb14_producer_feeds_runoff_percolation_and_et_lineage -- --nocapture` — invalid invocation; no evidence.

## Findings

### DC01-CX-001 — Rejected: clippy gate is red on new DC01 hour-bin casts

Evidence: Ran.
Files: `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs:549`,
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:654`,
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:1409`,
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:1435`,
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:1436`,
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:1467`,
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:1468`.

The independent clippy command fails under `-D warnings` with seven
`clippy::cast_precision_loss` errors introduced by the DC01 hour-bin math
(`usize as f64` and count-to-`f64` conversions). This contradicts package
acceptance criterion 6 (`fmt/clippy/deny clean`) and the Progress claim that
fmt/clippy were clean. This is a merge blocker even though `fmt` and `deny`
passed independently.

Disposition candidate: accept and fix before merge.

### DC01-CX-002 — Accepted: `erod14_qin_clamped_events` is not reset with the other audit counters

Evidence: Static.
File: `crates/openwepp-hillslope-orchestrator/src/direct_runtime/04_audit_error_helpers.rs:10`.
File: `crates/openwepp-hillslope-orchestrator/src/direct_runtime/04_audit_error_helpers.rs:120`.
File: `crates/openwepp-hillslope-orchestrator/src/direct_runtime/04_audit_error_helpers.rs:164`.

The new clamp counter is included in snapshots and incremented when the
decreasing-flow clamp fires, but `DirectRuntimeAuditCounters::reset()` clears
all other counters and omits `erod14_qin_clamped_events`. The package explicitly
requires a run-level occurrence counter for the held erosion boundary. Any
caller/test using `reset_direct_runtime_audit_counters()` after a clamp event
can carry stale clamp counts into later snapshots. The runner manifest delta
path captures a baseline/current pair, so this is not necessarily a single CLI
run defect, but it is a real audit-counter hygiene defect and should be fixed
with a regression test.

Disposition candidate: accept.

### DC01-CX-003 — Accepted: acceptance criterion 1 is under-tested at the requested constructed two-OFE/dry-runon level

Evidence: Static + Ran search.
File: `docs/work-packages/20260701-mofefid-dc01-runon-reinfiltration-closure-001/package.md:51`.
File: `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs:3069`.
File: `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs:3088`.
File: `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs:3107`.

The package acceptance criterion asks for a constructed two-OFE test proving
downstream WB14 infiltration includes the area-scaled runon share with exact
expected values, and that dry-runon days infiltrate through synthesized
intervals. The added DC01 tests cover helper behavior and a positive-runon
WB14 producer delta, but they do not construct the R4J→R4K two-OFE lane path,
do not assert the exact downstream area-scaled share, and do not cover a
dry-runon receiving lane. Existing MOFE integration tests exercise routing and
publication surfaces, but the DC01 diff did not add the acceptance-level
assertion.

Disposition candidate: accept unless the implementer can point to an existing
test/evidence artifact I missed.

### DC01-CX-004 — Deferred candidate: env-gated DC01 interval-basis selector remains as production behavior

Evidence: Static.
File: `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:1418`.
File: `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:1495`.

`OPENWEPP_DC01_DIAG_INTERVAL_BASIS=1` switches WB14 runon admission from the
default hourly re-binned basis to appended runon intervals while preserving the
local breakpoint basis. This is not a passive trace hook; it changes hydrology.
The package records the M5 decomposition diagnostic, but the selector is not
contracted, not surfaced in the manifest, and not described as a supported
runtime mode. Keeping the selector risks a hidden alternate production path
after the package closes.

Disposition candidate: remove before merge, or explicitly contract and manifest
it as package-bound diagnostic behavior. If intentionally retained only for
review reproduction, defer with a dated removal owner.

## Accepted Checks

- INV-031 text is directionally consistent with the pinned baseline source
  intent: it admits area-scaled upstream surface and lateral carry into WB14
  same-pass supply, preserves the R4A closure identity, keeps zero-upstream
  lanes bit-identical by design, and leaves decreasing-flow erosion under the
  INV-030 hold.
- The shape-channel redesign is mechanically sound: `surface_carry_m[0]`
  retains the exact total, `surface_hourly_weights` is a separate normalized
  shape channel, and the new consumer is the DC01 WB14 supply admission.
- Default single-OFE behavior is structurally guarded: zero runon returns before
  supply admission, and `compute_wb14_infiltration_depression_with_profile`
  keeps the original hyetograph interval basis when the runon total is zero.
- Dry-runon is plausible through the runner because
  `direct_production_hyetograph()` creates a positive-duration zero-intensity
  interval for missing climate time/intensity arrays, so a receiving lane has a
  time base to re-bin once runon is injected. This still needs the acceptance
  test noted above.
- The erosion clamp is hold-consistent in placement: it uses prior-lane erosion
  `qout`, clamps only `qin > qout + WB11_ZERO_THRESHOLD`, then computes the
  downstream EROD14 surfaces from the clamped pair while leaving sediment
  coupling explicitly unaccepted.

## Review Outcome

Do not merge yet. DC01-CX-001 is a hard gate failure; DC01-CX-002 and
DC01-CX-003 should be dispositioned before close. DC01-CX-004 is a governance
decision, but I would not leave it undocumented in a production-semantics
package.
