# Vegetation Radiation, Interception, And Conductance Slice

Status: `researching-authority-partial`

Date: `2026-08-08`

Package ID: `20260808-vegetation-radiation-interception-conductance-slice-001`

Plan class: `Critical constitutive science implementation and calibration-readiness`

## Objective

Admit independent scientific authority and implement one separately testable,
default-off `openwepp-vegetation` Stage A vertical slice. Given typed canopy,
weather, radiation, liquid-input, root-profile, and read-only soil-layer
observations, the slice must return reconstructible canopy-radiation allocation,
liquid-interception proposals, conductance, potential transpiration, and
layer-resolved water-demand requests without mutating hydrologic soil state.

## Purpose / Big Picture

After this package, developers can explicitly invoke a bounded vegetation crate
and observe a complete potential-response calculation whose equations, units,
parameters, domains, guards, and reference vectors are independently governed.
The result is not a production vegetation replacement: no existing hillslope
consumer calls it, no default selects it, and it publishes no WEPP output.

This is the first constitutive implementation beneath `SC-VEGETATION-001`. It
closes only the radiation/interception/conductance/layer-demand portion of
Stage A. Hydrologic allocation (Stage B), vegetation finalization (Stage C),
actual uptake, carbon and nutrient cycling, canopy snow, runtime activation,
and compatibility cutover require later packages.

## Rationale

The predecessor admitted topology, ownership, transaction ordering, and source
firewall boundaries but deliberately left physiology `AUTHORITY_MISSING`.
Implementing a crate before admitting equations would invite proxy ET or a
code-derived RHESSys translation. This package therefore admits one coherent
literature-derived constitutive chain first, binds it to contract-derived and
Level-4 authority tests, and only then implements the crate.

## Implementation Intent

Intent is `science implementation + calibration readiness`. Empirical
calibration and independent validation are excluded. Parameters must be typed,
enumerable, unit-defined, provenance-tagged, and supplied explicitly; the
package introduces no production defaults. Demonstration values and execution
grids must be labeled `ASSUMED_FOR_EXECUTION` and cannot be represented as
observations, calibrated values, probability priors, or physiological bounds.

Risk is `Critical`: the package admits new constitutive kernel authority, adds
a workspace crate, changes manifest resolution, creates a required A3 suite,
and implements water/energy-sensitive calculations. Campaign-strength
full-workspace correctness is an increment-closure requirement.

## Constitutive Closure Slice

Before production edits, the canonical contract must name one internally
coherent, independently sourced formulation for every item below:

1. canopy shortwave/radiation receipt and stratum allocation needed by the
   slice, with explicit energy basis and closure;
2. liquid interception capacity, storage change, evaporation/depletion, and
   named downward release needed by the slice;
3. aerodynamic and canopy/stomatal conductance needed to convert available
   energy and atmospheric state into potential water demand;
4. potential transpiration mass/energy conversion and interval integration;
5. use or derivation of a layer root-participation profile and the exact rule
   producing non-negative `D_s,l`; and
6. all constants, parameter domains, unit conversions, branch order,
   zero/empty behavior, tolerances, and typed failures used by items 1-5.

The authority phase may select among peer-reviewed formulations, but it may not
mix equations whose assumptions, scales, or state definitions are
incompatible. A selected source must be read independently and cited to an
equation, table, section, or stable locator. Citation names copied from RHESSys
or the predecessor's discovery ledger are leads only. If any required family
cannot be admitted through an allowed route, disposition is `executed-hold`
before production Rust; the missing family cannot be replaced with a proxy,
surrogate, heuristic, or silent default.

## Included Scope

- Independent literature and canonical-authority review for the bounded
  constitutive chain above; no RHESSys source-expression inspection.
- Contract-first amendment of `SC-VEGETATION-001` and only the minimum adjacent
  `SC-LANDSURFACEENERGY-001`, `SC-EVAP-001`, or `SC-WATBAL-001` ownership text
  needed to make the slice unambiguous.
- New `openwepp-vegetation` workspace crate with typed inputs, outputs,
  parameters, provenance identities, and error enums.
- One explicit Stage A evaluation API returning proposals only. It may construct
  a candidate canopy-liquid end state but cannot commit state outside the
  returned proposal.
- Exact radiation-energy, canopy-water, potential latent-energy, and
  layer-demand operand lineage with independently reconstructed tests.
- A required Level-4 constitutive suite with locked fixtures, provenance,
  required-case bindings, and hard-fail posture for every implemented equation
  family.
