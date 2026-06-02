# 20260602-hphys0247-h39-single-ofe-hourly-waterbalance-closure-001

This package is an ExecPlan-style living document. Maintain it according to
`docs/codex_exec_plans.md`: keep `Progress`, `Surprises & Discoveries`,
`Decision Log`, and `Outcomes & Retrospective` current as work proceeds, and
revise this file whenever implementation evidence changes the plan.


## Status

- state: queued
- date: 2026-06-02
- timezone: America/Los_Angeles
- decision: QUEUED_FOR_PACKAGE_END_TO_END_EXECUTION


## Purpose / Big Picture

After this package is complete, openWEPP should run hillslope `H39` from the
`unpalatable-rind` parity corpus as a single-OFE hourly-lane hillslope and
produce a complete, sensible daily water-balance output that semantically
matches the pinned legacy WEPP baseline for the water-balance columns. The
observable result is a successful local H39 hillslope run whose manifest proves
hourly scheduler selection, whose WAT parquet compares cleanly against
`baseline_H39.parquet`, and whose package artifacts explain the closure with
contract, test, implementation, and gate evidence.

This package deliberately takes a larger scope than the prior HPHYS packages.
The executing agent is not expected to ask the user to choose the next gate or
next residual family. The agent must follow the evidence, update canonical
contracts first, implement contract-derived tests, make production changes, run
the H39 hourly comparator repeatedly, and continue until the closure criteria
are met or a hard blocker is proven and recorded.


## Objective

Close H39 single-OFE hourly hillslope water balance end-to-end under
baseline-authoritative process physics. The primary closure gate is semantic
parity for H39 hourly WAT water-balance outputs against the pinned legacy
baseline, with an internal daily water-balance residual ledger proving that
storage and flux handoffs are coherent rather than compensated by tuning.


## Rationale

The HPHYS stream has already closed several narrow hourly ordering, carryover,
and WB18 storage defects, but H39 still has a large water-balance residual after
HPHYS0246. Continuing with one-surface packages would force the user to keep
manually selecting each next residual gate. This package instead authorizes one
agent to follow the water-balance evidence across all relevant hillslope
hydrology phases while preserving the correctness authority model: contracts
first, tests second, pre-implementation gate third, production code fourth, and
observable H39 closure as the final technical acceptance signal.


## Progress

- [x] (2026-06-02) Scaffolded HPHYS0247 as a queued autonomous ExecPlan-style
  work package.
- [ ] Read all required orientation, contract, prior-package, and baseline
  files listed in this package.
- [ ] Reproduce the current H39 hourly residual with manifest evidence proving
  hourly lane selection.
- [ ] Amend canonical `SC-*` contract authority for every water-balance surface
  that this package changes.
- [ ] Add contract-derived tests before production edits.
- [ ] Record pre-implementation contract-gate evidence.
- [ ] Implement baseline-authoritative production fixes for H39 hourly
  water-balance closure.
- [ ] Run H39 telemetry, internal residual ledger, semantic comparator, and
  workspace gates through disposition.
- [ ] Complete dual review, dual verification, worker handoff, and disposition
  artifacts.


## Surprises & Discoveries

- Observation: HPHYS0246 closed the WB18 aggregate storage writeback defect but
  did not close H39 water-balance parity.
  Evidence: `docs/work-packages/20260602-hphys0246-wb18-aggregate-storage-writeback-closure-001/artifacts/hphys0246-residual-analysis.md`
  records H39 day-1 post-WB18 `wb11_soil_water = 340.573894 mm`,
  `WB18 D/Pe = 22.980342 mm`, `WB13 Total-Soil = 260.660446 mm`, and
  `WB19 lateral day-1 delta = -79.515092 mm`.
- Observation: The comparator input has a legacy/openWEPP column alias seam for
  total soil water.
  Evidence: baseline H39 parquet exposes `Total-Soil Water`, while candidate
  H39 parquet exposes `Total-Soil`; the existing comparator normalizes this
  alias and the tolerance config names the canonical comparator column
  `Total-Soil`.
