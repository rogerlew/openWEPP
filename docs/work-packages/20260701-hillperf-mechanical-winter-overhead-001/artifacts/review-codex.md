# Codex Independent Review

Reviewer: Codex
Date: 2026-07-01
Scope: branch `worktree-hillperf-sub5x`, commits `06f5dc0a..4f9fd355` on
`6dbeda40`. Review only; no production code or tests modified.

Evidence classes used below:

- Static: source/artifact inspection.
- Ran: command executed by this reviewer in the worktree.

## Verdict

Static/Ran: I found no production-code rejection for F2/F5/F6/F7. The final
branch is byte-identical on the five H2637 protected outputs in my independent
release run. Package closure still needs disposition for the accepted/deferred
review candidates below before I would call the work package fully closed.

## Review Findings

### Accepted candidate A1: package exit gate 4 is not satisfied as written

Static: `docs/work-packages/20260701-hillperf-mechanical-winter-overhead-001/package.md:101`
requires a quiet-window 3-rep H2637 timing run and a backlog assessment update.
`docs/work-packages/20260701-hillperf-mechanical-winter-overhead-001/artifacts/gate-log.md:44`
records a loaded-window 3-rep timing run, and
`docs/work-packages/20260701-hillperf-mechanical-winter-overhead-001/artifacts/gate-log.md:48`
explicitly leaves quiet-window 3-rep confirmation as post-merge recommended.
Ran: `git diff --name-only 6dbeda40..HEAD docs/backlog/...` produced no
backlog change, so the measured deltas were not written back to
`docs/backlog/20260701-hillslope-sub5x-performance-assessment.md`.

Impact: the timing evidence is useful and the endpoint is under 5x in my
independent run, but the package's own current-scope gate is not met as written.
Disposition options: run and record the quiet 3-rep plus backlog update, or
amend/disposition the gate explicitly before close.

### Accepted candidate A2: F5 disposition artifact is stale

Static: `docs/work-packages/20260701-hillperf-mechanical-winter-overhead-001/artifacts/finding-dispositions.md:42`
still says `## F5 -- in progress`, while commit `2398ed44` landed F5 and
`gate-log.md` records F5/exit evidence. This is an artifact truthfulness issue,
not a production-code issue.

### Deferred candidate D1: requested SC binding-exposure lint cannot prove the F5 API change

Ran: `python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
failed with `missing Binding Exposure Index`
(`docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md:1`).
Static: `rg` found no contract/doc binding to the removed
`DirectFrostThermalInputs.monthly_max_c` / `monthly_min_c` field names; the
only `DirectFrostThermalInputs` contract reference I found binds
`residue_depth_m`, not the monthly arrays.

Impact: I do not see a field-removal regression, but the required BEI lint is
not green and cannot be used as authority evidence until SC-SNOWFREEZE has a
Binding Exposure Index or the package records a targeted substitute check.

### Accepted candidate A3: line-count governance is missing for a touched 3000+ line Rust file

Ran: `wc -l` shows
`crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
at 4141 lines, and F5 touches that file at
`crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs:1456`.
Static: `docs/work-packages/AGENTS.md` requires disposition/refactor handling
for touched non-exempt 3000+ line Rust files. I found no package-local
line-count disposition.

Impact: not a behavior defect, but it is a closure-governance gap.

### Deferred candidate D2: one eager guard-feeding symbol allocation remains outside the converted helper sites

Static: `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs:956`
still constructs `BoundarySymbol::from(FROST_LANDUSE_CLASS_PROXY_SYMBOL)` before
the following success-path checks. This was not one of the converted `_with`
guard call sites, so it does not reject F2 transform fidelity, but it makes the
artifact wording "all eager guard-feeding symbol constructions" too broad.

Impact: low. Either narrow the wording to the converted helper call sites or
queue this residual allocation as a tail follow-up.

### Rejected candidate R1: pandas follow-up is under-evidenced as worded

