# RHESSys East Coast Coupled Vegetation Slice

Status: `held / authority reframe complete / constitutive admission required`

Date: `2026-08-08`

Package ID: `20260808-rhessys-east-coast-coupled-vegetation-slice-001`

Plan class: `Critical source-aware constitutive migration and compatibility`

## Objective

After the code-to-literature precursor is complete, admit and implement the
first default-off `openwepp-vegetation` slice from the
pinned MIT-licensed RHESSysEastCoast source and GIS2RHESSys vegetation profiles.
The implementation must preserve the coupled process and state relationships
needed for deciduous, evergreen, and mixed forests, accept existing RHESSys
stratum vegetation definitions through a strict provenance-preserving loader,
accept caller-supplied site values and compatible initial state, and retain
`SC-VEGETATION-001` ownership and conservation boundaries. The native-forest
path must compute canopy transpiration, wet-canopy evaporation, forest-floor
evaporation, and layer-root uptake as independent closing components; it must
not preserve the agricultural `Kcb`/LAI PMET redistribution.

## User Direction And Big Picture

The user explicitly selected the licensed source-aware route on 2026-08-08,
does not want scope constrained merely to keep the earlier slice small, requires
deciduous and mixed-forest behavior, and wants to reuse existing RHESSys
vegetation files. The predecessor literature-only package is therefore
superseded rather than forced to choose an isolated empirical conductance law.
The user subsequently directed the source-versus-literature audit into a
separate hard-gate precursor so implementation does not discover authority gaps
after Rust work begins.

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

The precursor owns source, citation, parameter, concordance, deviation, and
authority-gap auditing. This package may start only after that predecessor has
a terminal disposition and has prospectively amended this package to the exact
verified implementation boundary.

## Precursor Hard Dependency

Required predecessors:

- `../20260808-rhessys-east-coast-code-literature-authority-audit-001/`;
- `../20260808-rhessys-east-coast-vegetation-authority-admission-001/`; and
- `../20260809-rhessys-east-coast-vegetation-authority-closure-001/`; and
- `../20260809-native-forest-ecohydrology-authority-reframe-001/`.

Before executing this package, verify that the precursor provides a complete
function/state inventory, 71-field/32-profile parameter matrix,
code-to-literature concordance matrix, deviation and authority-gap registers,
canonical authority/gap amendments, successor amendment report, accepted dual
review disposition, and passing terminal verification. Any required
`BLOCK_SUCCESSOR` row keeps this package held until its named authority need
is closed or this package is prospectively narrowed with severability evidence.
The authority-closure predecessor ended `executed-hold`; its premise that
openWEPP must select site values and construct an observed universal pine/oak
state is superseded by the authority-reframe predecessor. Site values and a
complete compatible state are caller configuration, but the schema and every
implemented constitutive family still require complete A0 authority. This
package remains held until those process families are admitted and its own
contract-first gate passes. `AUTH-RHEC-016` is already admitted authority; its
implementation and test vectors remain successor work.

## Precursor Audit Result (Prospective Amendment)

The precursor completed the population audit but did not release production
implementation. Its accepted boundary is licensed source/format provenance,
explicit multistratum identity, canonical Stage A/B/C ownership, and a strict
typed compatibility requirement. No constitutive process or profile parameter
set is admitted merely from source behavior.

The following required blocker families must be closed before Phase 2 or any
production Rust: `AUTH-RHEC-001` complete strict schema/key aliases and
required-presence replacement of 53 hidden defaults; `AUTH-RHEC-002` typed
caller-value ingestion and stratum identity without a default/transferability
claim; `AUTH-RHEC-003`
mixed/top-down radiation; `AUTH-RHEC-004` liquid interception cadence and
release taxonomy; `AUTH-RHEC-005/006` aerodynamic and stomatal/canopy
conductance; `AUTH-RHEC-007` independently owned native-forest canopy,
wet-canopy, and forest-floor flux equations and the explicit rejection of the
agricultural PMET partition;
`AUTH-RHEC-008` C3 photosynthesis and capacity/iteration authority;
`AUTH-RHEC-009` selected phenology; `AUTH-RHEC-010` layer root demand and
hydrologic arbitration; and the respiration portion of `AUTH-RHEC-011`.
Required blockers also included `AUTH-RHEC-014` available-energy ownership and
closure, `AUTH-RHEC-015` complete caller-state ingestion, and `AUTH-RHEC-016`
digest-bound local definition acquisition. The later authority-admission
package admitted `AUTH-RHEC-016`; the authority reframe corrected value/state
roles and the native ET target without admitting incomplete process equations.
`AUTH-RHEC-012` remains a deferred canopy-snow
boundary; `AUTH-RHEC-013` admits licensing/provenance only.

The audit rejects the source's hidden defaults, five incompatible GIS/parser
keys, `livewood_cn` overwrite, `9999.0`/`-999.9` sentinels, nonzero conductance
floor, unsupported Tmin/CO2 laws, omitted Tavg multiplier, forced-C3 branch,
fixed ten-pass respiration feedback, PM psychrometric-constant omission,
dead/ignored optical parameters, non-closing optical triples, unbounded LAI
iteration, dimensionally inconsistent heat storage, longwave/energy clamps,
mutable `master` fetch, unsupported initial-state synthesis, direct soil-store
mutation, and warn/clamp/exit failure posture. Do not recreate these behaviors
for compatibility.