- Observation: Existing H39 baseline and candidate WAT partitions are
  single-OFE daily partitions with complete row cardinality for the current
  parity corpus.
  Evidence: local parquet schema inspection of
  `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/baseline_H39.parquet`
  and `/tmp/hphys0243_20260602T042747Z/parity/hillslope_output/H39.wat.parquet`
  showed `ofe_id = [1]`, `OFE = [1]`, and `1461` rows in each file.


## Decision Log

- Decision: Use a larger autonomous package rather than another narrow
  one-surface HPHYS slice.
  Rationale: The user requested magnitude-order larger agent chunks with clear
  success criteria. H39 single-OFE hourly water-balance closure is narrow enough
  to validate decisively but broad enough to let the executing agent follow
  residual evidence across WB11/WB12/WB13/WB14/WB17/WB18/WB19 without stopping
  for user arbitration.
  Date/Author: 2026-06-02 / Codex.
- Decision: Treat H39 single-OFE hourly WAT semantic parity as the primary
  closure gate, not the 39-hillslope watershed cohort.
  Rationale: Root guidance classifies single-OFE daily water-balance evidence as
  higher-confidence acceptance signal than watershed/hourly aggregate evidence,
  and the current blocker is a hillslope water-balance residual.
  Date/Author: 2026-06-02 / Codex.
- Decision: Keep `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` as the physics authority.
  Rationale: Root governance requires baseline-authoritative migration for
  legacy parity work and prohibits heuristic/proxy process-physics formulas in
  production paths.
  Date/Author: 2026-06-02 / Codex.
- Decision: Require both external semantic parity and an internal residual
  ledger.
  Rationale: Comparator parity alone can hide compensating errors. A sensible
  water balance means the storage state, flux publications, and phase handoffs
  close coherently day by day under contract-authoritative symbols.
  Date/Author: 2026-06-02 / Codex.


## Outcomes & Retrospective

Queued. No implementation has run under this package yet. At disposition, this
section must state whether H39 hourly water-balance closure is `GO`, `HOLD`, or
`BLOCKED`, summarize the residual families closed, list any remaining residuals
with exact magnitudes and evidence paths, and explain what the next package
should do only if closure remains incomplete.


## Context and Orientation

openWEPP is the Rust simulation engine in this repository. The hillslope CLI
binary is `openwepp-cli-hill`, built from `crates/openwepp-runner`. A hillslope
is one WEPP land unit. `H39` means hillslope number 39 in the existing
`unpalatable-rind` parity corpus. Single-OFE means this hillslope has one
overland flow element, so there is no multi-OFE internal routing ambiguity for
the primary closure gate.

The hourly lane is the runtime mode in which the scheduler executes hydrology
with hourly carry surfaces and hourly-specific ordering semantics. This package
must prove hourly mode with the generated
`openwepp_hillslope_run_manifest.json`; do not infer the lane only from a run
file name or sidecar. The manifest keys that matter are
`selected_lane`, `mode_selection.effective_mode`, and
`mode_selection.scheduler_mode`, all of which must be `hourly` for the H39 run.

WAT is the daily hillslope water-balance output. In the baseline parquet for
H39, the total soil-water column is named `Total-Soil Water`. In openWEPP
candidate parquet it is named `Total-Soil`. The semantic comparator normalizes
that alias and reports it as `Total-Soil`. The comparator tolerance authority
for this package is
`tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`, which sets
specific tolerances for `P`, `RM`, `Q`, `Ep`, `Es`, `Er`, `Dp`, `Total-Soil`,
`frozwt`, `Snow-Water`, and `SoilWaterTotal`; columns not listed, including
`latqcc`, use the default tolerance in that file unless canonical contract
authority justifies a tighter or explicit column tolerance.

The prior packages matter because they define the current residual posture.
HPHYS0243 ran a post-HPHYS0242 39-hillslope parity readjudication. HPHYS0245
added H1/H7/H39 storage telemetry and identified WB18 aggregate storage
writeback plus WB19 day-1 lateral transfer as focus surfaces. HPHYS0246 closed
the WB18 aggregate storage writeback defect and concluded that the remaining
dominant H39 day-1 residual is WB19 lateral transfer, not WB18 D/Pe tuning.
This package starts from that posture but does not restrict the executing agent
to WB19 if the H39 ledger proves another water-balance phase is the actual
remaining root cause.

