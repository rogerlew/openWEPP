# RHESSys East Coast Vegetation Authority Admission

Status: `executed-hold / terminal verified`

Date: `2026-08-08`

Package ID: `20260808-rhessys-east-coast-vegetation-authority-admission-001`

Plan class: `Critical contract-first constitutive and parameter authority`

This ExecPlan is a living document maintained under `docs/codex_exec_plans.md`.

## Objective

Attempt to close the required `AUTH-RHEC-001..011` and `AUTH-RHEC-014..016`
gaps that hold the default-off RHESSys East Coast coupled vegetation successor.
Admit only primary-source-backed equations, parameters, domains, ownership,
units, guards, and independent vectors into canonical contracts. If an
authority route cannot close, retain a precise reviewed `executed-hold`; do not
invent physics, profile values, initialization rules, or empirical bounds.

## User Outcome

The user requested this package after the predecessor audit established that
licensed source code and profile tables were insufficient scientific authority.
The observable outcome is either a reviewed contract-first release of the
implementation successor or an exact evidence-backed hold identifying the
minimum unavailable authority. Production Rust, activation, calibration,
publication, and cutover are not outcomes of this package.

## Frozen First Boundary

- Candidate evergreen profile: GIS2RHESSys `eastern.white.pine`, ID `807`.
- Candidate deciduous profile: GIS2RHESSys `chestnut.oak.bgc`, ID `805`.
- Mixed forest: explicit two-stratum composition; never averaged values.
- Vascular C3 vegetation only; C4, nonvascular, and canopy snow reject.
- Existing openWEPP generalized GSI remains phenology owner unless independently
  superseded.
- Nitrogen competition and broad material cycling remain excluded behind an
  immutable-N/no-material boundary.
- Input definitions are explicit local bytes bound to repository, commit, path,
  and SHA-256; no network fallback.
- Hydrology alone mutates soil stores. Land-surface energy owns available-energy
  operands and latent-energy conversion.

The selected species rows are candidates, not admitted values. The first gate
must disposition every value required to initialize or execute them.

## Predecessors And Authority

- Completed code/literature audit:
  `../20260808-rhessys-east-coast-code-literature-authority-audit-001/`.
- Held implementation successor:
  `../20260808-rhessys-east-coast-coupled-vegetation-slice-001/`.
- Canonical boundary contract:
  `../../specifications/science-contracts/contracts/SC-VEGETATION-001.md`.
- Pinned RHESSysEastCoast commit:
  `375c75b1cd2202217651dff43aa113d80b9c1118`.
- Pinned GIS2RHESSys commit:
  `6b20883dea7c9fd92f71ec69eaca015ebf6dfe18`.
- Both pinned repositories carry the same MIT license SHA-256:
  `4fd4ecf2fd01cf53c99754bcac5a6dbee255a0be0539dd84ffe12e06808374be`.

Science authority is resolved from canonical contracts, primary literature,
physical conservation/dimensional invariants, and static source only as
secondary provenance. Comparator agreement and source defaults are not truth.

## Implementation Intent And Risk

Intent is `science-authority admission + parameter/data authority assessment`.
`science_implementation_status=NOT_IMPLEMENTED`,
`calibration_evidence_status=NOT_CALIBRATION_READY`, and
`identifiability_status=NOT_ASSESSED`. Risk is `Critical` because admitted text
would govern future coupled water, energy, and carbon behavior. No measured
data are assigned to calibration or independent validation.

## Included Scope

- Exact schema, alias, local-byte provenance, selected-profile, and initializer
  authority for `AUTH-RHEC-001/002/015/016`.
- Radiation, interception, aerodynamic/stomatal conductance, Penman-Monteith,
  and available-energy authority for `AUTH-RHEC-003..007/014`.
- C3 photosynthesis, phenology/LAI, layer root demand, respiration, allocation,
  and minimum persistent carbon-state authority for `AUTH-RHEC-008..011`.
- Rights-compliant primary-source reading and precise unavailable-source needs.
- Canonical contract and contract-derived test amendments only when a complete
  authority family closes.
