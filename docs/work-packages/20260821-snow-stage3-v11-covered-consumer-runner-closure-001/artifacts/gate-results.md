# Gate results

Status: `IN PROGRESS / HOLD`.

`Static:` The predecessor `EXECUTED HOLD` remains untouched. The new provider
bind rejects non-48 days, mismatched per-interval GSI receipt, incomplete or duplicate
destination sets, mismatched interval receipt, WB14 identity, parcel identity,
destination identity, or half-open support bounds. The parent support is exactly
1,800 seconds. Pre-execution validation joins the prepared beginning GSI state
and cursor to the committed owners and joins every lane's destination set to
its bound OFE. The V11/coupled-time
beginning owner must contain the canonical ordered Stage-3 snow envelope.

`Static:` The prepared capability now binds provider day-relative receipts to
run-relative supports. Parent 0 through 47 cover exactly one 86,400-second
day; the next day begins at 86,400 seconds. A per-support forcing digest is
derived from the sealed GSI, provider interval, lane/OFE/tile, exposure, WB14,
and precipitation identities and is used for ParentAuthority, StepConstraint,
and CoupledClockState construction. Parent construction does not pre-bind an
unknown future forcing receipt.

`Static:` The snow-present rejection in
`DirectSnowStage3V11Attachment`/`DirectV11RealConsumerStack` remains present.
Covered supports use the distinct `DirectV11SnowCoveredSegmentInput` and
`DirectV11SnowCoveredRealConsumerStack`; they do not enter the snow-free
adopter. The covered adopter derives carrier operands from committed V11 and
Stage-3 beginning state, emits one carrier receipt per lane, and supplies the
same exact Stage-3 boundary receipt to the Stage-3 evaluator. The V11 side
selects `CoveredColumnAuthority::V11SnowCovered`, and the LSE solver now has an
    explicit lower-boundary branch that suppresses covered ground water, sensible,
    soil-storage, and WB14-facing operators. The radiation projection now selects
    Stage-3 snow VIS/NIR albedos before the two-stream solve and emits a typed
    band/direction optical receipt that feeds the Stage-3 boundary on the final
    pass. The transaction payload now uses an explicit Stage-3 lower-boundary
    variant instead of carrying a generic ground operand for V11. Covered
    latent mass/energy is bound exactly, and weighted energy tolerance uses
    primitive Stage-3 boundary magnitudes. A provisional/final reciprocal-
    longwave correction now makes the persistent covered case close. Released
    precipitation-advection and soil-coupling custody, keyed heterogeneous
    per-tile physical LSE consumption, and independent ledger closure remain
    blockers. The bounded covered fixed-point loop is now implemented, but its
    heterogeneous physical consumer remains open. This is an incremental custody
    lift, not a passed full lower-boundary claim.

`Static:` Persistent support acceptance checks the Stage-3 result against the
carrier sensible, latent, vapor, longwave, advected, and ending-ice values,
rejects terminal events on the persistent branch, and rejects a partial or
non-active Stage-3 result. Carrier receipts are retained per lane and an exact
`(OFE, tile)` receipt set is constructed for covered destinations. The V11
solve still consumes a parent-level aggregate carrier state for shared canopy
forcing; keyed destination receipts are now area-weighted into the lane
Stage-3 boundary, while per-tile physical LSE consumption remains open.
Prepared supports contain only sealed covered forcing and do not expose live
carrier surfaces or carrier ledgers. The new support, configuration, V11, and
carrier forcing digests use explicit typed framing and fixed-width/f64-bit
fields.

`Ran:` focused Nix-provided `cargo check` for
`openwepp-land-surface-energy` and `openwepp-hillslope-orchestrator` passed;
land-surface-energy lib tests passed 65/65, including the typed weighted-OFE
decomposition and Stage-3 primitive tolerance evidence. The orchestrator lib
suite passed 747/747 executed tests with 0 failures and 1 deterministic
support-domain test ignored (748 total), including the formerly ignored
persistent covered support/rollback case. The focused covered persistence
case passed with positive shortwave, exact Stage-3 optical/latent joins, and
weighted OFE closure. The Stage-3 boundary receipt has a direct poison test
for latent mass-energy mismatch.

`Ran:` warnings-denied library Clippy remains non-clean on pre-existing
dead-code, large-enum, precision, and scheduler/attachment findings in
historical direct-runtime paths. The covered change's added argument-count
diagnostic is explicitly scoped at its existing large constructor; no other
new diagnostic remains in the land-surface-energy or covered carrier changes.
Formatting and `git diff --check` pass for the current worktree.

The release gates remain open: covered V11 executor, runner-owned physical
support construction, terminal-liquid consumption, additive restart, scenario
matrix, and independent reviews are not yet dispositioned.