The legacy physics authority for equations, constants, branch guards, and
symbol lineage is `/workdir/wepp-forest_260430_baseline` at commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`. `/workdir/wepp-forest` may be used
for exploratory discovery, but production contract and implementation
authority must default to the pinned baseline unless the package artifacts
justify an exception.


## Included Scope

This package includes contract-first closure for the full H39 single-OFE hourly
water-balance path needed to make WAT output sensible and semantically aligned.
That includes canonical contract amendments, contract-derived tests,
pre-implementation contract-gate evidence, and production changes for any
water-balance phase touched by the evidence. Expected likely surfaces include
WB11 soil-water storage, WB12 effective infiltration and runoff handoff, WB13
WAT publication, WB14 infiltration/runoff partitioning, WB17 evapotranspiration
storage withdrawal, WB18 percolation/deep seepage, and WB19 lateral/drainage
subsurface flow. The executing agent must update every relevant canonical
`SC-*` file before changing production code for that surface.

This package includes local H39 runner execution, H39 telemetry or trace
instrumentation as needed, H39 semantic comparator execution, a daily internal
water-balance ledger, targeted and workspace Rust tests, anti-evasion guards
when external-authority suite or fixture bindings are touched, dual review,
dual verification, worker handoff, and final disposition.


## Explicitly Excluded Scope

Do not expand this package into watershed routing closure, full 39-hillslope
cohort closure, multi-OFE routing behavior, erosion/sediment closure, or
production wepppy orchestration changes. Do not tune comparator tolerances to
make residuals pass. Do not add heuristic, surrogate, provisional, or
regression-fit hydrology formulas to production paths. Do not silently clamp or
default domain violations. Do not commit or push unless the user separately
asks.


## Deliverables

The technical deliverable is a local H39 hourly hillslope run that demonstrates
complete sensible water balance: manifest-proven hourly lane selection,
comparator-clean WAT water-balance columns, and an internal residual ledger that
closes storage and flux handoffs without tuning. The governance deliverable is a
fully dispositioned package with canonical contract amendments,
contract-derived tests, pre-implementation contract-gate evidence,
implementation/test evidence, workspace gate evidence, review/verification
posture, owned-file manifest, and worker handoff.

The checked-in deliverables are the package files under
`docs/work-packages/20260602-hphys0247-h39-single-ofe-hourly-waterbalance-closure-001/`
and any contract, test, or production files modified within the intended write
set. Runtime deliverables are the `/tmp/hphys0247_*` run root, H39 WAT parquet,
manifest, logs, trace sidecars, comparator JSON, and residual-ledger data named
in the artifacts.


## Closure Measures

The package reaches `GO` only when all closure measures below are satisfied and
recorded with `Ran:` evidence paths.

`MEASURE-HP247-001`: the H39 run completes with
`target/debug/openwepp-cli-hill` and writes `H39.wat.parquet`,
`openwepp_hillslope_run_manifest.json`, stdout, stderr, and any package trace
sidecars under one `/tmp/hphys0247_*` root.

`MEASURE-HP247-002`: the H39 manifest proves hourly mode. The artifact must
record exact values for `selected_lane`,
`mode_selection.effective_mode`, `mode_selection.scheduler_mode`, and the
presence or absence of the hourly carry sidecar fields. All three mode values
must be `hourly`.

`MEASURE-HP247-003`: H39 candidate WAT row overlap against
`/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/baseline_H39.parquet`
is at least 1461 rows after applying `--candidate-year-offset 2012`, unless the
executing agent records a baseline corpus defect with exact evidence and
updates this plan.

`MEASURE-HP247-004`: the semantic comparator reports no failures for the H39
primary water-balance closure set under
`tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`. The primary
set is `P`, `RM`, `Q`, `Ep`, `Es`, `Er`, `Dp`, `latqcc`, `Total-Soil`,
`SoilWaterTotal`, `frozwt`, `Snow-Water`, `ProfilePorosityCap`,
`ProfileFCStore`, `ProfileWPStore`, and `InterceptionStorage`. If a column is
not emitted or the comparator does not report it, the artifact must explain
whether that is expected by contract; otherwise closure remains `HOLD`.

`MEASURE-HP247-005`: an internal daily water-balance residual ledger for H39
shows coherent storage/flux closure for every simulated day with finite values.
The ledger must name the exact state and flux terms used, map each term to a
canonical contract symbol and production source, and use a residual threshold no
looser than `1.0e-6 mm` for purely internal conservation arithmetic unless a
larger threshold is contract-authorized because of output rounding.

`MEASURE-HP247-006`: every equation, constant, branch, and guard modified for
this package has canonical `SC-*` authority and pinned-baseline provenance.
Package-local artifacts may explain evidence but cannot replace canonical
contract text.

`MEASURE-HP247-007`: contract-derived tests fail against the pre-change defect
and pass after implementation. Tests must include typed guard behavior for
missing, non-finite, or out-of-domain required water-balance surfaces.

`MEASURE-HP247-008`: workspace gates pass or are dispositioned truthfully:
`cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
`cargo test --workspace`, and `cargo deny check`. If package edits touch
external-authority suite posture, cohort fixtures, or required-case bindings,
also run `bash tools/release/check_authority_suite_antievasion.sh` and
`cargo test --test auth11_required_suite_obligation_guards_contract`.


