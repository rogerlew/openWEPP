# RHESSys East Coast Code-To-Literature Authority Audit

Status: `complete / audit passed / successor blocked on authority`

Date: `2026-08-08`

Package ID: `20260808-rhessys-east-coast-code-literature-authority-audit-001`

Plan class: `High scientific-authority audit and successor admission`

## Objective

Audit the vegetation-relevant RHESSysEastCoast and GIS2RHESSys implementations
against their cited primary literature, physical/conservation invariants, and
openWEPP authority boundaries before implementation begins. Identify and, where
admissible evidence is available, close scientific-authority gaps; record every
remaining gap explicitly; and amend the successor
`20260808-rhessys-east-coast-coupled-vegetation-slice-001` so it begins with a
verified, bounded implementation target rather than discovering authority gaps
during Rust development.

## User Direction And Big Picture

The user directed this audit to run as a distinct precursor to the coupled
vegetation implementation. The purpose is prevention: compare code to the
actual literature now, identify silent deviations and unsupported parameters,
obtain missing sources deliberately, and carry exact dispositions into the
successor package.

This package is not a diagnostic relay. It owns the complete vegetation
authority audit, literature intake that can be completed in scope, bounded
canonical authority amendments supported by the audit, and the successor-plan
amendments needed to make every discovered gap visible before implementation.

## Pinned Sources

| Role | Repository | Local checkout | Commit | License |
|---|---|---|---|---|
| Coupled vegetation implementation | `laurencelin/RHESSysEastCoast` | `/workdir/RHESSysEastCoast` | `375c75b1cd2202217651dff43aa113d80b9c1118` | MIT; `LICENSE` SHA-256 `4fd4ecf2fd01cf53c99754bcac5a6dbee255a0be0539dd84ffe12e06808374be` |
| Profile data and definition generation | `laurencelin/GIS2RHESSys` | `/workdir/GIS2RHESSys` | `6b20883dea7c9fd92f71ec69eaca015ebf6dfe18` | MIT; same license digest |

The external checkouts are read-only. Any upstream movement, local source edit,
or alternate repository requires prospective amendment and a new identity
record. The separately licensed-ambiguous official RHESSys repository is not a
direct-translation source for this package.

## Audit Intent And Claim Boundary

Intent is `scientific-authority audit + authority-gap closure + successor
planning`. This package performs no Rust implementation, empirical calibration,
independent validation, runtime activation, or production cutover.

The audit may conclude successfully while some implementation surfaces remain
`AUTHORITY_MISSING`, provided every gap is explicit, evidence-backed, assigned
to a successor gate or excluded/deferred from the successor, and no package
artifact claims the missing behavior is authorized. An unidentified or
undispositioned audited surface prevents completion.

## Audit Universe

Audit the complete vegetation-relevant transitive closure needed to understand
deciduous, evergreen, and mixed multistratum behavior, including:

- stratum-default parsing, profile generation, and all 71 parameter fields and
  32 profile columns in pinned `vegCollection.csv`;
- canopy-stratum construction, cover/vertical ordering, LAI and canopy state;
- shortwave/PAR and longwave receipt, absorption, reflection, transmission, and
  sunlit/shaded or layer scaling where present;
- rain interception, storage capacity, throughfall, drainage, stemflow, and
  wet-canopy evaporation/depletion;
- aerodynamic transfer and Penman-Monteith resistance/energy conversion;
- vascular and nonvascular conductance response curves, aggregation, state
  dependence, numerical floors, and source call sites;
- Farquhar photosynthesis and its actual dependency direction with conductance;
- evergreen/deciduous phenology, leaf-on/leaf-off transfer, turnover, and
  allocation dependencies needed by canopy state;
- root distribution, soil/leaf-water-potential observations, layer demand,
  realized uptake feedback, and frozen/dry limiting behavior;
- relevant respiration, carbon/nitrogen, and litter transfers only far enough
  to identify whether the first coupled slice can stop without severing an
  authoritative state dependency; and
