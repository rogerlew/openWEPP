# SNOW-SURFACE-EB-01 Reconciliation And Factorial Design

Status: `queued`

Package ID:
`20260729-snow-surface-eb-01-reconciliation-factorial-design-001`

Campaign: `SNOW-SURFACE-EB`

Owner: Codex

Execution mode: `package-end-to-end`

Science intent: `authority reconciliation + calibration-readiness and
independent-validation design`; this package does not perform empirical
calibration or implement production physics.

## Purpose

Produce the evidence and experimental contract needed to test explicit
sub-canopy longwave exchange and snow sublimation separately and together.
After this package, a successor can implement only authority-backed physics and
can execute a comparable four-cell experiment without guessing which current
code, observations, selectors, or ledgers are authoritative.

The human-visible result is a compact science dossier explaining:

- what snow surface-energy and sublimation behavior exists now;
- what is missing or incompletely composed;
- which observations can distinguish the mechanisms;
- how baseline, longwave-only, sublimation-only, and combined effects will be
  measured; and
- exactly what would promote, reject, or stop the campaign.

## Progress

- [x] (2026-07-29) Package scaffolded and linked from the campaign roadmap.
- [ ] Record pre-execution intent, exact base commit, and current write set.
- [ ] Reconcile authority, implementation, selectors, and prior dispositions.
- [ ] Freeze observation roles and modeled-to-observed stratum correspondence.
- [ ] Freeze the four-cell experiment, response operands, and reconstruction.
- [ ] Complete readiness, stop-loss, and successor admission decisions.
- [ ] Produce figures and Markdown sidecars.
- [ ] Run applicable validation and reconcile the terminal diff.
- [ ] Complete dual independent review, finding disposition, dual verification,
  and final disposition.

## Context

Production openWEPP contains several snow mechanisms developed in different
packages. The WEPP Corps-of-Engineers melt lineage remains active. Shared
surface-energy primitives exist in `openwepp-meteorology`. Multilayer Stage 3
uses those primitives but currently constructs its surface sum from shortwave
while passing zero sensible, latent, conductive-surface, advected, and explicit
longwave terms. Existing Stage A/B sublimation candidates remove snow water
equivalent as vapor and record vapor mass, but were not promoted. Stage B
conserved mass and scored `15` robust failures / `178`, just below the current
default's `15` / `179`.

This package must verify those statements against the current source and
retained evidence. Planning prose is not implementation authority.

The central composition risk is double counting. A physically coupled
sublimation flux removes snow mass and consumes latent energy. When longwave is
added to the same surface balance, the combined model must debit latent energy
once, remove the matching vapor mass once, and allow only the remaining energy
to warm, melt, or refreeze snow. Treating sublimation as an unrelated mass sink
while also including its latent flux in the balance would be invalid.

## Governing Authority

- `docs/planning/snow-surface-energy-balance-roadmap.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/planning/snow-frost-fidelity-strategy.md`
- `docs/planning/paradigm2-multilayer-snow-specification.md`
- ADR-0011, ADR-0017, ADR-0028, ADR-0029, ADR-0042, and ADR-0043
- retained packages for Stage 0 surface energy, Stage 3 liquid/thermal routing,
  Stage A/B sublimation, canopy stratum correspondence, and cross-SNOTEL
  adjudication

This package is evidence and sequencing authority only. It cannot add or amend
production process physics. Any equation, constant, state interpretation, or
invariant needed by a successor must be admitted into the canonical science
contract before production edits.

## Included Scope

### Authority and implementation reconciliation

Trace the current source and retained evidence for:

- incoming and net shortwave;
- atmospheric and canopy longwave;
- sensible heat;
- latent heat and vapor mass;
- snow/ground conduction;
- advected precipitation heat;
- surface temperature and canopy temperature;
- snow cold content, melt, refreeze, retained liquid, and routed liquid;
- bulk and multilayer snow state;
- canopy cover, LAI, and seasonal canopy projection;
- the Stage A/B sublimation algorithms and their prior verdicts; and
- every default, opt-in, rollback, environment selector, and diagnostic output
  that could affect a four-cell experiment.

Classify every required equation, constant, state input, selector, and output as
`AUTHORITATIVE_CURRENT`, `AUTHORITY_ADMISSIBLE`,
`AUTHORITY_MISSING`, `IMPLEMENTATION_MISSING`, `REJECTED_PRIOR`,
`DIAGNOSTIC_ONLY`, or `OUT_OF_SCOPE`.

