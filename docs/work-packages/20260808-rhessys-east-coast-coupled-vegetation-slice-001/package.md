# RHESSys East Coast Coupled Vegetation Slice

Status: `active / licensed-source audit next`

Date: `2026-08-08`

Package ID: `20260808-rhessys-east-coast-coupled-vegetation-slice-001`

Plan class: `Critical source-aware constitutive migration and compatibility`

## Objective

Admit and implement the first default-off `openwepp-vegetation` slice from the
pinned MIT-licensed RHESSysEastCoast source and GIS2RHESSys vegetation profiles.
The implementation must preserve the coupled process and state relationships
needed for deciduous, evergreen, and mixed forests, accept existing RHESSys
stratum vegetation definitions through a strict provenance-preserving loader,
and retain `SC-VEGETATION-001` ownership and conservation boundaries.

## User Direction And Big Picture

The user explicitly selected the licensed source-aware route on 2026-08-08,
does not want scope constrained merely to keep the earlier slice small, requires
deciduous and mixed-forest behavior, and wants to reuse existing RHESSys
vegetation files. The predecessor literature-only package is therefore
superseded rather than forced to choose an isolated empirical conductance law.

This package does not presume that every source branch or parameter is correct.
It separates three authorities:

- RHESSysEastCoast is licensed implementation provenance and a comparator;
- GIS2RHESSys is licensed vegetation-format and parameter-profile provenance;
- canonical contracts, cited science, conservation, and admitted observations
  control scientific and production claims.

## Pinned Source Identity

| Role | Repository | Local checkout | Commit | License |
|---|---|---|---|---|
| Coupled process implementation | `https://github.com/laurencelin/RHESSysEastCoast` | `/workdir/RHESSysEastCoast` | `375c75b1cd2202217651dff43aa113d80b9c1118` | MIT; `LICENSE` SHA-256 `4fd4ecf2fd01cf53c99754bcac5a6dbee255a0be0539dd84ffe12e06808374be` |
| Profile collection and definition generation | `https://github.com/laurencelin/GIS2RHESSys` | `/workdir/GIS2RHESSys` | `6b20883dea7c9fd92f71ec69eaca015ebf6dfe18` | MIT; same license digest |

Both checkouts were clean on intake. Source identity is immutable for this
package: upstream movement requires a prospective package amendment and a new
inventory/diff. The separate unlicensed official RHESSys checkout is not an
authorized direct-translation substitute.

## Implementation Intent

Intent is `source-aware science implementation + format compatibility +
calibration readiness`. Risk is `Critical`. The package may inspect and adapt
the pinned MIT source directly, but production behavior still requires
contract-first admission. Default activation, empirical calibration, and
production cutover are excluded.

The first phase is an audit, not production coding. It must determine the
smallest coherent process boundary from evidence rather than assume that a
single function is independently meaningful.

## Required Coupling Interpretation

RHESSysEastCoast uses Jarvis-style conductance multipliers for absorbed PAR,
leaf water potential, CO2, minimum temperature, and vapor-pressure deficit,
scaled by maximum conductance and canopy state. Conductance participates in the
Penman-Monteith water-flux path and the Farquhar photosynthesis path. That is a
coupled water/conductance/carbon implementation, but it is not automatically a
Bonan-style bidirectional assimilation-solved stomatal formulation.

The source inventory must record exact dependency direction, state cadence,
iteration, and mutation. Documentation and contracts must not use “coupled” as
a stronger claim than the source and tests support.

## Deciduous And Mixed-Forest Acceptance

The supported data model is explicitly multistratum:

- a deciduous-only case with leaf-on, leaf-off, transfer, and dormancy behavior;
- an evergreen-only case with year-round canopy state;
- a mixed deciduous-evergreen case retaining at least two profile identities;
- deterministic vertical ordering, cover fractions, transmitted radiation, and
  root-profile identities for every stratum; and
- no averaging of unlike profile rows into an invented “mixed forest” default.

