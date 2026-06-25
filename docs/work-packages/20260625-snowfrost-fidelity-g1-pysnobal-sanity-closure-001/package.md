# SNOWFROST-FIDELITY-G1 PySnobal Sanity Closure

Status: complete

Package type: DC ExecPlan.

Primary defect: `PYSNOBAL-G1-001`.

Objective: close the G0 PySnobal bridge blocker by making the PySnobal harness
fast enough to isolate failed sensitivity lanes, by publishing real current
openWEPP snow comparison rows from WAT `Snow-Water` and `Snow-Depth`, and by
producing sane PySnobal SWE and snow-depth results for all five frost-depth
pilot sites so the program can decide whether PySnobal snow physics is worth
further evaluation for openWEPP.

This package follows `AGENTS.md`, `docs/work-packages/AGENTS.md`,
`docs/codex_exec_plans.md`, `docs/defect_closure_execplans.md`,
`crates/AGENTS.md`, `tests/AGENTS.md`,
`docs/specifications/science-contracts/AGENTS.md`,
`docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`,
`docs/work-packages/20260625-snowfrost-fidelity-g0-pysnobal-input-bridge-001/package.md`,
and the G0 disposition/handoff artifacts.

Subagent authorization: this package explicitly authorizes read-only
verification and harness-review subagents after implementation. Reviewers may
inspect code, package artifacts, and generated summaries; they may not edit
files. Each finding must be dispositioned as accepted, rejected, deferred, or
follow-up before closure.

## Purpose

G0 proved that openWEPP can export PySnobal forcing, and that 14 of 15
site/lane runs produce finite SWE and physical snow depth. It held because the
Morris `Tg=-0.5 degC` sensitivity lane aborts in PySnobal C code and because
`openwepp_snow.csv` was intentionally empty. After this package, a user can run
the Rust exporter and thin Python harness and see five frost sites with at
least one sane PySnobal lane each, plus PySnobal-vs-openWEPP snow-depth
comparison metrics sourced from current WAT publication.

PySnobal remains diagnostic evidence only. It is not a correctness authority
for openWEPP snow or frost physics, and no production physics is changed here.

## Correction Authority Envelope

Defect `PYSNOBAL-G1-001`: G0 cannot use PySnobal as a SNOWFROST-FIDELITY
diagnostic comparator because one sensitivity lane aborts and the harness has
no site/lane/window controls for minimal reproduction.

Defect `PYSNOBAL-G1-002`: G0 emits `openwepp_snow.csv` with no rows, so
PySnobal-vs-current-openWEPP depth comparisons are not metric-bearing.

In scope:

- `tools/snowfreeze_observed/pysnobal_compare.py`;
- `crates/openwepp-runner/src/hillslope/snowbench.rs`;
- `crates/openwepp-runner/src/bin/openwepp-snowbench.rs`;
- public re-exports in `crates/openwepp-runner/src/lib.rs` and
  `crates/openwepp-runner/src/hillslope/mod.rs`;
- focused integration tests under `tests/integration/`;
- package artifacts under this directory;
- documentation pointers in `tools/snowfreeze_observed/README.md` and
  `docs/work-packages/README.md`.

Allowed edit classes:

- add harness site/lane/window filtering;
- add a strict `all-lanes` route and an explicit `site-sane` route;
- add validated reuse of existing full-run PySnobal outputs;
- generate WAT-backed `openwepp_snow.csv` rows from the existing openWEPP
  compatibility publication path;
- add focused tests for schema, anti-alias, and WAT snow projection.

Out of scope:

- production snow/frost physics;
- snow/frost observation tolerances or science-contract threshold changes;
- PySnobal vendoring or source edits;
- default activation of direct runtime;
- declaring openWEPP snow or frost defective from PySnobal agreement alone.

## Conversion Rule

If the blocker is caused by harness controls, diagnostic route policy, or
missing current-openWEPP snow rows, fix it in this package. If the blocker is
inside PySnobal source code or a sensitivity lane proves out-of-envelope for a
diagnostic proxy, this package must record that boundary and still produce a
site-sane comparator route when every site has at least one passing lane. It may
not stop at "inspect another function."

## Seven-Gate Bar

1. Reproduce or preserve the G0 held symptom from existing artifacts.
2. Localize the symptom to a named mechanism.
3. Classify ownership: bridge/harness/openWEPP output projection, PySnobal
   source, or invalid diagnostic lane.
4. State authority: observed snow depth and `SC-SNOWFREEZE-001` remain
   correctness authority; PySnobal is hypothesis evidence only.
5. Add tests or artifact checks for the accepted correction.
6. Implement the correction without production physics edits.
7. Validate with focused tests, package evidence, dual review/disposition, and
   final route recommendation.

## Phase Plan

### Phase 0: Scaffold and Evidence Intake

Record the G0 hold, the Morris failed lane, the passing adjacent lanes, and the
empty `openwepp_snow.csv` gap.

Exit criteria: `artifacts/pre-implementation-evidence.md` exists and labels
claims as `Static:` or `Ran:`.

### Phase 1: Harness and Projection Edits

Add site/lane/window filters, validated reuse, `all-lanes` versus `site-sane`
route policy, and WAT-backed current-openWEPP snow projection.

Exit criteria: focused build, Python compile, and bridge contract tests pass.

### Phase 2: Five-Site Sane PySnobal Evidence

