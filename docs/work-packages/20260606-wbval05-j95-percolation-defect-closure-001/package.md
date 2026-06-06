# WBVAL05 J-95 Percolation Defect Closure

Status: hold-boundary

Package type: Defect-Closure ExecPlan (DC-ExecPlan)

## Objective

Close defect `WBVAL05-J95-HKERNEL-WB11-PERC-E-003` end-to-end: under the
WBVAL04 publication-safe climate, the Rocky Mountain single-OFE hillslopes
`p7`, `p11`, `p18`, and `p20` fail closed at `sim_day_index=95`,
`calendar_year=1990`, `julian_day=95` with `HKERNEL-WB11-PERC-E-003` before
WAT publication.

This package owns correction only inside the percolation/deep-seepage authority
envelope. If the root cause is in-envelope and authority-backed, this package
must land the contract-first fix rather than relaying another diagnostic step.

## Rationale

WBVAL04 proved the upstream climate is now valid for the DRIGGS
`indispensable-presenter` run and removed the prior `CLIM-RUNTIME-E-017`
radiation blocker. The four J-95 hillslopes still fail at the same percolation
guard on valid climate, making this a current openWEPP defect-closure target
rather than an upstream producer boundary.

This package is intentionally narrower than WBVAL03: it owns only the
fail-closed J-95 percolation class. The annual WAT residual over the 18
current emitters is split to WBVAL06 because it primarily owns WAT publication
and complete-identity closure surfaces.

## Correction Authority Envelope

### Defect IDs and Observed Violations

- `WBVAL05-J95-HKERNEL-WB11-PERC-E-003`
  - Observable failure: `p7`, `p11`, `p18`, and `p20` fail closed at
    `sim_day_index=95`, `calendar_year=1990`, `julian_day=95` with
    `HKERNEL-WB11-PERC-E-003`; no WAT is emitted.
  - Current substrate: WBVAL04 release batch under
    `/tmp/wbval04_rocky_mountain_20260606T000000Z/`.
  - Fixture input root:
    `/wc1/runs/in/indispensable-presenter/wepp/runs/`.

### In-Scope Contracts and Source Files

- Contracts:
  - `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` only for
    WB18/WB11 percolation and storage-accounting coupling.
  - `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` only
    if same-pass snowmelt/liquid ingress into percolation is the proven
    mechanism.
  - `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md` only
    if routed infiltration/runoff partition authority is required by the proven
    mechanism.
