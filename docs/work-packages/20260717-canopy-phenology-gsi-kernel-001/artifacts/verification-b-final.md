# Final Terminal Verification B

Evidence class: `Static`, `Ran`, and retained `Ran`

Disposition: `PASS`

Verification subject: corrected terminal-current working tree based on frozen
base `45d49090214b4702d11a04aafe5d5ccade7ba440`. Final Verifier B preserved all
existing artifacts and did not rerun full-workspace coverage.

## Checks Performed

- Inspected the corrected public restart integration test, the unit-test
  binary, public GSI state/restoration API, initial Review B and verification
  findings, producer disposition, gate artifacts, and CP-GSI01 contract hold.
- Ran package Nextest with the quick profile: 13/13 passed across two binaries,
  run `dfa58601-1285-433f-88a5-2a1f8f0e34a2`.
- Ran strict package Clippy, `cargo fmt --check`, and `git diff --check`; all
  passed.
- Independently reconstructed the retained instantaneous-GSI sequence used by
  `tests/restart.rs`.
- Recomputed the final CRAP/report/LCOV/manifest/registry hashes and the current
  production-source and restart-test hashes.
- Confirmed `source-manifest-final.json` contains the current hashes for both
  `src/lib.rs` and `tests/restart.rs`.
- Repeated the Rust reverse-consumer search. No canopy, biomass, litter, snow,
  ET, erosion, or assurance consumer reads the new process-kernel result.

## Restart Hold Closure

The prior uniform-zero FIFO hold is closed:

- `tests/restart.rs:7` now uses latitude `0.0`, so photoperiod is unconstrained
  for this vector rather than forcing every January GSI value to zero.
- Lines 19-23 admit 25 consecutive forcing days through the public `advance`
  API, leaving a full 21-sample FIFO.
- Lines 25-31 obtain history through the public accessor and explicitly require
  at least one adjacent pair with different bit patterns before restoration.
- Independent equation reconstruction gives 21 distinct retained values, 20
  nonzero values, a range from `0.0` to approximately
  `0.4165714285714284`, and a nonzero day-26 instantaneous GSI of
  approximately `0.40375000000000005`.
- Lines 32-34 restore the full nonuniform FIFO and newest date through public
  `try_from_history`, then assert exact state equality.
- Lines 36-44 admit the same next consecutive forcing day to restored and
  uninterrupted states and assert exact result and final-state equality.

This is a positive, public, non-aliasing anchored-restart/continuation vector.
It closes initial `VB-01` and repeat finding `VRB-01`.

## Finding Closure Audit

| Finding | Final verification | Evidence |
| --- | --- | --- |
| `B-01` traceability | `CLOSED` | Guard Map rows cover `INV-PLANT-028..032`; Symbol Alias Map rows cover forcing, parameters, indicators, result, FIFO, and date anchor. |
| `B-02` warm-up authority | `CLOSED` | The contract separates the published 21-day law from openWEPP's available-real-sample cold-start inference and restart policy. |
| `B-03` chronology/restart | `CLOSED` | Year-aware chronology, fail-before-mutation admission, anchored restoration, rollover tests, and the corrected positive nonuniform restart vector all pass. |
| `B-04` contract vectors | `CLOSED` | Unit tests retain the three-nontrivial-indicator product and independent first/20/21/eviction means. First admission is explicit at `lib.rs:623-625`. |
| `B-05` backlog truthfulness | `CLOSED` | Increment 3 owns canopy/litter/consumer integration and fixed-date replacement; GSI/FAO-56 choices are resolved. |
| `B-06` terminal evidence | `CLOSED` | Retained terminal evidence reports workspace Nextest 2,085/2,085, strict workspace Clippy, dependency-policy PASS, and fresh CRAP PASS with zero actionable rows. |
| `VB-01` / `VRB-01` | `CLOSED` | The external test now proves public anchored restoration and next-day equivalence with a demonstrably nonuniform full FIFO. |

## Terminal Evidence And Identity

`heavy-gates.md` records final full-profile Nextest as 2,085/2,085 across 192
binaries, run `3776d7c5-a5b6-4cdd-908c-c3320eeca8cc`, plus workspace Clippy
and `cargo deny check` PASS. Those expensive gates were retained rather than
rerun in this verification.

The heavy-gate identities match the actual final artifacts:

- workspace CRAP input JSON:
  `93c85e3c8c710e7460f612ee114a53dbf373bfc9bc11df94c0580c38800a01de`;
- adjudicated report JSON:
  `5a2f6762dca30c57ba06e720a7d5b77120a6cdcf502a41c25361ff6c855d369a`;
- workspace LCOV:
  `694152d3359cd50134c3702603dc945338922176ea31e736498526d541f035cb`;
- final source manifest:
  `8b446a40a7eca942f62047961910219b93c2511f2318a7deff571d7d1a2523ac`;
- adjudication registry:
  `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.

The current production source hash is
`53c50514fb13881983737f24125f0216aff45fb46b0dfb2a0c6a97b58e7c4243`;
the current public restart-test hash is
`ba962cb36c2ea6d5b627b4a90390dd87da5386de0d6fee4eaeaee447075f47fb`.
Both exactly match their entries in the final source manifest. The adjudicated
report is fresh, closure-eligible, and `PASS`: raw 2, adjudicated 2, actionable
0, touched actionable 0, and untouched actionable 0.

Line counts remain below governance thresholds: 920 lines for production
`lib.rs`, 45 for the public restart test, and 854 for `SC-PLANT-001`.

## Claim Ceiling And Conclusion

The source boundary, reverse-consumer search, CP-GSI01 integration hold,
roadmap, and package terminal language consistently limit the maximum claim to
`PASS-PROCESS-KERNEL`. This verification does not support an integrated-canopy,
empirical-validation, snow-model, assurance, or release claim.

Final Terminal Verification B is `PASS`. All initial Review B and terminal
verification findings are closed on the hash-identified terminal snapshot, the
first-admission and nonuniform anchored-restart obligations pass, terminal
evidence is current, CRAP has no actionable rows, and scope remains properly
contained.