`GIS2RHESSys/vegCollection.csv` currently exposes 71 fields and 32 profiles,
including generic and East Coast species profiles. These counts are intake
facts, not a claim that all rows are complete or validated. The parameter audit
must disposition each field and each candidate first-slice profile.

## Existing Vegetation-File Compatibility

The compatibility boundary includes:

- GIS2RHESSys `vegCollection.csv` at the pinned commit;
- generated RHESSys `stratum_*.def` key/value files;
- stable mapping from input key to typed parameter, units, source profile,
  original spelling/value, and migration disposition; and
- deterministic round-trip or normalized diagnostic output sufficient to
  identify every consumed, ignored, transformed, and rejected field.

The loader must fail closed for missing required keys, duplicate keys,
unsupported sentinels, non-finite values, invalid domains, and incompatible
profile/process combinations. It may retain unknown keys for diagnostics but
must not silently apply C-parser defaults. Compatibility means existing valid
files can be used; it does not require preserving accidental parser behavior.

## Included Scope

- Exact source/function/state/call inventory for RHESSysEastCoast canopy
  radiation, interception, conductance, Penman-Monteith, Farquhar
  photosynthesis, phenology, root demand, and the minimum coupled dependencies.
- Exact field/profile/citation/default inventory for GIS2RHESSys
  `vegCollection.csv` and generated stratum definition behavior.
- MIT notice custody and per-surface provenance records.
- Canonical contract amendments needed for the selected coherent slice.
- Contract-derived tests and one required Level-4 external-authority suite
  created before production Rust.
- A typed strict vegetation-definition loader and default-off Rust crate slice.
- Explicit deciduous, evergreen, and mixed multistratum fixtures.
- Independent conservation and operand reconstruction, source-differential
  comparison, and calibration-readiness evidence.

## Excluded Scope And Claim Limits

- No default activation, public output change, release, deployment, or cutover.
- No direct soil-water mutation; vegetation proposes layer demand and consumes
  hydrology-authorized withdrawals through the admitted Stage A/B/C protocol.
- No silent port of source sentinel `9999`, nonzero numerical floors, implicit
  defaults, commented experimental branches, or canonicalize-and-proceed logic.
- No claim that a GIS2RHESSys parameter row is calibrated, transferable, or
  scientifically authoritative merely because it exists in the licensed CSV.
- No universal mixed-forest profile synthesized by averaging component rows.
- No soil carbon/nitrogen mineralization, succession, fire, or canopy-snow law
  unless a prospective package amendment proves it is inseparable from the
  selected first slice and supplies the required authority and tests.
- No parity-as-truth posture. Source comparison flags differences; it does not
  override canonical science or conservation.

## Protected Boundaries

- Existing production behavior and consumers remain unchanged.
- Canopy, ground, litter, snow, and soil energy/water operands retain distinct
  identities and area/interval bases.
- Soil hydrology remains the sole owner of layer liquid/frozen storage and the
  arbiter of shared withdrawals.
- Existing generalized-GSI and ET authorities remain active until a later real
  consumer and cutover package explicitly retires them.
- Parameters are explicit, typed, unit-defined, source-tagged, and checksumable;
  no hidden global defaults enter the production API.

## Source-Aware Admission Rubric

Every candidate surface receives a recorded score and disposition:

| Criterion | Pass condition |
|---|---|
| License and identity | Pinned MIT source, exact file/function or data row, notice custody recorded. |
| Coupling completeness | All required callers, callees, state, cadence, feedback direction, and mutation are mapped. |
| Scientific traceability | Equation/parameter has a primary citation or is labeled source-observed/assumed with a bounded claim. |
| openWEPP ownership | State and flux owner agrees with canonical contracts; no duplicate storage or publication owner. |
| Numerical safety | Domains, units, sentinels, floors, defaults, branch order, convergence, and typed failure are adjudicated. |
| Forest coverage | Deciduous and mixed multistratum behavior is representable and testable without parameter averaging. |
| Compatibility | Valid existing vegetation definitions map deterministically and every unsupported field fails or reports explicitly. |
| Independent testability | Non-degenerate vectors permit reconstruction and make plausible wrong formulas numerically distinct. |

