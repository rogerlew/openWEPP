# hillstab03-contract-test-implementation-evidence

Status: complete  
Evidence mode: Static

## Contract-Derived Test Updates
- Updated `tests/integration/wb16_peak_runoff_kernel_contract.rs`:
  - added helper for WB16 `tc(vstar)` evaluation used by branch assertions,
  - replaced legacy branch assertions that depended on `timep`,
  - added explicit branch-4 vector for `vstar >= 1` constant-excess behavior,
  - adjusted branch-3 fixture geometry (`efflen=0.01`) to deterministically hit
    `0 < tstar <= tc`,
  - changed out-of-domain guard vector from `timep=1.5` to `m=0.0` to match
    amended WB16 domain posture,
  - added `wb16_contract_conformance_executes_without_timep_symbol` to lock the
    no-`timep` WB16 coupling requirement.

## Coverage Intent
- Test vectors now explicitly cover all WB16 branch-authoritative selectors:
  1. `tstar >= 1`,
  2. `vstar < 1` with `tc < tstar < 1`,
  3. `vstar < 1` with `0 < tstar <= tc`,
  4. `vstar >= 1` with `tstar < 1`.
- Guard vectors ensure WB16 still hard-fails true domain violations via typed
  `HKERNEL-WB16-PEAK-E-003`.
