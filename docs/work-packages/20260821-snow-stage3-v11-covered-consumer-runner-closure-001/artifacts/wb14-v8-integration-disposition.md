# WB14 v8 integration disposition

Date: 2026-08-23

Disposition: `EXECUTING / HOLD`.

Implemented and verified in the prospective increment:

- immutable scalar WB14 OFE/lane/configuration/model/parameter identity;
- proposal-versus-accepted-support validation and truncated scalar vectors;
- reconstructable scalar child and parent receipts;
- parent-local complete surface-liquid candidates with final-only persistent
  cursor publication;
- complete-owner beginning/ending chain checks;
- exact parcel/support partition plumbing and a coupled hard-boundary seam;
- one-child production-owner parity and focused 2x900/30x60 kernel/candidate
  tests.

Fresh independent hydrology and Rust reviews both returned `HOLD`. The live
complete-owner path does not yet consume the sealed
`DirectWb14ParentIntervalV1` receipt authority, and therefore does not bind and
reconstruct the exact coupled-parent receipt, selected proposal, child ordinal,
predecessor chain, and staged-owner digests in one production receipt chain.
The actual seven-owner 30x60, dynamic cadence, nonzero forcing remainder,
integrated substitution, and required rollback fixtures are also incomplete.

No finding is waived. `SC-SURFACELIQUID-001@8` remains `in_review` / `draft`,
v7 remains released authority, and the production 900/60/non-1800 pre-physics
guard remains in force.

Ran on the terminal worktree:

- orchestrator library: 779 passed, 1 skipped;
- surface-liquid contract: 11 passed;
- required-suite obligation guard: 3 passed;
- authority-suite anti-evasion guard: passed;
- formatting and `git diff --check`: passed;
- warnings-denied Clippy: blocked by four pre-existing
  `openwepp-land-surface-energy` lints outside this increment.

Next required lift: replace the live ad-hoc parent working identity with a
multi-OFE production coordinator that owns the scalar receipt authorities and
binds their ordered receipts into each complete-owner child join, then add the
remaining integrated fixtures before repeating dual review and promotion.
