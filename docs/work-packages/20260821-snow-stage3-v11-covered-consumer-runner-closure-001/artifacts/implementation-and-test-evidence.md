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

`Ran:` the current focused `nix develop --command cargo check -p
openwepp-land-surface-energy -p openwepp-hillslope-orchestrator` passed, with
the known historical direct-runtime dead-code warnings.

`Ran:` `nix develop --command cargo test -p
openwepp-hillslope-orchestrator --lib` passed: 745 passed, 0 failed, 2
ignored (747 total). The persistent covered V11/Stage-3 shared-carrier
integration/rollback test is intentionally one of the ignored tests because
released Stage-3 shortwave/soil boundary custody is incomplete. An explicit
`--ignored` run fails closed at `ControlVolumeClosure("weighted_ofe_energy")`;
this suite result is not a covered-physics closure claim.

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

`Ran:` the earlier six-package warnings-denied Clippy command remains blocked
by historical Stage-3 shadow dead-code/precision debt, pre-existing attachment
size/argument/precision debt, and scheduler/evaluator debt; all-target test
compilation also reports existing test-target line-count/float-comparison
findings. The current focused Clippy rerun reports the same historical classes
and no new lower-boundary diagnostic. No broad lint allowance was added.

`Static:` This checkpoint removes `SharedCarrierInput` and its caller-supplied
ledger from prepared covered supports. `SealedCoveredCarrierForcing` contains
only sealed atmosphere, exposure, participant, and support-receipt operands;
`DirectV11SnowCoveredRealConsumerStack::derive_live_carrier_input` derives
canopy and snow surfaces, conductances, longwave components, and the current
carrier input from committed V11/Stage-3 beginning owners. The carrier-side
ledger is still assembled before Stage-3 execution; a complete independent
candidate-outcome ledger remains open.

`Static:` The carrier receipt is now the Stage-3 lower-surface boundary. The
Stage-3 evaluator consumes `Stage3SnowSurfaceBoundaryReceiptV1`, and the
covered adopter independently compares sensible, latent, vapor, longwave,
advected, evaluated-duration, lifecycle, and ending-ice results before it
stages any V11 candidate. Per-lane receipts and an exact covered destination
`(OFE, tile)` receipt map are retained; the V11 projection still uses a
parent-level aggregate for the physical solve. The aggregate runtime receipt
digest also uses typed framing rather than JSON.

`Static:` `CoveredColumnAuthority::V11SnowCovered` is selected on the covered
candidate and the snow-free guard remains unchanged. The lower-boundary
implementation now has an explicit covered branch that holds covered ground
water, ground sensible/vapor, soil storage, and WB14-facing state, but it is
not closure-complete: canonical Stage-3 shortwave, precipitation advection,
soil coupling, fixed-point iteration, and independent outcome-ledger custody
are still open. No claim is made that Stage 3 is already the sole lower-surface
heat/vapor/radiation owner on the V11 side.

`Ran:` after the lower-boundary refactor, land-surface-energy lib tests passed
63/63, the orchestrator lib suite passed 745/745 executed tests with 0
failures and 2 ignored, and focused `cargo check` passed. The persistent
covered integration test is intentionally ignored with the reason
`covered V11 energy closure still needs released Stage-3 shortwave/soil
boundary custody`; its explicit run fails at
`ControlVolumeClosure("weighted_ofe_energy")`, and it is not a passing
covered-physics claim. Warnings-denied
Clippy remains blocked by pre-existing direct-runtime shadow and scheduler
debt, while the newly changed lower-boundary code has no additional Clippy
diagnostic.

`Ran:` the focused covered regression also mutates the committed Stage-3
beginning temperature and verifies that the derived carrier receipt changes;
the existing sealed-exposure poison still fails before staged owners are
retained.
