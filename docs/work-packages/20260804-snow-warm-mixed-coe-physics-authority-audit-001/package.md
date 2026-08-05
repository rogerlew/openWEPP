# SNOW-WARM-MIXED-COE-PHYSICS-AUTHORITY-AUDIT

Status: `queued / scaffolded`

Date: `2026-08-04`

Plan class: `Read-only first-principles physics-authority audit`

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.
The `Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes &
Retrospective` sections must remain current during execution.

## Purpose / Big Picture

Determine whether the active warm/mixed CoE snowmelt generator is a faithful
and sufficiently authorized production formulation, a faithful legacy port
with unresolved physics authority, or a Rust transcription defect. The audit
maps Chapter 3, the pinned post-2007 legacy source, current Rust, and independent
surface-energy authority; it then quantifies the exact empirical subcomponents
on 21L's checksum-bound canonical hourly population.

The observable result is a source-line authority map, formula chronology,
dimensional and state-eligibility audit, deterministic quantitative receipt,
independent reviews, and a bounded next-package recommendation. This package
does not amend a science contract, change production code, tune a coefficient,
or admit a correction.

## Context And Orientation

21L found that `99.61-99.91%` of median pre-peak pack loss occurs on its frozen
warm/mixed chronology and that aggregate `cmelt` is the largest annual-first
positive empirical term at all four canonical sites. It explicitly classified
that result as chronology-confounded and noncausal.

Current Rust computes four empirical melt-depth terms in legacy inches and
converts their signed sum to metres of water equivalent. The pinned
`dac3c950...` `melt.for` contains modifications dated 2007 and 2008 that differ
from the July 1995 Chapter 3 equations in cold-hour shortwave attenuation,
longwave/temperature partition, aerodynamic adjustment, canopy handling, and
rain heat. Independent energy-balance authority resolves net radiation,
turbulent exchange, precipitation advection, cold content, and surface
temperature as interacting physical states. Agreement among current Rust and
the pinned source therefore proves port fidelity but does not by itself prove
that the active default has sufficient physical authority.

## Implementation Intent

- Intent: `authority adjudication and characterization only`.
- Science implementation status: `IMPLEMENTED_AS_LEGACY_EMPIRICAL_COE`; this
  label is descriptive and is not a sufficiency verdict.
- Calibration evidence status: `NOT_APPLICABLE`.
- Identifiability status: `NOT_ASSESSED`; 21L remains diagnostic and cannot
  identify physical flux coefficients.
- Observation role: all SNOTEL-derived 21L evidence remains `DIAGNOSTIC_ONLY`.
- Production/kernel edit intent: `none`.
- Contract/test edit intent: `none`.
- Validation risk: `read-only scientific analysis plus package-local Python
  tooling and documentation`.

## Included Scope

1. Bind exact identities for the canonical Chapter 3 PDF, pinned legacy
   `melt.for`, current Rust CoE producer/caller, canonical snow contracts,
   independent physical sources, and accepted 21L tables/receipts.
2. Map each Chapter 3 term and every post-1995 legacy modification through the
   current Rust symbol, unit, sign, time base, canopy treatment, state input,
   and downstream consumer.
3. Independently reconstruct current `amelt`, `bmelt`, `cmelt`, and `dmelt`
   from 21L's hourly rows joined to its daily wind/dewpoint/canopy operands.
4. Decompose `bmelt` into temperature and clear-sky pieces, and `cmelt` into
   open aerodynamic/moisture and canopy-temperature pieces, without relabeling
   those empirical pieces as measured physical fluxes.
5. Quantify warm/mixed positive-melt exposure by site, including subfreezing
   hourly air temperature, sub-`350 kg m^-3` pack density, same-hour snowfall,
   and positive `cmelt` when dewpoint remains below freezing.
6. Convert empirical melt depths to latent-heat-equivalent flux only as a
   dimensional magnitude diagnostic; do not claim energy closure or a physical
   term partition.
7. Adjudicate authority using the frozen outcome matrix and produce the
   smallest justified contract-first follow-on, if any.
8. Complete dual independent science review, finding disposition, dual
   independent terminal verification, exact-diff reconciliation, and truthful
   closure or hold.

## Excluded Scope

- Production Rust, canonical contracts, tests, fixtures, observations,
  selectors, defaults, public schemas, or source PDFs.
- New model executions, release binaries, comparator cohorts, parameter
  sweeps, coefficient fitting, or result-aware thresholds.
- Treating `amelt`/`bmelt`/`cmelt`/`dmelt` or their subcomponents as separately
  measured radiation, sensible, latent, longwave, or precipitation heat.
