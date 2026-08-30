# Review C — Rust correctness, API, and performance

Status: **HOLD**

Evidence mode: `Static + Ran`.

Review snapshot: commit `2a9ca2d845bb4f128441ab01f79b341033a31c7d`
with a dirty shared worktree at `2026-08-29T00:58:14-07:00`. At that point
`git status --porcelain=v1` reported 196 tracked changes and 87 untracked
paths. This review was read-only except for this artifact and does not treat
the tracked Git diff as the complete terminal change set.

## Scope

Static review covered:

- adaptive direct/composed failure classification and complete-owner
  comparison in `snow_stage3_v11_adaptive_frontend.rs`,
  `snow_stage3_v11_adaptive_execution.rs`, and
  `canonical_owner_bytes.rs`;
- private accepted-publication/WB14 persistent-history implementation and its
  canonical materialization path;
- default-off fixed-point, comparison, closure, and completed-parent
  diagnostic surfaces, including their public visibility and serialization
  boundary;
- current Rust source line counts and package line-count artifacts;
- the optimized v7 one-day log and timing record; and
- current package reconciliation, warning/check, and critical-regression
  evidence.

## Findings

### C-01 — BLOCKER: no exact terminal diff or owned-file reconciliation

`terminal-diff-reconciliation.md` remains `queued`/`not-run` and
`owned-file-manifest.md` remains `queued`. The worktree contains untracked
production modules as well as tracked changes, so the tracked-only Git diff is
not a truthful terminal scope. Until the exact terminal file set is classified
and every file is mapped to package authority and gates, this reviewer cannot
establish API compatibility or complete review coverage for the terminal tree.

### C-02 — BLOCKER: the affected warnings-denied gate fails

The exact-current ordinary `cargo check` is green, but the package-required
warnings-denied affected Clippy gate is not. The lib-only command failed with
732 errors and the all-targets orchestrator/runner command failed with 930
errors. These are not solely remote workspace baseline findings: examples in
package-touched paths include `similar_names` in
`snow_stage3_v11_adaptive_execution.rs`, `too_many_lines` and truncating casts
in covered LSE owner projection, large enum variants and unnecessary wrappers
in the multi-tile runtime, and numerous numerical/runtime lint failures in the
Stage-3 solver path. No warning-clean terminal claim is therefore supported.

### C-03 — BLOCKER: no completed exact-current critical workspace PASS

The gate ledger's completed full-workspace result is the retained historical
failure (3,465 passed, 107 failed, 10 timed out). The first replacement attempt
in `full-workspace-final.log` was terminated by signal 2 during compilation.
At the review snapshot a second exact-current replacement was still compiling;
it had no test or terminal result and is not PASS evidence. Critical-package
closure requires a completed exact-current campaign after the terminal diff is
frozen.

### C-04 — MAJOR: “test audit” controls are unconditional public API

The diagnostic values are correctly absent from receipt, restart,
publication, and qualification serialization. However, the following are
compiled unconditionally and exposed through the public
`snow_stage3_v11_attachment` module rather than through `cfg(test)` or a
dedicated opt-in feature:

- `AdaptiveParentTelemetryV1` and its begin/take guard API;
- `CoveredFixedPointIterationAuditV1` and its begin/take guard API;
- `AdaptiveComparisonAuditV1` and the functions named
  `begin_adaptive_comparison_test_audit`/`take_adaptive_comparison_test_audit`;
  and
- the physical-outcome closure audit re-exports.

This is more than a passive observation surface. Public callers can enable a
completed-parent bound that causes production execution to return the public
`AdaptiveTelemetryStop` error. The runner's ignored qualification tests need
cross-crate access, but the current implementation expands the normal
production API and leaves diagnostic branches in the hot path. Before GO,
either gate these controls behind an explicitly enabled non-default diagnostic
feature (with the runner test target enabling it), or record explicit API
authority accepting them as supported production diagnostics and their
control-flow effect.

### C-05 — MAJOR: line-count source gate passes, but governance is incomplete

The exact-current scan found no Rust source file at or above 3,000 lines; the
largest was `direct_runtime/surface_liquid_closure.rs` at 2,991 lines and
`v11_covered/open_snow.rs` was 2,955 lines after its mechanical helper
extraction. Thus there is no current hard 3,000-line source blocker.

The same scan found 43 touched or untracked Rust files at or above the
2,000-line WARN threshold. `line-count-governance.md` is still queued and does
not provide their required rationale, split intent, owner, or disposition.
The source split is therefore useful but the governance gate is not complete.

## Positive static conclusions

### Comparator classification is narrow and fail-closed

