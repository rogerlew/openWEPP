# Review Finding Disposition

Evidence class: `Static` and `Ran`

All findings from Independent Reviews A and B were accepted. Terminal gate
closure remains separately recorded in `heavy-gates.md`.

| Finding | Disposition | Evidence |
| --- | --- | --- |
| A metadata | Fixed | `SC-PLANT-001` front matter now identifies revision 21 and review date 2026-07-17. |
| A/B product and FIFO vectors | Fixed | Crate tests use three nontrivial indicators and heterogeneous values with independent means at samples 20, 21, and post-eviction. |
| A FAO-56 anchor | Fixed | Added an absolute 20 degrees south, ordinal-day 246 daylight vector. |
| B-01 traceability | Fixed | Added Guard Map rows for `INV-PLANT-028..032` and Symbol Alias Map rows for every public GSI symbol/state surface. |
| B-02 warm-up authority | Fixed | `INV-PLANT-029` and CP-GSI01 now distinguish the published 21-day window from openWEPP's available-real-sample cold-start inference and explain its use. |
| B-03 chronology/restart | Fixed | Added `GsiDate`, Gregorian date validation, consecutive admission, year rollover, newest-date state, anchored restoration, typed errors, and tests. |
| B-05 backlog | Fixed | Fixed-date litter replacement is assigned to Increment 3; selected GSI/daylight design choices are marked resolved; integration questions remain open. |
| B-06 terminal evidence | Fixed | Focused tests pass 12/12; final full workspace tests pass 2,084/2,084; dependency policy passes; fresh adjudicated CRAP reports zero actionable rows. See `focused-gates.md` and `heavy-gates.md`. |

No finding was rejected, deferred outside the package, or used to broaden the
process-kernel claim into canopy integration.

## Initial Terminal-Verification Findings

Both terminal verifiers held the first terminal snapshot because it lacked a
successful anchored-restart vector; Verifier A also identified a missing
first-admission assertion and a mislabeled JSON hash. All were accepted:

- `tests/restart.rs` now reconstructs a heterogeneous full FIFO through public
  `history()` and `last_date()` accessors, then proves bit-identical result and
  state after the next consecutive day;
- the FIFO vector again asserts the first admission, in addition to samples 20,
  21, and post-eviction; and
- `heavy-gates.md` distinguishes the workspace CRAP input JSON from the
  adjudicated report JSON and records both exact hashes.

Focused, full-workspace, dependency-policy, and fresh CRAP gates were rerun
after these changes. Repeat terminal verification is required before closure.

The first repeat verification correctly found that the structurally varied
meteorological sequence still produced a uniform zero GSI FIFO because 44
degrees north in January was below the photoperiod threshold. That finding was
accepted. The public restart vector now uses equatorial photoperiod and asserts
that at least one adjacent retained GSI pair differs before restoration. The
focused, full-workspace, dependency-policy, and fresh CRAP gates were again
rerun on that final manifest. Final repeat verification remains required.