## Mandatory Sequence

Follow this sequence. Do not modify production hydrology code before the first
three steps are complete for the surface being changed.

1. Read required context and reproduce the current H39 failure.
2. Implement required canonical contract amendments.
3. Implement contract-derived tests.
4. Record pre-implementation contract-gate evidence.
5. Modify production code.
6. Run targeted tests and H39 hourly validation.
7. Iterate on contract, tests, and production fixes until H39 closure measures
   pass or a hard blocker is proven.
8. Run workspace gates and anti-evasion guards when applicable.
9. Complete dual review, dual verification, handoff, and disposition.


## Phase Plan

Phase 0 reproduces the current H39 hourly failure. At the end of this phase,
the package has a fresh `/tmp/hphys0247_*` root, manifest evidence proving
hourly mode, a comparator report showing current residuals, and a first
residual-ledger draft identifying where storage or flux continuity breaks.

Phase A consolidates canonical authority. At the end of this phase, every
water-balance surface that will be changed has updated `SC-*` contract text,
pinned legacy provenance, variable alias mappings, and a package-local
contract-authority map.

Phase B adds contract-derived tests and records the pre-implementation gate. At
the end of this phase, tests fail against the current defect, typed guard
behavior is covered for missing/non-finite/out-of-domain surfaces, and
`pre-implementation-contract-gate.md` proves production code has not yet been
changed for the tested surface.

Phase C implements baseline-authoritative production fixes. At the end of this
phase, hydrology kernel behavior follows the amended contracts, no heuristic or
proxy math has been introduced, and targeted tests pass.

Phase D validates H39 closure. At the end of this phase, the H39 hourly run,
manifest check, semantic comparator, and internal residual ledger satisfy the
closure measures or produce a proven blocker with exact evidence.

Phase E completes governance closure. At the end of this phase, workspace gates,
anti-evasion guards when applicable, dual review, dual verification,
disposition, owned-file manifest, and worker handoff are complete or the package
is explicitly held for the missing governance item.


## Truthfulness Labeling Requirement

Every evidence artifact must label statements as `Static:` when based on file
inspection and `Ran:` when based on a command that was actually executed. Do not
mark independent dual review or verification complete unless independent agent
outputs exist. If independent agents are unavailable, record that governance gap
and keep disposition in `HOLD` even if technical H39 closure passes.


## Dependencies

