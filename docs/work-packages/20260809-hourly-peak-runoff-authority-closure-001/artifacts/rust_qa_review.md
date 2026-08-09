# Rust QA Review

Status: `complete`

Review target: exact commit
`949349e7055c5d19277eeb708401c4614a52cd77`, against the declared
pre-implementation base `a65cc3973ddd04b07cad108fcb33d83a8c161abb`.

## Findings

### HIGH — resumable census records omit model-active sidecar provenance

Paths:

- `docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/tools/topanga_openwepp_census.py:117`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs:917`

The census command enables `--legacy-sidecar-discovery`, and the runner then
resolves and parses `snow.txt`, `frost.txt`, `wepp_ui.txt`, `pmetpara.txt`, and
`gwcoeff.txt`. The frozen Topanga source tree contains `snow.txt` and
`gwcoeff.txt`, so both are part of the executable input surface. However,
`case_input_hashes` binds only the four primary `p<ID>` inputs plus optional
`pmetpara.txt` and `wepp_ui.txt`.

Changing `snow.txt` or `gwcoeff.txt` therefore leaves `record_matches` true and
allows `--resume` to reuse stale output under new active inputs. The new tests
mutate only `p1.sol`; none mutates a discovered sidecar or drives the actual
`run_case(..., resume=True, ...)` reuse branch. Plan SHA, binary SHA, record
schema, canonical cohort counts, corrupt-record rejection, atomic replacement,
and heterogeneous plan-value serialization are now covered, but the prior
provenance/resume blocker is only partially resolved. Bind every resolved
sidecar (or a canonical resolved-input manifest) and test sidecar mutation plus
the end-to-end resume control flow before closure.

### HIGH — daily frost retention manufactures hourly timing

Paths:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:1432`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_dc01.rs:204`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md:297`

The exact commit proportionally removes a daily-only
`frost_retained_local_liquid_m` scalar from every positive WB14 hourly runoff
bin, then applies floating residual to the largest bin. The focused test
ratifies the synthetic 60/40 debit, but there is no producer-hour frost custody
surface or frost-active real-consumer fixture proving that distribution. This
changes the closing hourly depths and potentially the published peak based on
timing that the implementation explicitly says the producer does not have.

Local `INV-WATBAL-103` text was amended to authorize the proportional debit,
but it supplies no baseline or external physics provenance. That remains at
odds with the repository prohibition on heuristic production-physics timing
and with `INV-WATBAL-102`'s rule that normalized weights may not manufacture
hourly depths from a daily scalar. Preserve producer-hour custody or fail
closed when a material daily frost debit cannot be reconciled without
retiming; do not use a synthetic proportional allocation as closure evidence.

### MEDIUM — p61/p102 Parquet comparisons do not use the full event key

Paths:

- `tests/integration/erosion_single_ofe_p61_sediment.rs:161`
- `tests/integration/erosion_multi_ofe_p102_chain.rs:85`

Both real-consumer tests independently reconstruct HBP peak as
`max(V_h) / 3600` and compare it with HBP and Parquet `peakro`, which is a major
improvement. However, each selects the Parquet row by Julian day alone and
takes the maximum `runvol` among matches. The HBP event exposes
`sim_year_index` and `calendar_year`, and both fixtures span multiple years, so
Julian day is not a unique event key. A wrong-year Parquet row can satisfy the
comparison. Include year (and the outlet identity where applicable) in the
row model and select exactly one row by the full HBP event key.

### MEDIUM — line-count governance misses touched warning files and split intent

Paths:

- `docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/line-count-governance.md:1`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs:1`
- `crates/openwepp-runner/src/hillslope/03_tests.rs:1`

The exact terminal diff touches seven Rust files at or above the 2,000-line
warning threshold, but the governance artifact lists only five. It omits
`watershed_cli_behavior_contract.rs` at 2,996 lines and `03_tests.rs` at 2,892
lines. It also labels the listed rows retained debt without the required
decomposition rationale and follow-on split intent for each warning file.
No touched file reaches the 3,000-line blocking threshold, but the package's
maintainability disposition is incomplete under `crates/AGENTS.md`.

## Resolved prior blockers and exact evidence

- Ran, isolated archive of the exact target:
  `cargo fmt --all -- --check` passed.
- Ran, isolated archive of the exact target:
  `cargo clippy -p openwepp-hillslope-orchestrator -p openwepp-runner -p openwepp-hillslope-output --tests -- -D warnings`
  passed. The former overlong publication test is now split coherently.
- Ran, isolated archive of the exact target: the p61, p102, and four
  `peak_hourly_authority_contract` tests passed under Nextest (`6/6`). The real
  HBP consumers reconstruct `max(V_h) / 3600`, event-volume closure, and public
  Parquet peak; p61 also reconstructs rectangular-equivalent duration.
- Ran, isolated archive of the exact target:
  `publication_peak_scales_area_once_and_guards_its_boundary` passed (`1/1`).
  Its 100/200 m2 cases independently distinguish exactly-once area scaling,
  and the negative/nonfinite area plus contradictory runoff-basis guards are
  exercised.
- Ran, isolated archive of the exact target:
  `writer_emits_valid_parquet_file_with_schema_metadata` passed in both owning
  test binaries (`2/2`). It asserts `peakro` name, `Float64`, non-nullability,
  `m^3/s`, maximum-hour description, and serialized value readback.
- Ran, isolated archive of the exact target:
  `.venv/bin/python -B .../tools/test_topanga_openwepp_census.py` passed
  (`4/4`). Corrupt, binary-changed, primary-source-changed, nonfinite, negative,
  and structured plan-value cases are covered.
- Static, exact target: canonical plan SHA and exact 1,088 eligible / 280
  baseline counts gate `complete_frozen_cohort`; trial IDs are required unique;
  logs precede parsing; successful NPZ replacement is atomic.
- Static, exact diff: `git diff --check` passed.
- Not rerun by this reviewer: full-workspace quick/full/doctest/deny gates or
  the heavy 1,088-trial census. Those remain separate exact-anchor package
  evidence. A supplemental owning-crate quick run passed `467/467`, but it was
  not used for this verdict because concurrent workspace edits made it
  non-exact.

## Non-blocking debt and follow-ups

- Add direct cases for nonfinite/negative shadow runoff depth, peak depth-rate,
  and rectangular duration at the publication seam. Production has typed
  guards, but the focused boundary test does not exercise those operands.
- The 524-line census tool combines execution, process lifecycle, persistence,
  provenance, pairing, statistics, and reporting. Splitting record/provenance
  handling from execution and analysis would make failure-path tests easier to
  maintain.
- On batch failure, cancelling futures does not terminate already-running
  subprocesses or emit a terminal partial-run summary. This is operational
  debt rather than a successful-record correctness issue.

## QA Verdict

`HOLD — NOT ACCEPTABLE FOR CRITICAL CLOSURE.` Warnings-denied Clippy, public
HBP/Parquet reconstruction, output metadata, canonical completion identity,
atomic record replacement, and structured mutation values now pass focused
review. Closure remains blocked by incomplete model-input provenance for
resumed census records and unsupported daily-to-hourly frost retiming. The
event-key and line-count dispositions should also be corrected before the
package is accepted.
