# Vegetation Source-Provenance And Boundary Authority

Status: `queued`

Date: `2026-08-08`

Package ID: `20260808-vegetation-source-provenance-and-boundary-authority-001`

Plan class: `Critical cross-domain science-authority admission`

## Objective

Promote the RHESSys-derived vegetation concept into reviewed canonical
openWEPP boundary authority without implementing or activating a vegetation
kernel. Produce a sanitized process/state provenance ledger, explicit native
canopy-stratum semantics, and a typed Stage A/B/C vegetation-hydrology
arbitration contract that future `openwepp-vegetation`, land-surface energy,
soil hydrology, snow/frost, residue, and orchestration packages can implement
without duplicate state or flux ownership.

## Rationale

Native ET should emerge from a coherent vegetation state machine coupling
canopy radiation, interception, conductance, transpiration, phenology, carbon,
and litter—not another stand-alone demand-partition formula. The authority
boundary must precede the first vegetation crate slice and the land-surface
constitutive package so both consume the same typed vegetation and hydrology
handoffs.

RHESSys source is scientific comparison evidence, not automatic implementation
authority. Its repository lacks an adequate formal license grant for direct or
closely translated code. Literature-derived and independently re-derived
science may proceed under explicit provenance; direct/code-derived translation
remains prohibited unless licensing or permission is resolved.

## Implementation Intent

Intent is `source-provenance and boundary-authority admission`. This package
performs no Rust implementation, calibration, independent validation, runtime
selection, native-management schema activation, public output, or default
change.

## Included Scope

- Freeze a bounded RHESSys source-analysis request at inspected commit
  `f9d1bbf8d161aa55b6a51061dc320188ead44962`.
- Produce the complete sanitized handoff bundle:
  `source-analysis-request.md`, `approved-spec.md`,
  `compliance-review.md`, `provenance-manifest.md`, and
  `implementation-prompt.md`.
- Inventory candidate process/state families, cadence, units, state mutation,
  coupling order, literature anchors, openWEPP consumers, licensing
  disposition, and `adopt / independently re-derive / compare / reject /
  defer` outcome.
- Define explicit native-stratum cover, vertical overlap, ordering, parameter
  set, initial-state, rooting-profile, and aggregate compatibility semantics.
- Define Stage A potential ecosystem response, Stage B hydrologic arbitration,
  and Stage C vegetation finalization, including exact shared-withdrawal,
  interception, radiation, latent-energy, water, carbon, and litter ledgers.
- Define typed ownership boundaries among native management, vegetation, soil
  hydrology, land-surface energy, snow/frost, residue/biogeochemistry, and the
  hillslope orchestrator.
- Adjudicate canopy-snow ownership as a named boundary or retain it as an
  explicit non-promotable gap; do not duplicate it.
- Create canonical `SC-VEGETATION-001` target authority and make only the
  minimal reviewed amendments required to preserve compatibility/ownership in
  `SC-PLANT-001`, `SC-EVAP-001`, `SC-RESIDUE-001`,
  `SC-WATBAL-001`, and `SC-LANDSURFACEENERGY-001`.
- Add focused contract-derived integration tests and complete the full
  contract authoring review/verification cycle.
- Produce an implementation handoff for
  `VEGETATION-RADIATION-INTERCEPTION-CONDUCTANCE-SLICE`.

## Excluded Scope And Claim Limits

- No production or test-support Rust crate implementation.
- No direct, close, mechanical, or statement-by-statement RHESSys translation.
- No RHESSys-derived naming, comments, control-flow structure, code-only
  constants, or reversible pseudocode in implementation-facing artifacts.
- No native-management schema edit, parser change, migration tool, parameter
  database, runtime selector, fixture, output schema, or public API.
- No vegetation mutation of soil-layer water or frozen-water state.
- No replacement or retirement of current generalized-GSI, ET, canopy,
  interception, litter, residue, soil-water, snow/frost, or routing authority.
- No provisional physiology, hidden RHESSys defaults, proxy ET, invented
  parameter bounds, calibration, empirical validation, promotion, release, or
  default eligibility claim.
- No land-surface constitutive solver or Stage 3/CoE cutover work.

## Source-Analysis Firewall

The execution must keep source-aware analysis separate from independent
contract authorship:

1. Freeze a behavior-oriented request containing no translation instruction.
2. A source-aware analyst writes only a quarantined package artifact and
   returns its path plus digest.
3. An independent compliance reviewer checks the request and artifact for
   semantic rather than source-expression content.
4. Only a passing `approved-spec.md` is available to the contract author.
5. A separate source-aware reviewer may compare the completed contract to
   observed behavior but may not patch it or supply source expression.

Every entry must be labeled `LITERATURE`, `CODE-OBSERVED`, or `INFERENCE`;
scientific equations and constants require literature or separately admitted
canonical authority. RHESSys behavior is a comparator flag, never the
production target.

## Dependencies And Authority

- [RHESSys-derived vegetation crate backlog](../../backlog/20260806-rhessys-derived-vegetation-crate.md).
- RHESSys checkout `/workdir/RHESSys` at
  `f9d1bbf8d161aa55b6a51061dc320188ead44962`, read only and subject to the
  firewall above.
- `SC-PLANT-001`, `SC-EVAP-001`, `SC-RESIDUE-001`,
  `SC-WATBAL-001`, `SC-SNOWFREEZE-001`,
  `SC-LANDSURFACEENERGY-001`, native-management input authority, ADR-0011,
  ADR-0017, ADR-0042, science-contract schema/profile, and unit governance.