- corresponding openWEPP vegetation, phenology, evapotranspiration, management,
  residue, land-surface-energy, snow/frost, and water-balance ownership surfaces.

Generic allocation, I/O, and mathematical utilities are inventoried only when
they alter scientific meaning, units, state chronology, defaults, or errors.
Unrelated routing, erosion, channel, and watershed processes are excluded.

## Evidence Classes

Every audit statement must carry one evidence class:

| Class | Meaning |
|---|---|
| `LITERATURE_DIRECT` | Verified against an acquired primary source at an equation, table, section, or stable locator. |
| `LITERATURE_DERIVED` | Independently derived from cited primary equations with the derivation recorded. |
| `PHYSICAL_INVARIANT` | Supported by explicit dimensional, conservation, monotonicity, or limiting-case reasoning. |
| `CODE_OBSERVED` | Present in pinned source but not yet supported as scientific authority. |
| `PARAMETER_DATA` | Present in a pinned profile/table; provenance and calibration status remain separate. |
| `OPENWEPP_AUTHORITY` | Already governed by a named canonical contract/invariant. |
| `INFERENCE` | Analyst interpretation that requires confirmation and cannot alone authorize production physics. |

A citation in a comment or CSV header is a discovery lead, not
`LITERATURE_DIRECT`. The primary source must be read.

## Code-To-Literature Concordance

For every scientific function, branch, and parameter family, record:

- exact repository, commit, file, symbol or CSV row/column;
- callers, callees, state inputs/outputs, mutation, cadence, and dependency
  direction;
- source comments/citations and the acquired primary-source locator;
- code expression and literature expression in independently structured
  mathematical notation, with variable/alias mapping;
- units, area basis, rate/interval basis, spatial and temporal scale;
- validity domain, calibration domain, parameter origin, and transfer limits;
- default, sentinel, clamp/floor, branch order, convergence, and failure
  behavior;
- deciduous, evergreen, mixed-stratum, and zero/boundary applicability;
- openWEPP owner and affected canonical contract; and
- concordance and migration disposition.

Concordance values are:

- `MATCH` — code faithfully expresses the cited source within stated units and
  domain;
- `DOCUMENTED_ADAPTATION` — deviation is explicit and scientifically supported;
- `SILENT_DEVIATION` — source differs without adequate recorded authority;
- `PARTIAL_IMPLEMENTATION` — required terms, branches, or state coupling are
  absent or disabled;
- `CODE_ONLY` — behavior lacks admissible external or canonical authority;
- `CITATION_MISMATCH` — the named source does not support the implemented claim;
- `CONTRADICTORY_AUTHORITIES` — admissible sources disagree materially;
- `NOT_APPLICABLE` — nonscientific support code with rationale; or
- `NOT_YET_VERIFIED` — temporary execution state that blocks terminal closure.

## Migration Dispositions

Each audited surface receives exactly one terminal disposition:

- `ADOPT` — source behavior and authority are adequate as written;
- `ADAPT` — scientific basis is adequate but units, API, numerics, defaults, or
  ownership must change for openWEPP;
- `RE_DERIVE` — intended process is supported, but the implementation does not
  faithfully express the authority;
- `DEFER` — not needed by the selected first slice and safely severable with an
  explicit later owner/trigger;
- `REJECT` — unsupported, contradictory, defective, or incompatible behavior;
  or
- `BLOCK_SUCCESSOR` — required by the first slice but missing or contradictory
  authority remains after in-scope acquisition and adjudication.

`DEFER` is allowed only when call/state analysis proves the first slice does not
depend on the behavior. Package size or inconvenience is not severability
evidence.

## Authority-Gap Closure Routes

For each gap, attempt the applicable route before terminal disposition:

1. Read already-vendored or local copyrighted primary sources.
2. Retrieve an openly accessible primary source or authoritative report.
3. Follow citations to the actual equation/parameter source.
4. Independently derive the required relation from admitted primary equations
   or physical/conservation invariants when that route is valid and reviewable.
5. Reconcile with existing canonical openWEPP authority.
6. Reject or defer the source behavior when it is not required.
7. Mark `BLOCK_SUCCESSOR` only when the behavior is required and no admissible
   authority route closes it.