Read these files before editing:

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/work-packages/20260602-hphys0247-h39-single-ofe-hourly-waterbalance-closure-001/package.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260602-hphys0243-post-0242-39-hillslope-watershed-parity-readjudication-001/`
- `docs/work-packages/20260602-hphys0245-wb11-wb18-layer-telemetry-and-storage-continuity-001/`
- `docs/work-packages/20260602-hphys0246-wb18-aggregate-storage-writeback-closure-001/`
- `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`
- `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`

Use these existing run/evidence roots if still present:

- `/tmp/unpalatable_parity_20260529T192707Z/runs`
- `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/baseline_H39.parquet`
- `/tmp/hphys0243_20260602T042747Z/parity`
- `/tmp/hphys0245_20260602T051933Z`
- `/tmp/hphys0246_20260602T053935Z`


## Intended Write Set

The package may edit these files when justified by contract-first evidence:

- `docs/work-packages/README.md`
- `docs/work-packages/20260602-hphys0247-h39-single-ofe-hourly-waterbalance-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/00_pl_slot_resolution.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/01_phase_routing.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/04_kernel_execution.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/05_pl_phase_dispatch.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `crates/openwepp-runner/tests/simimpl04_wb13_publication_contract.rs`
- `tests/integration/wb11_hydrology_kernel_contract.rs`
- `tests/integration/wb12_reconciliation_kernel_contract.rs`
- `tests/integration/wb13_daily_water_balance_output_surface_contract.rs`
- `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs`
- `tests/integration/wb17_et_physics_kernel_contract.rs`
- `tests/integration/wb18_percolation_physics_kernel_contract.rs`
- `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs`
- `tests/integration/auth03_level4_constitutive_gate_contract.rs`
- `tests/integration/auth05_level4_constitutive_authority_hardening_contract.rs`
- `tests/fixtures/constitutive/**`

If the executing agent needs to edit outside this write set, update this
section first with the file path, reason, and contract authority that requires
the expansion.


## Plan of Work

Begin by reproducing the current H39 hourly failure from the existing parity
run inputs. Build `openwepp-cli-hill`, copy the known run corpus into a fresh
`/tmp/hphys0247_*` root, run only `p39_openwepp.run`, and capture the manifest,
logs, WAT parquet, and trace sidecar. Confirm that the manifest says the
selected, effective, and scheduler modes are all `hourly`.

Next, compare H39 candidate WAT output to the pinned baseline partition using
`semantic_hillslope_wat_compare.py` with the existing `pl14s` tolerance config
and `--candidate-year-offset 2012`. Record all failing water-balance columns
and top divergent row keys in `artifacts/h39-hourly-semantic-parity-evidence.md`.
Build an internal residual ledger in
`artifacts/h39-hourly-residual-ledger.md` that traces daily storage before/after
and flux terms across the scheduler phases. The ledger may use existing trace
surfaces such as `OPENWEPP_HPHYS0245_TRACE_PATH` if they contain enough detail.
If not, add contract-authorized diagnostics or tests that expose the required
symbols without changing production behavior.

Then update canonical contracts. If the residual root cause is WB19 lateral
transfer, amend `SC-SUBHYD-001.md` and any related `SC-WATBAL-001.md` text with
legacy symbol lineage, branch order, and handoff obligations before editing
production code. If the root cause spans WB12, WB13, WB14, WB17, or WB18,
update the matching `SC-*` contract first. Every contract amendment must cite
the pinned baseline source files and record variable aliases between legacy
symbols and openWEPP runtime names.

After contracts are amended, add or update contract-derived tests. Tests must
fail against the current pre-fix behavior. Prefer focused integration tests in
the existing water-balance contract test files over broad golden-output tests.
The tests must include typed error behavior for required missing or non-finite
state. Record pre-implementation contract-gate output before production edits.

Implement production fixes in the hydrology kernel with baseline-authoritative
logic. Preserve existing typed guard posture. Do not compensate for one residual
by tuning another flux. Do not change comparator tolerances as a substitute for
physics migration. If a temporary diagnostic surface is needed, gate it under an
explicit package trace environment variable or test-only helper and remove or
document it before disposition.

Finally, rerun H39 validation until the closure measures pass or a hard blocker
is proven. Run targeted Rust tests first, then workspace gates. Update every
artifact listed below, complete dual review and verification where available,
and write a final disposition that a later agent can use without any memory of
this conversation.


## Concrete Steps

Run all commands from `/home/workdir/openWEPP` unless noted otherwise.

Build the hillslope CLI:

    cargo build -p openwepp-runner --bin openwepp-cli-hill

Create an isolated HPHYS0247 root from the existing parity run corpus:

    ROOT=/tmp/hphys0247_$(date -u +%Y%m%dT%H%M%SZ)
    mkdir -p "$ROOT/runs" "$ROOT/hillslope_output" "$ROOT/logs" "$ROOT/reports"
    cp -a /tmp/unpalatable_parity_20260529T192707Z/runs/. "$ROOT/runs/"
    printf '%s\n' "$ROOT" | tee /tmp/hphys0247_latest_root.txt

