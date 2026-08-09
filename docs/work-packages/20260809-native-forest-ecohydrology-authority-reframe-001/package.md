# Native-Forest Ecohydrology Authority Reframe

Status: `complete / exact-head full-workspace pass`

Date: `2026-08-09`

Package ID: `20260809-native-forest-ecohydrology-authority-reframe-001`

Plan class: `Critical contract-first semantic correction`

This ExecPlan is a living document maintained under `docs/codex_exec_plans.md`.

## Objective

Correct the native-forest vegetation authority boundary before implementation.
Site-specific stratum parameters and compatible initial state are explicit
caller-supplied configuration, not values openWEPP must select or claim as
transferable. Canonical A0 authority instead owns each field's meaning, units,
scale, mathematical domain, required presence, process role, and guards.

For the native-forest route, retire the agricultural WEPP PMET partition as an
implementation target. Require independently computed and closed canopy
transpiration, wet-canopy evaporation, and forest-floor evaporation, with
layer-resolved root uptake. Loss of canopy demand must never be donated
automatically to soil evaporation. A scientifically admitted Penman-Monteith or
two-source resistance equation may still be used inside an explicitly owned
component; the prohibition is the agricultural `Kcb`/LAI demand partition and
its redistribution, not the name Penman-Monteith.

## User Outcome

The coupled vegetation successor can demonstrate that user values are parsed,
validated, kept distinct by stratum, and influence behavior as intended without
claiming those values are appropriate for another site. Its implementation
target becomes the forest ecohydrology mechanism needed to avoid the
Stevens Canyon peak-flow inversion: separate canopy, wet-canopy, forest-floor,
and root-layer controls rather than calibration of a fixed agricultural ET
partition.

## Authority Decision

The corrected boundary is:

- A0 scientific authority: field semantics, units, cadence, area/leaf basis,
  finite mathematical domain, process equations, constants classified as
  fixed science, numerical guards, conservation, state ownership, and test
  invariants.
- Caller authority: site-specific stratum parameter values, topology, and
  complete compatible initial state supplied as versioned external
  configuration or initial state.
- Demonstration authority: deliberately distinct `ASSUMED_FOR_EXECUTION`
  fixtures may prove parsing, domain rejection, stratum separation, monotonic
  response, limiting behavior, and closure. They confer no calibration,
  validation, ecosystem applicability, or transferability claim.
- Empirical authority: needed only when openWEPP distributes a named default,
  recommends a site value, or makes calibration/validation/transferability
  claims.

This supersedes the value-selection premise of
`20260809-rhessys-east-coast-vegetation-authority-closure-001` without changing
that package's historical terminal record.

## Stevens Canyon Mechanism Constraint

The wepppy investigation
`/workdir/wepppy/docs/investigations/2026-08-03-stevens-canyon-peak-flow-inversion/README.md`
is diagnostic mechanism evidence, not calibration or validation authority. It
showed that the agricultural PMET relationship

```text
K_Ep = Kcb_adjusted * (1 - exp(-0.45 * LAI))
K_Es = Kcb_adjusted * exp(-0.45 * LAI)
```

reassigns reduced canopy demand to soil evaporation. Parameter search and the
legacy-ET ablation did not recover the targeted post-fire ET composition. The
native route must therefore expose independently constrained component fluxes
and shallow root uptake rather than preserve that structural coupling.

## Included Scope

- Amend `SC-VEGETATION-001` and its index row with the corrected value/state
  authority boundary and native-forest component-flux invariants.
- Add contract-derived tests before any production implementation.
- Record primary literature supporting separate wet-canopy, canopy/soil
  resistance, radiation, root-uptake, and C3 response families as explicit
  implementation leads, without pretending a citation alone admits a complete
  equation family.
- Reframe `AUTH-RHEC-001`, `AUTH-RHEC-002`, `AUTH-RHEC-007`, and
  `AUTH-RHEC-015`; update related gap closures and the held implementation
  successor.
- Update roadmap, backlog, tracker, and work-package catalog status.
- Complete dual independent science review, finding disposition, dual terminal
  verification, direct validation, prompt archival, and terminal disposition.

## Excluded Scope And Claim Limits

- No production Rust, Cargo, selector, management schema, output, publication,
  deployment, default activation, calibration, validation, or cutover.
- No selected East Coast pine/oak values, mixed-profile averaging, recommended
  defaults, or transferability claim.
- No admission of an incomplete radiation, interception, conductance,
  photosynthesis, allocation, respiration, or root-uptake equation family.
- No silent defaults, sentinel repair, nonfinite replacement, arbitrary clamp,
  direct soil-store mutation, or automatic transfer of unmet canopy demand to
  a forest-floor evaporation component.
- No change to the existing aggregate `SC-EVAP-001` agricultural/legacy PMET
  compatibility path. Retirement applies only to the future native-forest
  implementation target; real cutover is separate work.

## Intended Write Set

- This package tree.
- `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` and
  `docs/specifications/science-contracts/index.md`.
- `tests/integration/vegetation_boundary_authority_contract.rs`.
- `docs/work-packages/20260808-rhessys-east-coast-coupled-vegetation-slice-001/`.
- `docs/ROADMAP.md`, `docs/backlog/TRACKER.md`,
  `docs/backlog/20260806-rhessys-derived-vegetation-crate.md`, and
  `docs/work-packages/README.md`.

No production path is authorized.

## Contract-First Sequence

1. Freeze intent, instruction map, diagnostic evidence role, and exact write set.
2. Add failing contract assertions for caller-supplied values/state and
   independent native-forest flux components.
