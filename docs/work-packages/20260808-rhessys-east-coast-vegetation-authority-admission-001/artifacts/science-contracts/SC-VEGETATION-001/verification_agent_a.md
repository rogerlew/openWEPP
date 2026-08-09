# Verification Agent A: SC-VEGETATION-001

Status: `complete`

Date: 2026-08-08 UTC

Evidence mode: `Ran + Static`

Verified contract SHA-256:
`7e62cf907eb328ad1b1aaf535ab1556896f686c7d0a8e01ed22a6ce81d635f7a`

Disposition source:
`artifacts/science-contracts/SC-VEGETATION-001/disposition.md`

## Closure Check

- `A-01`: `closed` — the exact White/Reich/Hwang/Ford/Coweeta/Harvard routes
  and the narrowed compatible-state blocker are recorded in
  `artifacts/authority-route-attempts.md:7-38`.
- `A-02`: `closed` — Gate 2 and Gate 3 each evaluate every named symbolic family
  independently and retain equation/domain/guard/vector blockers in
  `artifacts/canopy-water-energy-gate.md:7-29` and
  `artifacts/carbon-phenology-root-gate.md:7-26`.
- `A-03`: `closed` — acquisition, schema-form, selected-value, and dated-state
  results are separately stated in
  `artifacts/schema-profile-initial-state-gate.md:38-92`.
- `A-04`: `closed` — canonical refs/invariants/guards/schema/BEI/gaps are present
  at `SC-VEGETATION-001.md:88-90`, `:271-274`, `:299-303`, `:355-391`,
  `:470`, and `:495-497`; direct assertions are at
  `tests/integration/vegetation_boundary_authority_contract.rs:110-136`.
- `A-05`: `closed` — the readiness matrix separates partial typed/schema
  progress from blocked calibration/execution at
  `artifacts/calibration-readiness-matrix.md:11-27`.
- `A-06`: `closed` — final disposition names all three gates and limits the
  admission to an authority-only partial result at
  `artifacts/disposition.md:7-16`.

No `A-*` finding remains open. The accepted-action claims match the canonical
contract and package evidence.

Reviewer B's later `N-01` wording remediation preserves the canopy-snow,
fixed-point/fallback, and tolerance prohibitions explicitly through version 3.
A final narrow recheck confirmed its disposition locators and found no
regression to any `A-*` closure.

## Direct Checks

The verifier ran the focused contract suite (`9/9`), strict Binding Exposure
check (`3/3`), SC unit-compliance check, contract Markdown lint (zero errors and
warnings), and `git diff --check`; all passed on the verified bytes.

Residual selected-profile/state and Gate 2/3 gaps remain explicitly
non-promotable. They support the package's scientific `executed-hold` and are
not contract-cycle regressions.

## Verdict

`PASS`
