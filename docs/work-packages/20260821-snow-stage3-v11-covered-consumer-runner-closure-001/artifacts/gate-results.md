# Gate results

## 2026-08-23 provider-owned atmosphere checkpoint results

Ran: `cargo fmt --all -- --check` passed in the repository Nix development
shell.

Ran: `cargo nextest run -p openwepp-hillslope-orchestrator` passed 766/766
tests with one skipped.

Ran: the focused provider/mixed/open/lifecycle selection passed 3/3. It covers
one-bit base temperature and wind changes, covered temperature and wind
changes, Stage-3 pressure and dewpoint contradictions, derived raw-wind
provider/model identities, provider rain rejection without cursor/GSI
mutation, exact terminal-domain sides, liquid-only state, detached terminal
liquid, and new-snow custody rejection. The open-only real V11/Stage-3
consumer execution has no vegetation occupancies or covered carrier receipts,
produces only open boundary classes, advances Stage 3, and preserves exact LSE
tile plus soil-thermal physical payloads while receipt chronology advances.
The existing mixed open/covered real-consumer fixture continues to prove class
completeness and fail-closed missing/class-substituted destinations.

Ran: `bash tools/release/check_authority_suite_antievasion.sh` passed.

Ran: `cargo nextest run --test
auth11_required_suite_obligation_guards_contract` passed 3/3.

Ran: `cargo nextest run --workspace --profile frost` passed 411/411 with
2,833 skipped by the profile. The first attempt exposed the mechanically moved
Stage-3 source path and stale v14 assertions against the current v16
contract/index; both exact bindings were corrected, their focused tests and
anti-evasion guards passed, and the frost profile then passed.

Ran: warnings-denied Clippy did not pass. The first command failed on four
pre-existing `openwepp-land-surface-energy` lints; `--no-deps` then exposed 57
existing orchestrator lint debts across the active Stage-3/V11 implementation.
Newly introduced long-function/cast warnings were corrected or explicitly
scoped, but this increment does not claim workspace Clippy closure.

Static: `v11_covered/mod.rs` is seven lines of wiring. Extracted source sizes
are: `execution.rs` 1,347; `open_snow.rs` 961; `receipt_sets.rs` 349;
`fixed_point.rs` 284; and `regime.rs` 47. `owner_finalization.rs` is 1,210.
No active extracted file reaches the 2,000-line warning threshold.

Status: `IN PROGRESS / HOLD`.

Qualification limits: the heterogeneous real-consumer evidence is a mixed
open/covered destination set within one production lane; it is not yet the
requested two-production-lane atomic parent fixture. Provider rain is rejected
while binding the provider-owned atmospheric capability, before a prepared day
can exist. That proves the contradictory dry-sealed/rainy-provider state is
unrepresentable and leaves cursor/GSI inputs unchanged, but it is not a
`stage_prepared_day` seven-owner rollback exercise. Those two exact fixtures
remain required before this checkpoint can be dispositioned as complete.

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

## Destination-keyed open-snow boundary and mixed OFE checkpoint

`Static:` Prepared Stage-3 snow surfaces are keyed by `(OFE, tile)` and use a
closed covered/open forcing sum. Open exposure is independently sealed with
support, destination, forcing, wind-provider, projection-model, wind, and
geometry identity. The open producer derives snow temperature, ice-saturation
humidity, latent heat, VIS/NIR absorption, atmospheric longwave, and turbulent
exchange from the live Stage-3 state and sealed forcing; nonzero rain,
snowfall, or precipitation parcels fail closed.

`Static:` A Stage-3-owned open-snow tile is now an explicit pass-through member
of the heterogeneous LSE transaction. It emits a zero-demand D/A/F row solely
to preserve transactional topology, retains beginning LSE and soil-thermal
state exactly, contributes zero local LSE energy, and never invokes the
ordinary open-ground solver. Covered and open boundary candidates participate
in the same Stage-3 fixed point and aggregate as `sum(tile_fraction * flux)`
without normalization.

`Ran at exact worktree:`

