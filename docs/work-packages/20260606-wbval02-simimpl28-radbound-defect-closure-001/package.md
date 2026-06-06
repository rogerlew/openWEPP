# WBVAL02 SIMIMPL28 Radiation-Bound Defect Closure

Status: complete - validated invalid upstream input

Package type: Defect-Closure ExecPlan (DC-ExecPlan)

## Objective

Close defect `WBVAL02-CLIM-RUNTIME-E-017-RADBOUND` end-to-end: the six WBVAL01
single-OFE Rocky Mountain hillslopes `p2`, `p4`, `p6`, `p9`, `p14`, and `p17`
fail closed before WAT publication with `CLIM-RUNTIME-E-017` hourly radiation
domain errors. The package must either land a contract-first correction inside
the declared climate-forcing authority envelope, or reclassify each failure as
invalid upstream input with typed evidence.

This is not a diagnostic-only package. It owns diagnosis, correction when the
seven-gate bar is satisfied, validation, dual review, verification, disposition,
and any legitimate boundary handoff.

## Rationale

WBVAL01 established that `10/22` single-OFE hillslopes cannot enter the
conservation ledger because they fail closed before WAT publication. Six of
those failures are a distinct climate-forcing class: finite hourly radiation
values exceed the physical bound guarded by HPHYS0277 and
`SC-CLIMATE-001#INV-CLIMATE-013`. That class has a different authority and
write-set from snowmelt/percolation storage closure, so it is intentionally split
from WBVAL03.

The HPHYS0277 guard must not be loosened, clipped around, or bypassed. If the
six hillslopes are valid inputs, the fix is expected to be in the SIMIMPL28 daily
to hourly radiation projection, the physical-bound context used for that
projection, or the typed evidence emitted for upstream invalidity.

## Correction Authority Envelope

### Defect IDs and observed violations

- `WBVAL02-CLIM-RUNTIME-E-017-RADBOUND`
  - Observed failure: WBVAL01 `openwepp-cli-hill` runs for `p2`, `p4`, `p6`,
    `p9`, `p14`, and `p17` fail closed before WAT publication with
    `CLIM-RUNTIME-E-017`.
  - Fixture: `/wc1/runs/in/indispensable-presenter/wepp/runs/`, generated TOML
    wrapper pattern recorded in WBVAL01 `artifacts/run-manifest.md`.
  - Acceptance class: valid hillslopes run past SIMIMPL28 radiation projection
    without `CLIM-RUNTIME-E-017`; invalid upstream radiation inputs are
    reclassified with typed evidence and no production guard loosening.

### In-scope contracts and files

- Contracts:
  - `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
    for WAT-publication acceptance evidence only.
- Prior authority/evidence:
  - `docs/work-packages/20260603-hphys0277-climate-radiation-physical-flux-guard-001/package.md`
  - `docs/work-packages/20260603-hphys0277-climate-radiation-physical-flux-guard-001/artifacts/disposition.md`
  - `docs/work-packages/20260603-hphys0277-climate-radiation-physical-flux-guard-001/artifacts/worker-handoff.md`
  - `docs/work-packages/20260606-wbval01-rocky-mountain-daily-climate-single-ofe-wb-closure-validation-001/artifacts/run-manifest.md`
  - `docs/work-packages/20260606-wbval01-rocky-mountain-daily-climate-single-ofe-wb-closure-validation-001/artifacts/single-ofe-closure-ledger.md`
- Production/test files:
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
  - `crates/openwepp-climate-runtime-adapter/src/lib.rs`
  - `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
  - `tests/integration/**` for contract-derived regressions scoped to this
    defect.
  - This package directory under `docs/work-packages/`.

### Allowed production-edit classes

- Correct SIMIMPL28 hourly radiation synthesis, unit lineage, or
  physical-bound context when canonical `SC-*` text plus pinned-baseline
  provenance authorizes the behavior.
- Improve typed `CLIM-RUNTIME-E-017` evidence when the inputs are proven invalid
  upstream.
- Add trace/diagnostic publication needed to prove the radiation projection,
  daily-to-hourly conservation, and bound comparison.
- Add tests and validation harnesses for the six WBVAL01 hillslopes.

### Disallowed production-edit classes

- No clipping, saturation, or silent canonicalization of physically impossible
  radiation.
- No weakening/removal of the HPHYS0277 radiation guard without contract-first
  proof that the prior guard is overbroad.
- No snowmelt, ET, runoff, percolation, storage, WAT ledger, or comparator
  compensation to make the six runs pass.
- No fixed empirical radiation multiplier or local snow-magnitude tuning.

### Acceptance criteria

- Each of `p2`, `p4`, `p6`, `p9`, `p14`, and `p17` either:
  - runs through `openwepp-cli-hill` without `CLIM-RUNTIME-E-017` and reaches WAT
    publication, or
  - is reclassified as invalid upstream input with typed evidence identifying the
    invalid source value and contract boundary.
- Daily radiation lineage is conserved under the relevant `SC-CLIMATE-001`
  units: no double conversion, no Langley-scale value published under an MJ
  label, and no downstream radiation clipping.
- Existing HPHYS0277 impossible-radiation guard tests continue to pass, and any
  changed guard behavior has contract-derived red/green tests.
- WBVAL01 emitted-ledger hillslopes remain outside this package's correction
  target except as non-regression checks.

### Branch-out boundaries

- If the source `.cli`/CLIGEN daily radiation record is physically invalid under
  `SC-INFILE-CLIMATE-001` or `SC-CLIMATE-001`, close in `HOLD` only with typed
  upstream invalid-input evidence and a new defect target to close that input
  boundary.
- If the mechanism is aspect/slope geometry parsing outside SIMIMPL28 climate
  projection, branch to a new defect target naming the exact input-contract or
  terrain-contract authority.
