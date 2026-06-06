# WBVAL03 Snowmelt Water-Balance Closure Defect Closure

Status: executed-hold - blocked by upstream climate source boundary

Package type: Defect-Closure ExecPlan (DC-ExecPlan)

## Objective

Close two WBVAL01-observed invariant violations inside one snowmelt/percolation
water-balance authority envelope:

- `WBVAL03-HKERNEL-WB11-PERC-E-003-J95`: the four single-OFE Rocky Mountain
  hillslopes `p7`, `p11`, `p18`, and `p20` fail closed at Julian day `95` with
  `HKERNEL-WB11-PERC-E-003`.
- `WBVAL03-WAT-LEDGER-CONSERVATION-RESIDUAL`: the `12` WBVAL01 hillslopes that
  emit complete WAT ledgers all show annual conservation residuals above the
  `1.0 mm/year` tolerance for years `2..6`.

The closure-leak finding is diagnostic-first, but it stays inside this
Defect-Closure ExecPlan as an internal attribution milestone. The package must
complete the measured balance identity before attributing the residual, then
land any contract-backed correction whose root cause is inside this envelope.

## Rationale

WBVAL01 split the single-OFE population into emitted WAT ledgers and fail-closed
domain blockers. The `HKERNEL-WB11-PERC-E-003` blockers all occur at J-95, the
spring onset period where snowmelt, snow storage, local liquid ingress, and
percolation interact. The emitted ledger residuals also point at storage/flux
accounting over the same snowmelt/percolation/water-balance surfaces. Bundling
these findings is right-sized because they share authority, write-set, fixture
population, and validation evidence.

This package must not reopen the suspended HPHYS0298->0320 snow/`RM` comparator
route or adjudicate snow magnitude against the comparator. Its authority is
water-balance closure, typed guards, and baseline-authoritative
snowmelt/percolation/storage process behavior.

## Correction Authority Envelope

### Defect IDs and observed violations

- `WBVAL03-HKERNEL-WB11-PERC-E-003-J95`
  - Observed failure: `p7`, `p11`, `p18`, and `p20` fail closed before WAT
    publication with `HKERNEL-WB11-PERC-E-003` at `sim_day_index=95`, calendar
    year `1990`, Julian day `95`.
  - Fixture: `/wc1/runs/in/indispensable-presenter/wepp/runs/`, generated TOML
    wrapper pattern recorded in WBVAL01 `artifacts/run-manifest.md`.
- `WBVAL03-WAT-LEDGER-CONSERVATION-RESIDUAL`
  - Observed failure: `p1`, `p3`, `p5`, `p8`, `p10`, `p12`, `p13`, `p15`,
    `p16`, `p19`, `p21`, and `p22` emit WAT ledgers and all exceed the
    `1.0 mm/year` conservation tolerance for years `2..6`.
  - Symptom-existence gate: first audit the full balance identity before
    attribution, including `Tile`, populated interception-storage deltas,
    run-on/run-in terms such as `UpStrmQ` and `SubRIn`, and the fact that
    `SoilWaterTotal` already includes `frozwt`.

### In-scope contracts and files

