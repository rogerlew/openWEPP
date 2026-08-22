# Implementation and test evidence

Status: `IN PROGRESS / CHILD-1 HOLD RETAINED`.

No implementation or test closure is claimed by the scaffold. Append exact
commands, selected tests, result counts, and source-level consumer proof as
each phase lands.

`Static:` The exact-one snow custody and provider binding seams are now
implemented. Prepared-day fields and support identities are private, provider
binding returns an opaque validated capability, and destination coverage is
checked against every provider receipt. Terminal liquid uses a uniform
tile-ground depth basis and independently reconstructs the OFE-ground mass.
The parent support identity is now exactly 1,800 seconds
(`1_800_000_000_000` ns). Sequential provider validation joins the prepared
day to the committed beginning GSI state and cursor rather than requiring the
new receipt to equal the prior day receipt. Validated-day preflight also joins
each lane's complete destination set to its bound surface-liquid OFE; a lane
permutation fails before any Stage-3/V11 transition.
`Static:` `DirectV11SnowCoveredRealConsumerStack` is now a distinct typed
adopter beside the existing `DirectV11RealConsumerStack`. Its separate
`DirectV11SnowCoveredSegmentInput` prevents covered supports from entering the
snow-free interval type. For a persistent covered support it evaluates the
shared Child-2C carrier and the actual Stage-3 persistent support from the same
beginning states, then uses the common V11 resource/owner/finalization path to
stage canonical snow and V11-related endings. The snow-free snow-present guard
remains unchanged. Terminal event chronology, runner construction, and
complete-day installation are still open.

`Static:` Prepared support bounds now use run-relative nanoseconds: day 0 is
`[0, 86,400 s)`, day 1 begins at `86,400 s`, and every parent uses the exact
sealed support rather than reconstructing a zero-based interval. Each parent
forcing identity is derived from the day/interval, accepted GSI receipt, V11
forcing receipt, and ordered lane/OFE/tile exposure, WB14, precipitation, and
provider interval identities. Covered supports add tagged Stage-3 support
forcing, Stage-3 configuration, covered-V11 forcing, and carrier-configuration
projections. Parent authority, constraint, and coupled-clock construction all
use that digest. The next parent is constructed only when its sealed support
is available; the committed clock retains the completed support at a day
boundary. The obsolete static generic forcing receipt is removed.

`Ran:` `nix develop --command cargo check -p
openwepp-hillslope-orchestrator -p openwepp-vegetation
-p openwepp-land-surface-energy -p openwepp-biogeochemistry
-p openwepp-persisted-restart-v1 -p openwepp-runner` passed, with the known
11 dead-code warnings in the historical Stage-3 shadow path.

`Ran:` `nix develop --command cargo test -p
openwepp-hillslope-orchestrator --lib` passed: 746 passed, 0 failed, 1
ignored (747 total); this includes the persistent covered V11/Stage-3
shared-carrier support and exact rollback test.

`Ran:` the focused covered test passed: 1 passed, 0 failed. It verifies one
exact 1,800-second support, shared-air/carrier receipt production, canonical
Stage-3 ending progression, complete V11 owner output, and rejection without
staged state after carrier failure.

`Ran:` `nix develop --command cargo test -p openwepp-hillslope-orchestrator
--lib snow_stage3_v11_attachment::tests` passed: 5 passed, 0 failed. The
focused provider-bound regression passed: 1 passed, 0 failed, including
day-0-to-day-1 capability binding, absolute support bounds, day-replay,
skipped-day, substituted-GSI-state, rewound-cursor, and cross-day support
poisons.

`Ran:` `nix develop --command cargo test -p
openwepp-climate-runtime-adapter` passed 8/8; `cargo test -p
openwepp-coupled-time` passed 13/13 across unit, authority, and frozen-oracle
targets; and `cargo test -p openwepp-vegetation --lib v11` passed 9/9.

`Ran:` `nix develop --command cargo test -p
openwepp-hillslope-orchestrator --lib --no-run` passed; `nix develop
--command cargo fmt --all -- --check` passed after formatting. `git diff
--check` passed.

`Ran:` the six-package warnings-denied Clippy command remains blocked: the
library targets report 27 findings consisting of historical Stage-3 shadow
dead-code/precision debt, pre-existing attachment size/argument/precision
debt, and existing scheduler/evaluator debt; all-target test compilation also
reports existing test-target line-count/float-comparison findings. The one
new needless-borrow finding from this increment was removed; no broad lint
allowance was added.

`Static:` This checkpoint removes `SharedCarrierInput` and its caller-supplied
ledger from prepared covered supports. `SealedCoveredCarrierForcing` contains
only sealed atmosphere, exposure, participant, and support-receipt operands;
`DirectV11SnowCoveredRealConsumerStack::derive_live_carrier_input` derives
canopy and snow surfaces, conductances, longwave components, and the carrier
ledger from committed V11/Stage-3 beginning owners.

`Static:` The carrier receipt is now the Stage-3 lower-surface boundary. The
Stage-3 evaluator consumes `Stage3SnowSurfaceBoundaryReceiptV1`, and the
covered adopter independently compares sensible, latent, vapor, longwave,
advected, evaluated-duration, lifecycle, and ending-ice results before it
stages any V11 candidate. Per-lane receipts are retained and the V11
projection uses deterministic tile-fraction aggregation. The aggregate
runtime receipt digest also uses typed framing rather than JSON.

`Static:` `CoveredColumnAuthority::V11SnowCovered` is selected on the covered
candidate and the snow-free guard remains unchanged. The lower-boundary
implementation is not yet closure-complete: the current covered-column
operator still evaluates the existing generic ground/litter/mineral surface
terms after that authority selection. No claim is made that Stage 3 is already
the sole lower-surface heat/vapor/radiation owner on the V11 side.

`Ran:` after the DTO and identity cleanup, `cargo fmt --all`, focused covered
execution, the full orchestrator lib suite (746 passed, 0 failed, 1 ignored),
and land-surface-energy lib tests (63 passed, 0 failed) passed. Warnings-denied
Clippy still fails on the pre-existing direct-runtime shadow and scheduler
debt; the changed carrier/Stage-3 code has targeted dispositions for its new
line/argument and numeric-conversion diagnostics.

`Ran:` the focused covered regression also mutates the committed Stage-3
beginning temperature and verifies that the derived carrier receipt changes;
the existing sealed-exposure poison still fails before staged owners are
retained.