- Prospective amendment of the held implementation successor.
- Dual independent science review, finding disposition, direct validation, and
  dual independent terminal verification.

## Excluded Scope And Claim Limits

- No production Rust, Cargo, runtime selector, management schema, output,
  external-authority suite admission, default activation, deployment, release,
  calibration, independent validation, or cutover.
- No canopy-snow, C4, nonvascular, nutrient-competition, soil-biogeochemistry,
  mortality/disturbance, or broad material-cycle law.
- No silent use of 53 parser defaults, five mismatched-key behaviors, sentinels,
  floors, clamps, nonfinite-to-zero rules, mutable network input, or defective
  source equations.
- No profile-table value promoted merely because it is licensed, finite, or
  cited by an unscoped header bundle.
- No package-local artifact may replace canonical `SC-*` authority.

## Authority Gates

### Gate 1: Schema, Profiles, And Initial State

Close `AUTH-RHEC-001/002/015/016`. Require an exact selected-field ledger,
field-to-primary-locator and ecosystem-domain evidence, explicit alias/rejection
decisions, no hidden defaults, local digest-bound bytes, and independently
reconstructible LAI/C/N/root initial state. This gate precedes any selected
profile or constitutive promotion.

### Gate 2: Canopy Water And Energy

Close `AUTH-RHEC-003..007/014` as one operand chain. Require distinct direct and
diffuse radiation, optical closure, multistratum ordering, interception mass
ledger, aerodynamic regime, leaf/canopy conductance scale, correct
Penman-Monteith gamma, and exact available-energy ownership. Every plausible
wrong operand pairing must be numerically distinct in locked vectors.

### Gate 3: Carbon, Phenology, And Roots

Close `AUTH-RHEC-008..011`. Require a complete C3 route, bounded convergence,
selected photosynthetic/respiration parameters, nonduplicative phenology
ownership, layer root requests through Stage A/B/C, and the minimum persistent
carbon transitions needed for future LAI/root state.

Each gate is current-scope. A failed gate cannot be relabeled as future work
while this package is complete. A proved external authority boundary yields
`executed-hold`; implementation effort or package size does not.

## Contract-First Sequence

1. Freeze intent, identities, scope, roles, and reading budget.
2. Execute all applicable authority routes for Gate 1.
3. Execute Gates 2 and 3 only if their prerequisites are scientifically
   evaluable without inventing selected parameters/state.
4. Amend canonical contracts for each fully closed family.
5. Add contract-derived tests and locked vectors.
6. Record the pre-implementation contract gate.
7. Amend the implementation successor only to the proven boundary.
8. Complete dual review, disposition, direct validation, dual verification,
   prompt archival, lifecycle, and handoff.

Production code is outside this sequence.

## Intended Write Set

- This package tree.
- `docs/work-packages/README.md`, `docs/ROADMAP.md`,
  `docs/backlog/TRACKER.md`, and the RHESSys vegetation backlog note.
- Prospective dependency/status edits in the coupled vegetation successor.
- `SC-VEGETATION-001`, its index, and only the minimum adjacent contracts whose
  complete authority family passes an internal gate.
- Bounded contract-derived tests/fixtures only after their canonical authority
  is admitted.
- Reference bibliography, rights ledger, and vendorable or ignored restricted
  source custody required by actual acquisition.

No existing runtime/consumer, Rust production, Cargo, observed dataset,
required-suite registry, or deployment path is in scope.

## Validation Plan

Select commands directly from the exact diff. Minimum documentation/contract
commands are:

    markdown-doc lint --path docs/work-packages/20260808-rhessys-east-coast-vegetation-authority-admission-001 --format plain
    markdown-doc lint --path docs/work-packages/20260808-rhessys-east-coast-coupled-vegetation-slice-001 --format plain
    markdown-doc lint --path docs/backlog/20260806-rhessys-derived-vegetation-crate.md --format plain
    markdown-doc lint --path docs/ROADMAP.md --format plain
    markdown-doc lint --path docs/work-packages/README.md --format plain
    git diff --check