Run H39 in hourly mode with trace capture. The `OPENWEPP_HPHYS0245_*` names are
existing diagnostic hooks from prior packages; they may be reused unless this
package replaces them with clearer HPHYS0247 names:

    ROOT=$(cat /tmp/hphys0247_latest_root.txt)
    OPENWEPP_HPHYS0245_TRACE_PATH="$ROOT/hillslope_output/H39.hphys0247.trace.jsonl" \
    OPENWEPP_HPHYS0245_TRACE_MAX_DAYS=1461 \
      target/debug/openwepp-cli-hill \
        --run-dir "$ROOT/runs" \
        --run-file p39_openwepp.run \
        --output-dir "$ROOT/hillslope_output" \
        --policy compat \
        >"$ROOT/logs/H39.stdout.log" \
        2>"$ROOT/logs/H39.stderr.log"

Check that the manifest proves hourly mode:

    ROOT=$(cat /tmp/hphys0247_latest_root.txt)
    /workdir/wepppy/.venv/bin/python - <<'PY'
    import json
    import os
    from pathlib import Path

    root = Path(os.environ.get("ROOT", Path("/tmp/hphys0247_latest_root.txt").read_text().strip()))
    manifest_path = root / "hillslope_output" / "openwepp_hillslope_run_manifest.json"
    manifest = json.loads(manifest_path.read_text())
    values = {
        "selected_lane": manifest.get("selected_lane"),
        "effective_mode": manifest.get("mode_selection", {}).get("effective_mode"),
        "scheduler_mode": manifest.get("mode_selection", {}).get("scheduler_mode"),
    }
    print(values)
    assert values == {
        "selected_lane": "hourly",
        "effective_mode": "hourly",
        "scheduler_mode": "hourly",
    }, values
    PY

Run the H39 semantic comparator:

    ROOT=$(cat /tmp/hphys0247_latest_root.txt)
    /workdir/wepppy/.venv/bin/python \
      tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py \
      --baseline-wat /tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/baseline_H39.parquet \
      --candidate-wat "$ROOT/hillslope_output/H39.wat.parquet" \
      --report-json "$ROOT/reports/H39.semantic.json" \
      --tolerance-config tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json \
      --candidate-year-offset 2012 \
      --top-n 20

Run targeted tests as they are added. Use the most specific touched tests first,
then expand:

    cargo test --test wb19_lateral_drainage_physics_kernel_contract
    cargo test --test wb18_percolation_physics_kernel_contract
    cargo test --test wb13_daily_water_balance_output_surface_contract
    cargo test -p openwepp-runner --test simimpl04_wb13_publication_contract

Before final disposition, run workspace gates:

    cargo fmt --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo test --workspace
    cargo deny check

If this package changes external-authority suite posture, cohort fixtures, or
required-case bindings, also run:

    bash tools/release/check_authority_suite_antievasion.sh
    cargo test --test auth11_required_suite_obligation_guards_contract


## Validation and Acceptance

Acceptance is behavioral. A reviewer should be able to read the artifacts,
rerun the exact H39 commands, and observe hourly manifest selection, no H39
semantic comparator failures for the named water-balance closure set, and an
internal residual ledger that conserves storage and fluxes day by day.

The new or updated contract-derived Rust tests must fail against the defect
that this package fixes and pass after implementation. The H39 comparator
report at `$ROOT/reports/H39.semantic.json` must be copied or summarized into
`artifacts/h39-hourly-semantic-parity-evidence.md` with row counts, failed
column count, and top residual magnitudes. The internal residual ledger must
identify every nonzero residual before the fix and show closure after the fix.

The package cannot be `GO` if H39 passes only because tolerances were loosened,
diagnostic values were hidden, missing values were silently defaulted, or a
non-authoritative formula was added to production code. It also cannot be `GO`
if dual review or dual verification are required but unavailable; in that case
the technical closure can be marked complete while package disposition remains
`HOLD_PENDING_DUAL_REVIEW_VERIFICATION`.