- `cargo check -p openwepp-hillslope-orchestrator --tests`: PASS.
- focused real mixed `0.6 covered + 0.4 open` execution: 1/1 PASS.
- orchestrator nextest regression: 764/764 PASS, one intentional skip.
- two-crate orchestrator/vegetation nextest regression: 1036/1036 PASS, one
  intentional skip.
- `cargo fmt --all -- --check`: PASS after formatting.

`HOLD:` Precipitation custody is deliberately unavailable. Open-only and the
remaining exposure/model poison matrix, snow-soil heat, independent outcome
ledger, terminal chronology, restart, scenarios, and independent review remain
open.

## Snow-boundary orientation and forcing-custody correction

`Static:` Covered and open tile exchanges retain their positive-outward-from-
snow producer convention. The sole Stage-3 construction boundary now negates
sensible, vapor-mass, and latent-energy operands into Stage-3's positive-into-
snow control-volume convention. Warm/dry outward exchange therefore removes
snow energy and mass; cold/humid inward exchange adds them.

`Static:` Open forcing now persists and revalidates its zero rain, snowfall,
and parcel-count operands. Prepared snow-surface supports additionally reject
nonzero Stage-3 rain/snow/active precipitation, provider/support parcels,
V11/LSE precipitation or runon parcels, and vegetation rain. Open forcing is
joined to the exact provider interval receipt. Destination-keyed covered
forcing is the sole physical carrier source; the legacy lane map is admitted
only when it exactly equals the covered destination subset.

`Static:` The support exposes explicit `SnowFree`, `OpenSnowOnly`, and
`CanopyCoveredOrMixed` regimes. Open-only no longer requires a fabricated
covered carrier. Exact sealed replay re-merges the accepted Stage-3 snow state
before final component/destination receipts, eliminating tolerance-level
covered/open temperature divergence.

`Ran at exact worktree:` two-crate orchestrator/vegetation nextest passed
1037/1037 with one intentional skip. The real mixed covered/open regression
and the direct outward-to-Stage-3 sign vectors passed. Formatting and diff
hygiene passed.

`HOLD:` A complete one-tile `1.0 open-snow` parent fixture and the full
provider-rain rollback poison matrix remain required before claiming the
open-only regime closed. Precipitation physics, snow-soil heat, outcome ledger,
terminal chronology, restart, scenarios, and independent review remain open.

## State-derived regime and canonical open turbulence increment

`Static:` Open-snow turbulent exchange now calls the existing Stage-3
stability-aware Monin–Obukhov operator with the admitted 5 m transfer height
and 0.005 m roughness. The former neutral logarithmic resistance fork is gone.
Caller-supplied air density and heat capacity were removed from the open
receipt because the canonical operator owns those atmospheric properties.

`Static:` Active snow lanes are derived from immutable Stage-3 storage plus
current snowfall. An active lane requires its complete configured destination
set; a snow-free lane rejects any Stage-3 surface claim. The LSE projection can
now combine Stage-3-owned destinations with ordinary snow-free destinations in
one parent transaction, inactive Stage-3 lanes carry exactly, and the covered
component set may be empty for an open-only support. The duplicate legacy
lane-carrier map and digest were removed.

`Ran at current worktree:` two-crate orchestrator/vegetation nextest passed
1037/1037 with one intentional skip. `cargo check --tests`, `cargo fmt --all
-- --check`, and `git diff --check` passed.

`Static:` The public prepared-support builder now constructs the opaque open
forcing from its retained LSE/provider projection, updates the destination
exposure join, and accepts only the raw-wind-provider and admitted projection
model identities. The low-level scalar constructor remains crate-private.

`HOLD:` The real one-tile open-only parent fixture, independent proof of the
raw-wind provider identity, provider-rain full-owner rollback poison, and a
two-lane snow/snow-free integration fixture remain unproven. This increment
does not claim precipitation, restart, or terminal authority.

## 2026-08-23 current-state projector / cadence HOLD

`Ran:` `cargo nextest run -p openwepp-hillslope-orchestrator` passed 768/768
with one intentional skip. `cargo check -p openwepp-hillslope-orchestrator`,
`cargo fmt --all -- --check`, and `git diff --check` passed. Cargo retained 13
unrelated baseline dead-code warnings.