Export current five-site G1 artifacts, run the PySnobal harness in a
site-sane configuration, and record max SWE, max snow depth, observed residuals
where available, and PySnobal-vs-openWEPP depth residuals.

Exit criteria: every pilot site has at least one `PASS` lane and the route is
`PROCEED-SNOWFROST-FIDELITY-G1-SANE-SITE-LANES`, or the package reaches a
declared legitimate HOLD boundary.

### Phase 3: Review, Disposition, and Handoff

Run required gates, record line-count governance, disposition reviews, and
state whether PySnobal remains worth pursuing as a diagnostic comparator.

Exit criteria: no undispositioned findings, package status is complete or a
legitimate HOLD with a defect-shaped handoff.

## HOLD Legitimacy

This package may hold only if PySnobal is unavailable in the local environment,
if no site can produce any passing PySnobal lane, if current-openWEPP WAT
publication cannot produce snow rows, or if required evidence cannot be
generated. A failed optional sensitivity lane is not by itself a legitimate
HOLD after the site-sane route has passing coverage for all five sites.

## Progress

- [x] (2026-06-25) Scaffolded the DC ExecPlan and correction envelope.
- [x] (2026-06-25) Added WAT-backed `openwepp_snow.csv` projection and focused
  projection test.
- [x] (2026-06-25) Added harness site/lane/window filters, validated reuse, and
  explicit `site-sane` route policy.
- [x] (2026-06-25) Generated fresh five-site G1 exports under
  `target/snowfrost_fidelity_g1`.
- [x] (2026-06-25) Ran PySnobal site-sane evidence for all five sites and
  routed `PROCEED-SNOWFROST-FIDELITY-G1-SANE-SITE-LANES`.
- [x] (2026-06-25) Reproduced the Morris `Tg=-0.5 degC` strict full-lane
  PySnobal failure and proved a January 1980 window passes.
- [x] (2026-06-25) Recorded validation, review disposition, line-count
  governance, and final
  route recommendation.

## Surprises & Discoveries

- Observation: the original bridge integration test is expensive because it
  materializes a full 45-year hourly forcing fixture.
  Evidence: `cargo test --test snowfrost_fidelity_g0_pysnobal_bridge_contract`
  reported the exporter test running for over 60 seconds even with openWEPP snow
  projection disabled.

- Observation: WAT `year/month/day` columns are simulation-calendar values and
  are not suitable as external date strings for `openwepp_snow.csv`.
  Evidence: the first G1 reuse summary failed on dates such as `0001-02-29`.
  The fix maps WAT `sim_day_index` to `DailyForcingExport.date`.

- Observation: the Morris `Tg=-0.5 degC` PySnobal failure is not triggered by a
  short January 1980 window.
  Evidence: full-lane run reproduces `sati.c:17`; window
  `1980-01-01` through `1980-01-31` passes with max SWE 10.125579 kg/m2 and max
  depth 0.106678 m.

## Decision Log

- Decision: keep the harness default route as strict `all-lanes`, and require
  G1 to opt into `site-sane` explicitly.
  Rationale: this preserves G0's strict sanity gate while allowing the program
  to assess PySnobal viability when every site has at least one passing
  diagnostic lane and failed sensitivity lanes remain visible.
  Date/Author: 2026-06-25 Codex.

- Decision: project current openWEPP snow rows from compatibility WAT parquet
  instead of recalculating snow in the exporter.
  Rationale: WAT `Snow-Water` and `Snow-Depth` are the published comparison
  surfaces under SNOWFROST-FIDELITY-D/E/F; recalculating would create an alias.
  Date/Author: 2026-06-25 Codex.

- Decision: map WAT rows to external climate dates by `sim_day_index`.
  Rationale: WAT simulation-calendar fields can produce invalid ISO dates for
  leap years, while PySnobal forcing and observations are paired by external
  climate date.
  Date/Author: 2026-06-25 Codex.

## Outcomes & Retrospective

Complete. G1 resolves the G0 blocker for comparator-routing purposes by
retaining strict all-lane failure visibility while adding an explicit
site-sane route. Current openWEPP WAT-backed snow rows are available for all
five sites, selected `Tg=0.0 degC` PySnobal outputs are sane for all five
sites, and the final route is
`PROCEED-SNOWFROST-FIDELITY-G1-SANE-SITE-LANES`.

The Morris `Tg=-0.5 degC` full-run sensitivity lane still fails in PySnobal C
code; a short window passes. That lane remains a failed sensitivity probe, not
a blocker for the selected-lane diagnostic route and not evidence for
production physics changes.

## Validation Commands

Run from `/home/workdir/openWEPP`:

- `cargo build -p openwepp-runner --bin openwepp-snowbench`
- `cargo test -p openwepp-runner snowbench::tests`
- `cargo test --test snowfrost_fidelity_g0_pysnobal_bridge_contract`
- `.venv/bin/python -m py_compile tools/snowfreeze_observed/pysnobal_compare.py`
- five-site `openwepp-snowbench export-pysnobal` commands into
  `target/snowfrost_fidelity_g1/site{1..5}`
- G1 PySnobal harness command with `--route-policy site-sane`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `git diff --check`

## Defect-Shaped Handoff

If this package cannot close, the first actionable item must be: close defect
`PYSNOBAL-G1-001` or `PYSNOBAL-G1-002` at the named boundary, not perform a
next inspection step.
