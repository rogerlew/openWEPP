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
OFE-ground area basis. The receipt now uses the explicitly specified,
deterministic adopter wire rather than JSON serialization; it remains
prohibited from restart/parent authority until canonical framed adoption.
Final receipt validation includes snow
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

## Checkpoint amendment: OFE-ground authority and runtime basis

Ran at `cf178f5a41313dc71416e68e654a9aa71f72a51f`:

- `git diff --check`: `PASS` before commit.

Not run at exact HEAD `cf178f5a41313dc71416e68e654a9aa71f72a51f`:

- `cargo check`;
- Rust tests;
- `rustfmt`;
- Clippy;
- strict contract binding;
- assurance identity verification.

Reason: `cargo` and `rustfmt` were unavailable in that execution environment.
Prior results above remain historical increment evidence and are not exact-head
evidence for the enum rename, framed digest, changed validation, or new tests.

## V15 review-amendment exact-worktree qualification

Ran on the final in-review bytes before independent re-verification:

- `nix develop --command cargo fmt --all -- --check`: `PASS`.
- `nix develop --command cargo check -p openwepp-hillslope-orchestrator --tests`:
  `PASS` with one pre-existing dead-code warning.
- focused contract-derived v15 test: `PASS`, 1/1.
- focused OFE receipt, mixed aggregation, and mixed runtime fail-closed tests:
  `PASS`, 3/3.
- strict contract binding exposure: `PASS`, 14 rows.
- typed assurance `validate --all`: `PASS`.
- typed assurance `verify-generation --base-ref
  15763d7f6d5d4125333d9b7583424c714f5f5ea4`: `PASS`, generation
  `4151ae2aaacbd389f6ab163459f09aeb314fddd00d250fc8a817a432267dd12c`,
  86 transitions.
- `git diff --check`: `PASS`.

`Static:` The lane receipt constructor now requires an independently supplied
ordered topology expectation and rejects fresh-seal tile fraction, boundary
class, or boundary-model substitutions. The runtime expectation is projected
from the configured covered topology and the admitted covered-boundary model;
it cannot authorize an open-snow claim. Fresh-seal class and model poison tests
cover the join.

`FAIL (pre-existing):` Clippy with `-D warnings` remains blocked by existing
large-enum and `too_many_lines` findings outside this amendment. No passing
Clippy result is claimed.

## V15 approved/active promotion qualification

`Static:` Independent verification agents A and B each returned
`PASS-WITH-NOTES` on the amended in-review checkpoint. All accepted findings
closed before promotion. The package remains `EXECUTING / HOLD`.

Ran after promotion to `approved / active / 2026-08-22`:

- final typed assurance source adoption: `PASS`, transaction
  `d107bb417ecd0e340eb8e0a4b96477f751aff4a044093a7b88087536d9f63979`,
  generation `c88e6204e4f4fb5be440156d2764a63ae55646a55fcbcc563a98fe093522f182`;
- assurance `validate --all`: `PASS`;
- assurance generation verification: `PASS`, 87 transitions;
- strict contract binding exposure: `PASS`, 14 rows;
- science-contract admission against `cf178f5a`: `PASS`, `A0_ADMITTED`, 49
  contracts and 3 changed science surfaces;
- contract-derived approved-v15 test: `PASS`, 1/1;
- `cargo check -p openwepp-hillslope-orchestrator --tests`: `PASS` with one
  pre-existing dead-code warning;
- `cargo fmt --all -- --check`: `PASS`;
- `git diff --check`: `PASS`.

## Mechanical covered-consumer module split

`Static:` This checkpoint changes module ownership only. Covered carrier,
fixed-point, destination/lane receipt, Stage 3 boundary, and covered imported-
stack execution moved from `v9_real_consumer_shadow.rs` to
`v11_covered/mod.rs`; shared final-owner and V11 resource-lineage helpers moved
to `v11_covered/owner_finalization.rs`. No equation, guard, ordering, digest
preimage, or public type signature changed.

Governance line counts after extraction:

- `v9_real_consumer_shadow.rs`: 2,724;
- `v11_covered/mod.rs`: 2,189;
- `v11_covered/owner_finalization.rs`: 722.

Ran before and after extraction with the identical command:

- `nix develop --command cargo test -p openwepp-hillslope-orchestrator --lib
  -- --test-threads=1`: both runs `PASS`, 751 passed, 0 failed, 1 ignored, 752
  total.

Also ran after extraction:

- `cargo check -p openwepp-hillslope-orchestrator --tests`: `PASS` with the
  same pre-existing dead-code warning;
- `cargo fmt --all -- --check`: `PASS`;
- `git diff --check`: `PASS`.

