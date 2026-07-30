# SNOW-SURFACE-EB-02 Sub-Canopy Longwave Contract

Status: `complete / contract pass / runtime hold`

Package ID:
`20260730-snow-surface-eb-02-subcanopy-longwave-contract-001`

Campaign: `SNOW-SURFACE-EB`

Owner: Codex

Execution mode: `package-end-to-end`

Science intent: `implementation` of canonical equation and interface authority;
no empirical calibration or production runtime activation.

## Purpose

Create the canonical, reproducible contract for one-layer sub-canopy longwave
radiation without adding a user sky-view coefficient or requiring remote
sensing. Bind atmospheric longwave, daily cloud inference, canopy-to-sky-view
mapping, effective-unity exchange, canopy-temperature approximation limits,
units, guards, and analytical test vectors. Preserve the runtime implementation
hold until EB-03 selects one shared snow-surface-temperature and cold-content
provider.

## Progress

- [x] (2026-07-30) User authorized scaffolding and end-to-end execution.
- [x] (2026-07-30) Declared contract-only implementation intent and runtime
  hold boundary before canonical edits.
- [x] (2026-07-30) Completed source and current-state reconciliation.
- [x] (2026-07-30) Created and registered `SC-SNOWENERGY-001`.
- [x] (2026-07-30) Executed 38 deterministic analytical vectors and generated
  two accessible figures with Markdown sidecars.
- [x] (2026-07-30) Corrected all Review A/B findings and passed corrected-tree
  dual review.
- [x] (2026-07-30) Completed dual terminal verification, final lifecycle
  transition, and final disposition.

## Context

EB-01 froze an orthogonal `B/L/S/LS` factorial and requires all cells to use
one shared surface-energy carrier. EB-01A admitted the complementary
sky/canopy longwave equation, corrected Dilley-Unsworth atmospheric route,
effective-unity canopy and snow emissivity, and an FSM2 diffuse-transmission
base. The user directed that sky view be derived from existing canopy state,
not supplied as another coefficient or dataset.

The current runtime exposes three nonequivalent snow-temperature surfaces:
the Stage-B air-temperature cap, legacy frost `tmpadj/surtmp`, and opt-in
multilayer thermal state. EB-01 assigns selection of the shared
temperature/cold-content state to EB-03. This package therefore must not select
one implicitly or wire a production runtime that cannot close its energy
boundary.

## Governing Authority

- `docs/planning/snow-surface-energy-balance-roadmap.md`
- `SNOW-SURFACE-EB-01`
- `SNOW-SURFACE-EB-01A`
- ADR-0011, ADR-0017, ADR-0042, and ADR-0043
- Flerchinger et al. (2009), Essery et al. (2008), Rutter et al. (2023),
  Essery et al. (2025), and Leonardini et al. (2025)
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`

## Included Scope

- Create `SC-SNOWENERGY-001` as focused canonical authority for snow-surface
  energy radiation operands and interfaces.
- Bind the corrected Dilley-O'Brien clear-sky and Unsworth-Monteith all-sky
  atmospheric longwave equations.
- Bind daily clearness-index cloud inference with declared polar-night
  behavior.
- Derive hemispherical diffuse sky view from existing effective plan-view
  canopy cover through a source-backed Beer-law transformation; do not directly
  alias plan-view cover to sky view.
- State why LAI, structural cover, and height are not independently added when
  they already contribute to, or are redundant with, the effective canopy-cover
  state.
- Bind complementary sky/canopy incoming longwave, effective canopy and snow
  emissivities of exactly one, variable atmospheric effective emissivity,
  outgoing snow longwave, signs, units, and approximation limits.
- Define typed runtime obligations and analytical contract vectors without
  activating them in production.
- Produce accessible SVG figures with same-stem Markdown sidecars.

## Excluded Scope

- No production Rust, selector, parser, runfile, CLI, schema, fixture, default,
  output, or runtime-consumer edit.
- No selection or implementation of the snow-surface-temperature provider.
- No EB-03 sublimation/latent-mass implementation.
- No EB-04 result-bearing factorial execution.
- No canopy-temperature energy balance, explicit trunk model, gap-edge model,
  ray tracing, site fitting, or empirical coefficient calibration.
- No required hemispherical photography, LiDAR, remote sensing, or new
  user-entered radiative coefficient.

## Deliverables

- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`
- `docs/specifications/science-contracts/index.md`
- `artifacts/required-reading-map.md`
- `artifacts/pre-implementation-intent.md`
- `artifacts/source-and-state-reconciliation.md`
- `artifacts/canopy-sky-view-derivation.md`
- `artifacts/operand-lineage.csv`
- `artifacts/analytical-test-vectors.csv`
- `artifacts/calibration-readiness-matrix.md`
- `artifacts/figures/*.svg`
- `artifacts/figures/*.md`
- `artifacts/contract-implementation-evidence.md`
- `artifacts/contract-test-evidence.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/kernel-profile-checklist.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-evidence.md`
- `artifacts/exact-diff-reconciliation.md`
- `artifacts/line-count-governance.md`
- `artifacts/science-contracts/SC-SNOWENERGY-001/contract_ref.md`
- `artifacts/science-contracts/SC-SNOWENERGY-001/review_agent_a.md`
- `artifacts/science-contracts/SC-SNOWENERGY-001/review_agent_b.md`
- `artifacts/science-contracts/SC-SNOWENERGY-001/disposition.md`
- `artifacts/science-contracts/SC-SNOWENERGY-001/verification_agent_a.md`
- `artifacts/science-contracts/SC-SNOWENERGY-001/verification_agent_b.md`
- `artifacts/worker-handoff.md`
- `artifacts/final-disposition.md`