## Exit Criteria

The package exits `GO` only when all HPHYS0247 closure measures pass, the H39
hourly run evidence is reproducible, contract-first sequencing is documented,
workspace gates are recorded, and dual review/verification artifacts are
complete. The package exits `HOLD` when technical closure is achieved but a
governance gate such as independent review or verification remains incomplete,
or when a known contract/profile obligation is not yet satisfied. The package
exits `BLOCKED` only when a required local baseline corpus, source authority,
toolchain, or invariant is unavailable after concrete recovery attempts are
recorded in `artifacts/blocker-log.md`.


## Idempotence and Recovery

All H39 run commands write to a fresh `/tmp/hphys0247_*` root and are safe to
repeat. Keep `/tmp/hphys0247_latest_root.txt` updated so later commands target
the intended run. If a run fails before writing WAT output, keep stdout and
stderr under the run root and record the failure in
`artifacts/blocker-log.md`.

If the existing `/tmp/unpalatable_parity_20260529T192707Z` corpus is missing,
do not invent substitute baseline data. Search package artifacts for the prior
run command that produced it, rerun that documented corpus generation if
available, and record the regenerated root and command transcript. If the
corpus cannot be regenerated locally, mark the package `BLOCKED` with exact
missing path evidence.

If a contract amendment proves that this package needs to touch files outside
the intended write set, update `Intended Write Set` before editing, then
continue. If production implementation reaches a hard contradiction with the
pinned baseline, record the baseline source lines, openWEPP source lines,
failed command, and reason the package cannot proceed.


## Artifacts and Notes

Required artifacts live under
`docs/work-packages/20260602-hphys0247-h39-single-ofe-hourly-waterbalance-closure-001/artifacts/`.
Every artifact starts queued and must be updated during execution:

- `contract-authority-map.md`
- `contract-implementation-evidence.md`
- `contract-test-implementation-evidence.md`
- `pre-implementation-contract-gate.md`
- `implementation-test-evidence.md`
- `kernel-profile-compliance-checklist.md`
- `h39-hourly-baseline-reference-evidence.md`
- `h39-hourly-waterbalance-run-evidence.md`
- `h39-hourly-residual-ledger.md`
- `h39-hourly-semantic-parity-evidence.md`
- `h39-hourly-closure-dashboard.md`
- `gate-results.md`
- `owned-file-manifest.md`
- `blocker-log.md`
- `hphys0247_disposition.md`
- `worker-handoff.md`
- `review_agent_a.md`
- `review_agent_b.md`
- `verification_agent_a.md`
- `verification_agent_b.md`

At completion, package artifacts should make the final state reproducible from
the checked-in repository plus named `/tmp` evidence roots. Do not rely on
conversation memory.


## Interfaces and Dependencies

The executing agent should use existing Rust hydrology kernel modules rather
than creating a parallel water-balance engine. The likely implementation
interfaces are in `crates/openwepp-hillslope-orchestrator/src/hydrology/`, with
phase dispatch in `05_pl_phase_dispatch.rs`, kernel execution in
`04_kernel_execution.rs`, and WB phase support in
`03_kernel_support_01_kernel_phases.rs`. The runner-facing H39 CLI behavior is
in `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`.

Contract tests should stay near existing test surfaces: integration tests under
`tests/integration/` for canonical hydrology contracts and runner tests under
`crates/openwepp-runner/tests/` for WAT publication and CLI output behavior.
When adding fixture-based constitutive tests, update fixture provenance and
hash files and run the anti-evasion guards required by root `AGENTS.md`.

The H39 semantic comparator is a Python tool, not Rust production code:
`tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`. It compares
legacy and openWEPP WAT outputs and writes a JSON report. Use it as validation
evidence, not as production logic.


## Security Impact and Review Gate

- security_impact: low
- dedicated_security_review_required: no
- rationale: local Rust hydrology kernel, canonical contracts, tests, and local
  comparator runs; no credentials, network writes, authentication surfaces, or
  external service integration are in scope.

Revision note, 2026-06-02: initial scaffold created to convert H39 hourly
water-balance closure into a larger autonomous package with explicit success
criteria, correctness authority, commands, and evidence artifacts.