Generic `evergreen`, generic `deciduous`, `eastern.white.pine`, and
`chestnut.oak.bgc` remain format fixtures and possible caller inputs. Their
cells do not block schema implementation merely for lack of site-transfer
authority, but none may be distributed as a recommended default or used for a
calibration/validation claim. Demonstration values must be labeled
`ASSUMED_FOR_EXECUTION`, deliberately distinct, finite, domain-valid, and
behavioral rather than site-suitability evidence.

## Required Coupling Interpretation

RHESSysEastCoast uses Jarvis-style conductance multipliers for absorbed PAR,
leaf water potential, CO2, minimum temperature, and vapor-pressure deficit,
scaled by maximum conductance and canopy state. Conductance participates in a
source Penman-Monteith water-flux path and the Farquhar photosynthesis path.
The source PM routine is rejected as scientific authority, and the native path
does not preserve the agricultural WEPP PMET partition. A correct PM equation
may be admitted inside a separately owned component, but PM is neither required
nor broadly prohibited. The audited source is a coupled
water/conductance/carbon implementation, but it is not automatically a
Bonan-style bidirectional assimilation-solved stomatal formulation.

The accepted precursor inventory must record exact dependency direction, state
cadence, iteration, and mutation. Documentation and contracts must not use
“coupled” as a stronger claim than the audited source and tests support.

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
facts, not a claim that all rows are complete or validated. The precursor
parameter audit must disposition each field and each candidate first-slice
profile before this package begins.

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

- Consume and preserve the precursor's accepted source/function/state/call,
  field/profile, code/literature concordance, deviation, authority-gap, and MIT
  notice/provenance records.
- Close only implementation-phase authority questions newly exposed by the
  exact selected API/test design; any material expansion requires prospective
  package amendment and the same audit standard.
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
- No automatic reallocation of reduced canopy demand to forest-floor
  evaporation and no native-forest use of the agricultural `Kcb`/LAI PMET
  partition.
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
- Site-specific parameter values and complete compatible initial state are
  caller-supplied; openWEPP proves schema, domains, state separation, behavioral
  response, and closure rather than selecting values for a user's site.

## Source-Aware Admission Rubric

The precursor applies this rubric to every candidate surface. This package
must preserve the accepted result and may not reopen it without new evidence:

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

1. Verify the precursor's terminal identity, reviews, concordance/gap closure,
   successor amendment report, and absence of required `BLOCK_SUCCESSOR` rows.
   Freeze pre-implementation intent, exact base/diff, selected boundary,
   instruction map, source commits, required-reading budget, and owned write set.
2. Close every required schema, constitutive, ownership, and guard blocker
   contract-first in an authorized authority-admission amendment. Treat site
   values/state as caller configuration and reconcile the admitted process
   boundary with the concrete Rust API and fixture design;
   prospectively amend this package before implementation if new dependencies
   appear.
3. Amend canonical contracts as still required; add contract-derived tests,
   locked fixtures, and the required A3 suite; complete the pre-implementation
   contract gate.
4. Implement the strict loader and default-off Rust slice with typed errors and
   explicit source/provenance identities.
5. Add deciduous, evergreen, mixed-stratum, limiting, negative, property,
   conservation, independent-reconstruction, and source-differential tests.
6. Complete calibration-readiness analysis and classify all parameter claims.
7. Reconcile the exact diff and run every applicable focused, source-quality,
   A0/A1/A3, anti-evasion, dependency, security/license, quick, and Critical
   full-workspace requirement.
8. Complete independent science/source and Rust reviews, disposition findings,
   rerun invalidated gates, then perform independent terminal verification.
9. Archive the kickoff prompt, update lifecycle records, disposition the
   package, and commit/push the stable increment when authorized.

## Contract-First Hard Gate

No production Rust may be authored until the precursor dependency is complete,
no required `BLOCK_SUCCESSOR` row remains, and canonical contract text,
contract-derived tests, source/profile provenance fixtures, and the required A3
suite bind every selected equation, parameter class/schema, unit, guard, and compatibility
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

The execution package must refine exact test names from the accepted precursor
and concrete API before production edits. Minimum expected terminal commands
include:

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

Before the pre-implementation gate, contract-derived fixtures must also prove
exact recognition or explicit aliasing of all 71 raw keys; typed failure for
every selected missing/duplicate/non-finite/domain-invalid/sentinel value;
negative proof that none of the 53 source defaults can enter implicitly;
explicit preservation of all 32 raw profile identities; non-averaged mixed
composition; exact rejection of the nine non-closing optical triples unless
independently replaced; digest-bound local inputs with no network fallback;
distinct leaf/canopy/surface conductance scales; PM gamma reconstruction with
`EPS`; canopy/ground energy and storage-heat component closure; dated initial
C/N/LAI mass reconstruction; bounded LAI iteration failure; a C3-only guard
for the selected photosynthesis route; and layer-demand/hydrology dual
reconstruction. These are prospective obligations, not evidence that the
blocked gate has passed.

