# Review - Feynman

Status: DISPOSITIONED
Evidence mode: Static. Ran: read-only `git status --short --untracked-files=all`
and `git diff --check`.

## Findings

### FYN-H1 - Active clamp guard was after row consumption/commit

Severity: High.

Finding: `laned_active_enforce_day_closure` originally ran after active rows
were built, consumed, dynamically published, and committed in
`direct_runtime/03_executor.rs`. The new clamp guard therefore was not proven
to fail before active publication consumers observed a bad day.

Disposition: Accepted.

Fix: `03_executor.rs` now routes all lanes for the day into local `day_frames`
and route books first, then calls `laned_active_enforce_day_closure` before any
row construction, row consumption, dynamic transfer publication, or frame
commit. Erosion/ledger and row publication still run in lane order after the
guard, preserving downstream erosion-inflow semantics.

### FYN-M1 - Package write set omitted active runtime files

Severity: Medium.

Finding: The package write set named `ofe_routing/**` but the implementation
edits live in `direct_runtime/laned_active.rs` and then
`direct_runtime/03_executor.rs`.

Disposition: Accepted.

Fix: `package.md` now explicitly includes both active runtime files.

### FYN-M2 - Contract test-vector/BEI surfaces did not name rev-40 guard

Severity: Medium.

Finding: Rev 40 added a new guard but the Test-Vector Obligations and BEI
tables did not name the new boundary cases.

Disposition: Accepted.

Fix: `SC-OFEROUTE-001` now adds an active clamp-source publication guard
test-vector row and an `OFEROUTE-ACTIVE-CLAMP-SOURCE-GUARD` BEI row. Focused
tests cover below/equal/greater-than and zero-source/nonzero-clamp behavior.

## Re-check

Static re-check by package owner after fixes:

- `git diff --check`: PASS.
- Focused active guard test: PASS.
- WA expected-fail harness: PASS_EXPECTED_FAIL at
  `laned_active_clamp_exceeds_source` for fixed10 and dx5.
