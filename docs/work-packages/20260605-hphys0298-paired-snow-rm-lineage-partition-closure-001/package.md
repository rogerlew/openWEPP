# HPHYS0298 Paired Snow/RM Lineage Partition Closure

Status: hold

This work package is an autonomous ExecPlan and must remain a living document
during execution. It follows `/workdir/openWEPP/docs/codex_exec_plans.md`.

## Purpose / Big Picture

After this package, an agent can explain exactly where each of the nine
H1/H7/H39 snow/`RM` residual windows first diverges between openWEPP and the
pinned WEPP forest baseline. The package replaces aggregate reconstruction with
paired lineage observation: it instruments the same winter producer and
water-balance cut-points in both models, proves that observation does not change
baseline outputs, runs the full H1..H39 semantic suite, and publishes a
per-window source-partition ledger.

The goal is not to reproduce known baseline defects or compensate downstream
WB17/WB18/WB19/WB13 residuals. The goal is to partition defect source strongly
enough that a follow-on correction, or an in-package correction when fully
proven and contract-authorized, is directed at the first responsible process
boundary.

## Progress

- [x] (2026-06-05) Scaffolded autonomous package and kickoff prompt.
- [x] Amend canonical `SC-*` contracts for paired lineage partition authority.
- [x] Add contract-derived HPHYS0298 guard test.
- [x] Record pre-implementation contract gate before instrumentation work.
- [x] Build isolated instrumented baseline diagnostics and prove observe
  identity.
- [x] Add matching openWEPP lineage trace schema and capture all nine target
  windows.
- [x] Run full H1..H39 semantic suite on the same HEAD used for traces.
- [x] Publish per-window first-divergence partition ledger.
- [x] Apply only contract-authorized production fixes, if first-divergence
  evidence makes one unambiguous and in scope.
- [x] Run validation gates, dual reviews, dual verification, disposition, and
  worker handoff.

## Surprises & Discoveries

- Observation: Baseline observe instrumentation in the detached
  `/tmp/hphys0298_wepp_forest_obs` worktree is output-neutral for H1/H7/H39.
  Evidence: `artifacts/baseline-observe-identity.md`.
- Observation: All nine target windows first diverge before downstream storage
  consumers at the hourly snow/rain forcing cut-point.
  Evidence: `artifacts/paired-lineage-summary.md`.
- Observation: The `OPENWEPP-DEFECTIVE` verdict is a porting-fidelity defect
  against the unimpeached pinned-baseline precipitation-phase partition at
  `/workdir/wepp-forest_260430_baseline/src/winter.for:410-412`, not a generic
  baseline-diff claim.
  Evidence: `artifacts/claude-code-review-findings.md`.
- Observation: No production physics patch was applied because the first
  divergent snow/`RM` producer behavior requires a follow-on
  baseline-authoritative winter hourly snow/rain forcing migration, not
  compensation in WB13/WB17/WB18/WB19.
  Evidence: `artifacts/disposition.md`.

## Decision Log

- Decision: Use one large package for paired observation and source partition.
  Rationale: HPHYS0296 and HPHYS0297 both stalled on aggregate evidence. The
  next useful unit is not another single-window package; it is a shared
  instrumented lineage harness that can classify all nine target windows with
  the same evidence standard.
  Date/Author: `2026-06-05` / `Codex`.
- Decision: Treat `/workdir/wepp-forest_260430_baseline` observe
  instrumentation as diagnostic evidence only, not normative physics authority.
  Rationale: the pinned baseline path and commit remain comparator provenance;
  added observe calls are local diagnostics and must prove output identity
  before their trace values can support a verdict.
  Date/Author: `2026-06-05` / `Codex`.
- Decision: Keep the corrected openWEPP negative-melt safety posture unless a
  stronger contract-authoritative defect is proven.
  Rationale: the user explicitly rejected reproducing the known negative-melt
  bug; HPHYS0297 showed that the pinned negative-melt branch alone does not
  reconstruct the residuals.
  Date/Author: `2026-06-05` / `Codex`.

## Outcomes & Retrospective

HPHYS0298 executed through contract amendments, contract-derived tests, paired
baseline/openWEPP diagnostics, full H1..H39 semantic metrics, dual review,
dual verification, and disposition.

