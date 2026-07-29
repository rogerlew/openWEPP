# CANOPY-LITTER-SOURCE-AUTHORITY-01

Package ID: `20260728-canopy-litter-source-authority-001`

Status: `COMPLETE — EXTERNAL BOUNDARY IMPLEMENTED / PREDICTIVE AUTHORITY HOLD RETAINED`

Date opened: `2026-07-28`

Execution mode: `package-end-to-end`

Package type: contract-first forest litter-source authority, implementation,
calibration-readiness, and conservation closure.

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`.

## Objective

Adjudicate the recurring needle and fine-woody surface-litter source-authority
gaps identified by CAL-05. Determine separately whether predictive process
physics or an external boundary-input interface is authoritative, including
units, timing, required state, material applicability, and mass ledger.
Retain an explicit authority hold on unsupported predictive biology. Amend
canonical contracts first, add contract-derived tests, and implement only an
authority-supported in-scope path through the real residue/depth/frost and
erosion consumers.

The package may close a named substage in `HOLD` only if primary authority is
missing or contradictory after the recorded admission search, or if the
authoritative law requires a state surface outside this package's declared
envelope. Lack of calibration data alone is not a hold condition under
ADR-0042.

## Pre-implementation Intent

- `implementation`: implement authoritative recurring source laws and typed
  input/state surfaces that survive review; an authenticated external boundary
  is an interface, not a recurring predictive source law;
- `calibration-readiness`: `NOT_APPLICABLE` to externally supplied daily
  boundary values unless a successor prospectively defines an estimable
  parameter and objective;
- `empirical-calibration`: excluded unless independently admitted
  tissue-separated dry-mass evidence is sufficient;
- `independent-validation`: excluded unless a prospectively reserved,
  source-compatible cohort is available.

No production edit is authorized until the source-admission matrix, proposed
law, two independent prospective science reviews, finding disposition,
canonical contract amendment, contract-derived tests, and pre-implementation
contract gate pass in that order.

## Authority Questions

1. Does every mixed or conifer native-forest configuration require recurring
   needle litter at the modeled surface-residue boundary?
2. Can evergreen foliage turnover be computed from live evergreen foliar dry
   mass and leaf longevity without inventing a live-to-litter dry-mass
   conversion or seasonal timing law?
3. Does every structurally woody forest require fine-woody surface input, and
   can actual ground deposition be computed from available state without
   conflating branch mortality, attached dead branches, in-canopy mass loss,
   and litterfall?
4. Where a predictive process law is not supported, is an explicit measured
   or prescribed tissue-specific boundary flux scientifically valid, and what
   provenance, units, temporal distribution, and fail-closed completeness
   rules must govern it?
5. How do leaf, needle, and fine-woody inputs enter the existing
   surface/interrill/rill residue pools exactly once before decomposition,
   cover derivation, residue-depth conversion, frost, and erosion consumption?

## Current Evidence Boundary

- CAL-05 proved the direct-runtime surface source/rate operator is
  calibration-ready-data-limited and partially identifiable.
- Native GSI leaf-off transfer is implemented and conserved.
- The retained Hubbard Brook fine-litter object measures total sample dry mass
  and species leaf counts/masses; its comments identify needles and twigs but
  do not provide separated needle or twig dry mass for all samples.
- Harvard reports foliar and pooled nonfoliar carbon flux, not separated
  fine-woody dry mass.
- The pinned legacy WEPP baseline accepts residue additions and decomposition
  parameters but does not provide a general native-forest needle or
  fine-branch production law.
- No missing source may be assigned zero, hidden in decomposition, or inferred
  from pooled nonfoliar material.

## Candidate Authority Set

- `SC-PLANT-001` and `SC-RESIDUE-001`;
- WEPP Chapters 8 and 9 plus pinned baseline residue-addition/decomposition
  paths;
- White et al. (2000), BIOME-BGC parameterization and leaf longevity;
- Kloeppel, Harmon, and Fahey (2007), forest NPP measurement standards;
- CLM5 evergreen background-litterfall equation and its cited primary
  phenology lineage;
- Keane (2008), decade-scale separated surface-fuel litterfall observations;
- Lim et al. (2024), branch turnover, crown-ascent authority, and the
  branch-mortality versus ground-litterfall distinction;
- retained Hubbard Brook and Harvard source objects as
  `DIAGNOSTIC_ONLY` unless separately admitted.

Secondary model documentation may expose an equation but cannot override
peer-reviewed primary evidence or authorize a value by itself.

## Included Scope

- Primary-source and existing-contract admission matrix.
- Static pinned-baseline provenance map.
- Separate needle, fine-woody, and boundary-forcing adjudications.
- Canonical amendments to `SC-PLANT-001` and `SC-RESIDUE-001` when supported.
- Contract-derived tests before production edits.
- Typed schema, parser, projection, phenology/source ledger, runtime handoff,
  and research-output updates required by accepted authority.
- Independent mass reconstruction and real residue/depth/frost/erosion
  consumer proof.
- ADR-0042 readiness and additional-data matrices.

## Excluded Scope

- No universal turnover rate, monthly timing vector, dry-mass retention
  fraction, fine-branch fraction, or source magnitude inferred from a broad
  biome mean.
- No carbon-to-dry-mass conversion without tissue-specific authority.
- No branch mortality treated as immediate ground litterfall.
- No fine-woody source derived from aggregate structural biomass unless the
  contract admits the required branch-pool mapping.
- No empirical refit of leaf-off phenology or decomposition.
- No downstream snow, hydrology, or erosion tuning.

## Declared Write Set

- `docs/work-packages/20260728-canopy-litter-source-authority-001/`
- `docs/work-packages/README.md`
- `docs/planning/canopy-phenology-assurance-roadmap.md`
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
- `docs/specifications/science-contracts/index.md`
- root `Cargo.toml` and `Cargo.lock` for contract-test registration and the
  schema's SHA-256 dependency
- `crates/openwepp-management-schema/`
- `crates/openwepp-input-contract/`
- `crates/openwepp-plant-phenology/`
- `crates/openwepp-hillslope-orchestrator/`
- `crates/openwepp-runner/`
- contract-derived and integration tests under the affected crate trees and
  `tests/integration/`
- native canopy fixture YAML only if review admits non-fitted,
  source-provenance-bound values; otherwise fixtures remain read-only

Unrelated dirty-worktree changes, CAL-04B/CAL-05 evidence, protected
observation sources, climate, soils, slopes, and all other production domains
are read-only.

## Contract-first Phase Plan

1. Authenticate current gaps, source objects, baseline behavior, and candidate
   primary authorities.
2. Publish the source-admission matrix, proposed executable laws, required
   operands, unit map, and explicit rejected alternatives.
3. Obtain two independent prospective science reviews and disposition every
   finding.
4. Amend canonical contracts.
5. Add contract-derived tests and record the pre-implementation contract gate.
6. Implement the accepted typed source path and real consumers.
7. Run independent conservation reconstruction, failure vectors,
   sensitivity/identifiability, and applicable focused/domain gates.
8. Obtain two independent terminal reviews and two verifications; reconcile
   the exact diff and close or retain a precisely evidenced authority hold.

## Conservation and Output Acceptance

Before production edits, publish an operand-lineage table covering every live
pool, external source, annual/daily conversion, temporal weight, residue-pool
destination, and consumer. Tests must distinguish at least these rejected
formulas:

- evergreen foliar stock treated as annual litter;
- `evergreen_fraction * summer_foliar_biomass` treated as litter without a
  turnover operator;
- structural biomass treated as annual fine-woody litter;
- branch mortality treated as same-day surface deposition;
- pooled nonfoliar carbon treated as dry fine-wood;
- missing needle or wood source treated as zero; and
- duplicate addition to surface and ground pools beyond the contract's exact
  topology.

Acceptance requires independent reconstruction from produced outputs and a
real consumer proof through decomposition, cover, residue depth, frost, and
erosion. Self-consistency or a one-sided magnitude bound is supporting
evidence only.

## Stop-loss and Hold Legitimacy

Stop result-bearing or production work for:

- absent/contradictory authority for a proposed equation or conversion;
- post-result mutation of an objective, execution axis, or acceptance rule;
- a required state surface outside the declared envelope;
- unexplained mass imbalance, duplicate source application, or stale consumer;
- a production fallback/default masking missing tissue-specific input; or
- any attempt to convert diagnostic pooled observations into fitted authority.

A `HOLD` must name the missing authority or out-of-envelope state, cite the
evidence, describe the in-envelope route considered, explain why it cannot
close, and state the exact article/data acquisition or successor needed.

## Reviews and Delegation

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent prospective science reviewers, two
independent terminal science reviewers, two independent terminal verifiers,
and a comparator-suite runner if a heavy gate is selected. Reviewers and
verifiers may write only their assigned package-local artifact. The
comparator-suite runner is read-only except for package-local logs.

Reviews and verification must assess scientific authority, material and unit
mapping, conservation anti-tautology, gate legitimacy, ADR-0042 claim limits,
and whether every accepted finding is closed.

## Exit Criteria

- Every candidate source has an admission decision and citation anchor.
- Needle and fine-woody stages each receive separate science-implementation,
  calibration-evidence, and identifiability statuses.
- Every accepted law is canonical in `SC-PLANT-001`/`SC-RESIDUE-001` before
  tests and production implementation.
- Contract, test, pre-implementation, implementation, consumer, conservation,
  readiness, review, verification, exact-diff, and documentation gates pass.
- Every required current-scope gate is `PASS`; any `BLOCKED`, `FAIL`, or
  unjustified `NOT RUN` forces package `HOLD`.
- No production source magnitude or timing distribution lacks authority.
- No review finding remains undispositioned.

## Progress

- [x] (2026-07-28) Operator directed work to fill the source-authority gap and
  offered to acquire articles if needed.
- [x] (2026-07-28) Opened the contract-first package and began source
  admission.
- [x] (2026-07-28) Adjudicated all accessible primary sources; no article
  remains required from the operator.
- [x] (2026-07-28) Published the proposed prescribed-boundary law, operand
  lineage, rejected predictive laws, and prospective review packet.
- [x] (2026-07-28) First prospective reviews returned `FAIL / HOLD`; recorded
  blocking findings on source anchors, interval-versus-daily support,
  missing-versus-zero semantics, material applicability, parallel residue
  topology, and ADR-0042 claims.
- [x] (2026-07-28) Corrected the review packet with an authenticated source
  ledger, independent per-tissue support, measured-versus-prescribed modes,
  typed applicability, three-surface open-system closure, and exact ADR-0042
  rows.
- [x] (2026-07-28) Both independent final prospective re-reviews passed; all
  findings are closed.
- [x] (2026-07-28) Applied the stop-loss before contract or production edits:
  the requested predictive source authority remains missing, while the
  reviewed external-boundary interface is a distinct implementation choice.
- [x] (2026-07-28) Resumed on explicit direction to implement the
  authenticated prescribed/exhaustive-daily boundary interface; no new
  predictive deposition authority was claimed.
- [x] (2026-07-28) Operator explicitly directed implementation of the
  recommended authenticated external boundary interface; package resumed
  without lifting the predictive-physics hold.
- [x] (2026-07-28) Amended `SC-PLANT-001` and `SC-RESIDUE-001`, authored the
  contract-derived red gate before production edits, and implemented the
  authenticated per-tissue sidecar boundary.
- [x] (2026-07-28) Proved exact source-sum and parallel surface/interrill/rill
  recurrence through the real native-forest decomposition path and published
  explicit tissue status/mode.
- [x] (2026-07-28) Initial terminal reviews and verifications failed closed on
  authority authentication, identity support, drying provenance,
  missing-versus-zero publication, rejection vectors, consumer proof, and
  stale gates; every finding was accepted.
- [x] (2026-07-28) Corrected all terminal findings, expanded the contract
  suite to 16 tests, restored the exact frame-size bound, and extracted every
  touched file that crossed 3,000 lines.
- [x] (2026-07-28) Warnings-denied workspace Clippy and the final exact-head
  full profile pass; 2,117 tests passed and 29 profile-declared tests were
  skipped.
- [x] (2026-07-28) Both terminal re-reviews and both terminal
  re-verifications passed after independently closing every initial finding.
- [x] (2026-07-28) Reconciled the exact scoped diff, passed final
  documentation lint and whitespace validation, and closed the package with
  the predictive-authority hold retained.

## Decision Log

- Decision: Treat predictive canopy source generation and prescribed
  tissue-specific boundary forcing as distinct candidate authority routes.
  Rationale: a measured surface flux can be authoritative without implying
  that current canopy state predicts it; conflating the routes would hide
  missing biology.
  Date/Author: 2026-07-28 / Codex.
- Decision: Preserve branch mortality, attached dead branch storage, in-canopy
  mass loss, and actual surface deposition as separate quantities.
  Rationale: Lim et al. (2024) shows that branch litterfall can materially
  understate branch turnover and that turnover is not immediate deposition.
  Date/Author: 2026-07-28 / Codex.
- Decision: Do not implement a predictive recurring needle or fine-woody law
  in this package.
  Rationale: evergreen stock plus longevity omits live-to-litter dry-mass
  retention and timing; fine-wood turnover needs state not carried by
  openWEPP and is not ground deposition.
  Date/Author: 2026-07-28 / Codex.
- Decision: Send an exact-date, tissue-separated, dry-mass surface forcing
  interface to prospective review as the only implementation candidate.
  Rationale: the admitted measurement standards authorize this boundary
  quantity without claiming unavailable canopy biology or inventing a
  temporal distribution.
  Date/Author: 2026-07-28 / Codex.
- Decision: Retain package `HOLD` after authority synthesis rather than
  silently substitute an external forcing interface for predictive canopy
  source physics.
  Rationale: both reviews confirm that the interface is legitimate but does
  not close the requested natural needle/fine-wood generation law. Implementing
  that distinct interface is a material product choice; no canonical contract
  or production edit is needed to truthfully disposition the source search.
  Date/Author: 2026-07-28 / Codex.
- Decision: Resume only the authenticated external daily boundary after the
  operator explicitly selected the reviewed recommendation.
  Rationale: this implements a valid exogenous interface without representing
  it as a predictive needle/fine-wood process law.
  Date/Author: 2026-07-28 / Codex.
- Decision: Publish independent tissue status and source mode alongside
  numeric source operands and all three residue recurrence states.
  Rationale: zero mass on a supported day must remain distinguishable from
  unrepresented or inapplicable tissue, and consumer closure must be
  independently reconstructable from real output.
  Date/Author: 2026-07-28 / Codex.