- Deterministic calibration-readiness evidence: parameter enumeration,
  observation operator/objective reconstruction, sensitivity and
  identifiability analysis, boundary/failure/equifinality reporting, and
  synthetic recovery where structurally meaningful.
- Focused contract, component, property, negative, dimensional, chronology,
  default-off, and anti-alias tests.

## Excluded Scope And Claim Limits

- No RHESSys source read, source-derived translation, naming, comments,
  control-flow structure, constants, defaults, or reversible pseudocode.
- No direct or close implementation from `/workdir/RHESSys` or any other
  inadequately licensed implementation.
- No Stage B allocation policy, fairness/priority rule, or soil-layer mutation.
- No Stage C actual transpiration, vegetation-state commit, compatibility
  reduction, or atomic multi-owner scheduler transaction.
- No hydrologic withdrawal, infiltration, runoff, drainage, percolation,
  snow/frost, ground-surface, residue, carbon, nitrogen, litter, mortality,
  allocation, phenology, or photosynthesis implementation.
- No canopy-snow constitutive law; snow-present input must follow an explicit
  contract-authorized rejection or excluded-domain posture.
- No native-management schema, parser, parameter database, observed-data
  intake, public output, runtime selector, runner/orchestrator call site,
  production consumer, default activation, release, or cutover.
- No claim of empirical calibration, independent validation, transferability,
  production readiness, real-consumer closure, or parity with RHESSys.
- No modification of existing generalized-GSI, ET, interception, hydrology,
  land-surface, snow/frost, or direct-runtime behavior.

## Protected Boundaries

- Vegetation returns `D_s,l`; it never writes liquid or frozen soil storage.
- Existing production consumers and defaults remain byte-for-byte behaviorally
  unchanged. The new crate may be consumed only by its tests and explicit
  package-local diagnostic examples.
- All water and energy quantities crossing the API are interval amounts on one
  explicit horizontal-area basis. Rates are integrated through typed `dt`.
- `Q_rad,k,j` surfaces remain component- and lineage-specific; no universal
  net-radiation scalar may alias canopy, ground, litter, snow, or soil energy.
- Root participation is explicit by layer. Depth alone is not silently expanded
  into a layer profile.
- Empty vegetation, zero area index, zero incident liquid, and zero demand are
  valid only where the admitted authority says so. Invalid domains fail with a
  typed error; there is no canonicalize-and-proceed behavior.
- Existing active authorities remain authoritative until a later real-consumer
  cutover package passes.

## Dependencies And Authority

- Predecessor package and handoff:
  `../20260808-vegetation-source-provenance-and-boundary-authority-001/`.
- Canonical `SC-VEGETATION-001` boundary authority and its named adjacent
  contracts.
- Backlog concept `../../backlog/20260806-rhessys-derived-vegetation-crate.md`.
- ADR-0011 architecture-first authority, ADR-0017 comparator distrust,
  ADR-0042 implementation/calibration readiness, and ADR-0043 direct validation.
- Science-contract schema/profile, unit governance, correctness-authority
  model, and external-authority suite schema/promotion rules.
- Independently consulted peer-reviewed literature. The Gash (1979) and
  Shuttleworth-Wallace (1985) citations in `SC-VEGETATION-001` are discovery
  leads, not pre-admitted formulas.

## Intended Write Set

- This package tree.
- Lifecycle-only edits to `docs/ROADMAP.md`, `docs/work-packages/README.md`,
  `docs/backlog/TRACKER.md`, and
  `docs/backlog/20260806-rhessys-derived-vegetation-crate.md`.
- `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md`, the
  science-contract index, and only minimal ownership/lineage amendments to
  `SC-LANDSURFACEENERGY-001.md`, `SC-EVAP-001.md`, or `SC-WATBAL-001.md`.
- Contract-cycle artifacts under this package for every amended `SC-*` file.
- `crates/openwepp-vegetation/**`.
- Root `Cargo.toml` and `Cargo.lock` for workspace membership, test registration,
  and the minimum required dependency edge.
- `tests/integration/vegetation_radiation_interception_conductance_contract.rs`.
- One new Level-4 suite definition, its fixture directory, integration harness,
  `docs/specifications/external-authority/registry.yaml`, and
  `docs/specifications/external-authority/required-suite-obligations.json`.
- Assurance DRAFT locks/receipts only if exact-diff impact analysis proves an
  existing assurance subject is affected.

Any existing production crate, runner, orchestrator, management schema,
external observed dataset, public schema, release script, or unrelated suite is
outside the write set. Amend the package prospectively before touching one.

## Deliverables

1. Independent constitutive-authority admission with citation locators,
   equation/parameter/domain map, source-license posture, and gap disposition.
