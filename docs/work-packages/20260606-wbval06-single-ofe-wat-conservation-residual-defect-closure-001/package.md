# WBVAL06 Single-OFE WAT Conservation Residual Defect Closure

Status: queued

Package type: Defect-Closure ExecPlan (DC-ExecPlan)

## Objective

Close defect `WBVAL06-SINGLE-OFE-WAT-CONSERVATION-RESIDUAL` end-to-end: under
the WBVAL04 publication-safe climate, all `18` WAT-emitting Rocky Mountain
single-OFE hillslopes have annual complete-identity residuals above
`1.0 mm/year` for years `2..6`; the maximum current residual is `94.433 mm`
on `p4`, year `5`.

This package owns the WAT publication and complete-identity water-balance
closure envelope. If the root cause is in-envelope and authority-backed, this
package must land the contract-first fix rather than relaying another
diagnostic step.

## Rationale

WBVAL04 removed the upstream climate blocker and expanded the WAT-emitting
population from the original `12` WBVAL01 emitters to `18` current emitters.
Every current emitter still violates the complete annual conservation identity
for years `2..6`, so the residual is not explained by the prior radiation
boundary.

WBVAL06 is split from WBVAL05 because the remaining J-95 percolation blockers
do not emit WAT, while the residual evidence lives on emitted WAT publication
and annual accounting surfaces. WBVAL06 may consume WBVAL05 outputs if they are
available, but its required acceptance surface is the current 18-emitter
population from WBVAL04.

## Correction Authority Envelope

### Defect IDs and Observed Violations

- `WBVAL06-SINGLE-OFE-WAT-CONSERVATION-RESIDUAL`
  - Observable failure: `p1`, `p2`, `p3`, `p4`, `p5`, `p6`, `p8`, `p9`,
    `p10`, `p12`, `p13`, `p14`, `p15`, `p16`, `p17`, `p19`, `p21`, and `p22`
    emit WAT ledgers and all exceed the `1.0 mm/year` conservation tolerance
    for years `2..6`.
  - Maximum current residual: `94.433 mm` (`p4`, year `5`) in WBVAL04.
  - Current substrate: WBVAL04 WAT outputs under
    `/tmp/wbval04_rocky_mountain_20260606T000000Z/outputs/`.
  - Fixture input root:
    `/wc1/runs/in/indispensable-presenter/wepp/runs/`.

### In-Scope Contracts and Source Files

- Contracts:
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `docs/specifications/science-contracts/contracts/SC-PERC-001.md` when
    `D`/`Pe` or percolation state publication is implicated.
  - `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` when
    `Snow-Water`, melt, or snow storage publication is implicated.
  - `docs/specifications/science-contracts/contracts/SC-EVAP-001.md` when
    `Ep`, `Es`, or `Er` publication/accounting is implicated.
  - `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md` when
    runoff/infiltration partition publication is implicated.
- Production/test files:
  - `crates/openwepp-runner/src/hillslope/mod.rs`
  - `crates/openwepp-summary-accumulator/src/lib.rs`
  - `crates/openwepp-watershed-output/src/writers.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs` only if snow storage/melt publication is implicated.
  - `tests/integration/**wbval06**.rs`
  - `tests/integration/**watbal**.rs`, `tests/integration/**perc**.rs`,
    `tests/integration/**snow**.rs`, `tests/integration/**evap**.rs`, or
    `tests/integration/**runoff**.rs` only for contract-derived regressions.
  - `docs/work-packages/20260606-wbval06-single-ofe-wat-conservation-residual-defect-closure-001/**`
  - `docs/work-packages/README.md`

### Allowed Edit Classes

- Amend canonical `SC-*` authority for the proven WAT closure mechanism before
  production edits.
- Add contract-derived tests for annual/daily WAT closure, storage publication,
  and required term exposure.
- Add bounded diagnostic/trace surfaces needed to identify omitted/mis-signed
  WAT terms or storage deltas.
- Correct WAT publication, summary accumulation, storage publication, flux
  publication, or hydrology writeback ordering when the seven-gate bar is met.
- Add required initial-storage publication only when canonical authority
  requires it; year `1` may remain explicitly out of full-calendar closure
  until that authority exists.

### Protected Boundaries

- Do not edit WEPPpy climate producers or `/wc1` input artifacts.
- Do not use comparator magnitude agreement as acceptance.
- Do not tune snow, ET, percolation, runoff, or storage magnitudes with
  empirical compensation.
- Do not close `p7`, `p11`, `p18`, or `p20` J-95 percolation blockers; WBVAL05
  owns that envelope.
- Do not silently add or omit terms from the identity without canonical
  authority, units, and tests.

### Acceptance Criteria

- For all current WBVAL04 WAT emitters, years `2..6` complete-identity
  residuals are within `1.0 mm/year`, or the residual is reclassified at a
  declared authority boundary with explicit missing/invalid surface evidence.
- The declared closure identity states signs, units, source columns, storage
  surfaces, tolerance, and year-1 boundary before classification.
- Any in-envelope correction is backed by canonical `SC-*` text,
  contract-derived tests, pre-implementation failing evidence, and post-fix
  validation.