### Observation and fixture reconciliation

Inventory candidate warm-maritime, cold-continental, conifer, mixed,
deciduous/leaf-off, and open-control lanes. For each modeled/observed pairing,
record:

- exact source and custody identity;
- location, period, temporal resolution, and units;
- forcing source and uncertainty;
- modeled canopy stratum and observation stratum;
- observed quantity and comparison operator;
- whether the lane distinguishes longwave, sublimation, both, or neither;
- known scale/correspondence limitations; and
- prospective role: `CALIBRATION`, `INDEPENDENT_VALIDATION`, or
  `DIAGNOSTIC_ONLY`.

Freeze roles before any result-bearing execution. No observation may be both
calibration and independent validation.

### Four-cell factorial design

Specify a comparable experiment using:

| Cell | Longwave | Sublimation |
| --- | --- | --- |
| `B` | off | off |
| `L` | on | off |
| `S` | off | on |
| `LS` | on | on |

All four cells must share forcing, initial state, canopy projection, snow
density model, phase model, liquid-routing model, observation operator, and
reporting period. If current selectors entangle a target mechanism with another
process, record the exact prerequisite for orthogonal selectors; do not
construct non-comparable cells.

For every response `Y`, freeze:

- `longwave_effect = Y(L) - Y(B)`;
- `sublimation_effect = Y(S) - Y(B)`;
- `combined_effect = Y(LS) - Y(B)`; and
- `interaction = Y(LS) - Y(L) - Y(S) + Y(B)`.

The design must cover process operands and downstream responses, including
surface-energy components, vapor mass, SWE, depth, cold content, melt, refreeze,
retained/routed liquid, runoff timing, persistence, and snow-to-frost
insulation where evidence supports interpretation.

### Conservation and anti-tautology design

Author the operand lineage for the future energy and mass ledgers, including
units, sign convention, area/time basis, producer, consumer, authority, and
diagnostic/public status. Identify plausible wrong formulas and aliases,
including:

- sublimation counted as both vapor and routed liquid;
- latent energy debited twice or not at all;
- net longwave confused with incoming longwave;
- canopy longwave added without the displaced sky-view term;
- daily and hourly energy or mass mixed without duration conversion;
- SWE loss inferred from depth change without density/state accounting; and
- producer self-consistency presented as independent closure.

Define the independent reconstruction required of result-bearing successors.

### Human-interpretation artifacts

Produce at least:

- an implementation/authority coverage figure;
- a prior-candidate outcome figure; and
- a fixture/observation discrimination figure.

Figures contain plots only. Every figure has a Markdown sidecar with a
standalone caption, question, population, units, processing, uncertainty,
exclusions, interpretation, and limitations.

## Excluded Scope

- No production Rust edits.
- No canonical science-contract amendment.
- No new longwave, canopy-temperature, emissivity, turbulent-transfer, or
  sublimation equation.
- No selector, default, parser, runfile, fixture, public schema, or release
  change.
- No empirical fitting or calibration.
- No new result-bearing `B/L/S/LS` runtime claim.
- No reactivation of rejected candidates.
- No assurance approval, publication, or release transfer.

If execution proves a small documentation or campaign-local analysis-tool
correction is necessary, it may be made within the declared write set.
Production or canonical-authority changes require a successor package.

## Deliverables