2. Canonical contract amendments with complete invariant, guard, unit, alias,
   calibration-readiness, and A3-suite bindings.
3. Contract-derived tests and passing pre-implementation gate captured before
   production edits.
4. A small `openwepp-vegetation` crate implementing the admitted Stage A slice
   behind an explicit call and with typed failures.
5. Required hard-fail Level-4 authority suite and immutable fixture provenance.
6. Independent water/energy/demand reconstruction and anti-tautology evidence.
7. Calibration-readiness matrix and deterministic readiness evidence without
   empirical claims.
8. Dual implementation review, independent science-authority review, finding
   disposition, Critical validation, dual terminal verification, and a bounded
   successor handoff.

## Phase Plan

1. Freeze pre-implementation intent, exact base, write set, instruction map,
   required-reading budget, authority-selection criteria, and scaffold commit.
2. Independently obtain and inspect permissible scientific sources. Record
   citation-level authority, license/access posture, equations, units,
   parameters, validity domains, and incompatibilities. Do not inspect RHESSys
   source expression.
3. Amend canonical contracts for the complete selected chain. Add the
   contract-derived test and required A3 suite definition/fixtures before any
   production Rust.
4. Run two independent science/contract reviews, their finding disposition and
   verification, the pre-implementation contract gate, suite integrity checks,
   and anti-evasion guards. Record the exact clean/dirty identity. Production
   edits are forbidden until all pass.
5. Implement the new crate and explicit Stage A evaluation API. Keep all
   existing production dependency graphs and consumers unchanged.
6. Add component/property/negative tests, independent reconstructions,
   anti-alias fixtures, default-off proof, and calibration-readiness evidence.
7. Reconcile the exact diff and run all directly selected focused, source
   quality, A0/A1/A3, security/license, assurance, quick, and Critical
   full-workspace requirements.
8. Complete two independent implementation reviews, disposition and fix every
   finding, reconcile the resulting exact diff, and rerun every invalidated
   requirement. Then complete two independent terminal verifications on the
   final bytes.
9. Archive the kickoff prompt byte-for-byte, update lifecycle records, write
   disposition and handoff, and commit the stable increment.

## Contract-First Hard Gate

The sequence is binding: canonical contracts, contract-derived tests, A3 suite
fixtures/metadata, pre-implementation gate, then production code. The gate must
prove that every implemented equation, constant, conversion, parameter domain,
guard, and tolerance is already present in canonical authority and independently
testable. Failure or incomplete authority yields `executed-hold` with no
production implementation.

## Conservation And Reconstruction Acceptance

Before production edits, `artifacts/operand-lineage.md` must record every water
and energy operand, units, interval and area basis, source authority, and
authoritative/diagnostic status. Fixtures must make plausible wrong pairings
numerically distinct, including canopy-versus-ground radiation, incident water
versus storage, evaporation versus downward release, rate versus interval
amount, stratum versus stand area, and total demand versus each layer demand.

Acceptance requires independent reconstruction from the public crate result,
explicit rejection of those wrong formulas, canopy-water and radiation-energy
closure on non-degenerate cases, two-sided physically authorized magnitude or
ratio checks where available, and API/metadata alignment. Exact producer
self-consistency and one-sided bounds are supporting sanity evidence only.

## Calibration Readiness

The package performs no empirical calibration or independent validation. The
readiness matrix must disposition every obligation from
`science-contract-spec.md` as `PASS`, `BLOCKED`, or `NOT_APPLICABLE` and report:

- `science_implementation_status`;
- `calibration_evidence_status`; and
- `identifiability_status`.

If measurements are unavailable or non-identifying, continue through applicable
deterministic candidate execution, observation-operator and objective
reconstruction, sensitivity/identifiability diagnostics,
boundary/failure/equifinality reporting, and synthetic recovery. Synthetic
recovery proves machinery only and cannot support empirical or transferability
claims. A required readiness defect, unlike mere lack of data, blocks closure.

## Validation Plan And Exact Commands

The executor must revise this plan only when the exact diff proves a requirement
inapplicable or adds a more conservative requirement. Record argv, working
directory, source identity, result, and evidence path for every selected gate.
At minimum run from `/home/workdir/openWEPP`:

    cargo fmt --all -- --check
    cargo clippy -p openwepp-vegetation --all-targets -- -D warnings
    cargo nextest run -p openwepp-vegetation
    cargo nextest run --test vegetation_radiation_interception_conductance_contract
    cargo nextest run --test cas_l4_vegetation_radiation_interception_conductance_001_contract
    bash tools/release/check_authority_suite_antievasion.sh
    cargo nextest run --test auth11_required_suite_obligation_guards_contract
    bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md
    cargo test --doc --workspace
    cargo deny check
    cargo nextest run --workspace --profile quick
    cargo nextest run --workspace --profile full
    markdown-doc lint --path docs/work-packages/20260808-vegetation-radiation-interception-conductance-slice-001 --format plain
    git diff --check