Create a precise operator acquisition request only for inaccessible material
that blocks a required row. Name the full citation, DOI or stable locator,
exact equation/parameter need, source code coordinates, and consequence if it
cannot be obtained. Do not ask the user to collect a broad reading list.

## Literature And Rights Handling

- Add metadata, quality, kernel role, and rights disposition to
  `references/annotated_bibliography.md`.
- Place affirmatively redistributable sources in `references/vendorable/` with
  license/provenance; record them in the rights ledger.
- Place copyrighted or rights-ambiguous full text and transcriptions only in
  gitignored `references/copyrighted/`; track metadata and checksums, not the
  files.
- Preserve MIT notices with any bounded source-derived fixture or extract.
- Do not infer redistribution rights from free access, repository visibility,
  citation, institutional hosting, or an “open source” description.

## Included Scope

- Read-only inspection of the pinned source repositories and relevant openWEPP
  code/contracts/tests.
- Complete source/function/state/call, parameter/profile, and
  code-to-literature concordance matrices for the audit universe.
- Targeted literature discovery, acquisition, rights classification, and
  primary-source verification.
- Authority-gap and implementation-deviation registers.
- Independent equation/unit/domain reconstruction sufficient to adjudicate
  source fidelity; no production implementation.
- Bounded `SC-VEGETATION-001` and adjacent-contract amendments that record
  audit-backed authority, explicit gaps, ownership, aliases, or exclusions
  without activating runtime behavior.
- Prospective amendments to the coupled successor's objective, dependency,
  selected slice, write set, contracts, required tests/suites, acceptance
  criteria, reading map, and kickoff prompt.
- Dual independent science/source reviews, finding disposition, and dual
  terminal verification of the final audit and successor amendments.

## Excluded Scope And Claim Limits

- No production Rust, Cargo manifest, runtime, parser, fixture-consumer, output,
  schema, default, deployment, or release change.
- No empirical calibration, independent validation, parameter transferability,
  production readiness, parity, or cutover claim.
- No source comparator run represented as scientific verification.
- No automatic adoption of code defaults, sentinels, clamps, floors, disabled
  terms, comments, CSV values, or citations.
- No contract amendment that silently converts `CODE_OBSERVED` or
  `PARAMETER_DATA` into constitutive authority.
- No implementation package execution. This precursor may amend its plan but
  cannot begin its Rust phases.

## Dependencies And Retained Authority

- Pinned licensed sources and rights records from R-146/R-147.
- `SC-VEGETATION-001` Stage A/B/C ownership and its adjacent-contract map.
- The independent literature dossier and rights intake from
  `../20260808-vegetation-radiation-interception-conductance-slice-001/`.
- The held successor
  `../20260808-rhessys-east-coast-coupled-vegetation-slice-001/`.
- ADR-0011 architecture-first authority, ADR-0017 comparator distrust, ADR-0042
  science/calibration readiness, and ADR-0043 direct validation.
- Canonical science-contract, unit, correctness-authority, reference-vendoring,
  and testing standards.

## Intended Write Set

- This package tree.
- Lifecycle edits to `docs/ROADMAP.md`, `docs/work-packages/README.md`,
  `docs/backlog/TRACKER.md`, and
  `docs/backlog/20260806-rhessys-derived-vegetation-crate.md`.
- Prospective planning edits throughout
  `docs/work-packages/20260808-rhessys-east-coast-coupled-vegetation-slice-001/`.
- `references/annotated_bibliography.md`,
  `references/rights_classification_first_pass_2026-05-11.md`, bounded new files
  under `references/vendorable/`, and local-only ignored files under
  `references/copyrighted/`.
- `SC-VEGETATION-001`, its lifecycle/index records, and only minimum
  audit-backed authority/gap/ownership amendments to `SC-EVAP-001`,
  `SC-LANDSURFACEENERGY-001`, `SC-WATBAL-001`, `SC-PLANT-001`,
  `SC-RESIDUE-001`, or `SC-SNOWFREEZE-001`.