Outcome: `HOLD`. The package met its diagnostic objective and assigned all nine
target windows to `OPENWEPP-DEFECTIVE`, but it did not close production physics.
The required follow-up is a baseline-authoritative winter hourly snow/rain
forcing partition package scoped to the first divergent cut-point identified
here, specifically the `winter.for:410-412` precipitation-phase partition
lineage.

## Context and Orientation

openWEPP is the Rust simulation engine. WEPP forest is the legacy Fortran
baseline used for comparator evidence. The canonical baseline path for this
work is `/workdir/wepp-forest_260430_baseline`, not `/workdir/wepp-forest`,
unless a file is explicitly cited as corrected-fix history rather than
comparator authority.

HPHYS0297 found that all nine target windows remain unresolved. Six
first-2013/spring-2014 windows contain material negative raw hourly melt, but
the pinned-baseline negative-melt branch at
`/workdir/wepp-forest_260430_baseline/src/winter.for` does not reconstruct
baseline `RM` to the named tolerance. The three spring-2016 windows have
immaterial negative raw melt and remain winter producer magnitude/timing holds.

The existing baseline has an opt-in observe hook:
`/workdir/wepp-forest_260430_baseline/src/wepp_observe.for` writes
`wepp_observe.log` when a file named `wepp_observe.on` exists in the run current
working directory. The release binaries `wepp_260430` and `wepp_260430_hill`
contain this hook, but existing winter tags only report coarse guard/call
counts. HPHYS0298 must add targeted diagnostic tags in an isolated baseline
worktree or reversible dirty build and must prove that the added observation
does not change published baseline outputs.

The nine target windows are:

- `H1`, first-2013, year `2013`, days `112-127`.
- `H1`, spring-2014, year `2014`, days `120-146`.
- `H1`, spring-2016, year `2016`, days `104-111`.
- `H7`, first-2013, year `2013`, days `112-127`.
- `H7`, spring-2014, year `2014`, days `120-146`.
- `H7`, spring-2016, year `2016`, days `104-111`.
- `H39`, first-2013, year `2013`, days `97-112`.
- `H39`, spring-2014, year `2014`, days `120-146`.
- `H39`, spring-2016, year `2016`, days `104-111`.

The term `first divergent cut-point` means the earliest named state boundary in
the ordered lineage where baseline and openWEPP stop agreeing within the
package tolerance. The ordered cut-points are:

1. winter branch gate and pre-call forcing,
2. hourly snow/rain/temperature/radiation forcing consumed by winter,
3. raw hourly melt before negative-melt correction,
4. daily negative-melt summary and correction branch,
5. post-winter daily `wmelt`, rain restoration, and flags,
6. runoff/infiltration driver input,
7. WB13 `RM` publication and WB12 `Q` consumer identity,
8. WB17/WB18/WB19 aggregate storage consumers.

## Included Scope

This package includes contract-first authoring, diagnostic observability, source
partition, and bounded correction authority:

- Amend canonical `SC-SNOWFREEZE-001`, `SC-RUNOFFPART-001`, and
  `SC-WATBAL-001` with HPHYS0298 paired lineage partition requirements.
- Add a static contract-derived test
  `tests/integration/hphys0298_paired_lineage_partition_contract.rs`.
- Create an isolated instrumented baseline diagnostics worktree from
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`, or use a reversible local dirty
  patch only when a worktree cannot be created.
- Add targeted Fortran `wepp_observe` calls at the winter and water-balance
  cut-points needed to classify all nine windows.
- Add matching openWEPP trace fields with the same canonical symbol names,
  units, and target-window selectors.
- Build a paired harness under this package's `artifacts/` directory that runs
  full H1..H39 metrics and target-window traces.
- Publish an observe identity artifact proving instrumented baseline
  observe-off and observe-on outputs match the pinned baseline comparator to
  the declared tolerance.
- Publish a per-window partition ledger with first divergent cut-point,
  source-line references, values, tolerance, verdict, and next action.
- If, and only if, the first divergent cut-point proves an openWEPP production
  defect inside the declared snow/`RM` lineage and canonical contracts fully
  authorize the correction, apply the minimal production fix inside this
  package and rerun all required gates.

## Excluded Scope

- Do not reproduce the pinned-baseline negative-melt sign/scale bug as
  openWEPP target physics.
- Do not patch WB17 `Ep`/`Es`, WB18 percolation, WB19 lateral flow, or WB13
  aggregate publication as compensation for an upstream snow/`RM` residual.
- Do not reintroduce parser compatibility for `wepp_observe*` sidecars; they
  remain unsupported parser surfaces in openWEPP.
- Do not use instrumented baseline traces when observe identity fails.
- Do not leave `/workdir/wepp-forest_260430_baseline` dirty unless the final
  handoff explicitly records why cleanup was impossible and includes an exact
  recovery command.
- Do not classify a residual as accepted or exclude it from semantic accounting
  without reconstruction, first-divergence evidence, and independent correctness
  rationale.

## Deliverables

- Canonical `SC-*` amendments and revision-history entries.
- Contract-derived integration test and `Cargo.toml` registration.
- Isolated baseline instrumentation patch artifact.
- Baseline observe identity evidence.
- OpenWEPP paired lineage trace schema and implementation evidence.
- Full H1..H39 semantic suite metrics.
- Target-window partition ledger with all nine windows classified.
- Optional production correction evidence, only when contract-authorized.
- Kernel-profile compliance checklist, owned-file manifest, gate results,
  review artifacts, verification artifacts, review disposition, final
  disposition, and worker handoff.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/subsystems/observability/legacy-observe-migration.md`