Run affected adjacent-contract unit checks, contract admission checks,
assurance-source checks, placeholder/stub scans, and any additional test named
by the terminal diff. The full profile is required, not substitutable by quick.
Coverage/CRAP is observational and not required by this non-CQR package.

## Exit Criteria

- Every constitutive family in the bounded slice has admitted independent
  authority, explicit domains/parameters/units/guards, and no remaining
  `AUTHORITY_MISSING` row for the implemented behavior.
- Contract, contract-test, A3-suite, and pre-implementation evidence predates
  the first production edit and passes on the recorded source identity.
- The explicit crate API deterministically returns reconstructible radiation,
  interception, conductance, potential-transpiration, and layer-demand results
  for admitted non-degenerate vectors.
- Typed invalid-domain tests pass; no production `.unwrap()`, `.expect()`,
  silent default, unbounded clamp, or unauthorized normalization exists.
- Default-off proof shows no existing production manifest depends on the crate
  and no existing runner/orchestrator call site invokes it.
- Soil liquid/frozen storage cannot be passed mutably through the public API;
  tests prove the read-only input is unchanged and results are proposals only.
- A0 water/energy closure, A1 invariants, the required A3 suite, anti-evasion,
  calibration-readiness, and all exact-diff gates pass with current evidence.
- No current required gate is deferred. Any unmet requirement forces
  `executed-hold`; campaign deferral is allowed only if declared here before
  implementation with owner, trigger, rationale, and later boundary.
- Dual independent science/contract reviews and verifications, dual
  implementation reviews, finding disposition, and dual terminal verification
  pass with no undispositioned finding.
- All touched `.rs` files satisfy line-count governance: `2000+` is `WARN` with
  decomposition rationale/follow-on intent; `3000+` nonexempt files block
  closure.
- Terminal diff matches the intended write set, the kickoff prompt is archived
  byte-for-byte, lifecycle records are truthful, and final disposition makes no
  runtime, calibration, validation, or cutover claim.

## Gate Evidence Non-Deferral

Each phase may be marked complete only when every phase-owned required gate has
direct current evidence. A gate cannot be moved into a later package after
execution begins. If it cannot be produced inside the declared envelope, stop
at `HOLD` / `executed-hold`, name the blocker, and write a defect-shaped handoff.

## Security, Licensing, And Data Impact

The package may perform read-only literature retrieval and local repository
work. It may not send external messages, deploy, publish, ingest secrets, or
commit copyrighted source documents. Record citation provenance and access
terms; commit only independently authored contract text and small derived test
vectors with reproducible transforms. RHESSys remains outside the permitted
read set. Manifest changes require `cargo deny check`. Any observed dataset
requires prospective package amendment and frozen data-role assignment.

## Review And Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only science/contract reviewers,
one `rust_code_reviewer`, one `rust_qa_reviewer`, one
`comparator_suite_runner`, and two independent read-only terminal verifiers.
The science reviewers independently check citations, equation compatibility,
units, domains, parameters, absence of code-derived authority, and own the
corresponding package-local contract review/verification artifacts. The Rust
reviewers own the two implementation-review artifacts and inspect correctness,
API boundaries, typed failures, tests, maintainability, and gate legitimacy.
The comparator runner executes required quick/full and other heavy suites with
writes limited to ignored target/log/scratch paths. Terminal verifiers inspect
final tracked bytes and evidence read-only. Expected outputs are compact
severity-ranked findings, exact commands/counts, finding closure, and verdicts
in the named package artifacts.

Subagent requirement: REQUIRED during execution for dual science/contract
review and verification, both Rust reviews, Critical heavy gates, and both
terminal verifications. The executor must preserve reviewer independence and
may not prime any second reviewer with the first reviewer's findings before the
second returns an initial report.

## Progress

- [x] (2026-08-08) User authorized package scaffolding.
- [x] (2026-08-08) Package specification, queued artifacts, active kickoff
  prompt, and lifecycle links scaffolded for a local commit.
- [x] (2026-08-08) Independently reviewed available radiation, interception,
  wet-canopy evaporation, conductance, potential-demand, and layer-allocation
  sources without inspecting RHESSys source expression.