## Component-resolved carrier and post-finalization owner lineage

`Static:` The final covered LSE candidate retains exact sunlit-leaf,
shaded-leaf, wet-canopy, and stem/WAI component area, emissive area,
conductance, temperature, humidity, sensible-flux, and vapor-flux operands.
Validation reconstructs every component turbulent flux and the reciprocal
longwave exchange from emissive-area weights. The sealed carrier independently
proves component-to-canopy, canopy-plus-snow-to-reference-atmosphere, and
snow-to-final-Stage-3 boundary equality.

`Static:` `CoveredParentOwnerJoinReceiptV1` is created only after the imported
V11 segment returns all final candidates. Its canonical framed seal binds the
final destination-boundary and component-carrier receipt sets, the Stage 3
physical-state digest, and the actual complete vegetation, snow, LSE,
hydrology, BGC, soil-thermal, and surface-liquid owner envelopes. Replay
validation reconstructs every identity from those final objects.

`Ran:` on the exact worktree based on `580986051bbb9213bfb0937d69d88da0f23eb387`:

- `cargo fmt --all -- --check`: `PASS`.
- `cargo check -p openwepp-land-surface-energy -p openwepp-hillslope-orchestrator --tests`: `PASS` with existing dead-code warnings.
- two-crate nextest regression: `PASS`, 818 passed, 0 failed, 1 skipped.
- final-owner substitution replay: `PASS`, 1/1.
- mixed open/covered incomplete-surface fail-closed regression: `PASS`, 1/1.
- `git diff --check`: `PASS`.
- direct science-contract admission: `PASS`, `A0_ADMITTED`, 49 contracts and 6 changed science surfaces.
- authority-suite anti-evasion guard: `PASS`.
- `auth11_required_suite_obligation_guards_contract`: `PASS`, 3/3.

`HOLD:` Open-snow production, per-field Stage 3 convergence authority,
precipitation and soil-heat custody, independent outcome-ledger reconstruction,
terminal chronology/liquid, additive restart, scenario closure, and final
reviews remain open.

## Authoritative component carrier and hardened owner joins

`Static:` Reduced `SharedCarrierReceipt` values are retained only as the first
unsealed boundary guess and provisional identity evidence. They no longer
replace sealed reference-atmosphere temperature/humidity, no longer gate fixed-
point acceptance, and are not consumed by Stage 3 after the first LSE solve.
The component-resolved LSE sensible/vapor boundary drives every subsequent and
final Stage 3 candidate and replay.

`Static:` Leaf vapor conductance is reconstructed directly from dry area,
boundary resistance, and stomatal resistance; wet conductance is wet area times
wet boundary conductance; stem vapor conductance is zero. A separately typed
wet-liquid authorization operand owns capped evaporation without reverse-
deriving conductance from accepted flux.

`Static:` Component receipts bind support, shared canopy-air state, final
boundary digest, optical digest, reciprocal-longwave digest, canonical ordered
component operands, and their own replayed seal. The outer join validates all
nested receipts, reconstructs exact complete snow-owner bytes, and binds run,
parent transaction, segment, accepted slab, forcing, and beginning complete-
owner lineage. The former `ending_v11_state_sha256` boundary field is renamed
to `ending_v8_physical_candidate_sha256`.

`Ran:` focused stale-inner-seal/fresh-boundary-substitution, valid alternate
snow-owner, and outer-owner digest poison tests all passed. The two-crate
nextest regression passed 820/820 with one intentional skip. Exact-worktree
qualification also passed:

- `cargo fmt --all -- --check`;
- two-crate `cargo check --tests`, with the pre-existing projection dead-code warning;
- direct science-contract admission: `A0_ADMITTED`, 49 contracts and six science surfaces;
- authority-suite anti-evasion guard;
- required-suite obligation guard contract: 3/3;
- `git diff --check`.

## Exact Stage-3 feedback and sealed covered replay increment

`Static:` Every LSE iteration merges snow temperature and temperature-derived
latent heat from the latest persistent Stage-3 candidate with the radiative and
turbulent fluxes from the preceding component-resolved LSE candidate. The
reduced evaluator and its predeclared outcome ledger are absent from covered
execution. A one-time `CoveredCarrierInitialGuessV1` contains only numerical
snow heat/vapor/longwave guesses and a diagnostic digest.

`Static:` Retained destination, component, lane, snow-owner, and parent-owner
receipts are rebuilt from the replay selected for installation. A sealed replay
requires exact LSE iteration-state, vegetation-ending, and Stage-3-ending
identity. Canonical snow-owner V3 bytes and the parent-owner join bind the final
lane-receipt set. The unjoined global canopy-emission scalar was removed.