- Contracts:
  - `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  - `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md` only
    when routed snowmelt liquid/infiltration partition evidence requires it.
- Prior authority/evidence:
  - `docs/work-packages/20260606-wbval01-rocky-mountain-daily-climate-single-ofe-wb-closure-validation-001/artifacts/run-manifest.md`
  - `docs/work-packages/20260606-wbval01-rocky-mountain-daily-climate-single-ofe-wb-closure-validation-001/artifacts/single-ofe-closure-ledger.md`
  - `docs/work-packages/20260606-wbval01-rocky-mountain-daily-climate-single-ofe-wb-closure-validation-001/artifacts/worker-handoff.md`
  - `docs/defect_closure_execplans.md` worked guard for the WBVAL01 closure leak.
- Production/test files:
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
  - `crates/openwepp-summary-accumulator/src/lib.rs`
  - `crates/openwepp-watershed-output/src/writers.rs`
  - `tests/integration/**` for contract-derived regressions scoped to this
    defect.
  - This package directory under `docs/work-packages/`.

### Allowed production-edit classes

- Correct contract-backed active snowmelt onset, snow runtime storage, local
  liquid ingress, percolation-domain handling, or WAT ledger publication when the
  seven-gate bar is met.
- Preserve or improve typed fail-closed percolation evidence when the four J-95
  inputs are proven invalid upstream.
- Add balance-ledger terms or initial-storage publication when canonical
  `SC-WATBAL-001` authority requires the existing publication surface to expose
  them.
- Add trace/diagnostic surfaces needed to prove snowmelt/percolation/storage
  mechanism ownership inside this envelope.

### Disallowed production-edit classes

- No heuristic snowmelt, percolation, ET, runoff, or storage multipliers.
- No guard loosening, silent clamping, or canonicalize-and-proceed behavior for
  negative or physically impossible snow/percolation domains.
- No climate radiation fix; `CLIM-RUNTIME-E-017` belongs to WBVAL02.
- No MOFE routing correction except to prove that a single-OFE closure residual
  actually branches to a routing defect target.
- No HPHYS0298->0320 snow/`RM` comparator route reopening or snow-magnitude
  tuning.

### Acceptance criteria

- `p7`, `p11`, `p18`, and `p20` either:
  - run through `openwepp-cli-hill` without `HKERNEL-WB11-PERC-E-003` and reach
    WAT publication, or
  - are reclassified as invalid upstream input or correct typed fail-closed
    behavior with explicit evidence.
- The emitted-ledger residual is measured against a complete declared identity.
  If missing terms explain the residual, record validated non-defect closure. If
  an in-envelope process/publication defect remains and the seven gates are
  satisfied, land the contract-first correction in this package.
- For all hillslopes that emit WAT after package work, years `2..6` are
  reclassified against the WBVAL01 `1.0 mm/year` tolerance using the complete
  identity.
- Year-1 full-calendar closure is either enabled by an in-envelope initial
  storage publication correction, or explicitly excluded with a boundary and a
  new defect target.

### Branch-out boundaries

- If the mechanism is the six radiation-bound `CLIM-RUNTIME-E-017` blocker
  class, branch to WBVAL02.
- If the mechanism requires snow magnitude adjudication against the suspended
  HPHYS0298->0320 snow/`RM` comparator route, stop at the protected backlog
  boundary and route to `docs/backlog/20260605-snow-code-deferred-science-review.md`.
- If the complete balance identity proves the emitted residual is a MOFE routing,
  channel, or upstream run-on/run-in publication issue outside single-OFE
  vertical closure, create a new defect target for the owning routing authority.
- If canonical snow/percolation/water-balance authority is missing or
  contradictory, close in `HOLD` only with the missing decision named.
- If evidence cannot be generated in the local environment, record the exact
  missing substrate and route a defect target rather than a next trace step.

## Conversion Rule

If this package establishes a reproducible root cause inside the declared
snowmelt/percolation/water-balance envelope and the expected behavior is
supported by canonical `SC-*` authority, pinned-baseline provenance, or a
contract-authorized physical invariant, it must proceed through contract
amendment, contract-derived tests, pre-implementation gate evidence, production
correction, validation, review, and disposition in this package. It may not close
as `HOLD` merely because more attribution work is possible.

## Seven-Gate Bar

All seven true means `HOLD` is invalid and the package must land the fix:

1. Reproduction: the J-95 percolation failure or conservation residual is
   reproduced, or statically and unambiguously tied to the WBVAL01 evidence.
2. Mechanism: the symptom is reduced to a named mechanism such as snowmelt
   onset, stale snow runtime storage, local liquid ingress, percolation-domain
   handling, WAT term omission, or initial-storage publication.
3. Ownership: the mechanism lies inside the declared write-set and contract
   authority.
4. Authority: the expected behavior traces to `SC-SNOWFREEZE-001`,
   `SC-PERC-001`, `SC-WATBAL-001`, relevant pinned baseline provenance, or a
   contract-authorized physical invariant.
5. Safety: the fix does not loosen guards, silently clamp, invent physics, or
   canonicalize a domain violation away.
6. Testability: a contract-derived regression can fail before the fix and pass
   after it.
7. Validation: the WBVAL J-95 blocker and/or conservation-closure acceptance
   target is measurable before and after the change.

## Legitimate HOLD Conditions

This package may close in `HOLD` only when one of these boundaries is proven and
recorded:

- The mechanism is outside the declared snowmelt/percolation/water-balance
  envelope.
- Canonical authority is missing or contradictory.
- The inputs are invalid upstream and the typed fail-closed guard is correct.
- Required evidence cannot be generated in the local environment.
- The mechanism is protected by the suspended snow/`RM` comparator backlog
  boundary.

Forbidden grind-HOLD examples:

- "Inspect the next snow helper."
- "Trace percolation one level deeper."
- "Root cause is in the declared hydrology files, but implementation is
  deferred."
- "Another package should add the balance test this package identified."

## Included Scope

- Reproduce or statically anchor the four J-95 `HKERNEL-WB11-PERC-E-003`
  failures from WBVAL01.
- Complete the emitted-ledger balance identity before attributing the residual.
- Attribute both defect classes to named mechanisms or legitimate branch
  boundaries.
- Amend canonical contracts before any production correction.
- Add contract-derived tests and pre-implementation gate evidence.
- Implement in-envelope, contract-backed corrections when the seven-gate bar is
  satisfied.
- Validate the four previously blocked hillslopes and the `12` emitted-ledger
  hillslopes against the WBVAL acceptance surfaces.
- Complete dual review, finding disposition, dual verification, final
  disposition, and defect-shaped handoff.

## Excluded Scope

- No climate radiation correction.
- No comparator-match acceptance or snow-magnitude tuning.
- No frost `ksflag`/`ksatadj` implementation unless a proven in-envelope defect
  requires a typed branch-out target to that later rung.
- No MOFE routing closure beyond proving a branch boundary.
- No empirical compensation in any WB13/WB14/WB17/WB18/WB19 term.

## Deliverables

- `artifacts/j95-percolation-attribution-ledger.md`
- `artifacts/complete-balance-identity-audit.md`
- `artifacts/wbval03-validation-ledger.md`
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
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/decisions/0018-defect-closure-execplans-conversion-rule.md`
- `docs/backlog/20260605-snow-code-deferred-science-review.md`
- `docs/work-packages/20260606-wbval01-rocky-mountain-daily-climate-single-ofe-wb-closure-validation-001/`
- Run inputs: `/wc1/runs/in/indispensable-presenter/wepp/runs/`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md` only
  when routed snowmelt liquid/infiltration partition evidence requires it.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-summary-accumulator/src/lib.rs`
- `crates/openwepp-watershed-output/src/writers.rs`
- `tests/integration/**wbval03**.rs`
- `tests/integration/**snow**.rs`, `tests/integration/**perc**.rs`, or
  `tests/integration/**watbal**.rs` only for contract-derived regressions.
- `docs/work-packages/20260606-wbval03-snowmelt-wb-closure-defect-closure-001/**`

## Phase Plan

1. Symptom existence and contracts: complete the balance identity and confirm or
   amend `SC-*` authority for the observed mechanisms.
2. Contract-derived tests: add red tests for J-95 percolation and/or ledger
   closure mechanisms.
3. Pre-implementation gate: record failing evidence before production edits.
4. Production correction: implement only in-envelope, contract-backed fixes.
5. Validation: rerun the four J-95 blockers and the `12` emitted-ledger
   hillslopes against the complete WBVAL identity.
6. Review/disposition: complete dual review, finding disposition, dual
   verification, final disposition, and defect-shaped handoff.

## Exit Criteria

- The four `HKERNEL-WB11-PERC-E-003` WBVAL01 blockers are closed by correction
  or reclassified as invalid/correct fail-closed behavior with typed evidence.
- The emitted-ledger conservation residual is either eliminated under a complete
  identity, corrected by an in-envelope contract-backed fix, or routed at a
  legitimate declared boundary.
- No snow/percolation guard is loosened, clipped around, or silently bypassed.
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