- `docs/specifications/subsystems/observability/observability-subsystem-contract.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260605-hphys0296-snow-rm-producer-acceptance-authority-closure-001/artifacts/review-disposition.md`
- `docs/work-packages/20260605-hphys0297-snow-rm-defect-ledger-reconstruction-closure-001/package.md`
- `docs/work-packages/20260605-hphys0297-snow-rm-defect-ledger-reconstruction-closure-001/artifacts/defect-ledger.md`
- `docs/work-packages/20260605-hphys0297-snow-rm-defect-ledger-reconstruction-closure-001/artifacts/reconstruction-evidence.md`
- `/workdir/wepp-forest_260430_baseline/src/wepp_observe.for`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/contin.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/workdir/wepp-forest_260430_baseline/src/grna.for`
- `/workdir/wepp-forest_260430_baseline/src/idat.for`
- `/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill`
- `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `docs/work-packages/20260605-hphys0297-snow-rm-defect-ledger-reconstruction-closure-001/artifacts/hphys0297_defect_ledger.py`

## Intended Write Set

- `Cargo.toml`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/work-packages/README.md`
- `tests/integration/hphys0298_paired_lineage_partition_contract.rs`
- `crates/openwepp-hillslope-orchestrator/src/**`
- `crates/openwepp-runner/src/**`
- `tools/legacy_comparison_suite/**`, only if the paired harness belongs in
  shared tooling rather than package-local artifacts.
- `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/**`
- Diagnostic-only temporary worktree or reversible patch under
  `/workdir/wepp-forest_260430_baseline/**`, with final cleanup required.

## Contract-First Sequence

1. Implement required canonical contract amendments.
2. Implement contract-derived tests.
3. Record pre-implementation contract-gate evidence.
4. Modify diagnostics and production code only after the contract gate has run.

The package must keep truthfulness labels in every evidence artifact:
`Static:` for source/document inspection and `Ran:` for executed commands.

## Milestones

### Milestone 1: Contract Authority and Static Gate

Amend `SC-SNOWFREEZE-001` with the paired winter lineage requirements,
including canonical symbols for `snodpy`, `frdp`, `rain`, `wmelt`, `hrmlt`,
`hrrain`, `hrsnow`, `pstvML`, `ngtvML`, `pstvhr`, `snodpt`, `densgt`, and
target-window selectors. Amend `SC-RUNOFFPART-001` so `Q` and routing consumers
must use the snow/`RM` partition ledger before any downstream re-tiering. Amend
`SC-WATBAL-001` so storage/ET/percolation residuals cannot be patched as
compensation when the first divergent cut-point is upstream in winter
production.

Add `tests/integration/hphys0298_paired_lineage_partition_contract.rs` and
register it in `Cargo.toml`. The test must fail if the contract text omits the
baseline path `/workdir/wepp-forest_260430_baseline`, the nine target windows,
observe identity, first-divergence cut-points, no-downstream-compensation
language, or the requirement for dual review and verification.

Run:

    cargo test --test hphys0298_paired_lineage_partition_contract -- --nocapture

Record the result in `artifacts/pre-implementation-contract-gate.md` before
changing diagnostics or production code.

### Milestone 2: Baseline Observation Without Output Drift

Create an isolated diagnostic worktree when possible:

    git -C /workdir/wepp-forest_260430_baseline status -sb
    rm -rf /tmp/hphys0298_wepp_forest_obs
    git -C /workdir/wepp-forest_260430_baseline worktree add --detach /tmp/hphys0298_wepp_forest_obs dac3c950d8b16cc73774bf5ce2e7e11f80baac70

If worktree creation is not possible, apply a reversible patch directly in
`/workdir/wepp-forest_260430_baseline`, save the patch to
`artifacts/baseline-instrumentation.patch`, and restore the repo before final
handoff.

Add `wepp_observe` calls in the diagnostic tree only. Use the existing
`wepp_observe(tag, year, sdate, ielmt, ichan, iseg, v1, v2)` signature. Store
hour in `iseg` for hourly rows and OFE/hillslope in `ielmt`. Use stable tags
with an `H298_` prefix. At minimum emit:

- `H298_GATE_A`: `snodpy`, `tmin`.
- `H298_GATE_B`: `frdp`, `rain`.
- `H298_GATE_C`: `warain`, `xmxint`.
- `H298_RAW_A`: `hrmlt(hour,iplane)`, `hrrain(hour)`.
- `H298_RAW_B`: `hrsnow(hour)`, `snodpt(iplane)`.
- `H298_NEG_A`: `pstvML`, `ngtvML`.
- `H298_NEG_B`: `float(pstvhr)`, correction factor when defined.
- `H298_POST_A`: `wmelt(iplane)`, `totmel`.
- `H298_POST_B`: `snodpt(iplane)`, `densgt`.
- `H298_DRV_A`: post-winter `rain(iplane)`, `wmelt(iplane)`.
- `H298_DRV_B`: `float(nomelt)`, `xmxint(iplane)`.
- `H298_WB_A`: WB `rain(iplane)`, `wmelt(iplane)`.
- `H298_WB_B`: `irdept(iplane)`, `iraplo(iplane)`.
- `H298_WB_C`: computed `rm`, `runoff(iplane)`.

Build an instrumented diagnostic binary and run three lanes for H1, H7, and
H39 target cases:

1. pinned release comparator with no observe file,
2. instrumented binary with no `wepp_observe.on`,
3. instrumented binary with `wepp_observe.on`.

The observation lane is usable only if lane 2 matches lane 1 and lane 3 matches
lane 2 for target published outputs within declared tolerance. Record exact
commands, binary path, compiler, git status, and checksum/equality summary in
`artifacts/paired-observe-identity-evidence.md`.

### Milestone 3: Matching openWEPP Trace Schema

Add an explicit openWEPP trace schema for HPHYS0298, preferably package-local
diagnostics first and production trace fields only where existing trace
infrastructure already supports opt-in publication. The schema must use the
same canonical symbol names and units as the baseline observe tags. It must be
disabled by default and selected by explicit target windows, not by parser
sidecar compatibility.

The trace rows must include:

- `run_id`, `hillslope_id`, `year`, `julian_day`, `hour`, `cut_point`.
- `canonical_symbol`, `openwepp_symbol`, `unit`, `value`.
- `source_path`, `source_line_or_function`, `schema_version`.
- `target_window_id`.

Record schema details in `artifacts/target-window-lineage-schema.md`.

### Milestone 4: Paired Harness and Partition Ledger

Implement a package-local harness, for example
`artifacts/hphys0298_paired_lineage_partition.py`, that:

1. runs the full H1..H39 semantic suite on current openWEPP HEAD,
2. runs the three baseline observation identity lanes,
3. captures baseline and openWEPP target-window traces for all nine windows,
4. normalizes units to meters for internal comparisons and millimeters for
   ledger presentation,
5. joins rows by `(hillslope_id, year, julian_day, hour, cut_point,
   canonical_symbol)`,
6. identifies the first divergent cut-point using declared tolerances,
7. writes JSON and Markdown ledgers.

The ledger row for each target window must contain:

- target window ID and date range,
- baseline published `RM`, openWEPP published `RM`, residual, and tolerance,
- first divergent cut-point,
- first divergent canonical symbols,
- baseline values and source lines,
- openWEPP values and source lines,
- whether `Q` remains closed,
- whether WB13 `RM` identity remains closed,
- final verdict: `LEGACY-DEFECTIVE`, `OPENWEPP-DEFECTIVE`, or `UNRESOLVED`,
- next action and any prohibited compensation paths.

Write the Markdown summary to `artifacts/partition-ledger.md` and the full JSON
to the package run root. Copy or summarize the full H1..H39 semantic metrics in
`artifacts/full-39-suite-metrics.md`.

### Milestone 5: Optional Minimal Correction

If the partition ledger proves an `OPENWEPP-DEFECTIVE` first-divergence inside
the declared snow/`RM` lineage, the executor may apply the minimal production
fix inside this package only when all of the following are true:

1. canonical `SC-*` contracts already authorize the exact behavior,
2. the fix is baseline-authoritative or physically/correctness-authoritative
   under ADR-0011/ADR-0012,
3. the fix does not reproduce a known defective baseline branch,
4. the fix does not compensate in WB17/WB18/WB19/WB13,
5. contract-derived tests and full H1..H39 metrics are rerun after the fix.

If any condition fails, do not patch production physics. Keep the package in
`HOLD` with the ledger and handoff identifying the next work item.

### Milestone 6: Review, Verification, and Cleanup

Run all required validation gates that are feasible in the local environment:

    cargo fmt --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo test --workspace
    cargo deny check
    bash tools/release/check_authority_suite_antievasion.sh
    cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture
    wctl doc-lint --path docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001

Dispatch two independent reviews and two independent verifications. Every
finding must be dispositioned as `accepted`, `rejected`, `deferred`, or
`follow-up`. Accepted findings must be fixed and verified. Rejected findings
must include rationale. The package cannot close while any review finding is
undispositioned.

Before final handoff, verify:

    git -C /workdir/wepp-forest_260430_baseline status -sb
    git -C /workdir/openWEPP status -sb

Record cleanup status and any remaining dirty diagnostic worktree paths in
`artifacts/worker-handoff.md`.

## Validation and Acceptance

This package is successful when all of the following are true:

- Canonical contracts and the HPHYS0298 contract test exist and pass.
- Baseline observe identity is proven before trace values are used.
- Full H1..H39 semantic metrics are recorded on the same openWEPP HEAD used for
  paired traces.
- All nine H1/H7/H39 target windows have a partition ledger row with a first
  divergent cut-point and verdict.
- No residual is accepted or excluded from semantic accounting without a
  ledger-backed verdict and correctness rationale.
- No downstream WB17/WB18/WB19/WB13 compensation patch is applied.
- `/workdir/wepp-forest_260430_baseline` is returned to clean state or an exact
  recovery command is recorded.
- Dual review, disposition, and verification artifacts are complete before any
  non-HOLD closure.

## Idempotence and Recovery

The package should be safe to rerun. Use run roots under `/tmp/hphys0298_*` and
remove/recreate them between attempts. Prefer a detached baseline worktree over
editing `/workdir/wepp-forest_260430_baseline` directly. If a direct baseline
patch is unavoidable, save the patch artifact first and restore with:

    git -C /workdir/wepp-forest_260430_baseline checkout -- src/wepp_observe.for src/winter.for src/contin.for src/watbal.for src/watbal_hourly.for src/grna.for src/idat.for
    git -C /workdir/wepp-forest_260430_baseline status -sb

If observe identity fails, stop using the trace for verdicts, record
`UNRESOLVED` with reason `OBSERVE_IDENTITY_FAIL`, and do not patch production
physics.

## Security-Impact Gate

No external systems, credentials, or network calls are required. Work is local
flat-file reads/edits, local builds, and local diagnostics. Subprocess commands
must use explicit argument arrays in Rust code; no shell interpolation is
allowed in production paths.

## Artifacts and Notes

Primary artifacts for execution:

- `artifacts/baseline-instrumentation-patch.md`
- `artifacts/paired-observe-identity-evidence.md`
- `artifacts/target-window-lineage-schema.md`
- `artifacts/openwepp-lineage-instrumentation-evidence.md`
- `artifacts/full-39-suite-metrics.md`
- `artifacts/partition-ledger.md`
- `artifacts/gate-results.md`
- `artifacts/disposition.md`
- `artifacts/worker-handoff.md`

Plan revision note: initial scaffold created on `2026-06-05` to convert
HPHYS0297 unresolved aggregate evidence into paired first-divergence lineage
partition evidence.