If a canonical contract changes, also run its schema/profile/unit checks,
strict Binding Exposure Index check, and every affected contract-derived test.
No Rust gate is applicable solely because the repository contains Rust.

## Review And Delegation Requirements

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only science/source reviewers and
two independent read-only terminal verifiers. Expected outputs are compact,
severity-ranked findings and verdicts recorded in the named package artifacts;
the primary executor owns all writes. Reviewer B and Verifier B must not read
their A counterpart before their initial verdict.

Subagent requirement: REQUIRED for dual science review and dual terminal
verification. No comparator runner or heavy workspace gate is selected unless
the exact diff later triggers one.

Reviews and verifications must check gate legitimacy, source/claim alignment,
rights, exact diff, prompt lifecycle, and whether any `HOLD` boundary was
actually exhausted rather than merely inconvenient.

## Security, Licensing, And Data Impact

External checkouts are read-only. Preserve MIT notices with any distributed
source-derived bytes. Copyrighted or rights-ambiguous full text remains only in
ignored `references/copyrighted/`; tracked records contain metadata/checksums,
not restricted text. No secrets, personal data, deployment, external messages,
or source mutation are authorized.

## Exit Criteria

- Every required authority route has a terminal admission or precise blocker.
- No unscoped citation, source behavior, or profile cell is promoted.
- Every canonical amendment has complete contract-cycle evidence and passing
  applicable direct gates.
- Review findings are individually dispositioned and accepted fixes verified.
- Exact terminal diff, reading budget, lifecycle, prompt archive, rights, and
  source identities reconcile.
- Both terminal verifiers pass the claimed final disposition.
- `complete` is allowed only if all three authority gates pass. Otherwise the
  package closes `executed-hold` and the implementation successor stays held.

## Progress

- [x] (2026-08-08) User authorized scaffold and end-to-end execution.
- [x] (2026-08-08) Froze the first boundary, source identities, intent, and
  authority gates.
- [x] (2026-08-08) Executed Gate 1: acquisition authority passed, schema-form
  authority partially passed, and selected values/state remained blocked.
- [x] (2026-08-08) Executed parameter-independent Gates 2 and 3; every required
  chain retains at least one equation/domain/guard/vector gap.
- [x] (2026-08-08) Amended `SC-VEGETATION-001` to version 3 and added focused
  contract-derived assertions without admitting values or runtime behavior.
- [x] (2026-08-08) Completed dual independent review and accepted every finding.
- [x] (2026-08-08) Completed final validation, dual terminal verification,
  prompt archival,
  and disposition/handoff.

## Surprises And Discoveries

- ORNL/White and the header's primary-source routes recover meaningful partial
  lineage; the earlier “header cannot help” statement was too broad.
- Hwang Table 2/3 shows that the oak profile mixes species observations,
  composition-weighted catchment values, transformations, and apparent
  misassignment rather than forming a species-authoritative row.
- Ford supplies dated pine/oak observations, but from adjacent watersheds and
  without a complete compatible C/N/root state surface.

## Decision Log

- Decision: use one integrated admission package with three internal gates.
  Rationale: the selected parameter/state, water-energy, and carbon-root chains
  are coupled, while the gate structure preserves an exact stop boundary.
  Date/Author: 2026-08-08 / user and Codex.
- Decision: use named East Coast pine and oak candidates rather than average a
  mixed profile.
  Rationale: this directly exercises the requested evergreen, deciduous, and
  mixed identities without inventing a third parameter set.
  Date/Author: 2026-08-08 / Codex.

## Outcomes And Retrospective

The package admits strict local acquisition and schema-form authority only. It
does not admit a complete selected schema, profile value, initializer, or
constitutive family. All three gates were executed; residual value/state and
equation/domain/guard/vector gaps require `executed-hold`. The implementation
successor remains held.

## Idempotence And Recovery

All inspection is read-only and pinned. Evidence edits are additive. Re-run
deterministic counts/checks after any amendment. Never reset unrelated work or
modify either external checkout.