`Ran/negative discovery:` a real two-subslab execution reached the second V11
physical call and was rejected by the existing WB14 continuation guard with
`WB14 day or interval continuation mismatch`. This proves the current contract
does not define how the shared WB14 owner maps internal subslabs onto one of its
48 daily parent intervals.

`HOLD:` The runtime rejects selected `900 s` or `60 s` cadence before physical
execution. Required short-cadence fixtures, full attachment fixtures, rollback
poisons, warnings-denied changed-file Clippy, authority/admission guards, and
package terminal validation were not claimed or run after the authority gap
was identified.

## 2026-08-23 WB14 prospective parent authority

| Gate | Result | Evidence |
|---|---|---|
| Exact clean start | PASS | `HEAD=8566ebd8a432b56c0ce60051175a9e3ce1872d03`; empty status before edits. |
| Focused WB14 vectors | PASS | `cargo test ... surface_liquid_wb14::tests`: 13/13. |
| Affected-crate nextest | PASS | Exact worktree: 774/774, one intentional skip. |
| Affected-package check | PASS | Completed with 13 documented pre-existing dead-code warnings. |
| Formatting / diff hygiene | PASS | `cargo fmt --all -- --check`; `git diff --check`. |
| Warnings-denied Clippy | BLOCKED by baseline debt | Stopped in unchanged `openwepp-land-surface-energy` on four existing `too_many_lines`/`large_enum_variant` findings before the changed orchestrator file; no clean claim. |
| Authority anti-evasion | PASS | Canonical shell guard passed. |
| Required-suite guard | PASS | 3/3. |
| Science-contract admission | PASS | `A0_ADMITTED`, 49 contracts, one changed science surface, base `8566ebd...`. |
| Independent hydrology review | FAIL/HOLD | Complete-owner integration, dynamic cadence, and production parity absent. |
| Independent terminal review | FAIL/HOLD | Complete rollback/install and attachment vectors absent. |
| Child evaluator release | BLOCKED | Existing typed 900/60 pre-physics rejection retained. |
| Green-Ampt/default/output/restart noninterference | PASS (static) | Only unreachable prospective state machine, tests, contract/evidence, and roadmap changed. |

The package remains `EXECUTING / HOLD`; review failures are not waived.
## 2026-08-23 provider-atmosphere/lifecycle follow-on intake

Static: the reviewed `71ecc65642aaec793dbb8fd82d177f232102978e`
increment was retained at clean `HEAD == origin/main`. The package remains
`EXECUTING / HOLD`. The new increment records provider-atmosphere closure,
terminal-aware lifecycle routing, the requested fixtures/poisons, and the
mechanical covered-module split as current-scope obligations.

Ran: `nix develop --command bash -lc 'cargo fmt --all && cargo check -p
openwepp-hillslope-orchestrator'` completed successfully. Cargo reported 13
pre-existing dead-code warnings; this was a compile check, not a
warnings-denied lint or workflow regression.

Ran: `nix develop --command bash -lc 'cargo nextest run -p
openwepp-hillslope-orchestrator snow_stage3_v11_attachment'` passed 5/5 tests
with 761 filtered out.

Static: the first bounded correction exposes the canonical owner predicates
for represented density-layer mass and the `> 1 kg m^-2` resolved Stage-3
thermal domain. Attachment routing now classifies `SnowFree`, `ResolvedSnow`,
`TerminalPending`, and `SolidPrecipitationPending`; only `ResolvedSnow` enters
the covered solver, while the two pending classes fail closed. Atmospheric
receipt construction, exact atmosphere joins, full fixtures/poisons, module
split, Clippy, and frost regression remain not run, so this is not checkpoint
closure evidence.

## 2026-08-23 exact-head workspace regression after LLVM tooling repair

`Ran:` At clean `HEAD=e47b028a57b2dbedf8821bb8d82de3c13bb0f224`,
`nix develop --command tools/dev/heavy cargo nextest run --workspace`
discovered 3,263 tests across 243 binaries with six configured skips. The run
advanced beyond the former `cargo llvm-cov` discovery failure after the Nix
shell supplied LLVM 21.1.8 `llvm-cov` and `llvm-profdata`, matching the LLVM
version used by the shell's Rust compiler.

