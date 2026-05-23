# PL11 Kernel Profile Compliance Checklist

Status: `complete`
Evidence mode: `Static + Ran`

Reference profile: `docs/specifications/science-contracts/kernel-process-contract-profile.md`

1. Canonical `SC-*` file updated: `met`
- Static: `SC-PLANT-001` amended to version `5` with PL11 runtime-projection reconciliation.

2. Required schema sections present: `met`
- Static: SC contract retains purpose/scope, authority anchors, variable/units, algorithm surfaces/spec, branch/guard table, invariants, alias map, constants, tolerances, test vectors, and gap register.

3. Algorithm and branch table updated for changed behavior: `met`
- Static: annual/perennial transition-control families and guard semantics are explicitly specified.

4. Guard/error mapping aligned with implementation: `met`
- Static: runtime errors `HS-RUNTIME-E-046..051` map to PL11 guard classes.
- Ran: typed error assertions added in integration conformance tests.

5. Test-vector obligations reflected in executable tests: `met`
- Ran: PL10b conformance tests for annual extension projection, cutday projection, grazing payload projection, invalid window reject, and empty cardinality reject all pass.

6. Pre-implementation contract gate captured before production edits: `met`
- Ran: failing pre-implementation gate executed and recorded in `pl11-preimplementation-contract-gate.md`.