Critical full correctness, contract, external-authority, and exact-diff gates
are non-deferrable once implementation begins. Coverage/CRAP remains optional
observational QA for this non-CQR package.

## Exit Criteria

- Both source identities and MIT notices are preserved and auditable.
- The precursor has a passing terminal disposition and the selected
  source/profile/concordance inventory has no unresolved or required
  `BLOCK_SUCCESSOR` row.
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

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only science/source reviewers, one
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
- [x] (2026-08-08) User moved the code-to-literature authority audit into
  `../20260808-rhessys-east-coast-code-literature-authority-audit-001/` as a
  hard precursor.
- [x] (2026-08-08) Precursor completed concordance, gap classification,
  successor amendment, dual review, validation, and terminal verification.
- [x] (2026-08-09) Consumed the terminal disposition from
  `../20260809-rhessys-east-coast-vegetation-authority-closure-001/`; that
  package executed through hold and did not release this successor.
- [x] (2026-08-09) User reframed site values/state as caller configuration and
  prohibited agricultural PMET redistribution in the native-forest target;
  `../20260809-native-forest-ecohydrology-authority-reframe-001/` amended the
  canonical boundary.
- [ ] Admit the complete schema and constitutive families in
  `AUTH-RHEC-001/003..011/014`; implement caller-value/state ingestion for
  `AUTH-RHEC-002/015` and the admitted `AUTH-RHEC-016` acquisition boundary
  only after the contract-first gate passes.
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
- Decision: Move the complete code-to-literature audit into a separately
  dispositioned hard precursor.
  Rationale: the implementation package should begin from reviewed concordance
  and authority-gap evidence rather than encounter missing or contradictory
  science after production edits begin.
  Date/Author: 2026-08-08 / user and Codex.
- Decision: Treat site-specific stratum values and complete compatible initial
  state as caller configuration; demonstrate typed behavior with
  `ASSUMED_FOR_EXECUTION` fixtures instead of selecting universal values.
  Rationale: site suitability belongs to users unless openWEPP makes a default,
  calibration, validation, or transferability claim.
  Date/Author: 2026-08-09 / user and Codex.
- Decision: Reject the agricultural `Kcb`/LAI PMET partition as the native
  forest target and require independent canopy, wet-canopy, forest-floor, and
  root-layer components. Penman-Monteith may be admitted within a component but
  is neither mandated nor broadly prohibited.
  Rationale: the Stevens Canyon investigation identified structural demand
  redistribution, while both PMET calibration and legacy-ET ablation failed.
  Date/Author: 2026-08-09 / user and Codex.

## Outcomes And Retrospective

The precursor audit is complete and amended this package, but did not release
production. `SC-VEGETATION-001` version 4 now records licensed provenance,
strict schema identity, caller-owned site values/state, independent native
forest flux obligations, admitted local acquisition authority, and explicit
constitutive gaps. No constitutive law or profile set was admitted. No Rust source, fixture,
external-authority suite, production consumer, or runtime default changed. The
next action is complete schema and constitutive admission, followed by this
package's contract-first gate. Site-value selection is not a blocker. It must
then implement caller configuration/state, independent native-forest component
fluxes, and the already-admitted `AUTH-RHEC-016` boundary.

## Idempotence And Recovery

Consume the precursor's pinned read-only audit without modifying either external
checkout. If its accepted authority boundary cannot support implementation,
keep this package held or close `executed-hold` without production Rust.
Never reset unrelated user work.

Revision note (2026-08-08): initial licensed-source successor scaffolded from
the user's deciduous/mixed-forest and vegetation-file compatibility direction.

Revision note (2026-08-08): the distinct code-to-literature authority-audit
precursor completed and amended this package. The later authority-admission
package admitted `AUTH-RHEC-016`; implementation remains held until residual
`AUTH-RHEC-001..011` plus `AUTH-RHEC-014/015` authority admission and the
contract-first gate pass.

Revision note (2026-08-08): the integrated authority-admission predecessor was
added as a second hard dependency. Its selected profile/initializer Gate 1 must
pass before this package can execute.

Revision note (2026-08-08): that authority-admission predecessor closed
`executed-hold`. It admitted strict acquisition/schema form only; residual
selected manifest/value/state and Gate 2-3 constitutive gaps keep this package
held pending a newly authorized authority-closure package.

Revision note (2026-08-09): the authority-closure package was scaffolded as a
hard predecessor. This implementation package remains non-executable until a
closure effort admits every residual authority family and explicitly releases
the selected boundary.

Revision note (2026-08-09): the authority-closure package executed through all
lanes and closed held because exact selected values/state and complete
constitutive authority remain missing. This successor remains non-executable.

Revision note (2026-08-09): the authority reframe superseded site-value/state
selection as a release condition. Users supply those surfaces. Complete schema
and constitutive authority, independent native-forest component fluxes, and the
successor's contract-first gate remain required before production edits.