`Ran:` The complete orchestrator nextest regression passed during the increment:
754 tests passed with one intentional skip. Final exact-worktree qualification
is recorded in the terminal entry for this checkpoint.

`HOLD:` Per-field Stage-3 convergence authority, real open-snow production,
mixed-surface closure, precipitation, snow-soil heat, postcandidate outcome
ledger, terminal chronology/liquid, restart, scenarios, and terminal reviews
remain open.

`Ran at exact checkpoint:`

- `nix develop --command cargo nextest run -p openwepp-land-surface-energy -p openwepp-hillslope-orchestrator` — 820 passed, 0 failed, one intentional skip.
- `nix develop --command cargo fmt --all -- --check` — passed.
- `git diff --check` — passed.

## Final replay-object and receipt-set semantic join

`Static:` The Stage-3 object that passes exact sealed-replay equality is now
returned from the fixed-point block and is the same object serialized into the
snow owner and staged for installation. No equivalent post-block Stage-3
evaluation remains.

`Static:` Parent-owner construction now requires exact map-key/internal-ID
joins, exact Stage-3/lane key sets, exactly-once destination coverage, and
bitwise equality between every lane contribution and its referenced final
destination receipt. Component surfaces bind an explicit physical vertical
occupancy ordinal; receipt ordering no longer depends on lexical stratum IDs.
The diagnostic initial-guess digest now binds destination, duration, exposure,
forcing, beginning V11 state, beginning Stage-3 state, and numerical values.

`Ran:` Focused component receipt tests passed 2/2, including a two-stratum case
whose IDs sort opposite to physical vertical order. Final exact-worktree gates
are recorded after terminal execution.

`Ran at exact checkpoint:` the two-crate nextest regression passed 821/821 with
one intentional skip. Formatting and diff hygiene passed.

`HOLD:` The adopter-specific v15 lane wire still names source-set index zero as
the provisional carrier source. Its preimage is now fully contextualized and
diagnostic-only, but renaming/removing that canonical field requires a
prospective contract revision before restart authority. Constitutive component
receipt replay and installed-LSE semantic reconstruction likewise remain
restart-scope obligations.

## SC-SNOWENERGY-001@16 covered convergence and restart posture

`Static:` Revision 16 admits absolute physical-class Stage-3 convergence
bounds, independently reconstructed candidate fingerprints, and exact
density/structural/count-like comparison. It explicitly makes the v15 lane
receipt non-restorable and holds additive restart until a normative V2 wire and
complete topology/owner replay join exist. The broader package remains
`EXECUTING / HOLD`.

`Static:` Two independent reviewers returned HOLD with findings. All findings
were accepted and amended. Independent verification A returned
`PASS-WITH-NOTES`; verification B returned `PASS` after the requested direct
cumulative-mass threshold vector passed 4/4 focused convergence tests. The
approved v15 artifact files remain byte-unchanged; v16 evidence is isolated in
the revision-scoped directory.

`Ran at final promoted worktree:`

- two-crate nextest regression: 826 passed, 0 failed, one intentional skip;
- `cargo check -p openwepp-hillslope-orchestrator --tests`: PASS with one
  pre-existing dead-code warning;
- focused approved-v16 contract test: 1/1 PASS;
- strict binding exposure: PASS, 15 rows;
- direct science-contract admission against `27146d851...`: `A0_ADMITTED`, 49
  contracts and two changed science surfaces;
- typed assurance source adoption: PASS, sequential transactions `34ec67f0...`
  and `0a30310a...`, final generation `637f4522...`;
- assurance `validate --all`: PASS;
- assurance generation verification from retained genesis `15763d7f...`: PASS,
  final generation `637f4522...`, 89 transitions;
- `cargo fmt --all -- --check` and `git diff --check`: PASS.

## Immutable Stage-3 beginning-mass convergence join

`Static:` `initial_ice_kg_m2` and `initial_retained_liquid_kg_m2` now compare
bitwise exactly after each candidate fingerprint is independently
reconstructed. Only detached retained liquid and cumulative mass fields use the
`1e-6 kg m^-2` v16 convergence bound.

`Ran:` Five focused convergence-policy tests passed, including resealed one-ULP
poisons for both immutable initial-mass fields and the retained cumulative-mass
inside/outside vectors. The exact-worktree two-crate nextest regression passed
827/827 with one intentional skip. Formatting and diff hygiene passed.

`HOLD:` The next physical increment must introduce a destination-keyed sealed
open-snow exposure and a real open-snow boundary producer. The existing
lane-keyed covered/sub-canopy exposure is not an admissible substitute.
