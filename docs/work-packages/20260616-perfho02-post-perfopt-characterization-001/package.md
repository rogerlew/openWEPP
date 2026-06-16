# PERFHO02 - Post-PERFOPT01 High-OFE Performance Characterization

Status: complete 2026-06-16 (operator-directed by `scaffold and execute PERFH02`; normalized ID: `PERFHO02`)

Package type: **Performance characterization only**. No production Rust, science-contract, physics, formula, threshold, output-schema, or fail-closed behavior change is in scope.

## Objective

Characterize the post-PERFOPT01 dominant cost on the H2637 19-OFE hillslope. PERFOPT01 removed part of the PERFHO01 runtime-surface/writeback-validation hot path and improved H2637 by roughly 12-13%, but optimized GDB samples shifted residual time toward hydrology/transfer guards, repeated symbol formatting, and remaining lane-surface clone/drop. PERFHO02 must produce profiler-backed attribution and a concrete next-optimization recommendation.

## Rationale

PERFHO01 proved the original H2637 gap was CPU-bound, not output I/O, and named `execute_persistent_scheduler_kernel_lifecycle` plus symbol-keyed runtime-surface/writeback validation as the first target. PERFOPT01 then landed a bit-identical optimization:

- H2637 without UI: `978.55s -> 849.86s`.
- H2637 with UI: `968.73s -> 851.40s`.
- `anchor_mismatches = 0`.

The optimized GDB re-check showed no `collect_field_violations` or `apply_kernel_writeback` samples in a 10-sample window. Remaining samples pointed at:

- per-lane daily surface clone/drop in `execute_persistent_scheduler_kernel_lifecycle`;
- hydrology/transfer guard and symbol-formatting paths such as `ensure_no_overflow_indexed_symbols_for_decomposition`, `validate_transfer_array`, `require_erod14_state_scalar`, WB16 seed publication, and WB18 percolation formatting.

## Included Scope

- Scaffold the work-package directory, kickoff prompt, required-reading map, runfile, and evidence artifacts.
- Build or validate the optimized release binary used for sampling.
- Re-probe profiler availability.
- Run GDB user-space sampling on H2637 post-PERFOPT01 steady-state execution.
- Classify samples by mechanism and source path.
- Produce a verdict and next-optimization recommendation.
- Update the work-package execution log and roadmap if PERFHO02 closes.

## Excluded Scope

- No production Rust edits.
- No science-contract edits.
- No H2637 behavior or output acceptance change.
- No attempt to implement the next optimization inside PERFHO02.
- No branch creation or switching.

## Intended Write Set

- `docs/work-packages/20260616-perfho02-post-perfopt-characterization-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

## Dependencies

- PERFHO01: `docs/work-packages/20260616-perf-high-ofe-hillslope-characterization-001/`
- PERFOPT01: `docs/work-packages/20260616-perfopt01-runtime-surface-map-churn-001/`
- H2637 run inputs staged under `/tmp/perfho01/run-dirs/h2637`
- Release binary: `target/release/openwepp-cli-hill`

## Phase Plan

1. **P0 scaffold** - create package files, kickoff prompt, runfile, and required-reading map.
2. **P1 profiler setup** - confirm release binary and H2637 staged inputs; rebuild release if needed; re-probe `perf`.
3. **P2 sampling** - run H2637 under GDB, collect at least 20 steady-state stack samples unless the process exits or GDB is unavailable.
4. **P3 attribution** - classify samples into residual mechanism buckets and cite representative stacks.
5. **P4 verdict** - write profiler evidence, residual verdict, next-optimization recommendation, gate results, review, verification, disposition, and worker handoff.
6. **P5 catalog update** - update `docs/work-packages/README.md` and `docs/ROADMAP.md` to reflect closure or hold.

## Exit Criteria

- Profiler availability is recorded with command-level evidence.
- GDB sampling raw log path is recorded, or a blocker is recorded if GDB is unavailable.
- Sample classification has current evidence and names exact functions/modules.
- Verdict says whether the residual is still CPU-bound and whether output writers are sampled.
- Next optimization recommendation is concrete enough to scaffold a follow-on package.
- Docs/path sanity gate runs: `git diff --check`.
- Review and verification artifacts explicitly check the Gate Evidence Non-Deferral Rule.
- Line-count governance states that no `.rs` files were edited.

## Subagent Requirement

Subagent requirement: none. This package is profiling/documentation-only and the user did not explicitly request subagents. Local review and verification artifacts are required and must be labeled as local, not independent delegated review.

## Deliverables

- `artifacts/required-reading-map.md`
- `artifacts/perfho02-profiler-evidence.md`
- `artifacts/perfho02-residual-verdict.md`
- `artifacts/perfho02-gate-results.md`
- `artifacts/perfho02-review.md`
- `artifacts/perfho02-verification.md`
- `artifacts/perfho02_disposition.md`
- `artifacts/perfho02-worker-handoff.md`

## Security Impact

No security-sensitive code, external authority suite posture, fixture binding, or release artifact is modified. No source-level anti-evasion guard is required for this characterization package.