| Gate | Status | Evidence |
| --- | --- | --- |
| contract/operand mapping | `PASS` | `covered-consumer-contract-map.md` |
| covered executor | `IN PROGRESS / HOLD` | explicit covered lower-boundary seam and keyed destination receipts exist; released Stage-3 energy custody, keyed physical solve, fixed point, ledger, physical matrix, and terminal branch remain |
| runner 48-support capability | `IN PROGRESS` | provider bind implemented; support construction pending |
| terminal liquid exact once | `BLOCKED` | `event-and-terminal-liquid-lineage.md` |
| complete restart | `BLOCKED` | `restart-schema-and-equivalence.md` |
| physical scenarios | `NOT RUN` | `scenario-matrix.md` |
| reviews/verifiers/exact-head | `NOT RUN` | to be appended |

## Checkpoint amendment: converge and seal keyed covered boundaries

`Static:` The covered path now constructs a `FinalStage3CanopyBoundaryReceiptV1`
per `(OFE, tile)` after the optical and reciprocal-longwave values are known.
The final receipt binds the beginning V11 and Stage-3 digests, provisional
carrier digest, optical digest, reciprocal-longwave digest, accepted exchange
terms, and its own canonical digest. Stage-3 boundary operands, covered-column
operands, the sealed snow-owner bytes, and the retained final receipt map carry
the final identity. Provisional solves retain no final receipt identity.

`Static:` The covered loop restarts every LSE and Stage-3 candidate from the
immutable beginning owner set. It is bounded by 32 iterations and compares
keyed canopy-air temperature/humidity, snow temperature and exchange terms,
snow latent flux, snow net longwave, component canopy temperatures, and the
Stage-3 candidate fingerprint. A nonconvergent loop returns the typed
`FixedPointIterationLimit` error before any staged ending or last-receipt field
is published. The final unsealed candidate is rerun, sealed, rerun again, and
must self-reconstruct the accepted boundary exactly.

`Static:` Destination carrier receipts are area-weighted by the exact surface
configuration tile fraction into one lane Stage-3 boundary. The prior
`CoveredTileGround` evidence is superseded by the prospective Option-A
authority: the only admitted basis is OFE ground, no covered-fraction
normalization is performed, and an incomplete snow-surface tile set fails
closed. Keyed destination receipts remain available
for final receipt joins. Physical LSE forcing now consumes the matching keyed
carrier receipt for each covered destination; no parent aggregate carrier is
used as the physical LSE forcing source.

`Static:` `CoveredTileEnergyOperandSet::validate()` now cross-joins the final
Stage-3 lower-boundary representation with the covered-column shortwave,
canopy-air, longwave, boundary-energy, and receipt-identity representations.
Independent closure remains active for provisional predictors; a sealed final
receipt requires all exact joins. One-bit optical/receipt and longwave poison
tests cover the fail-closed joins.

`Ran:` `nix develop --command cargo check -p openwepp-land-surface-energy -p
openwepp-hillslope-orchestrator` passed. `nix develop --command cargo test -p
openwepp-land-surface-energy --lib` passed 66/66. `nix develop --command cargo
test -p openwepp-hillslope-orchestrator --lib -- --test-threads=1` passed
750/750 executed tests with one historical deterministic support-domain test
ignored. The focused
covered persistent test and the final-receipt poison test passed. Formatting
and `git diff --check` passed.

`HOLD:` No heterogeneous two-unequal-tile physical integration fixture,
longwave-only/sublimation-only/positive-shortwave scenario matrix, explicit
nonconvergence fixture, independent snow outcome ledger, canopy rain
interception/throughfall-stemflow custody, snow-soil heat receipt, runner-owned
48-support construction, terminal chronology, or additive restart closure is
claimed by this amendment. Child 3 remains blocked.

## Checkpoint amendment: correct candidate feedback and lane lineage

`Static:` The keyed carrier is rebuilt from each iteration's candidate V8
canopy state and candidate Stage-3 state, and matching keyed carriers are fed
to each covered physical LSE tile. The accepted Stage-3 pass consumes the
sealed lane boundary values; provisional carrier identities are not reused as
final physical flux identities.

`Static:` Lane receipts bind ordered destinations, exact tile fractions,
topology, provisional/destination/lane receipt identities, and the explicit
OFE-ground area basis. The receipt now uses a versioned framed digest rather
than JSON serialization. Final receipt validation includes snow
temperature, latent heat, support duration, latent mass/energy, and ending
V11/Stage-3 joins. Partial final identity states are structurally rejected.

`Static:` The reviewed convergence policy is 32 maximum iterations with
separate absolute/relative tolerances for temperature, humidity, sensible or
longwave flux, vapor flux, and Stage-3 state components. `FixedPointIterationLimit`
rolls back before staged output fields are populated.

`Ran:` The final focused evidence is 66/66 land-surface-energy tests, 750/750
serialized orchestrator library tests with one deterministic ignored test, four
handoff receipt tests, a passing persistent covered regression, passing cargo
check, passing formatting check, and passing `git diff --check`.

`HOLD:` Precipitation custody, snow-soil heat, independent outcome-ledger
closure, heterogeneous physical scenario matrix, runner construction,
terminal liquid, restart, and reviews remain open.