- No production change introduces heuristic process math, silent defaults,
  unbounded clamping, or comparator-target tuning.

## Conversion Rule

If this package establishes a reproducible root cause inside the declared WAT
publication/water-balance closure envelope and the expected behavior is
supported by canonical `SC-*` authority, pinned-baseline provenance, or a
contract-authorized physical invariant, it must proceed through contract
amendment, contract-derived tests, pre-implementation gate evidence,
production correction, validation, review, and disposition in this package. It
may not close as `HOLD` merely because more investigation is possible.

## Seven-Gate Bar

All seven true means `HOLD` is invalid and the package must land the fix:

1. Reproduction: the `18`-emitter residual is reproduced or statically tied to
   WBVAL04 evidence.
2. Mechanism: the residual is reduced to a named WAT publication,
   storage/flux accounting, or hydrology writeback mechanism.
3. Ownership: the mechanism lies inside the declared write set and contract
   authority.
4. Authority: expected behavior traces to canonical `SC-*`, pinned baseline, or
   a contract-authorized physical invariant.
5. Safety: the fix does not loosen guards, silently clamp, invent physics, or
   canonicalize a domain violation away.
6. Testability: a contract-derived regression can fail before the fix and pass
   after it.
7. Validation: the WBVAL annual residual acceptance target is measurable before
   and after the change.

## Legitimate HOLD Conditions

- The mechanism is proven outside the declared WAT/water-balance envelope.
- Canonical authority is missing or contradictory.
- Required evidence cannot be generated in the local environment.
- The root cause requires a different process-family authority not declared
  here.
- The current residual dissolves as a validated non-defect under a complete,
  contract-authorized identity.

Forbidden grind-HOLD examples:

- "Inspect the next WAT writer."
- "Trace one more storage symbol."
- "Root cause is in WAT publication, but implementation is deferred."
- "Another package should add the closure regression this package identified."

## Included Scope

- Reproduce or statically anchor the `18` WBVAL04 WAT-emitter residuals.
- Verify and, if needed, amend the complete closure identity before
  attribution.
- Attribute the residual to a named mechanism or legitimate branch-out
  boundary.
- Amend canonical contracts before any production correction.
- Add contract-derived tests and record pre-implementation failing evidence.
- Land in-envelope, authority-backed production corrections when the seven-gate
  bar is met.
- Validate current WAT emitters on years `2..6`.
- Complete dual review, finding disposition, dual verification, disposition,
  and worker handoff.

## Excluded Scope

- No J-95 percolation fail-closed closure; WBVAL05 owns that package.
- No WEPPpy or climate producer edits.
- No comparator-match acceptance or snow-magnitude tuning.
- No MOFE/channel/routing correction beyond a defect-shaped branch-out.
- No empirical compensation in any WAT term.

## Deliverables

- `artifacts/complete-balance-identity-audit.md`
- `artifacts/wat-residual-attribution-ledger.md`
- `artifacts/wbval06-validation-ledger.md`
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
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/decisions/0018-defect-closure-execplans-conversion-rule.md`
- `docs/backlog/20260605-snow-code-deferred-science-review.md`
- `docs/work-packages/20260606-wbval04-rocky-mountain-daymet-wbval01-redo-001/`
- Optional predecessor evidence:
  `docs/work-packages/20260606-wbval05-j95-percolation-defect-closure-001/`
  if WBVAL05 has executed before WBVAL06 starts.
- Run inputs: `/wc1/runs/in/indispensable-presenter/wepp/runs/`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md` when
  `D`/`Pe` or percolation state publication is implicated.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` when
  snow storage/melt publication is implicated.
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md` when ET
  publication/accounting is implicated.
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md` when
  runoff/infiltration partition publication is implicated.
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-summary-accumulator/src/lib.rs`
- `crates/openwepp-watershed-output/src/writers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`
  only if snow storage/melt publication is implicated.
- `tests/integration/**wbval06**.rs`
- `tests/integration/**watbal**.rs`, `tests/integration/**perc**.rs`,
  `tests/integration/**snow**.rs`, `tests/integration/**evap**.rs`, or
  `tests/integration/**runoff**.rs` only for contract-derived regressions.
- `docs/work-packages/20260606-wbval06-single-ofe-wat-conservation-residual-defect-closure-001/**`
- `docs/work-packages/README.md`

## Phase Plan

1. Reproduce and audit the current 18-emitter annual residuals under the
   complete declared identity.
2. Confirm or amend canonical `SC-*` authority for the named mechanism.
3. Add contract-derived regression tests and record pre-implementation failing
   evidence.
4. Implement only in-envelope, contract-backed production correction.
5. Validate years `2..6` for the current WAT emitters and record any branch-out
   boundary.
6. Complete dual review, finding disposition, dual verification, gate results,
   worker handoff, and final disposition.

## Exit Criteria

- Current WBVAL04 WAT emitters are closed by correction, validated non-defect
  reclassification, or legitimate declared boundary.
- No WAT term is silently invented, dropped, re-signed, or normalized without
  contract authority and tests.
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
