# Land-Surface Energy-Balance Authority

Status: `queued`

Date: `2026-08-08`

Package ID: `20260808-land-surface-energy-balance-authority-001`

Plan class: `Critical kernel science-authority admission`

## Objective

Define a canonical first-class land-surface energy-balance subsystem contract
for openWEPP. Bind the snow-free and post-snow control volume, state variables,
flux signs, water/energy ledgers, branch ordering, typed failures, and ownership
boundaries needed by a later implementation package without changing current
production physics or consuming censored snow-terminal handoffs.

## Implementation Intent

Intent is `science-authority and implementation-readiness`, not production
implementation, calibration, validation, activation, publication, or cutover.
This package authors canonical authority and executable contract obligations.
It must not introduce partial, proxy, or heuristic land-surface physics into a
runtime path.

## Included Scope

- Create `SC-LANDSURFACEENERGY-001` under the kernel contract profile.
- Inventory authoritative WEPP baseline routines and existing openWEPP
  soil/frost/ET/runoff/snow boundaries relevant to the surface control volume.
- Define control-volume state, units, sign conventions, flux/component ledgers,
  liquid and energy custody, event chronology, branch/guard mapping, aliases,
  constants, tolerances, calibration posture, test vectors, and gap register.
- Define exact ownership boundaries with snow, infiltration/runoff, soil heat,
  frost, evaporation/ET, and precipitation without duplicating wepppy climate,
  GIS, orchestration, or run state.
- Add contract-derived integration tests and pass the pre-implementation
  contract gate, dual contract review, finding disposition, and dual terminal
  verification.
- Produce an implementation-ready handoff that names the first executable
  production slice and its real-consumer proof.

## Excluded Scope And Claim Limits

- No Rust production implementation or runtime selector.
- No consumption or disposition of schema-v8 terminal liquid, unallocated
  energy, or unevaluated time.
- No new snow-free evaporation, infiltration, runoff, ponding, soil heat,
  frost, or ET equations in production.
- No CoE or Stage 3 ownership change, public output, default, calibration,
  validation, promotion, release, or cutover claim.
- No surrogate, provisional, proxy, or heuristic physics.

## Dependencies And Authority

- `SC-SNOWENERGY-001`, `SC-SNOWFREEZE-001`, applicable hydrology/frost/ET
  contracts, ADR-0011, ADR-0012, ADR-0017, ADR-0042, and unit governance.
- WEPP technical reference `references/50201000` and canonical pinned baseline
  `/workdir/wepp-forest_260430_baseline` at
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Completed terminal numerics package, used only to define a censored upstream
  handoff boundary.

## Intended Write Set

- This package tree, `docs/ROADMAP.md`, and `docs/work-packages/README.md`.
- `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`,
  the contract index, and assurance DRAFT locks/receipts only if required.
- A focused contract-derived integration test and `Cargo.toml` registration.

Any production Rust, runtime trace, output schema, fixture, or external-suite
write requires prospective package amendment before edits.

## Phase Plan

1. Freeze scope, reading map, source/provenance inventory, operand lineage,
   intended write set, validation intent, and scaffold commit.
2. Author the canonical contract and contract-derived tests.
3. Run and record the pre-implementation contract gate.
4. Complete two independent science/architecture reviews and disposition every
   finding.
5. Run focused, quick, full-workspace, documentation, assurance, security/data,
   and authority anti-evasion gates selected for the exact diff.
6. Complete two independent terminal verifications, archive the kickoff prompt,
   update roadmap/catalog, disposition the package, and commit the stable result.

## Contract And Conservation Acceptance

The contract must define independently reconstructible water and energy
identities with explicit units, time/area basis, source authority, sign, owner,
and authoritative/diagnostic status. Test vectors must separate adjacent flux
aliases and reject omitted/double-counted precipitation, snow handoff, latent,
sensible, radiation, soil heat, storage, evaporation, infiltration, runoff, and
surface-water operands. Contract authority may define future acceptance but
cannot claim runtime closure before a real consumer exists.

## Validation And Exit Criteria

- Contract satisfies the complete kernel-process profile and canonical schema.
- Baseline provenance map covers every admitted equation/process family or
  records an explicit non-promotable authority gap.
- Contract-derived tests bind purpose, state, units, equations, branch order,
  typed guards, ownership, calibration posture, test vectors, and gap labels.
- Pre-implementation contract gate passes before any production edit; this
  package is expected to make no production edit.
- Focused contract, quick, and critical full-workspace gates pass directly.
- Dual independent reviews, finding disposition, and dual terminal verification
  pass with no unresolved current-scope requirement.
- Prompt archive, assurance, security/data, line-count, exact-diff, and docs
  evidence reconcile to the terminal tree.

## Calibration Readiness

This package defines fixed process authority and prospective parameter
classification. It performs no empirical calibration or independent
validation. The readiness matrix must use canonical ADR-0042 enums and
disposition all ten obligations.

## Security And Data Impact

Local source/reference reads and Markdown/Rust contract tests only. No secrets,
network actions, deployments, external messages, observations, or public
release.

## Review And Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only science/architecture reviewers,
one read-only baseline-provenance reviewer, one heavy-gate runner with writes
limited to ignored target logs, and two independent read-only terminal
verifiers. Expected outputs are compact source maps, severity-ranked findings,
exact commands/counts, finding closure, and verification verdicts.

## Progress

- [x] (2026-08-08) User authorized scaffold and end-to-end execution.
- [ ] Scaffold and commit the authority package.
- [ ] Author and gate canonical contract authority.
- [ ] Complete reviews, exact-head gates, verification, and disposition.
