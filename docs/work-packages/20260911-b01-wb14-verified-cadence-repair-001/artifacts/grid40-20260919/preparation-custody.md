# Grid40 preparation custody

Prepared 2026-09-19 before prospective authority approval. No Rust source,
contract, package record, build output, test result, or solver result was
created or changed.

## Verified frozen inputs

- frozen source: `/home/roger/openwepp-experiments/b01-wb14-lse-first-trial-source-20260918`, 748 entries, SHA-256 `8cd4866f66b06d6cf96403753cdb3d41ba75f6ca87652341ebe082a86a1b4bc5`;
- recovery baseline: `/home/roger/openwepp-experiments/b01-wb14-observer-source-cut02-20260918`, 747 entries, SHA-256 `85b8314efcd53ccb8111aa97af1f752ed49a99f46b805e754db5a1b2c7059963`;
- recovery patch: `final-from-baseline-relative.patch`, SHA-256 `214a32362cd53708330843120a3bd59c1ee025928738ce201dae74ff82058fde`; `patch --dry-run -p1` against the baseline checked all three patch files cleanly;
- five source support symlinks and all nineteen frozen external build inputs matched the freeze manifest;
- frozen diagnostic binary `final-diagnostic.frozen` is 452610288 bytes and SHA-256 `551738aa5123cffa9b24f3e7e7b193cfaa95f2c16a803df5f2eb278ed9c12ba0`;
- original trace `final-first-failure-trace.json` is SHA-256 `527208c3e5ad07f1744f1bd92c9e4ac17cf7efd9469968a97d7964c5db482dfd` and contains the one recorded `solver_input` with 29 initial-trial coordinates.

## Authorized mutable source

`/home/roger/openwepp-experiments/b01-wb14-grid40-source-20260919` was made
only after the checks above, via metadata-preserving copy. Its snapshot has the
same 748 entries and frozen source SHA-256, and retains these exact symlink
targets: `.config`, `docs`, `src`, `tests`, and `tools`. It contains no edits.

## Frozen implementation plan pending authority release

The sole shared covered solve body is
`crates/openwepp-land-surface-energy/src/solver_covered_solve.rs`:
`solve_covered_column_observed` owns the 0..=20 strict search at lines 985-1058,
the b1..20 no-update witness at 933-963, and the failure count at 1059-1062.
The future detached/test-only route must keep the existing `numerics.rs`
`MAX_BACKTRACKING_HALVINGS` and the no-update witness unchanged. It will expose
a private captured-case entry that bit-authenticates the recorded JSON input,
rejects every other case, selects b0..20 or b0..40 before entering this one
solver body, and cannot be selected by transaction/runtime production callers.

Cost guards must wrap the actual call sites: base evaluation at 606; each
Jacobian assembly at 654 and each signed probe around 706/293-366; domain
validation before the full, witness, and strict trials; and complete residual
evaluation at 906, 950, and 1003. They must count starts/completions/errors,
identity anchors, and nested work without double-counting, stop before the next
attempt at the specified arm ceilings, and retain a partial terminal trace on
typed error/resource stop. The old post-refusal loop at 1063-1117 must be
disabled for both arms, yielding zero post-refusal probes. Existing test-only
`lse_first_trial_diagnosis` and `solver_mechanism_audit` are the confined
recording/cost surfaces; their current trace only finalizes on backtracking
rejection, so success, iteration/pivot/domain error, and interruption need
explicit completion handling before a result-bearing arm is permitted.

The approved counting semantics to apply on release are: attempted operations
include errors and consume their ceiling; ordinary calls are base, strict-search,
and witness calls; signed-probe starts are classified identity-anchor or complete;
every actual domain-predicate invocation counts; and a denied operation has no
executed start. This changes no numerical method or source in preparation.

Future focused tests: preservation of b0..20 and b1..20 witness behavior;
lawful b21+ strict acceptance, b0..40 exhaustion, outward exact-bound,
nonfinite/evaluation failure, rounded duplicate, residual failure despite a
small step, wrong-capture/non-production rejection, exact original JSON f64 bit
replay, all terminal trace forms, zero probes, and every real-callsite ceiling.
Run only after amendment plus both prospective reviews approve the authority.

## Execution and failure accounting

Ran read-only hash/inventory, symlink resolution, patch dry-run, and source
inspection commands. All verification commands above passed. No build, Rust
test, model/reader/downstream run, or result-bearing solve was run. Two early
wrong-path evidence lookups returned nonzero before the separate evidence root
was located; no correction was attempted. Carry failure floor is therefore at
least 358 mixed outcomes (356 inherited plus 2 new navigation failures), with
the inherited uncertainty retained. Authority acceptance, contract amendment,
review, source edits, focused checks, baseline arm, treatment arm, independent
operand reconstruction, and study disposition remain pending.