Static: `tools/owcmp/requirements.lock.txt:3` contains only `pyarrow`, and
`tools/snowfreeze_observed/pysnobal_compare.py:29` imports `pandas`. However,
`rg` found no `pandas` import in the HPHYS0298 harness or its HPHYS0297 import
target. The dependency follow-up may still be real, but
`finding-dispositions.md:88` should name the actual failing test/module rather
than attributing it to the HPHYS0298 harness without a visible import chain.

## Required-Point Assessment

- F2 transform fidelity: Static accepted. `require_state_range_with` preserves
  the old min/max/value logic and materializes the same symbol only in the
  error branch; `require_dynamic_state_range_with` delegates exactly to that
  body. `rg` found no surviving callers of `require_state_range_for_symbol`,
  `require_dynamic_state_range`, `require_dynamic_state_range_for_symbol`, or
  `require_direct_typed_snow_value`. Spot checks beyond the implementer's
  tmpadj/layer clusters covered `frost_fine_layer_count_for_layer`, snow-melt
  guards, and snow runtime-after guards.
- F5 value identity: Static accepted. `direct_monthly_max_c()` and
  `direct_monthly_min_c()` are const accessors over stored monthly climate
  normals with no day dependence. The fitted curve is built once in the frost
  typed authority and carried through `DirectWinterFrostComputeInputs.thermal`
  into both the builder-side and executor-side frost solves. I agree with the
  public `FrostSeasonalTemperatureCurve` replacement: it makes the fitted curve
  the single authority instead of carrying both raw arrays and derived state.
- F7 equivalence: Static/Ran accepted. The direct cast and decimal parse both
  round the integer to the nearest `f64`; the test pins representative
  boundaries including `2^53 - 1`, `2^53`, `2^53 + 1`, and `usize::MAX`. Ran:
  `cargo test -p openwepp-hillslope-orchestrator diagnostic_count_to_f64_matches_decimal_string_parse_bit_for_bit`
  passed.
- F6 gating: Static accepted. With traces enabled, the same trace event
  constructors feed the same writers; with traces disabled, only event
  construction is skipped. The erosion disabled-path hoist reads the same
  `wave1_enabled` / `wave2_enabled` flags that the cloned inputs would have
  carried.
- F3/F8/F6-tail dispositions: Static accepted with the artifact caveats above.
  F3's two `DirectWinterHourlyContext` argument sets are not field-identical.
  F8 is manifest-entangled because `phase_view_constructions` is emitted in
  `direct_runtime_counters`. F6 tail deferral is reasonable after the exit
  re-profile if the gate mismatch in A1 is dispositioned.

## Commands Run

- `git status --short`
- `git log --oneline --decorate --graph --max-count=20`
- `git diff --stat 6dbeda40..HEAD`
- `git diff --name-status 6dbeda40..HEAD`
- `rg`/`nl`/`git diff` inspections over F2/F5/F6/F7 touched surfaces
- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
- H2637 release identity run from staged WB05A inputs:
  `target/release/openwepp-cli-hill --run-dir <scratch>/h2637/runs --run-file /tmp/codex-hillperf-review-h2637.run --output-dir /tmp/codex-hillperf-review-h2637-out --policy compat --legacy-sidecar-discovery`
- `sha256sum -c expected.sha256` in `/tmp/codex-hillperf-review-h2637-out`
  Result: `H2637.hbp`, `H2637.loss.json`, `H2637.pass.parquet`,
  `H2637.wat.parquet`, and `H2637.plot.parquet` all OK. Runtime: 45.89 s,
  82828 KiB max RSS.
- `cargo test -p openwepp-hillslope-orchestrator diagnostic_count_to_f64_matches_decimal_string_parse_bit_for_bit`
- `python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  Result: failed, missing Binding Exposure Index.
- `python3 tools/release/check_sc_unit_compliance.py --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  Result: failed with existing SC-SNOWFREEZE unit/alias findings.
- `python3 tools/release/check_raw_unit_conversions.py`
  Result: failed with existing raw-literal findings in enforced files.
- `bash tools/release/check_authority_suite_antievasion.sh --base-ref 6dbeda40 --head-ref HEAD`
  Result: PASS.
- `wc -l` on touched Rust files.