## Intended Write Set

- `docs/ROADMAP.md`
- `docs/planning/snow-surface-energy-balance-roadmap.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260730-snow-surface-eb-02-subcanopy-longwave-contract-001/**`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`

## Implementation Intent And Risk

Intent is canonical contract implementation plus deterministic analytical
evidence. The terminal diff is documentation and package-local analysis only;
it creates no executable runtime behavior. Validation risk is
`authority-documentation / integrated-domain`: contract schema, units, links,
analytical equations, generated artifacts, and registry consistency are
increment requirements. Rust workspace, comparator, empirical, conservation,
and runtime-consumer gates are not increment requirements because no executable
or produced runtime surface is changed.

Campaign-owned runtime obligations are declared before implementation:

- Owner: `SNOW-SURFACE-EB-03`.
- Trigger: EB-03 canonical selection of one shared snow-surface-temperature and
  cold-content provider.
- Later boundary: EB-02 runtime implementation/consumer increment before EB-04.
- Required evidence: typed provider, common `B/L/S/LS` consumer path,
  independent energy reconstruction, and nondefault selector proof.
- Rationale: outgoing snow longwave and turbulent/phase fluxes cannot be
  coherently activated without the same surface state.

## Phase Plan

1. Reconcile primary-source equations, openWEPP canopy-state semantics, forcing
   cadence, and competing snow-temperature surfaces.
2. Author `SC-SNOWENERGY-001` in full kernel-profile form.
3. Execute a package-local independent evaluator for analytical vectors and
   figures; no production path may import it.
4. Run documentation, contract, CSV, SVG, link, spelling, and diff checks.
5. Complete independent contract reviews, disposition findings, correct the
   tree, and complete two independent terminal verifications.
6. Close as `COMPLETE / CONTRACT PASS / RUNTIME HOLD` only if every
   increment-scope requirement passes.

## Validation And Acceptance

- Deterministically regenerate analytical vectors and figures.
- Require all CSVs to parse with rectangular nonblank rows.
- Require each SVG to parse and carry `role="img"`, `<title>`, `<desc>`, and a
  same-stem Markdown sidecar.
- Verify analytical limiting cases and independently reconstructed equations.
- Run scoped `markdown-doc lint` and `markdown-doc validate`.
- Run science-contract unit-compliance inventory and record its scoped result.
- Run strict Binding Exposure Index lint when applicable; the new contract has
  no addenda or sidecar binding residue.
- Validate all local Markdown links.
- Preview `uk2us` normalization and apply only safe source-prose changes.
- Run `git diff --check` after files are staged or otherwise include untracked
  files in the equivalent check.
- Reconcile the exact terminal diff to this intended write set.

## Exit Criteria

1. The canonical contract contains every required profile section, equation,
   unit, constant, validity domain, guard, obligation, alias, calibration
   posture, test vector, gap, and change-log entry.
2. Sky view is derived without a new input, direct cover alias, site fit, or
   double counting of LAI/structural cover.
3. Atmospheric, sub-canopy incoming, outgoing-snow, and net-longwave equations
   reproduce independently calculated vectors.
4. Polar-night cloud inference and missing snow-temperature behavior fail
   closed or retain an explicit governance hold as applicable.
5. Production runtime remains unchanged and truthfully held for EB-03.
6. All increment requirements report `PASS`; no required row is `BLOCKED`,
   `FAIL`, or unjustifiably `NOT RUN`.
7. Both independent reviews, finding disposition, both terminal verifications,
   exact-diff reconciliation, and line-count governance pass.

## Stop-Loss

Stop canonical promotion and close `HOLD` if existing canopy state cannot
support an authority-backed deterministic sky-view transformation without a
new coefficient, a direct cover/sky-view alias, or a required external
dataset. Stop runtime implementation if any snow-temperature surface is
silently selected, if legacy cloud state is reused without the contract
mapping, or if energy terms would be double counted.

## Review And Verification

Require two independent contract reviews:

- Review A: scientific authority, equations, canopy mapping, regimes, units,
  signs, and claim limits.
- Review B: contract profile, guard/alias/unit maps, runtime hold legitimacy,
  reproducibility, package governance, and roadmap/catalog consistency.

Every finding must be dispositioned. After accepted corrections, require two
independent terminal verifications of the exact final tree.

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two read-only contract-review subagents and two
read-only terminal-verification subagents. Expected outputs are the four named
package-local review/verification artifacts. Write access is limited to each
agent's assigned artifact; production code, canonical contracts, tests,
roadmaps, and other package artifacts are read-only.

Subagent requirement: none for heavy batch execution because no full,
comparator, cohort, or external-authority suite is selected.

## Security Impact

No security boundary, dependency, subprocess, external input, secret, or
network behavior changes. Security impact: `NOT APPLICABLE`.

## Decision Log

- Decision: make EB-02 a contract-completion increment and retain runtime
  implementation as a named campaign hold.
  Rationale: EB-03 owns the shared snow-temperature/cold-content provider
  required by outgoing longwave and the common factorial carrier.
  Date/author: 2026-07-30 / user and Codex.
- Decision: author a focused new `SC-SNOWENERGY-001` contract instead of
  appending another large addendum to `SC-SNOWFREEZE-001`.
  Rationale: the focused contract can satisfy the current contract profile,
  expose its interfaces explicitly, and avoid expanding the legacy aggregate
  contract's binding-residue debt.
  Date/author: 2026-07-30 / Codex.

## Surprises And Discoveries

- Observation: native-forest structural canopy cover is an effective overhead
  cover floor, not a stem-area index.
  Evidence: `SC-PLANT-001` and CANOPY-DOC-01 coefficient authority.
  Consequence: EB-02 may not add structural cover to LAI as if it were woody
  VAI.
- Observation: substituting daily-mean temperature into the nonlinear
  atmospheric equations is not cadence-neutral; the two-hour analytical
  contrast differs by `2.411912116322 W m^-2`.
  Evidence: `analytical-test-vectors.csv`.
- Observation: the empirical atmospheric route needs a no-clamp derived
  emissivity authority guard because the source review does not establish a
  transferable meteorological input envelope for every openWEPP climate.
  Evidence: corrected Review A and `SC-SNOWENERGY-001#INV-SNOWENERGY-014`.

## Outcomes And Retrospective

EB-02 achieved the contract objective and stopped at the declared runtime
boundary. The compact effective-cover derivation eliminated a new user
coefficient while preserving the difference between vertical cover and
hemispherical sky view. Independent review materially improved the contract by
correcting cadence and emissivity contradictions, binding the empirical
authority envelope, and replacing self-referential analytical evidence.

The next campaign action is EB-03. It must resolve the coherent thermal/cloud
provider and exact-one sublimation composition before any longwave runtime
implementation or EB-04 factorial execution.