- `artifacts/pre-execution-intent.md`
- `artifacts/required-reading-map.md`
- `artifacts/dependency-manifest.csv`
- `artifacts/current-implementation-ledger.csv`
- `artifacts/authority-gap-ledger.csv`
- `artifacts/selector-composition-ledger.csv`
- `artifacts/prior-candidate-disposition.csv`
- `artifacts/observation-fixture-ledger.csv`
- `artifacts/observation-role-freeze.csv`
- `artifacts/factorial-design.md`
- `artifacts/factorial-cells.csv`
- `artifacts/response-operator-ledger.csv`
- `artifacts/mass-energy-operand-lineage.csv`
- `artifacts/rejected-formulas.md`
- `artifacts/calibration-readiness-matrix.md`
- `artifacts/source-acquisition-needed.csv`
- `artifacts/successor-admission-decision.md`
- `artifacts/stop-loss.md`
- `artifacts/science-summary.md`
- `artifacts/figures/*.svg`
- `artifacts/figures/*.md`
- `artifacts/gate-evidence.md`
- `artifacts/exact-diff-reconciliation.md`
- `artifacts/line-count-governance.md`
- `artifacts/review-agent-a.md`
- `artifacts/review-agent-b.md`
- `artifacts/finding-disposition.md`
- `artifacts/verification-agent-a.md`
- `artifacts/verification-agent-b.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

Campaign-local deterministic scripts may be added under `tools/` and must be
documented in `artifacts/README.md`.

## Intended Write Set

- `docs/ROADMAP.md`
- `docs/planning/snow-surface-energy-balance-roadmap.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260729-snow-surface-eb-01-reconciliation-factorial-design-001/**`

All production source, canonical science contracts, fixtures, assurance
sources, public outputs, and external repositories are protected read-only
inputs.

## Phase Plan

### Phase 0 — Intake and freeze

Record the exact Git base, status, intended write set, required-reading budget,
dependency identities, pre-implementation intent, and observation-role
freeze procedure. Reconcile any dirty worktree without overwriting unrelated
user changes.

### Phase 1 — Current-state reconciliation

Trace the real production and opt-in consumer paths. Reconstruct selector
composition and compare source behavior with retained package claims. Record
stale planning statements rather than silently treating them as current.

### Phase 2 — Authority and observation sufficiency

Map every required process operand to canonical authority and implementation.
Freeze observation correspondence and roles. Produce a concise acquisition list
only for sources or data that materially block a successor decision.

### Phase 3 — Factorial and closure contract

Pre-register comparable `B/L/S/LS` cells, response operators, marginal/combined
effects, interaction, mass/energy operands, rejected formulas, independent
reconstruction, and stop-loss.

### Phase 4 — Successor admission

Decide separately whether:

- `EB-02` may implement sub-canopy longwave;
- `EB-03` may compose or revise sublimation; and
- `EB-04` will be executable after those prerequisites.

Each decision must be `GO`, `HOLD_FOR_AUTHORITY`, `HOLD_FOR_DATA`,
`NOT_NEEDED`, or `REJECTED`, with evidence. Missing data alone must not block
authoritative implementation/readiness work allowed by ADR-0042.

### Phase 5 — Evidence, review, and disposition

Generate the human-readable figures and sidecars, run applicable direct
validation, reconcile the exact terminal diff, complete dual independent
scientific/implementation reviews, disposition every finding, complete dual
terminal verification, and close truthfully.

## Validation And Acceptance

Before execution, record the exact selected validation in
`artifacts/pre-execution-intent.md` under
`docs/standards/testing-and-gate-strategy.md`. This package is documentation and
analysis only unless its exact terminal diff proves otherwise.

Minimum current-scope checks:

- parse every CSV with `.venv/bin/python`;
- parse every generated SVG as XML and verify one `<title>`, one `<desc>`, and
  `role="img"`;
- verify every figure has exactly one Markdown sidecar;
- validate all local Markdown links in the package and campaign roadmap;
- run the repository's canonical scoped Markdown validation;
- run deterministic analysis-tool self-checks, if tools are added;
- inspect `git diff --check` for the declared non-generated write set; and
- reconcile intended versus actual terminal paths and validation scope.

If execution discovers any Rust, test, contract, fixture, or production impact,
stop before making that edit, amend the package prospectively, and select the
additional direct gates required by the canonical testing strategy. Do not
silently expand this characterization package into implementation.

## Exit Criteria

The package may close `COMPLETE` only when:

1. Current source, selectors, ledgers, and retained verdicts are reconciled.
2. Every required equation, constant, state input, and output has a truthful
   authority/implementation classification.
3. Observation roles and canopy-stratum correspondence are frozen.
4. `B/L/S/LS` cells are fully specified and comparable, or each missing
   prerequisite is assigned to a named successor.
5. Marginal, combined, and interaction estimands are frozen for every response.
6. The future mass and energy ledgers have explicit operands, signs, units,
   rejected formulas, and independent reconstruction.
7. Double counting between latent energy, vapor mass, melt, and liquid routing
   has an explicit prevention and verification design.
8. Stop-loss and successor admission decisions are machine-readable and
   evidence-backed.
9. The calibration-readiness matrix dispositions every applicable obligation.
10. Required figures, sidecars, validation, reviews, finding disposition,
    verification, and exact-diff evidence pass.

Any unresolved current-scope criterion is `HOLD`, `FAIL`, or `NOT RUN`; it may
not be relabeled as passed or retroactively deferred.

## Stop And Hold Boundaries

Valid hold boundaries are:

- missing or contradictory process authority after the admitted authority
  routes are exhausted;
- unavailable source bytes that are necessary to verify a load-bearing equation
  or constant;
- observation identity/correspondence that cannot be resolved enough to assign
  a truthful role; or
- a required conclusion that would need production edits outside this package.

Large scope, difficult analysis, non-identifying data, or an inconvenient prior
nonpromotion are not themselves hold boundaries.

## Protected Boundaries And Safety

- No secrets, network mutations, external messages, or destructive actions.
- External acquisition is read-only and only when a load-bearing source is not
  locally available; record provenance and licensing.
- No surrogate physics, silent default, fitted forcing, site tuning, or
  canonicalize-and-proceed behavior.
- Preserve current defaults, rollback selectors, public schemas, fixtures, and
  canonical contracts.
- Treat the pinned legacy comparator and PySnobal/libsnobal as reference or flag
  evidence according to their admitted authority, not automatic correctness
  targets.

Security-impact gate: `NOT APPLICABLE` unless execution discovers a parser,
selector, external-input, or publication change. Such discovery requires
prospective package amendment before edits.

## Review And Verification

Require two independent reviews:

- Review A: science authority, observation roles, experimental design,
  identifiability, and stop-loss.
- Review B: source/selector/consumer reconciliation, units, mass/energy
  anti-double-counting, reproducibility, and package governance.

Every finding must be dispositioned as `accepted`, `rejected`, `deferred`, or
`follow-up`, with rationale. Accepted findings must be corrected and rechecked.
Undispositioned findings block closure.

After finding resolution, require two independent terminal verifications of the
exact final tree. Both must verify the Validation Evidence Non-Deferral Rule,
the intended/actual write set, artifact completeness, and the truthful final
disposition.

Subagent authorization: this package explicitly authorizes spawning/delegating
to two read-only review subagents and two read-only terminal-verification
subagents for the scopes above. Expected outputs are
`artifacts/review-agent-a.md`, `artifacts/review-agent-b.md`,
`artifacts/verification-agent-a.md`, and
`artifacts/verification-agent-b.md`; write access is bounded to those exact
package-local artifacts. This package does not authorize subagents to edit
production code, canonical contracts, fixtures, or roadmap decisions.

Subagent requirement: none for ordinary analysis. If execution selects a heavy
batch, comparator suite, or campaign-strength full-workspace gate, spawning a
`comparator_suite_runner` is required by prompt-wording governance; it receives
read-only source access and may write compact metrics/log paths only under this
package.

## Line-Count Governance

Record all touched `.rs` files even though none are intended. Any touched file
at or above 2,000 lines is `WARN`; any nonexempt file at or above 3,000 lines
requires refactor before closure. A discovered Rust edit triggers prospective
scope amendment and additional validation before the edit.

## Decision Log

- Decision: use a four-cell factorial rather than a sequential one-mechanism
  comparison.
  Rationale: marginal effects do not determine the combined response when both
  processes share snow temperature, cold content, mass, and energy.
  Date/author: 2026-07-29 / user and Codex.
- Decision: require one physical mass/energy composition ledger.
  Rationale: sublimation must remove vapor mass and latent energy exactly once;
  adding longwave to a separate melt pathway could otherwise create a
  compensating or double-counted result.
  Date/author: 2026-07-29 / Codex.
- Decision: EB-01 is reconciliation and experimental design only.
  Rationale: current source and authority must be reconciled before any new
  production physics or selector architecture is authorized.
  Date/author: 2026-07-29 / Codex.

## Surprises And Discoveries

Populate during execution. The scaffold-time source inspection indicates that
the multilayer Stage 3 surface-energy sum currently passes shortwave with other
surface terms zero, while Stage A/B sublimation is applied in a separate
mass-loss path. Execution must verify this against the exact base commit.

## Outcomes And Retrospective

Pending execution.