- Contract-cycle evidence under this package for every amended `SC-*` file.

Production Rust, Cargo files, test sources, external-authority suite registries,
observed datasets, and existing runtime/consumer files are outside the write
set. Amendments cannot widen this precursor into implementation.

## Deliverables

1. Exact audit-scope and source/license manifest.
2. Source/function/state/call inventory with transitive coupling map.
3. Complete 71-field/32-profile parameter authority matrix.
4. Code-to-literature concordance matrix with primary locators.
5. Code/literature deviation and authority-gap registers.
6. Rights-compliant literature intake and a narrow operator acquisition queue,
   if inaccessible sources remain blocking.
7. Audit-backed canonical authority/gap amendments where warranted.
8. Successor amendment report and prospectively revised successor package.
9. Dual independent reviews, finding disposition, validation evidence, dual
   terminal verification, disposition, and worker handoff.

## Phase Plan

1. Freeze pre-audit intent, exact openWEPP/source identities, dirty state,
   instruction map, audit universe, reading budget, and owned write set.
2. Build the transitive source/function/state/call inventory and exact citation
   discovery ledger without making fidelity judgments prematurely.
3. Audit all vegetation fields/profiles, implicit defaults, sentinels, units,
   citations, and source-generator/parser behavior.
4. Acquire and read primary sources under the rights policy; complete equation,
   parameter, domain, and scale concordance. Maintain a precise missing-source
   queue as gaps are discovered.
5. Classify deviations and gaps; exhaust applicable authority-admission routes;
   amend canonical contracts only where evidence supports binding authority or
   an explicit authority gap/ownership statement.
6. Select the coherent first implementation boundary. Amend the successor so
   every included dependency is admitted or explicitly blocking, and every safe
   deferral has severability evidence, owner, trigger, and later boundary.
7. Complete two independent science/source reviews, disposition every finding,
   fix accepted findings, and verify their closure.
8. Reconcile the exact diff and run all applicable documentation, contract,
   unit-governance, rights/license, source-identity, and focused contract tests.
   Escalate validation if the exact contract diff changes active authority.
9. Complete two independent terminal verifications on final bytes, archive the
   kickoff prompt, update lifecycle records, write disposition/handoff, and
   commit/push the stable increment when authorized.

## Validation Plan

The audit is documentation/reference/contract-only. The executor must select
additional contract tests from the exact diff and record argv, working
directory, identity, and result. Minimum commands are:

    markdown-doc lint --path docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001 --format plain
    markdown-doc lint --path docs/work-packages/20260808-rhessys-east-coast-coupled-vegetation-slice-001 --format plain
    markdown-doc lint --path docs/backlog/20260806-rhessys-derived-vegetation-crate.md --format plain
    markdown-doc lint --path references/annotated_bibliography.md --format plain
    markdown-doc lint --path references/rights_classification_first_pass_2026-05-11.md --format plain
    bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md
    git diff --check

If canonical binding authority changes, run its contract admission/schema tests,
affected contract-derived tests, and every conservatively selected requirement
under `docs/standards/testing-and-gate-strategy.md`. No heavy workspace run is
selected by the scaffold alone. Coverage/CRAP is not applicable.

## Exit Criteria

- Every in-universe source function, scientific branch, parameter field, and
  profile has an exact inventory identity, evidence class, concordance result,
  and migration disposition; no `NOT_YET_VERIFIED` row remains.
- Every source citation used for an authority claim was read at a primary
  equation/table/section locator; citation-only leads remain explicitly gaps.
- Units, scales, cadence, area/interval basis, dependency direction, mutation,
  defaults, sentinels, floors, disabled branches, and failure behavior are
  reconciled for every first-slice dependency.
- All authority gaps are closed, rejected/deferred with severability evidence,
  or marked `BLOCK_SUCCESSOR` with an exact acquisition/decision need.
- Rights classification and repository placement follow open-versus-copyrighted
  conventions; tracked files contain no restricted full text.
- Canonical contract changes contain only audit-supported authority or explicit
  gap/ownership text and pass all exact-diff requirements.