- The only receipt-container cross-factorization exception is the exact
  `surface_liquid.wb14_parent_working_state.per_ofe_authorities.<ofe>.receipts`
  path. It is classified as `ReceiptLineage`; every other receipt container
  remains `ReceiptOrdering` and therefore an exact cross-path discrete
  surface. Each trial still binds the complete exact WB14 receipt-map digest.
- The adaptive comparison audit now excludes the same `ReceiptLineage` class
  as production comparison, while retaining `ReceiptOrdering`; it no longer
  reports a diagnostic-only mismatch that production intentionally excludes.
- Refinable trial failures are enumerated rather than inferred from display
  strings. The set includes the bounded numerical solver failures, the exact
  Stage-3 lower-boundary/column closure, covered fixed-point exhaustion, and a
  fully populated finite coarse open-snow temperature-domain record.
  `UnsupportedDomain("hydraulic_redistribution")`, constitutive-domain errors,
  unrelated closure errors, and owner identity failures remain non-refinable.
  The exact 60-second floor still propagates a refinable failure fail-closed
  when no smaller child is admissible.

No comparator-scope correctness defect was found in these reviewed paths.

### Private history/caching does not expose allocation identity

The accepted-publication history and persistent WB14 replay chain use private
`Arc`-backed types. Pointer identity is an allocation fast path only; equality
falls back to length, canonical digest, and materialized bytes. Canonical wire
projection traverses values, and the types are not public DTO fields. This
preserves the intended public/wire API boundary.

### Diagnostic persistence boundary is covered

`Stage3AdaptiveControllerTelemetryV1` has no serialization implementation and
is reconstructed from validated authoritative receipt vectors. Receipt
deserialization denies an injected `diagnostics` field, and qualification day
and accumulator serialization tests reject/omit the diagnostic key set. The
public/API concern in C-04 does not imply a persisted-authority leak.

## Optimized v7 performance evidence

Ran evidence in
`/tmp/adaptive_microstep_amendment/one-day-final-v7-opt.log` and `.time` is a
real downstream-consumer PASS:

- 48 completed parents;
- 497 accepted decisions, 206 rejected candidates, 703 direct trials, and
  1,368 split-child trials;
- 975 retained publication supports and 61 events;
- 155 fixed-point nonconvergence rejections, 16 scaled comparison rejections,
  and zero discrete comparison rejections;
- 1,578 independently checked physical ledgers, with maximum residuals
  `1.77635683940025046e-15 kg m^-2` and
  `1.39698386192321777e-9 J m^-2` inside unchanged bounds; and
- test body 357.55 seconds, versus the retained 485.858-second optimized
  baseline (26.41% lower).

The adaptive-parent cumulative timer was 328.329 seconds. The `/usr/bin/time`
wall was 561.39 seconds and maximum RSS was 5,894,016 KiB, but both include the
optimized cold build and therefore do **not** establish runtime-only RSS or a
memory improvement. Telemetry was enabled during this run. The current source
also received a mechanical `open_snow` helper extraction after the v7 log, so
v7 is strong behavior/performance evidence for the immediately preceding
source, not a substitute for exact-current terminal qualification.

## Ran evidence

- `nix develop --command cargo check -p openwepp-hillslope-orchestrator -p
  openwepp-runner --all-targets` — PASS in 5.38 seconds.
- `nix develop --command cargo fmt --all -- --check` — PASS.
- `git diff --check` — PASS.
- `nix develop --command cargo clippy -p
  openwepp-hillslope-orchestrator --lib --no-deps -- -D warnings` — FAIL,
  732 errors.
- `nix develop --command cargo clippy -p
  openwepp-hillslope-orchestrator -p openwepp-runner --all-targets --no-deps
  -- -D warnings` — FAIL, 930 errors.
- `nix develop --command cargo test -p openwepp-hillslope-orchestrator
  trial_local_numerical_failures_refine_without_weakening_constitutive_guards`
  — PASS, 1/1.
- `nix develop --command cargo test -p openwepp-hillslope-orchestrator
  qualification_serialized_surfaces_omit_microstep_diagnostics` — PASS, 1/1.
- `nix develop --command cargo test -p openwepp-hillslope-orchestrator
  support_receipt_serialization_omits_diagnostics_and_rejects_unknown_diagnostic_field`
  — PASS, 1/1.
- Exact current Rust line-count scan — PASS at the 3,000-line BLOCK threshold;
  43 touched/untracked files remain at the 2,000-line WARN threshold.

## Terminal disposition

**HOLD.** Comparator classification, private immutable history, diagnostic
serialization exclusion, and optimized one-day behavior are substantively
sound in the reviewed paths. GO is not truthful until C-01 through C-05 are
resolved on one frozen exact tree, the warnings-denied affected gate passes,
and the exact-current full-workspace critical campaign completes successfully.