3. Amend the canonical contract and index until the focused contract test passes.
4. Prospectively amend the held successor to the corrected implementation target.
5. Reconcile the exact diff and run direct documentation, contract, and Critical
   full-workspace gates.
6. Complete two independent science reviews, disposition all findings, then
   complete two independent terminal verifications.
7. Archive the kickoff prompt and record terminal lifecycle disposition.

## Validation Plan

Minimum direct commands:

```bash
markdown-doc lint --path docs/work-packages/20260809-native-forest-ecohydrology-authority-reframe-001 --format plain
markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md --format plain
cargo nextest run --test vegetation_boundary_authority_contract --profile quick
cargo nextest run --workspace --profile full
```

The full-workspace run is a Critical semantic-change requirement and must be
delegated to the `comparator_suite_runner` role. Exact commands, outcomes, and
limitations belong in `artifacts/gate-results.md`.

## Review And Delegation Requirements

The executing agent is explicitly authorized to spawn subagents for this
package. Required roles are:

- two independent forest-ecohydrology/science reviewers of the canonical
  amendment, literature roles, Stevens Canyon interpretation, and successor
  boundary;
- two independent terminal verifiers after finding disposition; and
- one `comparator_suite_runner` for the full-workspace correctness gate.

Reviewers and verifiers must inspect the exact current diff and write separate
artifacts. Author self-review does not satisfy either pair.

## Exit Criteria

- Contract tests fail before and pass after the canonical amendment.
- A0/caller/demonstration/empirical authority roles are explicit and tested.
- The native-forest target rejects agricultural `Kcb`/LAI PMET redistribution
  and requires independently closed component fluxes plus layer-resolved roots.
- Penman-Monteith is neither blindly required nor broadly prohibited; any
  chosen component equation still needs complete independent authority.
- The successor no longer waits for selected site values or a co-observed
  universal pine/oak initial state, but remains held on incomplete process
  physics and pre-implementation gates.
- Focused, documentation, and full-workspace gates pass.
- Dual reviews have no unresolved material findings and dual terminal
  verifications pass.
- Prompt lifecycle, terminal diff, line-count governance, and disposition are
  complete.

## Progress

- [x] 2026-08-09: User corrected the site-value and native-forest ET premises.
- [x] 2026-08-09: Stevens Canyon diagnostic artifacts and primary literature
  leads were inspected.
- [x] 2026-08-09: Applicable instruction chain and existing contract/package
  boundary were inspected.
- [x] 2026-08-09: Added failing-first contract assertions and recorded the red
  result.
- [x] 2026-08-09: Amended canonical authority, contract tests, and successor
  scope; focused tests and Markdown gates pass.
- [x] 2026-08-09: Completed dual science review and dispositioned all findings.
- [x] 2026-08-09: Remediated root disk exhaustion by moving two recoverable
  stale gate scratch clones to ignored `/home` storage.
- [x] 2026-08-09: The separately authorized assurance defect-closure package
  proved the prior failure was an invalid in-repository `TMPDIR` invocation;
  the corrected external-scratch exact workspace passed 2,325/2,325 tests.
- [x] 2026-08-09: Dual terminal verification passed the truthful executed-hold
  disposition with no residual findings.
- [x] 2026-08-09: Archived the kickoff prompt byte-for-byte; canonical
  reference-aware move was abandoned only after 14 CPU-bound minutes without a
  filesystem change, then a direct rename and digest/reference checks passed.

## Decision Log

- 2026-08-09: Site-specific stratum values and compatible initial state are
  caller-owned; openWEPP proves the typed contract and behavior, mirroring the
  canopy-phenology configuration posture.
- 2026-08-09: `AUTH-RHEC-007` no longer means port or independently re-derive
  the audited RHESSys Penman-Monteith routine. It means admit independently
  owned native-forest component flux equations and reject agricultural PMET
  redistribution. A correct PM equation remains one eligible component method.
- 2026-08-09: The Stevens Canyon investigation is mechanism-diagnostic evidence
  that constrains architecture but is not empirical validation authority.

## Surprises And Discoveries

- The legacy ET ablation also failed the Stevens Canyon target, so merely
  switching PMET off is not a sufficient forest solution.
- RHESSys computes wet-canopy evaporation, dry-canopy transpiration, and
  sunlit/shaded terms separately, but its audited PM and energy operands contain
  defects; component separation is useful provenance, not permission to port.
- `markdown-doc mv` scanned roughly 19,700 Markdown files for 14 minutes at high
  CPU without reaching its move phase. Direct rename plus digest and reference
  checks completed the byte-preserving prompt archive; this is a confirmed
  docs-tool performance painpoint.

## Outcomes And Retrospective

The authority-reframe objective is implemented: `SC-VEGETATION-001` version 4,
its focused test, the implementation successor, and lifecycle documents agree
that users supply site values/state and that native forest requires independent
component closure rather than agricultural PMET redistribution. Dual science
review findings are closed.

The exact lift condition is now satisfied by separately authorized package
`20260809-assurance-draft-publication-defect-closure-001`. It proved the prior
failure was not a production assurance defect: setting `TMPDIR` below the
repository correctly triggered root confinement before DRAFT lifecycle
validation. With external `/home/workdir/openwepp-task-tmp`, the isolated DRAFT
case passed and the exact workspace passed 2,325/2,325 selected tests in
3,300.706 seconds. The reviewed assurance test diagnostic now also proves the
exact lifecycle error, complete public-tree non-mutation, and absence of
snapshot/receipt output.

The unrelated-gate hold is lifted and this authority-reframe package is
complete. This does not release production implementation: the coupled
successor remains independently held on complete schema/constitutive authority,
contract-first tests, conservative coupling, evaluation, and later promotion
gates. Site-value selection is not reopened.
