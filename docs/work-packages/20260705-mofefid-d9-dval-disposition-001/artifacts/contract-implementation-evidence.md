# Contract Implementation Evidence

Status: executed
Evidence mode: Static

## Contract Amendment

Static:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
  advanced from rev 16 to rev 17.
- `last_reviewed` moved to `2026-07-05`.
- `INV-OFEROUTE-011` now records the D9 current status:
  Cases 1-3 re-run and dispositioned; Zone taxonomy run and passing;
  Case 4 isolated to `GAP-OFEROUTE-005`.
- The invariant guard map now names the Figure 9 taxonomy harness and records
  `[DIRECT][Ran]` evidence.
- The Binding Exposure Index activation-validation row now states that D9
  closes the non-numerics surface while `INV-011` remains open through Case 4.
- `GAP-OFEROUTE-005` now includes the exact D10 handoff and explicitly excludes
  Cases 1-3 and Zone taxonomy from D10 ownership.
- Revision history gained the rev 17 D9 row.
- `docs/specifications/science-contracts/index.md` updates the registry
  `SC-OFEROUTE-001` `last_reviewed` date to `2026-07-05`.

## Profile / BEI Impact

Static:

- No new production runtime binding or publication surface was added.
- No unit-governance rows changed; the new harness reads already-normalized
  supplemental `I*`, `Q*`, `S*`, and `Psi*` derived columns.
- BEI status remains active/prospective as before; the amendment only narrows
  the `INV-OFEROUTE-011` open blocker to Case 4 / `GAP-OFEROUTE-005`.
- BEI lint returns global `PASS-DEFERRED` because existing Lane D rows remain
  `science-review-follow-on`; D9 does not close those rows or claim full BEI
  consolidation. The D9 current-scope BEI check is recorded in
  `artifacts/gate-results.md`.
