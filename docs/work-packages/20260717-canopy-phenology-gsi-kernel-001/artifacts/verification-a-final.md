# Final Terminal Verification A

Evidence class: `Static`, `Ran`, and retained `Ran`

Disposition: `PASS`

The corrected terminal snapshot closes the prior restart-vector hold. The
public integration test now retains a nonuniform 21-value GSI FIFO, restores it
with its newest date through the public API, and continues identically to the
uninterrupted state. The explicit first-admission vector remains present. The
refreshed heavy-gate identities match their target artifacts, and the final
source manifest binds both the production source and restart test exactly.

## Restart And First-Admission Verification

`crates/openwepp-plant-phenology/tests/restart.rs` now uses zero latitude, so
photoperiod is above the generalized 11-hour unconstrained threshold throughout
the day 1 through 26 vector. Temperature and VPD therefore produce distinct
instantaneous GSI values instead of the former all-zero history.

The test:

1. advances the uninterrupted state through 25 consecutive days, leaving a
   full 21-value FIFO;
2. explicitly requires adjacent retained values with different `f64` bit
   patterns;
3. restores through public `history()` and `last_date()` accessors;
4. asserts restored and uninterrupted state equality before continuation; and
5. admits the same consecutive day 26 to both states and asserts result and
   final-state equality.

Static inspection of `try_from_history` confirms that it copies each retained
`f64` without recomputation and preserves the supplied date anchor. Both states
then execute the same deterministic eviction, indicator, summation, and mean
path, so the nonzero continuation is bit-identical.

The unit test `moving_window_has_explicit_warmup_transition_and_fifo_eviction`
still asserts the first admission independently: sample count 1 and GSI 0.05.
It also retains the separate heterogeneous 20-, 21-, and post-eviction mean
checks.

## Gate And Identity Verification

The current `heavy-gates.md` records terminal full-profile Nextest as PASS:
2,085/2,085 tests across 192 binaries, five configured skips, 24 slow tests,
559.630 seconds, run `3776d7c5-a5b6-4cdd-908c-c3320eeca8cc`. It also records
workspace formatting, strict Clippy, dependency policy, and fresh adjudicated
CRAP as PASS.

Every recorded target identity was recomputed:

| Artifact | Recorded and actual SHA-256 |
| --- | --- |
| `target/adjudicated-crap/workspace-crap.json` | `93c85e3c8c710e7460f612ee114a53dbf373bfc9bc11df94c0580c38800a01de` |
| `target/adjudicated-crap/adjudicated-crap-report.json` | `5a2f6762dca30c57ba06e720a7d5b77120a6cdcf502a41c25361ff6c855d369a` |
| `target/adjudicated-crap/workspace.lcov` | `694152d3359cd50134c3702603dc945338922176ea31e736498526d541f035cb` |
| `target/adjudicated-crap/source-manifest-final.json` | `8b446a40a7eca942f62047961910219b93c2511f2318a7deff571d7d1a2523ac` |
| `target/adjudicated-crap/adjudication-registry.json` | `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f` |

The adjudicated report is fresh and closure-eligible with status PASS,
9,746 production entries, two raw rows, two adjudications, zero actionable
rows, zero touched actionable rows, and zero untouched actionable rows.

Current source identities also match the final manifest:

- `crates/openwepp-plant-phenology/src/lib.rs`:
  `53c50514fb13881983737f24125f0216aff45fb46b0dfb2a0c6a97b58e7c4243`;
- `crates/openwepp-plant-phenology/tests/restart.rs`:
  `ba962cb36c2ea6d5b627b4a90390dd87da5386de0d6fee4eaeaee447075f47fb`.

Ran focused package Nextest on the current snapshot: PASS, 13/13 across two
binaries, run `4904de61-61ee-4182-bff9-3f3f1e3d7689`. `git diff --check` also
passes. The full workspace coverage workflow was not repeated during this
read-only verification.

## Scope Ceiling

Cargo metadata finds no package that depends on `openwepp-plant-phenology`, and
source search finds no production consumer outside the new crate. The contract
integration hold remains intact: this package supplies a verified process
kernel only and does not support a canopy, LAI, biomass, litter, snow, ET,
erosion, or empirical-validation claim.

The supportable terminal disposition is `PASS-PROCESS-KERNEL`.