Disposition is `ADOPT`, `ADAPT`, `RE-DERIVE`, `DEFER`, or `REJECT`. No surface
may enter production Rust with an unresolved rubric row.

## Phase Plan

1. Freeze pre-implementation intent, exact base/diff, instruction map, source
   commits, license digests, required-reading budget, and owned write set.
2. Produce the function/state/call inventory and 71-field/32-profile audit.
   Identify source defects, implicit defaults, citations, units, and coupling.
3. Select the smallest coherent coupled boundary that satisfies deciduous and
   mixed-forest acceptance. Amend this package prospectively if its exact write
   set changes.
4. Amend canonical contracts; add contract-derived tests, locked fixtures, and
   the required A3 suite; complete the pre-implementation contract gate.
5. Implement the strict loader and default-off Rust slice with typed errors and
   explicit source/provenance identities.
6. Add deciduous, evergreen, mixed-stratum, limiting, negative, property,
   conservation, independent-reconstruction, and source-differential tests.
7. Complete calibration-readiness analysis and classify all parameter claims.
8. Reconcile the exact diff and run every applicable focused, source-quality,
   A0/A1/A3, anti-evasion, dependency, security/license, quick, and Critical
   full-workspace requirement.
9. Complete independent science/source and Rust reviews, disposition findings,
   rerun invalidated gates, then perform independent terminal verification.
10. Archive the kickoff prompt, update lifecycle records, disposition the
    package, and commit/push the stable increment when authorized.

## Contract-First Hard Gate

No production Rust may be authored until canonical contract text,
contract-derived tests, source/profile provenance fixtures, and the required A3
suite bind every selected equation, parameter, unit, guard, and compatibility
transformation. An incomplete source inventory, unresolved scientific-authority
gap, or unresolved ownership conflict yields `executed-hold`; licensed source
code is not a substitute for this gate.

## Intended Write Set

- This package tree.
- Lifecycle edits to `docs/ROADMAP.md`, `docs/work-packages/README.md`,
  `docs/backlog/TRACKER.md`, and the vegetation backlog note.
- Reference bibliography/rights records and a bounded vendored source-notice or
  profile fixture subtree derived from the two pinned MIT repositories.
- `SC-VEGETATION-001`, its index/lifecycle records, and only minimum necessary
  adjacent energy, evapotranspiration, water-balance, plant, management, or
  residue contract amendments.
- `crates/openwepp-vegetation/**`, root workspace manifests, and bounded tests,
  fixtures, and external-authority registry/obligation files for this slice.

Existing production consumers, runners, management schemas, output schemas,
and unrelated crates are excluded. Amend the package before touching them.

## Validation Plan

The execution package must refine exact test names after source audit and before
production edits. Minimum expected terminal commands include:

    cargo fmt --all -- --check
    cargo clippy -p openwepp-vegetation --all-targets -- -D warnings
    cargo nextest run -p openwepp-vegetation
    bash tools/release/check_authority_suite_antievasion.sh
    cargo nextest run --test auth11_required_suite_obligation_guards_contract
    bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md
    cargo test --doc --workspace
    cargo deny check
    cargo nextest run --workspace --profile quick
    cargo nextest run --workspace --profile full
    markdown-doc lint --path docs/work-packages/20260808-rhessys-east-coast-coupled-vegetation-slice-001 --format plain
    git diff --check

Critical full correctness, contract, external-authority, and exact-diff gates
are non-deferrable once implementation begins. Coverage/CRAP remains optional
observational QA for this non-CQR package.

## Exit Criteria

- Both source identities and MIT notices are preserved and auditable.
- The selected source/profile inventory has no unresolved rubric row.
- Canonical contracts and pre-implementation tests predate production Rust.
- Valid pinned vegetation definitions parse deterministically; invalid,
  incomplete, duplicate, sentinel, and non-finite inputs fail as typed errors.
