# Lane D Active Baseflow Export Closure

Status: `EXECUTED-COMPLETE`
Package ID: `20260709-laned-active-baseflow-export-closure-001`
Queue row: `M-T2`
Evidence mode: `Static + ran`

Closure: `2026-07-09 UTC`

## Objective

Close the remaining Lane D active groundwater/baseflow export boundary after
M-T2B by proving the real HBP and watershed/channel consumers read generated
groundwater-reservoir baseflow (`gwbfv`) and deep seepage (`gwdsv`), and by
implementing the `bftharea` threshold branch without falling back to `cbase`
under `lr_bf=1`.

## Rationale

M-T2B implemented the Srivastava linear groundwater reservoir and WAT `Base`
publication, but held on two consumer-facing boundaries:

- generated `gwdsv` lacked a real downstream consumer;
- `bftharea` was parsed but not evaluated by watershed/channel routing.

M-T2 owns only that closure slice. It must not change the Lane D surface-router
source series, routing numerics, mesh policy, or groundwater recurrence.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
- `/workdir/wepp-forest/docs/contracts/hillslope-binary-pass-format.md`
- `docs/work-packages/20260708-groundwater-baseflow-laned-single-ofe-mofe-implementation-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260708-groundwater-baseflow-laned-single-ofe-mofe-implementation-001/artifacts/final-disposition.md`

## Included Scope

- Contract amendment for HBP parser/writer semantics where the existing trailing
  payload pair is named as `gwbfv`/`gwdsv`.
- Direct HBP writer population from `DirectPublicationDayRow.subsurface`.
- HBP parser exposure of `baseflow_volume_m3` and
  `deep_seepage_volume_m3`.
- Watershed pass inventory and `HillslopeContribution` handoff of generated
  baseflow/deep seepage.
- Watershed-side `gwcoeff.txt` authority parsing for `lr_bf` and `bftharea`.
- Channel-kernel branch separation:
  - `lr_bf=0`: retain `cbase` branch;
  - `lr_bf=1`: consume generated HBP `gwbfv`, reject `cbase` substitution, and
    evaluate `bftharea` against contributing area hectares.
- Real consumer-path tests and artifacts.
- Roadmap/readme disposition updates for M-T2B and M-T2.

## Excluded Scope

- New groundwater/baseflow recurrence physics.
- New HBP schema-major redesign or payload layout reordering.
- Lane D active routing source changes.
- HBP hourly water/sediment consumption beyond the baseflow closure leg; that
  remains M-T3.
- Nonlinear Srivastava et al. groundwater algorithms.

## Intended Write Set

- `docs/work-packages/20260709-laned-active-baseflow-export-closure-001/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
- `docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md`
- `crates/openwepp-input-contract/src/parsers/hbp/**`
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/src/watershed_supervisor.rs`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/**`
- unit registry/output metadata files only if new public fields are added.

## Phase Plan

1. Completed: Scaffold package and record pre-implementation source/consumer map.
2. Completed: Amend HBP/baseflow contract text before code edits.
3. Completed: Thread HBP `gwbfv`/`gwdsv` writer/parser fields.
4. Completed: Thread parsed values into watershed `HillslopeContribution`.
5. Completed: Add watershed `gwcoeff` authority and channel branch selection.
6. Completed: Add tests proving generated baseflow changes routed channel output while
   `cbase` does not substitute under `lr_bf=1`, and proving `gwdsv` is read.
7. Completed: Run gates, update artifacts, and update roadmap/catalog disposition.

## Acceptance Criteria

- HBP writer emits nonzero generated `gwbfv`/`gwdsv` when direct groundwater
  recurrence produces them.
- HBP parser exposes those fields on `HbpLatestEventPayload`.
- Watershed pass inventory validates and carries those fields into
  `HillslopeContribution`.
- Watershed channel routing consumes generated `gwbfv` under `lr_bf=1`.
- `cbase` remains exclusive to the `lr_bf=0` branch.
- `bftharea` is evaluated in hectares against contributing area; below-threshold
  generated baseflow is suppressed/carried as the current openWEPP channel
  closure behavior.
- `gwdsv` has a real consumer path at least through pass inventory and typed
  watershed publication/diagnostics; producer-only evidence is insufficient.
- Missing malformed present `gwcoeff.txt` fails closed; absent file keeps
  `lr_bf=0` behavior.
- Protected legacy/off behavior remains unchanged for no-generated-baseflow
  inputs.

## Required Gates

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile quick`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- Focused parser/runner/watershed tests named in `artifacts/gate-results.md`
- `markdown-doc lint --path docs/work-packages/20260709-laned-active-baseflow-export-closure-001 --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md --path docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md`
- `git diff --check`

## Closure Statuses

`EXECUTED-COMPLETE`:

- All acceptance criteria pass with current evidence, and M-T3 is unblocked on
  the baseflow/export leg.

`EXECUTED-HOLD-*`:

- A named current-scope consumer, authority source, or topology-area proof is
  unavailable after implementation attempts, and the hold artifact names the
  exact unblocker.
