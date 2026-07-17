# Independent Review A — Process Law and Numerics

**Evidence class:** Static and Ran
**Disposition:** HOLD

The production kernel matches the Jolly generalized growing-season-index law
and the FAO-56 daylight equations. Its units, signed-latitude convention,
polar clamp, typed failures, exact 21-sample FIFO implementation, and
deterministic replay posture are consistent with the amended process law.
Closure is held by canonical metadata drift and incomplete contract-test
evidence, not by a detected production-equation defect.

## Findings

### High — Canonical contract metadata does not identify revision 21

`docs/specifications/science-contracts/contracts/SC-PLANT-001.md:7` still
declares `contract_version: 20`, and line 19 still declares
`last_reviewed: 2026-07-13`. The same contract records the CP-GSI01 amendment
as revision 21 dated 2026-07-17 at line 804, while the package and contract
index also identify revision 21/current review on 2026-07-17. This makes the
canonical authority internally inconsistent and leaves `CP-GSI-001` without
an unambiguous version anchor.

**Required disposition:** Update the contract front matter to version 21 and
the 2026-07-17 review date, then rerun the contract/document checks.

### High — Required product and FIFO vectors are not fully demonstrated

The contract requires a product vector with three nontrivial indicators and
independently reconstructed output
(`docs/specifications/science-contracts/contracts/SC-PLANT-001.md:767`) and
explicit first-admission, 20-to-21-fill, and 21-sample-eviction vectors with
independent means (lines 769–770).

- `crates/openwepp-plant-phenology/src/lib.rs:404` exercises nontrivial
  temperature and VPD indicators, but photoperiod is exactly 1.0 at line 415.
  The asserted 0.25 product therefore does not satisfy the required
  three-nontrivial-indicator vector.
- `crates/openwepp-plant-phenology/src/lib.rs:451` proves one-sample warm-up and
  a full-window mean, but does not assert the 20-to-21 transition. Because all
  21 pre-eviction values are identical, the history assertion at line 471
  cannot distinguish correct oldest-member eviction from eviction of another
  member.

**Required disposition:** Add an independently calculated daily vector in
which all three indicators are strictly between zero and one. Add a
distinct-value FIFO history that explicitly checks the 20-sample mean, the
21-sample mean, and eviction of a uniquely identifiable oldest value, with
each expected mean reconstructed independently of the production helper.

### Low — FAO-56 tests lack an absolute published numeric anchor

`crates/openwepp-plant-phenology/src/lib.rs:475` verifies hemispheric phase and
line 488 verifies polar limits, but no test asserts an independently evaluated
FAO-56 daylight value at an ordinary latitude and day. The source is statically
faithful to FAO-56 equations 24, 25, and 34; an absolute reference vector would
better protect the constants, day phase, degree-to-radian conversion, and hour
conversion from a mutually consistent regression.

**Recommended disposition:** Add at least one documented FAO-56 numerical
example or independently calculated ordinary-latitude reference value with an
explicit tolerance.

## Verified Strengths

- Jolly thresholds at `crates/openwepp-plant-phenology/src/lib.rs:36` match
  -2/5 degrees C, 900/4100 Pa, 10/11 hours, and a 21-day window.
- Indicator interpolation and multiplication at lines 211–240 match Jolly
  equations 1–4, including the decreasing VPD response.
- FAO-56 declination, sunset-hour angle, and maximum-daylight equations at
  lines 251–268 preserve signed latitude and use a bounded geometric clamp for
  polar day/night.
- Forcing and parameter domains fail with typed errors at lines 47–73,
  92–106, and 320–370; invalid values are not silently normalized.
- The state transition at lines 180–195 removes the oldest value only when the
  21-sample window is full, admits the new daily value, and divides by the
  actual retained count. Static inspection found no synthetic-zero warm-up.
- Bit-identical replay is exercised at lines 559–582. The crate remains an
  isolated process kernel and does not violate the SC-PLANT-001 integration
  hold.

Primary equation checks used the [Jolly, Nemani, and Running 2005 paper](https://www.frames.gov/documents/catalog/spa/jolly_nemani_running_2005.pdf)
and the [FAO-56 extraterrestrial-radiation and daylight equations](https://www.fao.org/4/x0490e/x0490e07.htm).

## Execution Evidence

Ran:

```text
cargo nextest run -p openwepp-plant-phenology --profile quick
```

Result: PASS, 7 tests passed, run ID
`aed93d1a-3456-4105-a1ae-39fa72a22c5e`.