- [x] (2026-08-08) Vendored five explicitly CC-BY articles and placed four
  restricted or rights-ambiguous artifacts in the gitignored copyrighted cache.
- [x] (2026-08-08) Recorded `AUTHORITY_PARTIAL`: bounded candidates exist for
  radiation, interception, and layer participation, while conductance and its
  dependent potential-transpiration chain remain blocked.
- [ ] Freeze the exact implementation intent and independent authority set.
- [ ] Complete contract-first admission and the pre-implementation gate.
- [ ] Implement and validate the default-off Stage A crate slice.
- [ ] Complete reviews, Critical gates, verification, lifecycle updates, and
  disposition.

## Surprises & Discoveries

- Observation: `SC-VEGETATION-001` admits Stage A/B/C ownership and closure but
  intentionally leaves every constitutive family in this slice
  `AUTHORITY_MISSING`.
  Evidence: predecessor handoff and `GAP-VEGETATION-002/004/008/009`.
- Observation: the strongest open forest-model sources couple stomatal
  conductance to photosynthesis and/or plant hydraulics, both excluded from the
  current package.
  Evidence: R-136 Sect. 2.2, R-138, and R-139 Sect. 2.1.3.
- Observation: the superficially simpler uncoupled alternatives are not clean
  substitutes. MOD16 is a daily remote-sensing algorithm with biome lookup
  parameters and humidity-diagnosed wetness; Jarvis-style ponderosa-pine
  conductance has published vapor-pressure-deficit counterevidence.
  Evidence: R-132 Sect. 2.5.1 and R-144.
- Observation: an open forest-light study makes extinction-coefficient
  transferability a first-class domain issue; the coefficient varies with
  species, architecture, structure, and period.
  Evidence: R-137 Eq. 1, Sect. 2.2, and results.

## Decision Log

- Decision: Bound this package to a complete Stage A proposal chain with no
  production consumer, Stage B, or Stage C.
  Rationale: the roadmap explicitly separates first-slice implementation from
  hydrologic arbitration and real-consumer shadow packages.
  Date/Author: 2026-08-08 / Codex.
- Decision: Require one required Level-4 suite spanning the coherent chain,
  with individual cases for each constitutive family.
  Rationale: each new constitutive law needs non-deferrable A3 authority, while
  one suite preserves cross-family compatibility and shared operand lineage.
  Date/Author: 2026-08-08 / Codex.
- Decision: Define default-off structurally as no production dependency or call
  site, rather than adding a runtime selector.
  Rationale: runtime selection and consumer integration belong to later roadmap
  items and are explicitly excluded here.
  Date/Author: 2026-08-08 / Codex.
- Decision: Stop at `AUTHORITY_PARTIAL` after the independent literature
  intake; do not select MOD16 merely because it avoids photosynthesis.
  Rationale: its temporal scale, wetness state, parameter topology, and
  empirical remote-sensing purpose are incompatible with the other candidate
  equations absent a reviewed authority bridge.
  Date/Author: 2026-08-08 / Codex.
- Decision: Preserve the radiation extinction coefficient and all conductance
  parameters as explicit provenance-tagged inputs; admit no literature table
  value as a production default during research.
  Rationale: the reviewed sources demonstrate domain and model dependence, and
  the package prohibits invented or silently transferred physiological bounds.
  Date/Author: 2026-08-08 / Codex.

## Outcomes & Retrospective

Research outcome: the package now has an independent literature dossier, a
tracked rights/provenance intake, five licensed source PDFs, and a precise
conductance authority decision point. The research eliminated an unsafe quick
route: stitching MOD16 conductance into JULES storage/root equations would mix
daily remote-sensing, prognostic-storage, and layer-extraction assumptions.

No science contract, suite, fixture, manifest, Rust, runtime, or test
implementation has been performed. The contract-first gate remains closed
pending primary-source adjudication of empirical conductance or a prospective
scope decision to include coupled photosynthesis and plant hydraulics.

## Idempotence And Recovery

Research and validation steps are read-only or additive. If authority admission
fails, retain reviewed evidence and close `executed-hold` without production
Rust. If an implementation edit occurs before the hard gate, restore only that
package-owned production edit, preserve the authority evidence, and return to
Phase 3. Never reset unrelated user work. Heavy gates may reuse evidence only
when source and all relevant inputs are identical under the canonical testing
strategy.

Revision note (2026-08-08): initial scaffold created from the admitted
vegetation boundary handoff and current roadmap objective.

Revision note (2026-08-08): independent literature intake completed to
`AUTHORITY_PARTIAL`; licensed sources vendored and conductance authority gap
recorded before canonical or production edits.