- The coupled successor names this package as a hard dependency and has been
  amended to the audited boundary, gaps, tests, write set, and claim limits.
- Dual independent reviews and finding disposition leave no unresolved finding;
  dual terminal verifications pass on final bytes.
- Terminal diff matches the intended write set, kickoff prompt is archived
  byte-for-byte, lifecycle records are truthful, and disposition makes no
  implementation, calibration, validation, runtime, or cutover claim.

## Review And Delegation Requirements

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only science/source reviewers and
two independent read-only terminal verifiers. Expected outputs are compact
severity-ranked findings and verdicts in the named package artifacts; write
access is read-only except that the primary executor owns the bounded
package/reference/contract write set.

Subagent requirement: REQUIRED for dual science/source review and dual terminal
verification when session-level user/policy authorization permits delegation.
No heavy comparator runner is selected by the scaffold. Preserve independent
initial reviews. If delegation is unavailable, record the policy block and do
not claim the required gate or close the package.

## Security, Licensing, And Data Impact

Work is local flat-file audit and read-only source/literature retrieval. Do not
modify external checkouts, deploy, publish, send external messages, or ingest
secrets. Record source URLs and rights; never commit copyrighted or
rights-ambiguous full text. Validate all vendored-source licenses and checksums.
No observed dataset or personal data is authorized.

## Progress

- [x] (2026-08-08) User directed a code-to-literature authority audit as a
  separate precursor to the coupled vegetation implementation package.
- [x] (2026-08-08) Scaffolded package scope, artifacts, active prompt, and
  lifecycle dependency edits.
- [x] (2026-08-08) Froze the exact audit identity and inventory universe.
- [x] (2026-08-08) Completed source/citation, 71-field, 32-profile, and
  53-parser-only-default inventories.
- [x] (2026-08-08) Completed concordance/deviation review and made every
  authority gap explicit.
- [x] (2026-08-08) Amended canonical authority and the successor without
  admitting source physics or parameter values.
- [x] (2026-08-08) Completed dual review, direct validation, dual terminal
  verification, disposition, and handoff.

## Decision Log

- Decision: Separate authority audit from implementation.
  Rationale: implementation should consume an explicit concordance/gap record,
  not discover unsupported science while production Rust is underway.
  Date/Author: 2026-08-08 / user and Codex.
- Decision: Audit the vegetation-relevant transitive closure rather than only
  named headline functions.
  Rationale: state, cadence, parser defaults, and indirect dependencies can make
  an apparently correct local equation scientifically incompatible.
  Date/Author: 2026-08-08 / Codex.
- Decision: Permit bounded contract and successor amendments but prohibit Rust.
  Rationale: authority gaps should be filled or made bindingly visible here;
  implementation remains the successor's responsibility.
  Date/Author: 2026-08-08 / Codex.

## Outcomes And Retrospective

The docs-only audit completed. It found 31 grouped transitive source surfaces,
35 concordance rows, 30 source/literature deviations, and 16 authority gaps.
Key defects include five key mismatches, 53 hidden defaults, mutable `master`
fallbacks, dead/ignored optics and nine non-closing profiles, defective PM
gamma, dimensionally inconsistent heat storage, longwave/energy clamps,
unbounded LAI iteration, unsupported initialization, and incomplete persistent
carbon/root coupling.

`SC-VEGETATION-001` version 2 admits only the exact licensed-provenance and
strict-schema boundary while recording explicit non-promotable gaps. The
coupled implementation successor remains blocked on `AUTH-RHEC-001..011` and
`AUTH-RHEC-014..016`. No Rust, runtime, calibration, validation, parity,
activation, publication, or cutover work occurred.

## Idempotence And Recovery

External source inspection is read-only and pinned. Literature intake is
additive and rights-classified. If an authority route fails, retain the evidence
and mark the successor blocker; do not invent or silently transfer authority.
Never reset unrelated user work.

Revision note (2026-08-08): initial precursor scaffolded from the user's request
to verify implementations against literature before coupled Rust development.