- Deciduous, evergreen, and mixed multistratum non-degenerate fixtures pass with
  independent water, energy, radiation, conductance, and carbon reconstruction
  for every implemented process.
- Source comparator deltas are explained; comparator equality is not used as
  sole scientific promotion authority.
- Existing production manifests and call sites do not consume the new crate.
- Calibration readiness is recorded without empirical validation or
  transferability claims.
- Required independent reviews, Critical gates, terminal diff reconciliation,
  and terminal verifications pass on the final bytes.

## Review And Delegation Requirements

Subagent authorization: this package explicitly authorizes spawning/delegating
to two independent read-only science/source reviewers, one
`rust_code_reviewer`, one `rust_qa_reviewer`, one
`comparator_suite_runner`, and two independent read-only terminal verifiers.
Expected outputs are the named review, review-disposition, gate, and verification
artifacts; write access is read-only for reviewers/verifiers and limited to
ignored target/log/scratch paths for the comparator runner.

Execution requires those roles as defined by `docs/work-packages/AGENTS.md`.
Actual subagent use also requires session-level user or policy authorization.
If unavailable, record the block and do not claim the required review or gate.

## Progress

- [x] (2026-08-08) User selected the source-aware path and required deciduous,
  mixed-forest, and existing RHESSys vegetation-file support.
- [x] (2026-08-08) Pinned clean RHESSysEastCoast and GIS2RHESSys commits and
  verified matching MIT license files.
- [x] (2026-08-08) Superseded the narrow literature-only implementation package
  while retaining its evidence.
- [x] (2026-08-08) Scaffolded this package, prompt, evidence placeholders, and
  lifecycle/backlog/reference updates.
- [ ] Complete source/function/state and field/profile inventories.
- [ ] Select and admit the exact coherent first implementation boundary.
- [ ] Pass the contract-first gate before production Rust.
- [ ] Implement, validate, review, verify, and disposition the slice.

## Surprises And Discoveries

- The GIS2RHESSys collection has profiles suitable for deciduous, evergreen,
  shrub, grass, and numerous East Coast taxa, but no single authoritative mixed
  profile. Mixed stands therefore need explicit multistratum composition.
- RHESSysEastCoast conductance is Jarvis-style and shares state with both
  Penman-Monteith water flux and Farquhar photosynthesis. Dependency direction
  must be audited before describing it as bidirectionally coupled.
- The C implementation contains behavior that conflicts with openWEPP's typed
  failure posture, including implicit parameter defaults, sentinels, and
  conductance floors. Licensing permits inspection; it does not require porting
  those behaviors.

## Decision Log

- Decision: Use only the two pinned Laurence Lin MIT repositories for direct
  source-aware migration.
  Rationale: they provide explicit permission and the East Coast profiles and
  process variants the user selected; the separate official source remains
  license-ambiguous.
  Date/Author: 2026-08-08 / user and Codex.
- Decision: Treat mixed forest as explicit multistratum composition.
  Rationale: distinct phenology, optics, conductance, height, cover, and rooting
  cannot be conserved by an undocumented averaged profile.
  Date/Author: 2026-08-08 / Codex.
- Decision: Widen the first-slice audit to coupled conductance,
  Penman-Monteith, photosynthesis, phenology, and root demand.
  Rationale: package size is subordinate to a coherent source and state
  boundary; actual implementation remains contract-gated.
  Date/Author: 2026-08-08 / user and Codex.

## Outcomes And Retrospective

Planning/scaffold outcome only. No canonical science contract, Rust source,
fixture, external-authority suite, production consumer, or runtime default has
changed. The next actionable step is the pinned source and parameter inventory.

## Idempotence And Recovery

Source audits are read-only against pinned clean checkouts. Record hashes before
and after any extracted fixture. Do not modify either external checkout. If the
contract-first gate fails, preserve the audit and close `executed-hold` without
production Rust. Never reset unrelated user work.

Revision note (2026-08-08): initial licensed-source successor scaffolded from
the user's deciduous/mixed-forest and vegetation-file compatibility direction.