- If radiation closure succeeds and remaining failures are snowmelt,
  percolation, ET, runoff, or WAT ledger residuals, route them to WBVAL03 or the
  later frost/MOFE rungs with a defect-shaped handoff.
- The HPHYS0298->0320 snow/`RM` comparator route remains suspended behind
  `docs/backlog/20260605-snow-code-deferred-science-review.md`.

## Conversion Rule

If this package establishes a reproducible root cause inside the declared
climate-forcing envelope and the expected behavior is supported by canonical
`SC-*` authority, pinned-baseline provenance, or a contract-authorized physical
invariant, it must proceed through contract amendment, contract-derived tests,
pre-implementation gate evidence, production correction, validation, review, and
disposition in this package. It may not close as `HOLD` merely because further
radiation investigation is possible.

## Seven-Gate Bar

All seven true means `HOLD` is invalid and the package must land the fix:

1. Reproduction: the `CLIM-RUNTIME-E-017` failure is reproduced for at least one
   of the six WBVAL01 hillslopes, or statically tied to the recorded WBVAL01
   run evidence.
2. Mechanism: the symptom is reduced to a named mechanism such as unit
   conversion, hourly partitioning, bound-context construction, or invalid
   upstream daily radiation, not "trace the next variable."
3. Ownership: the mechanism lies inside the declared climate write-set and
   contract authority.
4. Authority: the expected behavior traces to `SC-CLIMATE-001`, pinned
   `/workdir/wepp-forest_260430_baseline` provenance, or a
   contract-authorized physical radiation invariant.
5. Safety: the fix does not loosen guards, silently clamp, invent physics, or
   canonicalize a domain violation away.
6. Testability: a contract-derived regression can fail before the fix and pass
   after it.
7. Validation: the six-hillslope WBVAL acceptance target is measurable before
   and after the change.

## Legitimate HOLD Conditions

This package may close in `HOLD` only when one of these boundaries is proven and
recorded:

- The mechanism is outside the declared climate-forcing envelope.
- Canonical authority is missing or contradictory for the expected radiation
  behavior.
- The six inputs are invalid upstream and the existing typed fail-closed guard
  is correct.
- Required evidence cannot be generated in the local environment.

Forbidden grind-HOLD examples:

- "Inspect the next SIMIMPL28 helper."
- "Trace radiation one level deeper."
- "Root cause is in `06_simimpl28_hourly_forcing.rs`, but implementation is
  deferred."
- "Another package should add the contract test identified here."

## Included Scope

- Reproduce or statically anchor the six `CLIM-RUNTIME-E-017` failures from
  WBVAL01.
- Attribute the radiation-bound mechanism inside or outside the declared
  envelope.
- Amend canonical contract text before any production correction when the
  mechanism is in-envelope.
- Add contract-derived tests and pre-implementation gate evidence.
- Implement an in-envelope correction when the seven-gate bar is satisfied.
- Validate all six blocked hillslopes and record whether WAT publication is now
  reachable.
- Run targeted non-regression for existing HPHYS0277 radiation guard behavior.
- Complete dual review, finding disposition, dual verification, final
  disposition, and defect-shaped handoff.

## Excluded Scope

- No snowmelt-onset, snow-storage, percolation, frost, ET, runoff, MOFE routing,
  or WAT conservation-leak correction.
- No comparator-match acceptance.
- No snow/`RM` magnitude adjudication.
- No heuristic radiation multiplier or residual-reduction tuning.

## Deliverables

- `artifacts/radbound-attribution-ledger.md`
- `artifacts/radbound-validation-ledger.md`
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
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/decisions/0018-defect-closure-execplans-conversion-rule.md`
- `docs/backlog/20260605-snow-code-deferred-science-review.md`
- `docs/work-packages/20260603-hphys0277-climate-radiation-physical-flux-guard-001/`
- `docs/work-packages/20260606-wbval01-rocky-mountain-daily-climate-single-ofe-wb-closure-validation-001/`
- Run inputs: `/wc1/runs/in/indispensable-presenter/wepp/runs/`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `crates/openwepp-climate-runtime-adapter/src/lib.rs`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `tests/integration/**wbval02**.rs`
- `tests/integration/**climate**.rs` only for contract-derived radiation tests.
- `docs/work-packages/20260606-wbval02-simimpl28-radbound-defect-closure-001/**`

## Phase Plan

1. Contracts: confirm or amend `SC-CLIMATE-001` authority for the observed
   failure mechanism.
2. Contract-derived tests: add red tests for the defect mechanism and regression
   tests preserving HPHYS0277 guard semantics.
3. Pre-implementation gate: record failing evidence before production edits.
4. Production correction: implement only in-envelope, contract-backed climate
   fixes.
5. Validation: rerun the six WBVAL01 blocked hillslopes and targeted radiation
   guard tests.
6. Review/disposition: complete dual review, finding disposition, dual
   verification, final disposition, and defect-shaped handoff.

## Exit Criteria

- The six `CLIM-RUNTIME-E-017` WBVAL01 blockers are closed by correction or
  reclassified as invalid upstream input with typed evidence.
- No radiation guard is loosened, clipped around, or silently bypassed.
- Contract-first sequencing and truthfulness labels are visible in artifacts.
- Dual review findings are fully dispositioned before final package disposition.

## Security-Impact Gate

No external systems or network actions are required. This package is local
repository engineering work limited to flat-file reads/edits and local command
execution in the worktree.

## Autonomy

Execute the package end-to-end for the declared scope. Do not ask for user
direction for intermediate diagnostic steps. Ask only if hard-blocked by missing
local authority, unavailable validation substrate, or a proven boundary that
requires a new defect target outside this envelope.