- Existing crates `openwepp-management-schema`,
  `openwepp-plant-phenology`, `openwepp-meteorology`, and the direct
  hillslope runtime are architecture inventory only.

## Intended Write Set

- This package tree.
- `docs/ROADMAP.md`, `docs/work-packages/README.md`,
  `docs/backlog/TRACKER.md`, and the concept backlog note only for lifecycle
  and canonical cross-links.
- New `SC-VEGETATION-001`, the science-contract index, and minimal
  ownership/compatibility amendments to the named adjacent contracts.
- One focused contract-derived integration test and `Cargo.toml`
  registration.
- Assurance DRAFT locks/receipts only if exact source validation proves an
  existing assurance subject is affected.

Any Rust production/test-support module, management schema, parser, fixture,
parameter dataset, observed dataset, runtime trace, output schema, comparator
result, or external-suite write requires prospective package amendment before
edits.

## Deliverables

1. Complete sanitized source-analysis handoff bundle with digests and
   compliance disposition.
2. Process/state provenance and licensing manifest.
3. Canonical vegetation boundary contract plus minimal adjacent-contract
   ownership reconciliation.
4. Native-stratum semantic specification.
5. Stage A/B/C synthetic soil-water arbitration specification.
6. Canopy-snow ownership disposition.
7. Contract-derived tests, pre-implementation gate, review cycle, and
   implementation-ready next-slice handoff.

## Phase Plan

1. Freeze scope, write set, required-reading budget, source request, provenance
   categories, validation intent, and scaffold commit.
2. Run the source-aware inventory through the sanitized firewall and complete
   independent compliance review before contract authors consume it.
3. Author canonical vegetation/boundary authority and minimal adjacent-owner
   amendments.
4. Add contract-derived tests and pass the pre-implementation contract gate;
   make no Rust production edit.
5. Complete two independent science/architecture reviews, a separate
   source-expression/compliance review, and disposition every finding.
6. Run focused, documentation, unit/schema, assurance-impact, quick, and
   Critical full-workspace gates selected for the exact diff.
7. Complete two independent terminal verifications, archive the kickoff prompt
   byte-for-byte, update queue/catalog/backlog lifecycle, disposition the
   package, and commit the stable result.

## Boundary Acceptance

- Native-management configuration and vegetation runtime state are distinct.
- Vertical-layer cover closure and cross-layer overlap are explicit.
- Vegetation owns demand and finalization but never mutates soil storage.
- Hydrology owns admissible layer withdrawals and returns reason-coded
  allocations.
- Actual transpiration and latent energy share one exact mass/energy identity.
- Canopy, ground, litter, snow, and soil radiation/latent terms cannot be
  omitted, aliased, or double counted.
- Every shared transfer is independently reconstructible by both owners.
- The compatibility reduction is a named adapter, not a second vegetation
  model, and cannot carry a cutover claim without real downstream consumption.

## Validation And Exit Criteria

- The five-file sanitized handoff bundle passes independent compliance review
  and contains no source excerpt, reversible pseudocode, or code-only authority.
- The provenance manifest covers every admitted or deferred initial-scope
  process family and records source coordinates, external scientific authority,
  ownership, licensing, and disposition.
- Canonical contract artifacts satisfy the complete schema and kernel profile.
- Contract tests bind strata semantics, Stage A/B/C ordering, typed failures,
  ownership, shared water/energy ledgers, compatibility limits, calibration
  posture, and every non-promotable gap.
- Pre-implementation contract gate passes before any production edit; this
  package must contain no production edit.
- Dual independent science/architecture reviews, firewall compliance review,
  finding disposition, and dual terminal verification pass with no unresolved
  current-scope requirement.
- Focused, quick, Critical full-workspace, documentation, unit/schema,
  assurance-impact, security/data, line-count, prompt-archive, and exact-diff
  evidence reconcile to the terminal tree.
- The next-slice handoff is bounded to radiation/interception/conductance and
  layer-resolved demand without placeholder physiology or soil mutation.

## Calibration Readiness

This package defines authority and parameter classification but performs no
empirical calibration or independent validation. The readiness matrix must use
canonical ADR-0042 enums and disposition all ten obligations. Missing
physiological authority remains `AUTHORITY_MISSING`; values introduced only
for an executable example must be `ASSUMED_FOR_EXECUTION` and cannot become
parameter-set bounds or defaults.

## Security, Licensing, And Data Impact

Local read-only RHESSys inspection and openWEPP Markdown/Rust contract tests
only. No secrets, network actions, deployments, external messages, source
redistribution, observed-data intake, or public release. The source firewall
and provenance manifest are closure gates.

## Review And Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to one source-aware read-only analyst with writes limited
to the quarantined source-analysis artifact; one independent read-only
source-expression/licensing compliance reviewer; two independent read-only
science/architecture reviewers; one comparator-suite runner for required heavy
gates with writes limited to ignored target logs; and two independent read-only
terminal verifiers. Expected outputs are artifact paths/digests, compact
provenance maps, severity-ranked findings, exact commands/counts, finding
closure, and verification verdicts.

Subagent requirement: REQUIRED during execution for the source-aware firewall,
independent compliance decision, dual contract review, Critical heavy gates,
and dual terminal verification. The parent must not inspect RHESSys source
expression before receiving the passing sanitized artifact.

## Progress

- [x] (2026-08-08) User authorized package scaffolding.
- [x] (2026-08-08) Scaffolded the package and prepared the local scaffold
  commit; the source-analysis request remains queued for execution.
- [ ] Freeze the source-analysis request and execute the sanitized firewall.
- [ ] Execute source firewall and author canonical boundary authority.
- [ ] Complete reviews, exact-head gates, verification, and disposition.