- Production/test files:
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs` only if same-pass snowmelt ingress is implicated.
  - `crates/openwepp-runner/src/hillslope/mod.rs` only for fixture/runtime projection surfaces needed by the percolation defect.
  - `tests/integration/**wbval05**.rs`
  - `tests/integration/**perc**.rs`, `tests/integration/**watbal**.rs`, or
    `tests/integration/**snow**.rs` only for contract-derived regressions.
  - `docs/work-packages/20260606-wbval05-j95-percolation-defect-closure-001/**`
  - `docs/work-packages/README.md`

### Allowed Edit Classes

- Amend canonical `SC-*` authority for the proven WB18/WB11 percolation
  mechanism before production code edits.
- Add contract-derived tests for the J-95 percolation failure.
- Add bounded diagnostic/trace surfaces needed to reduce the guard to a named
  mechanism inside the envelope.
- Correct WB18 percolation domain handling, lower-layer saturation attenuation,
  same-pass liquid ingress, lane substep execution, restrictive-layer
  conductivity, or aggregate soil-water recomputation when the seven-gate bar
  is met.
- Preserve or improve typed fail-closed evidence if the inputs are proven
  invalid or the current guard is correct.

### Protected Boundaries

- Do not loosen `HKERNEL-WB11-PERC-E-003`, clamp invalid domains away, or
  canonicalize-and-proceed on physically invalid storage/flux state.
- Do not edit WEPPpy climate producers or `/wc1` input artifacts.
- Do not tune snow magnitude or reopen the suspended snow/`RM` comparator route.
- Do not close the annual WAT residual class; WBVAL06 owns that envelope.
- Do not make MOFE routing/channel corrections except to emit a defect-shaped
  branch-out target.

### Acceptance Criteria

- `p7`, `p11`, `p18`, and `p20` either:
  - run through `openwepp-cli-hill` on current valid climate, reach WAT
    publication, and avoid `HKERNEL-WB11-PERC-E-003`, or
  - are reclassified as invalid upstream input or correct typed fail-closed
    behavior with explicit evidence and no guard loosening.
- Any in-envelope correction is backed by canonical `SC-*` text,
  contract-derived tests, pre-implementation failing evidence, and post-fix
  validation.
- No production change introduces heuristic percolation, silent defaults,
  unbounded clamping, or process-physics approximation.

## Conversion Rule

If this package establishes a reproducible root cause inside the declared
percolation/deep-seepage envelope and the expected behavior is supported by
canonical `SC-*` authority, pinned-baseline provenance, or a
contract-authorized physical invariant, it must proceed through contract
amendment, contract-derived tests, pre-implementation gate evidence,
production correction, validation, review, and disposition in this package. It
may not close as `HOLD` merely because more investigation is possible.

## Seven-Gate Bar

All seven true means `HOLD` is invalid and the package must land the fix:

1. Reproduction: the J-95 `HKERNEL-WB11-PERC-E-003` failure is reproduced or
   statically tied to WBVAL04 evidence.
2. Mechanism: the symptom is reduced to a named percolation/deep-seepage
   mechanism.
3. Ownership: the mechanism lies inside the declared write set and contract
   authority.
4. Authority: expected behavior traces to canonical `SC-*`, pinned baseline, or
   a contract-authorized physical invariant.
5. Safety: the fix does not loosen guards, silently clamp, invent physics, or
   canonicalize a domain violation away.
6. Testability: a contract-derived regression can fail before the fix and pass
   after it.
7. Validation: the four-hillslope WBVAL acceptance target is measurable before
   and after the change.

## Legitimate HOLD Conditions

- The mechanism is proven outside the declared percolation/deep-seepage
  envelope.
- Canonical authority is missing or contradictory.
- The input is proven invalid upstream and the typed fail-closed behavior is
  correct.
- Required evidence cannot be generated in the local environment.
- The fix requires a different process-family authority not declared here.

Forbidden grind-HOLD examples:

- "Inspect the next percolation helper."
- "Trace layer storage one level deeper."
- "Root cause is in WB18 percolation, but implementation is deferred."
- "Another package should add the contract test this package identified."

## Included Scope

- Reproduce or statically anchor the four WBVAL04 J-95 fail-closed runs.
- Attribute `HKERNEL-WB11-PERC-E-003` to a named mechanism or legitimate
  branch-out boundary.
- Amend canonical contracts before any production correction.
- Add contract-derived tests and record pre-implementation failing evidence.
- Land in-envelope, authority-backed production corrections when the seven-gate
  bar is met.
- Validate the four target hillslopes on current valid climate.
- Complete dual review, finding disposition, dual verification, disposition,
  and worker handoff.

## Excluded Scope

- No annual WAT residual closure; WBVAL06 owns that package.
- No WEPPpy or climate producer edits.
- No snow magnitude tuning or comparator-match acceptance.
- No MOFE/channel/routing correction beyond a defect-shaped branch-out.
- No heuristic process-physics formulas.

## Deliverables

- `artifacts/j95-percolation-attribution-ledger.md`
- `artifacts/wbval05-validation-ledger.md`
- Standard contract evidence, gate, review, verification, disposition, and
  worker-handoff artifacts listed in `artifacts/README.md`.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/defect_closure_execplans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/decisions/0018-defect-closure-execplans-conversion-rule.md`
- `docs/backlog/20260605-snow-code-deferred-science-review.md`
- `docs/work-packages/20260606-wbval04-rocky-mountain-daymet-wbval01-redo-001/`
- Run inputs: `/wc1/runs/in/indispensable-presenter/wepp/runs/`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` only for
  percolation/storage coupling.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` only
  when same-pass snowmelt ingress is the proven mechanism.
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md` only
  when routed infiltration/runoff partition authority is required.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`
  only if same-pass snowmelt ingress is implicated.
- `crates/openwepp-runner/src/hillslope/mod.rs` only for required projection
  surfaces.
- `tests/integration/**wbval05**.rs`
- `tests/integration/**perc**.rs`, `tests/integration/**watbal**.rs`, or
  `tests/integration/**snow**.rs` only for contract-derived regressions.
- `docs/work-packages/20260606-wbval05-j95-percolation-defect-closure-001/**`
- `docs/work-packages/README.md`

## Phase Plan

1. Reproduce and instrument the J-95 percolation failure on the four target
   hillslopes.
2. Confirm or amend canonical `SC-*` authority for the named mechanism.
3. Add contract-derived regression tests and record pre-implementation failing
   evidence.
4. Implement only in-envelope, contract-backed production correction.
5. Validate the four target hillslopes and record any branch-out boundary.
6. Complete dual review, finding disposition, dual verification, gate results,
   worker handoff, and final disposition.

## Exit Criteria

- The four J-95 blockers are closed by correction or reclassified as invalid
  input/correct typed fail-closed behavior with evidence.
- No percolation guard is loosened or bypassed.
- Contract-first sequencing is visible in artifacts.
- Any remaining `HOLD` names a legitimate boundary and defect-shaped handoff.
- Dual review findings are fully dispositioned before final package
  disposition.

## Security-Impact Gate

Security impact: none. The package reads local run artifacts, executes local
openWEPP binaries/tests, and edits local repository files. It adds no network,
auth, secret, upload, download, queue, or public API behavior.

## Autonomy

Execute the package end-to-end for the declared scope. Do not ask for user
direction for intermediate diagnostic steps. Ask only if hard-blocked by
missing authority, unavailable validation substrate, or a proven boundary that
requires a new defect target outside this envelope.