`FAIL:` Nextest stopped fail-fast after 269 passes and one deterministic
failure; 2,993 tests were consequently not run. The failing test was
`direct_hydrology_persisted_restart_implementation_contract::production_checkpoint_contains_complete_typed_direct_hydrology_owner`.
It rejected
`docs/work-packages/20260817-direct-hydrology-persisted-restart-authority-001/artifacts/checkpoint-in-progress-vector.json`
with `NoncanonicalBytes` while decoding
`DirectV10RealConsumerCheckpointV1`.

`Ran / confirmation:` The failing integration test was rerun alone with
`cargo nextest run --test direct_hydrology_persisted_restart_implementation_contract --no-capture`
and failed identically, proving this was deterministic rather than parallel
test interference. The formerly blocked LLVM-dependent
`cqr_quality_evidence_self_test_passes` was also run alone after the shell fix
and passed 1/1 in 238.649 seconds.

`HOLD:` The workspace correctness gate is not passed. The persisted-restart
fixture must be reconciled with its canonical schema and the full exact-head
workspace run repeated before this result can support package closure. The
LLVM tooling defect is closed and is no longer the blocker.

## 2026-08-23 persisted-restart V1 wire reconciliation

`Static:` Starting identity was clean `HEAD=origin/main=c29b8f38fcff24e0e6058a167b0b690dd653647d`.
The exact failure was production DTO drift: `DirectHydrologyRestartV1`
serialized optional runtime-only `snow_stage3_shadow`, while the frozen V1
schema and vectors omit that closed-object member. Missing input admitted as
`None`, then typed serialization emitted explicit `null` and failed exact
canonical-byte admission.

`Static / correction:` The frozen V1 DTO again contains only its released
members. Projection rejects a configured Stage-3 attachment with typed
`HydrologyRestartError::Unsupported("snow_stage3_shadow_requires_successor_restart")`;
an absent attachment continues to project, restore, and reproject exactly.
The candidate Stage-3 payload is documented as requiring a versioned successor
restart. No frozen schema or vector byte changed, and no short-cadence guard,
selector, default, or `SC-SURFACELIQUID-001@8` authority changed.

`Ran / PASS:` The isolated formerly failing implementation contract passed
`1/1`. `openwepp-persisted-restart-v1` passed `27/27`. The production V1
checkpoint contract passed `4/4`, including all four frozen byte-identical
round trips, explicit `snow_stage3_shadow:null` rejection, duplicate and
noncanonical poisons, and a production-member-set versus frozen closed-schema
guard. The implementation contract passed `1/1`; restart authority integration
passed `7/7`; and the package-local authority reference passed `28/28` plus
doc tests. A fresh generated schema was byte-identical to the frozen schema.
Fresh vector generation was deliberately not installed: current unrelated
vegetation calculations differ by one-bit floating results, confirming that
regeneration is not an admissible V1 correction.

`Ran / PASS:` The affected hillslope-orchestrator suite passed `790/790` with
one configured skip. `cargo fmt --all -- --check` passed. The independent
LLVM/CQR self-test passed `1/1` in `237.624s`.

`Ran / unrelated Clippy debt:` Warnings-denied Clippy for the affected restart
package stopped in the reverse dependency `openwepp-land-surface-energy` on
four pre-existing findings: three `too_many_lines` findings and one
`large_enum_variant`. No restart-owned Clippy finding was emitted before that
failure.

`Ran / full workspace HOLD:`
`nix develop --command tools/dev/heavy cargo nextest run --workspace --no-fail-fast`
completed all `3,266` tests across `243` binaries: `3,253` passed, `13` failed,
and six were configured skips. The persisted-restart failure is closed and no
restart test failed. The remaining failures are the already unrelated Stage-3
solver line-count guard, stale snow/vegetation contract-index assertions,
stale assurance identity/digest bindings, and the V10 definition digest.
Therefore the active Child-1 package truthfully remains `EXECUTING / HOLD`;
this result does not reopen V1 wire authority or the retained WB14 v8 release.
