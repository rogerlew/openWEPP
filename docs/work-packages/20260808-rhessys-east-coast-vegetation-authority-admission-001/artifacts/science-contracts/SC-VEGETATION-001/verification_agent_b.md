# Verification Agent B: SC-VEGETATION-001

Status: `complete`

Date: 2026-08-08 UTC

Evidence mode: `Ran + Static`

Verified contract SHA-256:
`7e62cf907eb328ad1b1aaf535ab1556896f686c7d0a8e01ed22a6ce81d635f7a`

Disposition source:
`artifacts/science-contracts/SC-VEGETATION-001/disposition.md`

## Closure Check

- `B-01`: `closed` — concrete primary-source routes and residual blockers are
  recorded in `artifacts/authority-route-attempts.md:7-38`.
- `B-02`: `closed` — partial observations and the precise selected
  stand/date/topology blocker are separated at
  `artifacts/schema-profile-initial-state-gate.md:61-76`.
- `B-03`: `closed` — the ledger contains exactly 71 rows for both profiles; an
  independent comparison to pinned `vegCollection.csv` found zero key/value
  mismatches.
- `B-04`: `closed` — source/contract alias authority precedes implementation
  proof at `artifacts/schema-profile-initial-state-gate.md:42-47` and
  `SC-VEGETATION-001.md:372-380`.
- `B-05`: `closed` — wet-canopy `gsurf_*` rows are consumed and remain
  `REJECT/HOLD` at `artifacts/selected-field-ledger.md:89-90`.
- `B-06`: `closed` — `AUTH-RHEC-001` is limited to schema-form partial admission
  at `SC-VEGETATION-001.md:358-361`.
- `B-07`: `closed` — the no-value-admission sentence is complete at
  `SC-VEGETATION-001.md:346-351`.
- `N-01`: `closed` — active canopy-snow, fixed-point/fallback, invariant,
  tolerance, and test-vector clauses explicitly preserve the prohibitions
  through version 3. The remaining version-2 reference in BEI-002 is correctly
  historical.
- `N-02`: `closed` — B-05 review and disposition references identify ledger
  lines 89-90.

No finding remains open. No finding was rejected, deferred, or waived, so no
rejected-finding rationale required validation. The disposition matches the
final canonical bytes and evidence.

## Direct Checks

The verifier reran the focused contract suite (`9/9`), SC unit-compliance check,
strict Binding Exposure check (`3/3`), and `git diff --check`; all passed. No new
regression was found.

## Verdict

`PASS`