- Treating latent-heat-equivalent conversion as physical energy closure.
- Declaring openWEPP defective from a handbook, legacy, SNOTEL, or alternative
  model discrepancy without independent correctness authority.
- Selecting or implementing a replacement equation.

## Intended Write Set

- `docs/work-packages/20260804-snow-warm-mixed-coe-physics-authority-audit-001/`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/planning/snow-surface-energy-balance-roadmap.md`
- `target/snow_warm_mixed_coe_physics_authority_audit/` (untracked execution
  logs and scratch output)

Everything else is read-only. Reviewer and verifier subagents are read-only;
the orchestrator owns every tracked and target write.

## Authority And Dependencies

- Root and package governance, including
  `docs/standards/testing-and-gate-strategy.md`.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` and
  `SC-SNOWENERGY-001.md`.
- `references/50201000/chap3.pdf`, especially Section 3.6.
- `/workdir/wepp-forest_260430_baseline/src/melt.for` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Current CoE producer and runtime caller in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/`.
- Independent physical authority in Marks et al. 1998, Marks et al. 1999,
  Ohmura 2001, and Walter et al. 2005 from `references/copyrighted/`.
- 21L package, accepted receipt, scientific synthesis, daily table, and hourly
  classification table.

Package artifacts are evidence and cannot replace canonical `SC-*` authority.

## Prospective Quantitative Audit Contract

The analysis freeze must bind every input hash and these rules before the
package-local analyzer is run:

- population: canonical 21L lanes only; preserve 21L's accepted windows,
  hourly eligibility, and warm/mixed labels exactly;
- joining: daily wind, dewpoint, and canopy values join by exact `(lane,date)`;
- formula: reconstruct current Rust arithmetic and constants without algebraic
  simplification that changes operation order;
- tolerance: every published CoE term must reproduce within `1e-12 m` absolute;
- subcomponents: `B_temp + B_clear = B` and `C_open + C_canopy = C` within the
  same tolerance;
- exposure counts: report exact integer counts and corresponding melt-depth
  sums; do not hide site heterogeneity behind pooled fractions;
- magnitude conversion: use `rho_w=1000 kg m^-3`,
  `L_f=333550 J kg^-1`, and `3600 s h^-1`, labeled
  `ASSUMED_FOR_DIMENSIONAL_AUDIT` rather than calibrated or observed values;
- no counterfactual model result: handbook or physical equations are compared
  structurally unless every required state/forcing operand is present.

## Frozen Authority-Disposition Matrix

The audit selects one primary result:

- `CURRENT_AUTHORITY_SUFFICIENT`: current Rust, admitted contract, pinned
  provenance, Chapter 3, and independent physical requirements are coherent
  for the claimed production role, with no material unresolved assumption;
- `RUST_TRANSCRIPTION_DEFECT`: same-input current Rust materially differs from
  the pinned source or violates a binding admitted contract;
- `BASELINE_FIDELITY_WITH_AUTHORITY_GAP`: Rust faithfully implements the pinned
  empirical lineage, but its production sufficiency is unresolved or
  contradicted by handbook chronology/provenance or independent physical-state
  requirements;
- `UNRESOLVED_EVIDENCE`: identities, operands, or authority are insufficient
  even to choose among the preceding outcomes.

No outcome authorizes a production edit in this package.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes and requires
spawning/delegating to two independent read-only science reviewers and two
independent read-only terminal verifiers. Expected outputs are compact
evidence-classified findings, source/line citations, independent quantitative
reconstruction, gate-legitimacy checks, and final verdicts. They may not edit
files. The orchestrator integrates their reports and dispositions.

No full-workspace, comparator, release, population, or other heavy batch is
selected; `comparator_suite_runner` is not required.

## Deliverables

- Prospective `artifacts/audit-freeze.json` and exact execution receipt.
- `artifacts/authority-lineage-map.md` and
  `artifacts/formula-chronology-audit.md`.
- `artifacts/physical-authority-matrix.md`.
- Package-local deterministic analyzer and unit tests.
- `artifacts/quantitative-audit.json` plus an integrated scientific audit.
- Required contract/test nonimplementation, gate, security, line-count,
  exact-diff, review, verification, disposition, and handoff artifacts.

## Phase Plan

### Phase A: Scaffold And Freeze

Create and commit the package, prompt, required-reading map, queued artifacts,
prospective audit freeze, and active roadmap/catalog state. Freeze input
identities and quantitative rules before result-bearing execution.

### Phase B: Static Authority And Chronology Audit

Read Chapter 3, the exact pinned `melt.for`, current Rust, canonical contracts,
and independent physical sources. Produce the source-line lineage, formula
chronology, dimensional map, state chronology, and explicit authority gaps.

### Phase C: Quantitative Reconstruction

Implement and unit-test the streaming package-local analyzer. Run it once over
the accepted 21L daily/hourly tables, fail closed on identity/join/formula
errors, and retain the exact result/receipt.

### Phase D: Integrate And Adjudicate

Combine the static and quantitative evidence without converting association to
causation. Apply the frozen outcome matrix and write a bounded next action.

### Phase E: Review, Verify, And Close

Complete dual review, disposition every finding, remediate accepted findings,
run direct terminal gates, obtain dual verification, archive the active prompt
byte-identically, reconcile the exact diff, update roadmaps/catalogs, and
record truthful final disposition.

## Validation And Exit Criteria

- All frozen input identities reproduce at execution and terminal verification.
- Chapter, pinned legacy, current Rust, contracts, and independent sources are
  mapped by equation, variable, unit, sign, time base, state dependency,
  canopy treatment, and authority role.
- Current Rust term reconstruction and both subcomponent identities pass for
  every eligible canonical warm/mixed hourly row within `1e-12 m`.
- Site-specific exposure counts and magnitudes reproduce independently.
- Every physical claim distinguishes empirical lineage, dimensional analogy,
  and state-resolved physical authority.
- The selected disposition follows the frozen matrix and authorizes no
  correction.
- Package-local Python syntax and focused unit tests pass.
- JSON parse, Markdown lint/validate, spelling preview, reference/path checks,
  prompt archive, protected-path identity, and `git diff --check` pass.
- Exact terminal diff remains inside the declared tracked write set. No Rust
  diff means no new `.rs` line-count exposure.
- Dual reviews, explicit finding disposition, dual verification, gate
  legitimacy, and truthful status pass with no unresolved current-scope gate.

No Rust, domain-profile, quick-workspace, full-workspace, comparator, or
anti-evasion run is selected because production, contracts, tests, fixtures,
observations, and authority-suite bindings remain byte-identical.

## Security And Data Impact

Security impact is `none expected`. The package reads local repository and
target evidence only, performs no network or credential access, and writes
only its declared package/catalog/roadmap surfaces plus an untracked target
namespace. No raw provider response, secret, or credential path may be
committed.

## Progress

- [x] (2026-08-04) User authorized scaffolding and end-to-end execution.
- [x] (2026-08-04) Resolved repository, package, contract, Rust, and test
  instruction chains before edits.
- [x] (2026-08-05 UTC) Froze exact identities and the analysis contract before
  result-bearing execution.
- [ ] Commit the validated scaffold checkpoint.
- [ ] Complete static authority and chronology audit.
- [ ] Implement, test, and execute quantitative reconstruction.
- [ ] Integrate evidence and apply the frozen disposition matrix.
- [ ] Complete dual review, finding disposition, dual verification, prompt
  archival, exact-diff reconciliation, and final disposition.

## Surprises & Discoveries

- Observation: 21F already excluded a Rust transcription error at aggregate
  equation level, but it left Chapter-versus-post-2007 authority choice and
  active warm/mixed state sufficiency unresolved.
  Evidence: predecessor `authority-equation-map.md` and `integrated-audit.md`.
- Observation: 21L's accepted hourly table can be joined to its daily table to
  recover exact daily wind, dewpoint, and canopy operands without reading or
  rerunning multi-gigabyte production traces.
  Evidence: accepted table headers and receipt-bound identities.
- Observation: the nominal baseline repository is clean but currently checked
  out at `2f65506d`, not the normative `dac3c950`; its `melt.for` adds ten
  observe-only lines.
  Evidence: exact commit/worktree hashes and pinned `git show` comparison in
  `artifacts/audit-freeze.json`.

## Decision Log

- Decision: Keep 21M read-only and reuse accepted 21L tables.
  Rationale: the open question is authority sufficiency, not another model
  response experiment; rerunning production would add no authority.
  Date/Author: 2026-08-04 / Codex.
- Decision: Quantify exact empirical subcomponents but prohibit physical-flux
  relabeling and replacement-formula counterfactuals.
  Rationale: available operands support current-formula reconstruction, while
  handbook and state-resolved equations require unavailable roughness,
  surface-temperature, stability, and cold-content states for like-for-like
  numeric comparison.
  Date/Author: 2026-08-04 / Codex.
- Decision: Bind pinned legacy authority from the commit blob, not the clean
  convenience worktree.
  Rationale: a clean checkout can still point at a later commit; normative
  provenance is the exact `dac3c950` object named by repository governance.
  Date/Author: 2026-08-04 / Codex.

## Outcomes & Retrospective

Queued. Populate after the accepted audit and terminal review.

## Revision Note

2026-08-04: Initial scaffold created from the roadmap-authorized 21M objective
and 21L handoff.
