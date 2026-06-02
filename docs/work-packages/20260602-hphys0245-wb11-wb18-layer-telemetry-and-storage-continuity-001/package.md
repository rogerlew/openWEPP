# 20260602-hphys0245-wb11-wb18-layer-telemetry-and-storage-continuity-001

## Status
- state: completed
- date: 2026-06-02
- timezone: America/Los_Angeles
- decision: HOLD_PENDING_WB18_AGGREGATE_WATER_ACCOUNTING_AND_WB19_DAY1_LATERAL_CLOSURE

## Objective
Add and execute gated diagnostics-only telemetry for `H1`, `H7`, and `H39`
covering day `1..30`, tracing WB11/WB18 layer/storage/percolation symbols
across scheduler boundaries before authorizing production physics changes.

## Why This Package Exists
HPHYS0244 confirmed that current emitted artifacts expose WAT symptoms but not
the internal layer/storage trajectories needed to isolate the WB11/WB18
residual. HPHYS0245 creates that missing telemetry under an explicit diagnostic
gate so production physics remains unchanged while the next root-cause
decision becomes observable.

## Scope
### Included
- Add a gated diagnostics sidecar to the hillslope runner, disabled by default.
- Emit JSONL trace rows for `H1`, `H7`, and `H39` day `1..30` when the
  diagnostics environment variable is set.
- Trace these scheduler boundaries:
  - `post_seed`
  - `post_scheduler`
  - `post_wb13`
- Include storage/percolation state sufficient to evaluate the HPHYS0244 exit
  signal:
  - `wb18_perc_theta_*`
  - `wb18_perc_pei_*`
  - `D`
  - `Pe`
  - `wb11_soil_water`
  - WB13 `Total-Soil`
  - WB13 `SoilWaterTotal`
- Execute targeted `H1`, `H7`, and `H39` runs with diagnostics enabled.
- Compare telemetry-derived storage signals against baseline-accessible WAT and
  `H.soil.parquet` aggregate surfaces.

### Explicitly Out of Scope
- Production process-physics changes.
- Percolation clamps, constants tuning, or heuristic storage corrections.
- Science-contract amendments.
- Watershed rerun.
- Commit/push unless separately requested.

## Closure Measures
1. `MEASURE-HP245-001`: diagnostics sidecar is disabled by default and only
   writes when the explicit environment variable is set.
2. `MEASURE-HP245-002`: targeted telemetry exists for `H1`, `H7`, and `H39`
   with day `1..30` rows.
3. `MEASURE-HP245-003`: telemetry includes WB11 aggregate storage, WB18 layer
   theta sum, WB18 per-layer `pei` sum, `D`, `Pe`, WB13 `Total-Soil`, and WB13
   `SoilWaterTotal`.
4. `MEASURE-HP245-004`: analysis identifies whether the next code change is
   most likely WB11 seed/carry, WB18 writeback, WB19 aggregate mutation, or
   WB13 publication.

## Deliverables
1. `artifacts/hphys0245-implementation-evidence.md`
2. `artifacts/hphys0245-telemetry-run-evidence.md`
3. `artifacts/hphys0245-storage-continuity-analysis.md`
4. `artifacts/hphys0245-focus-recommendations.md`
5. `artifacts/gate-results.md`
6. `artifacts/owned-file-manifest.md`
7. `artifacts/hphys0245_disposition.md`
8. `artifacts/worker-handoff.md`

## Mandatory Sequence
1. Implement diagnostics-only telemetry behind an explicit environment gate.
2. Add focused tests proving the default path is silent and the enabled path
   writes expected fields.
3. Build/run targeted `H1`, `H7`, and `H39` telemetry.
4. Analyze storage continuity and compare against baseline-accessible aggregate
   surfaces.
5. Publish recommendation and disposition.

## Autonomous Execution Intent
Execute this package end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts explicitly label `Static:` and/or `Ran:`.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0244-h1-h7-h39-storage-wb18-lineage-diagnostics-001/`
- `/tmp/unpalatable_parity_20260529T192707Z/runs/`
- `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/`
- `/wc1/runs/un/unpalatable-rind/wepp/output/interchange/`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260602-hphys0245-wb11-wb18-layer-telemetry-and-storage-continuity-001/**`
- `crates/openwepp-runner/src/hillslope/mod.rs`

## Phase Plan
### Phase A - Instrument
- Add an environment-gated diagnostics JSONL sidecar and focused runner tests.

### Phase B - Execute
- Run `H1`, `H7`, and `H39` with diagnostics enabled and collect day `1..30`
  telemetry.

### Phase C - Analyze and Disposition
- Compare telemetry to aggregate baseline surfaces and publish the next
  implementation target.

## Exit Criteria
- Telemetry sidecar behavior is validated.
- Targeted telemetry is produced for all three hillslopes.
- The next implementation target is narrowed to a specific storage/percolation
  handoff surface or explicit blocker.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local diagnostics/tests/docs only; no credentials/network writes.
