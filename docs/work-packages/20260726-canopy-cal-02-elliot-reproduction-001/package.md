# CANOPY-CAL-02 Elliot Reproduction

Package ID: `20260726-canopy-cal-02-elliot-reproduction-001`

Status: `COMPLETE / NOT_REPRODUCIBLE / HOLD LIFTED`

Date opened: `2026-07-26`

Execution mode: `package-end-to-end`

Package type: scientific characterization, legacy comparator reproduction, and
campaign evidence.

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`. The `Progress`, `Surprises & Discoveries`,
`Decision Log`, and `Outcomes & Retrospective` sections must remain current
during execution.

## Purpose / Big Picture

Bill Elliot calibrated perennial WEPP forest managements by matching long-term
live biomass, annual biomass-to-residue transfer, and current/previous/old
forest-floor residue before comparing hydrology and erosion with constant
cover. This package converts that manual, screenshot-oriented analysis into a
machine-readable and reproducible experiment.

After execution, a reader can rerun the admitted Hubbard Brook and Santee
experiments, see whether the delivered or report-described management
reproduces Bill's figures and tables, and understand exactly which differences
come from missing source inputs, soil format, executable lineage, management
parameters, output parsing, or comparison scale.

This is a characterization package. It does not change native CP2 physics or
fit native parameters.

## Progress

- [x] (2026-07-26) Scaffolded the package and dependency boundary.
- [x] (2026-07-26) Located Bill's two report-linked WEPPcloud runs, preserved
  the selected 100-year source inputs/outputs, and identified the exact
  BLARHG Windows executable.
- [x] (2026-07-26) Consumed the superseding CANOPY-CAL-01 `READY_BOUNDED`
  admission record.
- [x] (2026-07-26) Admitted exact run inventory, executable lineage, result schema, tolerances,
  and gate plan.
- [x] (2026-07-26) Built and focused-tested the deterministic reproduction harness.
- [x] (2026-07-26) Recorded the superseded initial comparator attempt; WEPP 2012.800
  rejected the exact source-native 9002 soil before the first simulation.
- [x] (2026-07-26) Recorded the superseded initial process, hydrology, sediment, and return-period
  surfaces as `NOT_REPRODUCIBLE` or `NOT_EVALUATED` without crossing scales.
- [x] (2026-07-26) Issued the superseded initial `NOT_REPRODUCIBLE` verdict and CAL-03 handoff.
- [x] (2026-07-26) Completed the superseded initial review, finding disposition,
  verification, and final `EXECUTED_HOLD` package disposition.
- [x] (2026-07-26) Reopened under operator authority, built paired WEPPpy
  `2006.2` soils from mukeys 665220 and 131976, and completed all five arms.
- [x] (2026-07-26) Reconstructed daily, annual, equilibrium, hydrology,
  sediment, and daily-runoff return-period surfaces.
- [x] (2026-07-26) Issued the `NOT_REPRODUCIBLE` scientific verdict from a
  successful bounded reconstruction.
- [x] (2026-07-26) Executed the operator-authorized Linux WEPP 260725 lane with
  source-native 9002 soils, `wepp_ui.txt`, and `wepp_observe.on`.
- [x] (2026-07-26) Reconstructed annual gross aboveground live-to-current-residue
  transfer from the authoritative producer formula and rounded daily biomass;
  the operator accepted the bounded precision and lifted the sole hold.
- [x] (2026-07-26) Corrected review findings and passed two independent
  scientific reviews and two independent terminal verifications.

## Objective

Execute the CAL-01-admitted Hubbard Brook and Santee experiments for Bill's
delivered management files, the report-described Hubbard `dropfc=0.92` branch,
and like-for-like constant-cover comparators; publish machine-readable daily,
annual, equilibrium, and return-period results; reproduce or truthfully bound
the report's figures and tables; and issue one explicit reproduction verdict
without modifying canopy, litter, residue, hydrology, or erosion equations.

## Rationale

Bill's equilibrium-based workflow is the analytical foundation proposed for
the canopy campaign, but the delivered evidence is manual and may depend on
unavailable WEPP Windows inputs. A dedicated characterization package is needed
to separate an exact reproduction from a bounded reconstruction, expose the
92/95 management effect, and prevent soil-format or scale confounders from
being misattributed to canopy management.

## Dependency Admission Gate

CANOPY-CAL-01 must be complete and its exact
`artifacts/cal02-admission.json` must be current.

- `READY_EXACT` admits exact reproduction.
- `READY_BOUNDED` admits only the substitutes and claim limits enumerated by
  CAL-01. This package must use `BOUNDED_NOT_EXACT` language for affected
  results.
- `BLOCKED_SOURCE_BUNDLE` prevents execution. Do not run a guessed experiment
  and do not reinterpret the dependency as optional.

The executor must recompute the CAL-01 artifact identities before admitting
this package. Dependency drift returns CAL-02 to scaffolded/blocked state.

## Context And Orientation

The campaign roadmap is
`docs/planning/canopy-phenology-assurance-roadmap.md`. CAL-02 is Order 2.

Bill used WEPP Windows 2012.8, a stochastic 100-year climate, manually
transcribed 2006.5-format soils, and perennial forest managements. He inspected
aboveground live biomass plus current, previous, and old residue pools over
30-, 40-, and 100-year runs. The final report also compares annual runoff,
sediment delivery, daily-runoff return periods, and peak-flow return periods
against constant-cover WEPP Windows and WEPPcloud results.

The report-linked WEPPcloud runs are `unassailable-sensuousness/disturbed9002`
for Hubbard Brook and `clean-burning-griddle/disturbed9002` for Santee. The
selected report surfaces are Hubbard `p1`/`H1` and Santee `p2`/`H2`. Their
exact 100-year source inputs, selected outputs, metadata, and WEPPcloud
return-period JSON are preserved under this package's fixture subtree.

BLARHG exposes `C:\WEPP\wepp\wepp_2012.exe`, the report-era Windows executable,
with SHA-256
`6104a3440624ad54aa6c3660794280adfd600d4a11b98559c6205a73cd47fc3f`.
Bill's manually transcribed Windows soil/slope projects, Windows run controls,
and Windows text outputs are still absent, so historical Windows byte/output
equivalence remains bounded. They do not block the analytical comparison:
source-native `p1.man` and `p2.man` are the hash-bound site-specific
constant-cover comparators described by Bill's method.

The openWEPP production native canopy has one aggregate residue pool. This
package reproduces Bill in the admitted legacy/perennial representation; native
aggregate and shadow-cohort analysis belongs to later roadmap packages.

## Included Scope

- Install deterministic Hubbard Brook and Santee reproduction fixtures from
  CAL-01-admitted source-native inputs.
- Preserve source-native files plus manifests; document every transformation.
- Derive a diagnostic run control from each preserved source control by
  enabling only WEPP's existing daily plant/residue output and assigning its
  arm-local output path. This output-only switch does not alter forcing,
  management, soil, slope, or process physics.
- Build a deterministic analysis harness under `tools/canopy_phenology/`.
- Record exact executable identity, build/provenance, run commands, environment,
  and output paths.
- Run these minimum management arms:
  - Hubbard source-native `p1.man` constant cover;
  - Hubbard delivered hardwood (`dropfc=0.95`);
  - Hubbard report-described hardwood (`dropfc=0.92`);
  - Santee source-native `p2.man` constant cover; and
  - Santee delivered mixed forest.
- Use exact source-native `p1.man` and `p2.man` as the hash-bound,
  site-specific constant-cover comparators. Compare their analytical results
  with Bill's Windows rows while labeling historical Windows byte/output
  equivalence as unavailable.
- Keep 7777 and 2006.5 soil representations separate. Run paired soil-format
  arms only when CAL-01 authorizes a like-for-like conversion or exact source.
- Operator follow-on authorization on 2026-07-26 admits a bounded, paired
  WEPPpy SSURGO `2006.2` serialization for exact mukeys `665220` and `131976`.
  This may lift the executable-format hold but may not be described as Bill's
  byte-identical manually transcribed Windows soil.
- Operator follow-on authorization also admits a separate Linux lane using
  `/workdir/wepp-forest_260430_baseline/release/wepp_260725`, exact source-native
  9002 soils, an empty `wepp_ui.txt` hourly-water-balance switch, and
  `wepp_observe.on`. This lane must remain separate from Windows/2006.2 results.
  Observe logs may support litter-transfer extraction only when a real
  producer callsite is demonstrated. The operator subsequently accepted a
  producer-authoritative reconstruction bounded by the crop output's
  `0.001 kg/m2` precision as close enough for this campaign.
- Parse daily live biomass, LAI, canopy, current/previous/old/total residue,
  runoff, peak runoff, and sediment surfaces where the admitted executable
  publishes them.
- Compute annual transfer, annual extrema/means, practical equilibrium, pool
  sums, annual runoff/sediment, daily-runoff return periods, and peak-flow
  return periods.
- Recreate Bill-style figures and report tables from retained result objects.
- Compare chart-derived targets using predeclared chart-resolution tolerances.
- Distinguish hillslope surface runoff and sediment from watershed/channel
  outputs.
- Issue one reproduction verdict and a handoff that tells CAL-03 which targets
  are independently reproduced, bounded, contradicted, or not reproducible.

## Excluded Scope

- Changing any openWEPP or legacy WEPP equation.
- Calibrating native GSI, foliar, structural, canopy, litter, or decomposition
  operands.
- Adding evergreen needle turnover or fine-woody litter physics.
- Modifying canonical science contracts, management schemas, production crates,
  or public output schemas.
- Treating agreement with legacy WEPP as independent correctness authority for
  native openWEPP.
- Comparing 7777 and 2006.5 results as if only management changed.
- Comparing hillslope surface runoff directly with watershed discharge
  containing lateral flow, baseflow, roads, or channels.
- Using Bill's unsourced/AI-attributed values as acceptance targets.

## Dependencies

- Completed `20260726-canopy-cal-01-source-target-ledger-001` with an admissible
  current verdict.
- Exact commissioned sources installed by CAL-01.
- Any executable or source bundle explicitly admitted by CAL-01.
- `docs/planning/canopy-phenology-assurance-roadmap.md`.

## Declared Write Set

- `docs/work-packages/README.md`
- `docs/work-packages/20260726-canopy-cal-02-elliot-reproduction-001/**`
- `tools/canopy_phenology/**`
- `tests/fixtures/canopy_phenology/elliot_reproduction/**`
- `.gitattributes` for path-confined Git LFS routing of large source-native
  climate and selected output files.

CAL-01 artifacts and references, existing canopy-gradient fixtures, production
code, contracts, schemas, other tests, and the sibling WEPPcloud repository are
read-only.

If execution proves a parser or output surface is unavailable, amend this
package before implementation to authorize the smallest analysis-only parser or
campaign output path. Do not write around the declared boundary and do not
change production physics.

## Required Deliverables

Execution must create and maintain:

- a fixture-family README, source manifest, SHA-256 manifest, and run recipes;
- deterministic harness source and harness tests;
- `artifacts/dependency-admission.md`
- `artifacts/executable-provenance.md`
- `artifacts/experiment-matrix.csv`
- `artifacts/result-schema.md`
- `artifacts/tolerance-rationale.md`
- `artifacts/run-manifest.json`
- `artifacts/daily-results.csv` or a manifest to retained partitioned results
- `artifacts/annual-results.csv`
- `artifacts/equilibrium-results.csv`
- `artifacts/return-period-results.csv`
- `artifacts/report-comparison.csv`
- `artifacts/figures/`
- `artifacts/reproduction-verdict.json`
- `artifacts/reproduction-verdict.md`
- `artifacts/cal03-handoff.md`
- `artifacts/intent-plan.md`
- `artifacts/gate-evidence.md`
- two independent review artifacts;
- `artifacts/finding-disposition.md`;
- two independent verification artifacts; and
- `artifacts/final-disposition.md`.

Large generated run directories, caches, and temporary renders remain outside
Git. Retain only the smallest sufficient source fixtures, result objects,
figures, logs/manifests, and evidence needed to reproduce the verdict.

## Phase Plan

1. Verify CAL-01 admission and freeze the experiment, tolerance, output, and
   executable identities.
2. Install deterministic source fixtures and implement the confined harness.
3. Execute and reconstruct the five minimum process-reproduction arms.
4. Analyze admitted soil-format, hydrology, sediment, event, and return-period
   comparisons without crossing scale boundaries.
5. Issue the structured verdict and CAL-03 handoff.
6. Reconcile the exact diff, review, verify, and disposition the package.

## Reproduction Verdict

`artifacts/reproduction-verdict.json` must emit exactly one top-level verdict:

- `REPRODUCED`: the exact admitted source representation reproduces all
  load-bearing Bill process targets within predeclared tolerances and every
  material difference is explained.
- `BOUNDED_NOT_EXACT`: the experiment is reproducible under declared
  substitutes or missing-source limits, but cannot claim exact reconstruction.
- `NOT_REPRODUCIBLE`: one or more load-bearing process results contradict the
  admitted report/management evidence or cannot be generated without an
  unauthorized assumption.

Each verdict must separately classify Hubbard process results, Santee process
results, hydrology/sediment context, return periods, and the 92/95 branch.
Aggregate success cannot erase a failed site, pool, or comparison scale.

`BOUNDED_NOT_EXACT` and `NOT_REPRODUCIBLE` are scientifically useful completed
characterization outcomes when supported by complete current evidence. They do
not authorize CAL-03 to present Bill's outputs as exactly reproduced.

## Plan Of Work

### Milestone 1: dependency and experiment admission

Verify CAL-01 artifact identities and verdict. Freeze the exact source files,
management arms, executable/version, run length, random climate realization,
soil/slope inputs, output variables, units, result schema, tolerances, and
comparison rules. Stop before running if the admitted experiment would require
an unrecorded substitution.

### Milestone 2: deterministic fixture and harness

Install source-native run inputs with provenance and checksums. Build a harness
that prepares isolated run directories, applies only declared branch edits,
invokes the exact executable, captures stdout/stderr and output identities, and
parses machine results into stable tidy tables. The `0.92` branch must be
derived visibly from the preserved delivered management; the source file
itself remains byte-identical. The harness may derive an arm-local run control
that changes only the source control's plant/residue output answer from `No` to
`Yes` and inserts the corresponding confined output filename. This prospective
analysis-only amendment is required because the preserved source control does
not request the daily crop/residue surface used by Bill's analysis.

### Milestone 3: process reproduction

Run the five minimum management arms. Reconstruct Bill's live biomass and
current/previous/old/total residue trajectories, annual transfer, equilibrium
values, LAI, and canopy behavior. Recreate figures from machine tables and
compare them with report targets using predeclared tolerances.

### Milestone 4: downstream and soil-format context

Reproduce annual runoff/sediment and return-period tables where the admitted
outputs permit. Keep hillslope, watershed, channel, 7777, and 2006.5 surfaces
separate. Analyze event dates and antecedent precipitation, soil water, snow,
canopy, and residue when those outputs exist; otherwise classify the proposed
explanation as not evaluated.

### Milestone 5: verdict, review, and handoff

Issue the structured reproduction verdict. State which Bill targets CAL-03 may
use as reproduced evidence, which remain Bill-derived assumptions, and which
are contradicted or unavailable. Complete independent review, finding
disposition, terminal reconciliation, verification, and final disposition.

## Concrete Steps

Work from the openWEPP repository root.

1. Verify CAL-01's final disposition, admission JSON, source hashes, and target
   ledger identities.
2. Run `tools/agents/find-agents --for` over the exact admitted write set and
   update `artifacts/required-reading-map.md`.
3. Use the repository TESTGATE planner to admit an exact intent plan before
   harness or fixture edits; select only proportionate analysis/fixture gates
   unless the terminal diff expands.
4. Verify every installed fixture hash before a run.
5. Build or identify the exact release executable required by the admitted
   experiment and record its path, version/commit, hash, size, and build command
   before accepting outputs.
6. Execute each arm in an isolated temporary run root. Never reuse mutable
   outputs between arms.
7. Validate parser row counts, dates, units, pool sums, and rejected aliases
   before computing summaries.
8. Generate all figures and tables from retained result objects, never by
   manual spreadsheet edits or screenshots.
9. Independently reconstruct equilibrium and return-period summaries.
10. Run selected fixture/harness tests, documentation/path checks, checksum
    verification, protected-output scans, and `git diff --check`.
11. Complete two independent reviews, finding disposition, and two independent
    terminal verifications.

Exact commands and observed transcripts belong in
`artifacts/gate-evidence.md`. Scaffold text is not execution evidence.

## Exit Criteria And Validation

The package may close with a supported characterization verdict only when:

- CAL-01 dependency identity and admission are current;
- every run binds exact management, climate, soil, slope, run control,
  executable, seed/realization, output directory, and result identity;
- source fixtures have provenance, checksums, documented transformations, and
  no committed generated run debris;
- the five minimum management arms execute or receive explicit
  `NOT_REPRODUCIBLE` findings with retained cause evidence;
- delivered `0.95` and derived `0.92` Hubbard branches are both run and compared;
- daily live biomass and current/previous/old residue are independently parsed
  and their total is reconstructed;
- annual litter/transfer and practical equilibrium are calculated from machine
  results rather than transcribed report prose;
- chart-derived tolerances are declared before score computation;
- soil-format, hillslope/watershed, and channel boundaries remain explicit;
- hydrology/sediment and return-period values are not allowed to compensate for
  failed biomass or residue reproduction;
- figures and tables rebuild deterministically from retained results;
- the top-level and per-surface reproduction verdicts are independently
  reconstructable;
- the CAL-03 handoff preserves assumptions, contradictions, and claim limits;
- all intent-selected gates pass with current evidence;
- two independent reviews and two independent verifications leave no
  undispositioned finding; and
- the exact terminal diff is contained by the declared or prospectively amended
  write set.

This package is conservation- and publication-sensitive because it aggregates
mass and hydrology outputs. Before accepting results it must record operand
lineage, units, area/time basis, rejected aliases, independent pool/annual
reconstruction, and two-sided magnitude comparisons where report targets
permit. Exact self-consistency alone is supporting evidence.

Broad workspace Rust tests, coverage, and CRAP are not automatically required
for analysis-only files. The accepted intent/terminal plan selects the actual
gates. Any production, schema, parser-crate, or existing-test change escalates
scope and gates before the edit.

## Security Impact

The package invokes a local scientific executable against untrusted flat-file
inputs in isolated temporary directories. It must prohibit shell interpolation
from input content, path escape, symlink/special-file fixtures, executable
replacement during runs, output writes outside the declared temporary root,
credentials, and committed caches.

Security-impact acceptance requires confined paths, argument-safe process
invocation, exact executable identity before and after runs, source/output
inventory, and proof that no external network or service mutation occurred.

## Delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to one read-only `comparator_suite_runner` for the multi-arm 40/100-year WEPP
run matrix, two independent read-only scientific/reproduction reviewers, and
two independent read-only terminal verifiers; expected outputs are compact run
metrics, exact log/result paths, findings, and verdict reconstructions; write
access is read-only.

Subagent requirement: the `comparator_suite_runner` is REQUIRED for admitted
long-running comparator batches and must execute them rather than the parent
agent when available. If session-level policy does not authorize spawning, stop
before the batch and request the required authority; do not silently substitute
parent execution or mark the batch complete.

## Idempotence And Recovery

Each arm uses a fresh explicit temporary directory and content-identified input
manifest. Rerunning the same admitted arm must produce identical normalized
inputs and deterministic summaries, while stochastic climate identity remains
fixed by the admitted source file rather than regenerated. A failed arm retains
its manifest and logs, cleans no source fixture, and can be rerun without
affecting another arm.

## Surprises & Discoveries

- Observation: Bill's report-linked WEPPcloud runs remain intact and expose the
  exact selected source inputs and outputs.
  Evidence: source recovery resolved Hubbard `p1`/`H1` to Topaz ID `22` and
  Santee `p2`/`H2` to Topaz ID `23`, with 100-year controls and hash-bound
  climate, slope, soil, management, output, and controller metadata.
- Observation: the WEPP 2012 Windows executable is still present on BLARHG.
  Evidence: live read-only inspection identifies
  `C:\WEPP\wepp\wepp_2012.exe` by SHA-256
  `6104a3440624ad54aa6c3660794280adfd600d4a11b98559c6205a73cd47fc3f`.
- Observation: this recovery does not recreate Bill's manually transcribed
  Windows project.
  Evidence: his Windows 2006.5 soil/slope/run files, machine outputs, and
  original runtime-library state remain absent.
- Observation: the recovered source managements are sufficient to build the
  constant-cover arms.
  Evidence: both files say `With no Senescence or decomposition` and
  `For no growth, no decomp, no senescence`; Bill states that his Windows
  constant-cover comparison transcribed the WEPPcloud mature-forest
  management. Hubbard and Santee differ only in initial residue mass.
- Observation: the recovered source run controls do not request WEPP's daily
  plant/residue output.
  Evidence: source control line 12 is `No`; pinned legacy `outfil.for` maps that
  answer to the existing `crop` diagnostic, whose header publishes daily live
  biomass plus current, previous, and old flat-residue pools. The package was
  prospectively amended before harness implementation to enable only that
  diagnostic surface.

## Decision Log

- Decision: make CAL-02 characterization-only.
  Rationale: Bill's approach must be reproduced before it is used to change or
  fit native CP2.
  Date/Author: 2026-07-26 / Codex from operator direction.
- Decision: require both `dropfc=0.92` and `0.95` Hubbard arms.
  Rationale: the report and delivered management conflict; the experiment, not
  an editorial choice, should determine which reproduces the reported plots.
  Date/Author: 2026-07-26 / Codex.
- Decision: admit source-native `p1.man` and `p2.man` as constant-cover
  comparators.
  Rationale: they are exact site inputs, encode the constant-cover mechanism,
  and implement the source that Bill says he transcribed. A Windows-builder
  serialization is unnecessary for reproducing the analytical approach.
  Date/Author: 2026-07-26 / Codex from operator direction.
- Decision: allow a complete `BOUNDED_NOT_EXACT` characterization verdict.
  Rationale: missing source-native inputs can be a valid scientific finding, but
  must narrow downstream claims and cannot be called exact reproduction.
  Date/Author: 2026-07-26 / Codex.
- Decision: keep soil-format and spatial-scale effects outside the management
  verdict.
  Rationale: Bill identified both as large confounders.
  Date/Author: 2026-07-26 / Codex.
- Decision: enable the existing daily plant/residue diagnostic in a derived run
  control.
  Rationale: it is the minimum output-only transformation needed to reconstruct
  Bill's process trajectories; it changes no forcing or physics and leaves the
  source control byte-identical.
  Date/Author: 2026-07-26 / Codex.
- Decision: reopen CAL-02 with WEPPpy-built SSURGO 2006.2 soil arms.
  Rationale: operator supplied the exact recovery route and the mukeys already
  bind the admitted sites. This is a bounded format reconstruction, not
  historical byte equivalence.
  Date/Author: 2026-07-26 / operator and Codex.
- Decision: add a Linux WEPP 260725/source-native-9002 lane and accept its
  gross-transfer reconstruction at crop-output precision.
  Rationale: the native soils match the intended WEPPcloud management context;
  the operator explicitly judged the `0.001 kg/m2` publication precision close
  enough for this campaign. This does not authorize a direct or exact claim.
  Date/Author: 2026-07-26 / operator and Codex.

## Outcomes & Retrospective

The source-native 9002 attempt failed at soil input conversion, after which
operator authority admitted exact-mukey WEPPpy `2006.2` reconstruction. All
five 100-year arms then completed. The delivered Hubbard 0.95 management
reproduces live biomass but not total residue; the report's 0.92 branch does
not reproduce live biomass. Santee stocks and the compared hydrology,
sediment, and daily-runoff return levels differ materially from Bill's report.
The useful campaign conclusion is `NOT_REPRODUCIBLE` from a successful bounded
reconstruction: sufficient to stop historical recovery and advance the next
characterization step, but not Bill's missing byte-identical project and not
independent calibration authority.

The additional Linux 260725/source-native-9002 matrix also completed all five
arms and preserved the verdict: 9 of 64 targets pass bounded tolerances and 55
are contradicted. Hourly water balance activated in every arm. Observe exposed
no litter-specific tag, but pinned release source and rounded crop output
reconstruct gross aboveground live-to-current-residue transfer to publication
precision, independently agreeing within `0.00088 kg/m2/year`. The operator
accepted that bound; dual review and terminal verification passed.

## Revision Note

2026-07-26: initial scaffold created to implement roadmap Order 2 with an
explicit dependency gate, bounded characterization verdicts, and no production
physics authority. Before implementation, execution amended the package to
authorize the minimum output-only run-control derivation needed for WEPP's
daily plant/residue diagnostic.
