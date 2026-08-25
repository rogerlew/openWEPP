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
advected, evaluated-duration, and lifecycle results before it stages any V11
candidate. Per-destination receipts remain keyed by `(OFE, tile)`; lane
aggregation is explicit and uses the declared OFE-ground basis. The complete
snow-surface tile fractions must close to one and are not renormalized. The
lane receipt digest uses an explicitly specified deterministic adopter wire
rather than JSON and is prohibited from restart/parent authority until a
canonical framed domain is admitted.

`Static:` `CoveredColumnAuthority::V11SnowCovered` is selected on the covered
candidate and the snow-free guard remains unchanged. The lower-boundary
implementation now has an explicit covered branch that holds covered ground
water, ground sensible/vapor, soil storage, and WB14-facing state, but it is
not closure-complete: precipitation advection, snow--soil coupling, and
independent outcome-ledger custody are still open. Destination-keyed physical
LSE consumption and the bounded covered fixed-point iteration are now
implemented and tested. No claim is made that Stage 3 is already the sole lower-surface
heat/vapor/radiation owner on the V11 side.

`Ran:` after the lower-boundary refactor, land-surface-energy lib tests passed
63/63, the orchestrator lib suite passed 745/745 executed tests with 0
failures and 2 ignored, and focused `cargo check` passed. The persistent
covered integration test remains historical non-closure evidence; its former
reason named missing Stage-3 shortwave even though shortwave has since landed.
The accurate current blockers are precipitation/advection, snow--soil heat,
and independent outcome custody. Its historical explicit run failed at
`ControlVolumeClosure("weighted_ofe_energy")`, and it is not a passing
covered-physics claim. Warnings-denied
Clippy remains blocked by pre-existing direct-runtime shadow and scheduler
debt, while the newly changed lower-boundary code has no additional Clippy
diagnostic.

`Ran:` the focused covered regression also mutates the committed Stage-3
beginning temperature and verifies that the derived carrier receipt changes;
the existing sealed-exposure poison still fails before staged owners are
retained.

## Checkpoint amendment: converge and seal keyed covered boundaries

`Static:` Final covered boundaries are now explicit keyed receipts. The
optical and reciprocal-longwave identities are joined into the Stage-3
boundary, the final digest is carried through the LSE operand set and the
canonical snow owner bytes, and accepted final receipt maps are retained only
after the complete V11 candidate is ready. Keyed destination carrier receipts
are area-weighted into lane Stage-3 terms under the explicit OFE-ground basis,
without covered-subset normalization, and are also supplied directly to the matching
physical LSE tile solve.

`Static:` The former provisional-correction sequence is now a bounded loop
over immutable beginning owners. Each pass evaluates provisional LSE, extracts
keyed optical/longwave candidates, evaluates Stage 3, rebuilds sensible/vapor
boundary terms, and compares canopy-air, snow, longwave, component-temperature,
and Stage-3 candidate state. The accepted path reruns an unsealed final LSE,
seals the final receipts, reruns the sealed final LSE, and requires final
self-reconstruction. `FixedPointIterationLimit` exits before the stack's
staged fields are populated.

`Static:` `CoveredTileEnergyOperandSet::validate()` now rejects mismatches
between the Stage-3 optical terminal/absorbed/reflected terms and column
shortwave, Stage-3 sensible/vapor and column canopy-air ground exchange,
Stage-3 longwave and column ground longwave, Stage-3 boundary energy and the
column energy, independently recomputed energy, and all three receipt joins.
The strict cross-join applies to the sealed final pass so provisional LSE can
produce the correction it is meant to be checked against.

`Ran:`

- `nix develop --command cargo fmt --all -- --check` — passed.
- `git diff --check` — passed.
- `nix develop --command cargo check -p openwepp-land-surface-energy -p openwepp-hillslope-orchestrator` — passed, with the existing direct-runtime dead-code warnings.
- `nix develop --command cargo test -p openwepp-land-surface-energy --lib` — 66 passed, 0 failed.
- `nix develop --command cargo test -p openwepp-hillslope-orchestrator --lib -- --test-threads=1` — 750 passed, 0 failed, 1 historical ignored.
- Focused persistent covered integration, final receipt poison, and Stage-3/column one-bit poison tests — passed.

`HOLD:` The second checkpoint remains open for canopy liquid interception and
actual throughfall/stemflow parcels, precipitation-advection energy, an
independent snow mass/energy outcome ledger, snow-soil conductive heat receipt,
heterogeneous physical integration fixtures, runner-owned 48-support
construction, terminal liquid, additive restart, and physical scenario
matrix closure.

## Checkpoint amendment: correct candidate feedback and lane lineage

`Static:` Every covered iteration now rebuilds keyed carrier receipts from the
iteration's candidate V8 canopy state and candidate persistent Stage-3 state.
The physical LSE projection receives those keyed receipts directly; no
parent-level aggregate carrier is used as the LSE forcing source. The final
sealed Stage-3 pass consumes lane-boundary sensible, vapor, latent, shortwave,
and longwave values rather than the original provisional carrier fluxes.

`Static:` Lane receipt aggregation binds ordered `(OFE, tile)` contributions,
exact tile fractions, topology digest, receipt-set identities, and the sole
`Stage3LaneAreaBasisV1::OfeGround` basis. Fractions must close to one;
covered-subarea normalization is prohibited and an absent open-snow surface
therefore fails closed. Final receipts independently validate snow temperature, latent heat,
support duration, latent mass/energy, and ending V11/Stage-3 state joins.

`Static:` The convergence policy is explicit: maximum 32 iterations; separate
temperature, humidity, flux, vapor-flux, and Stage-3-state absolute/relative
tolerances; deterministic ordering; and typed `FixedPointIterationLimit`
rollback before staged fields are populated. A final unsealed physical rerun,
receipt sealing, and sealed rerun are all self-checked.

`Ran:`

- `nix develop --command cargo test -p openwepp-land-surface-energy --lib` — 66 passed, 0 failed.
- `nix develop --command cargo test -p openwepp-hillslope-orchestrator --lib -- --test-threads=1` — 750 passed, 0 failed, 1 historical ignored.
- `nix develop --command cargo test -p openwepp-hillslope-orchestrator --lib snow_stage3_terminal_handoff::tests` — 4 passed, 0 failed.
- The persistent covered regression passed with candidate-feedback convergence, keyed lane aggregation, final receipt sealing, and rollback coverage.
- `nix develop --command cargo check -p openwepp-hillslope-orchestrator -p openwepp-land-surface-energy`, `cargo fmt --all -- --check`, and `git diff --check` passed.

`HOLD:` This increment does not close heterogeneous physical scenario
fixtures, precipitation custody, snow-soil heat, independent outcome-ledger
closure, runner-owned 48-support construction, terminal chronology/liquid,
additive restart, or the full physical scenario matrix. Child 3 remains
blocked.

## Component-resolved carrier and final-owner join

`Static:` The accepted component-authority and installed-owner-lineage findings
are implemented. Actual final LSE component surfaces—not midpoint or equal-
occupancy reductions—own the carrier proof. A separate post-finalization
canonical receipt replays the final physical receipt sets and all seven actual
installed resource-owner envelopes.

`Ran:` `cargo fmt --all -- --check`, `git diff --check`, the two-crate test
check, focused owner substitution and mixed-surface fail-closed tests, and the
complete LSE/orchestrator nextest regression all passed. The complete regression
ran 818 tests successfully with one intentional skip. Direct science-contract
admission passed for 49 contracts and six changed science surfaces.

`HOLD:` The package and Child 3 disposition are unchanged; this increment does
not claim open-snow, precipitation, snow-soil heat, outcome-ledger, terminal,
restart, scenario, or terminal-review closure.

## Corrective carrier-authority and semantic-lineage increment

`Static:` The reduced carrier is demoted to initialization; sealed atmospheric
forcing plus the component-resolved LSE canopy-air residual now define the
single physical carrier node, and Stage 3 consumes that boundary before each
candidate evaluation. Nested carrier/boundary seals, exact complete snow bytes,
all installed owners, and parent execution lineage are replayed before parent
acceptance.

`Ran:` 820 LSE/orchestrator tests passed with one intentional skip. Focused
fresh-seal and alternate-valid-owner poisons passed. Package disposition remains
`EXECUTING / HOLD` pending the complete mixed-surface physical producer and the
remaining convergence, precipitation, soil-heat, ledger, terminal, restart,
scenario, and review work.

## Exact Stage-3 feedback and installed replay

`Static:` Candidate Stage-3 surface temperature now feeds the next LSE solve;
component LSE fluxes no longer freeze snow-owned operands. The outcome-bearing
shared-carrier receipt and `last_carrier_receipts` evidence surface were removed
from covered execution. A one-time outcome-free diagnostic guess seeds the
solve without claiming melt, refreeze, runoff, ending mass, or ending energy.

`Static:` The replay selected for installation is the source of the retained
component and destination receipts. Its final lane receipts enter canonical
snow-owner V3 bytes and the complete parent-owner receipt. Exact replay guards
reject physical differences in LSE, vegetation, or Stage-3 endings.

`Ran:` `cargo nextest run -p openwepp-hillslope-orchestrator` passed 754/754
tests with one intentional skip during this increment.

## Final nested receipt semantics

`Static:` The exact-accepted Stage-3 state is retained directly through snow-
owner construction and staging. Lane receipts are not merely co-hashed with
destination receipts: every contribution is joined to the actual keyed
destination receipt and its source identities and physical operands.

`Static:` Component receipt order now carries `vertical_occupancy_ordinal`.
The focused nonlexical fixture admits `z-upper` before `a-lower`, proving that
physical order rather than identifier spelling controls receipt order.

## Real mixed covered/open OFE execution

`Static:` The runtime now consumes a destination-keyed closed forcing sum and
constructs a distinct sealed open-snow boundary from live Stage-3 state and a
destination-specific open exposure. The installed owner join accepts component
carrier receipts only for the covered subset while binding the complete typed
covered/open destination and lane sets.

`Static:` Open-snow destinations are Stage-3-owned pass-through members in the
LSE transaction. Their LSE tile state and soil-thermal layers remain bitwise
unchanged; a zero-demand protocol row preserves exact receiver topology but
cannot authorize or finalize water use. Their local LSE energy operands are
exactly zero, preventing the ordinary open-ground surface solve from becoming
a second physical owner.

`Ran:` The real `0.6 covered + 0.4 open` fixture passed with distinct tile
boundary classes, a common Stage-3 lane state, exact unnormalized OFE-ground
aggregation, sealed replay, and atomic staging. The full orchestrator suite
passed 764/764 and the orchestrator/vegetation suite passed 1036/1036; each
scope retained one intentional skip.

## Boundary orientation and forcing custody

`Static:` Tile producers retain positive-outward exchange because the covered
LSE column consumes that convention. A single typed conversion at the Stage-3
receipt boundary negates sensible energy, vapor mass, and latent energy into
the snow control volume. Both covered and open contributions use this same
conversion after OFE-ground aggregation.

`Static:` Destination forcing now owns covered physical operands. Any retained
lane-keyed compatibility forcing must exactly reconstruct the covered subset.
Open forcing stores its zero precipitation claims in its seal, joins the
provider interval receipt, and is backed by support-wide zero-precipitation
checks across Stage 3, provider parcels, V11/LSE parcels/runon, and vegetation.

`Ran:` The exact-worktree two-crate regression passed 1037 tests with one
intentional skip. Direct sign vectors prove outward sensible/vapor/latent loss
becomes negative Stage-3 input and inward exchange becomes positive input.

## State-derived snow ownership and open constitutive alignment

`Static:` Open-snow exchange now reuses the canonical Stage-3
Monin–Obukhov implementation instead of a neutral-only fork. Snow ownership is
selected per lane from the committed Stage-3 state and current snowfall, not
from destination-map presence. Active lanes require complete tile coverage;
inactive lanes reject Stage-3 boundary entries and remain exact carried state.

`Static:` Destination-keyed covered forcing is the only carrier schema. The
legacy lane map and its duplicate digest are removed. Covered-component
extraction and final receipt construction admit an empty covered subset, which
removes the internal canopy assumption from the open-only execution path.
The public prepared-support builder derives the open atmospheric scalars from
the retained interval projection; the low-level scalar sealing API is no
longer available to external callers.

`Ran:` The two-crate nextest regression passed 1037/1037 with one intentional
skip after these changes. Test compilation, formatting, and diff hygiene also
passed. Integrated open-only, heterogeneous-lane, and provider-rain rollback
fixtures remain required and are not claimed here.

## Current-state active-volume projection

`Static:` `Stage3SurfaceStateV1` projects the existing Stage 3 sequential
control-volume partition and binds active mass, depth, cold content, T0, latent
heat, selected cadence, beginning-state identity, and active/lower partition
identity. Open snow and covered carrier construction consume this projection;
depositional `layers[0].temperature_c` is no longer their physical surface
authority. A multilayer regression crosses a density-layer boundary inside the
0.25 m active volume and proves canonical T0 differs from the first layer.

`Static:` the covered numerical initialization receipt binds its complete
rho/cp/reference-atmosphere/longwave/cover/participant/support/exposure context
as diagnostic seed identity and does not claim carrier authority.

`HOLD:` synchronized short cadence exposed missing WB14 parent/subslab
continuation authority. The retained runtime admits the single `1800 s` parent
segment and fails closed before shorter physical work. No partial owner
transition or invented continuation rule is installed.

## Prospective WB14 parent/local-child continuation

`Static:` The prospective state machine separates the persistent cursor from
parent-local support, cumulative supply/infiltration, ordinal, and canonical
chain head. Parent identity binds the enclosing coupled parent, complete owner
beginning, selected day/bin including rollover, support, and cursor. Child and
parent receipts use canonical tagged coupled-time framing and retain the
Green-Ampt input and outcome bits needed for closure reconstruction.

`Static:` Every child delegates to the unchanged shared WB14 transition. The
API remains unreachable outside its contract tests and the attachment still
rejects 900/60-second execution before physics.

`Ran:` focused WB14 tests passed 13/13 after review corrections. A complete
orchestrator run earlier in this checkpoint passed 773/773 with one intentional
skip; exact terminal results are recorded separately.

`HOLD:` complete physical-owner staging, exact-once installation, dynamic
latest-Stage-3 cadence, actual production-owner parity, and complete-owner
rollback remain absent.
# 2026-08-23 two-OFE and refactor closure increment

Ran: `cargo test -p openwepp-hillslope-orchestrator complete_owner_two_ofe_child_routes_upstream_runoff_as_downstream_runon -- --nocapture` — PASS, 1/1.

Ran: `cargo nextest run -p openwepp-hillslope-orchestrator` — PASS, 786 passed, 1 configured skip, 96.396 seconds (final test-gated diff).

Ran: `cargo check -p openwepp-hillslope-orchestrator` — PASS; existing non-fatal dead-code warnings remain.

Ran: `cargo fmt --all` and `git diff --check` — PASS.

Static: all refactored/touched Rust modules are below the 3,000-line closure ceiling. The fixture proves positive upstream supply, positive downstream same-child supply/infiltration, seven finalized ending owners, and one persistent interval advance.
# 2026-08-24 BGC fold, scope, and endpoint corrective increment

`Static:` Review confirmed `CHILD1-BGC-FOLD-002`: the BGC candidate folded
configured protocol order while transition linkage and generic V11 custody
folded SHA-256 receipt order. `SC-VEGETATION-001@28` defines the pre-hash
semantic tuple `(stratum_id,soil_layer_id,species)` and prohibits digest-order
physical arithmetic. The BGC candidate, emitted debits, transition linkage,
live validator, and restore validator now use that order.

`Static:` `CHILD1-BGC-SCOPE-003` is corrected in the generic V11 custody path.
Live acceptance and checkpoint restoration independently validate the BGC
owner/mineral key, `stratum_scoped` literal, configured stratum identity,
layer, species/source, basis, and common admitted OFE. Re-sealed live and
serialized-checkpoint poisons cover wrong literal, unknown stratum, source,
layer, and basis.

`Ran:` The adversarial three-stratum vector distinguishes
`0.0016262541963340254` from `0.0016262541963340251` and proves semantic linkage
selection through live parent acceptance, atomic changed-order rejection,
positive checkpoint restoration, and reordered-checkpoint rejection. The
generic validator also requires the exact configured
`(stratum,root-layer,species)` identity set and rejects BGC-owned non-mineral
debits. The complete real consumer now executes OFE 1 open and OFE 2
vegetated with distinct LSE/vegetation tile IDs, positive mineral-N use, seven
ending owners, independently decoded transition/pool reconstruction, and an
injected failure after BGC transition construction with all parent beginnings
unchanged. This fixture exposed and corrected two stale LSE assumptions that
local LSE tile identity equaled
vegetation tile identity, plus open-tile projection requiring a nonexistent
vegetation topology tile.

`Static / line-count WARN disposition:` `v11.rs` is 2,814 lines after the
adversarial fixture moved to `v11/tests/v11_bgc_tests.rs` (279 lines), clearing
the mandatory 3,000-line ceiling. The remaining 2,000-line warning is accepted
for this corrective increment because `v11.rs` is the single transaction and
checkpoint custody boundary whose private validators share canonical receipt
types. Follow-on decomposition should extract the remaining generic custody
tests and then the private receipt-validation helpers without changing their
visibility or arithmetic order; that mechanical split is not mixed into this
science correction.

# 2026-08-24 persistent covered physical-custody checkpoint

`Static:` `SC-SNOWENERGY-001@18` and `SC-LANDSURFACEENERGY-001@8` admit the
complete persistent-support precipitation and OFE/lane snow--soil boundary.
The real covered consumer now builds an exact producer manifest of sealed
atmospheric snow/open-rain and vegetation terminal throughfall, stemflow, and
initial/second drainage parcels. Stage 3 derives mass and precipitation
advection from that same set; raw rain and canopy release are destination-
exclusive. Zero precipitation is a complete empty set.

`Static:` the snow--soil Crank--Nicolson receipt uses typed participating soil
nodes, explicit conductivity/path operands, OFE-ground aggregation, positive-
to-snow sign, and equal-and-opposite soil credits. Installed snow and soil
owner identities are joined in the same candidate-only transaction. The
postcandidate physical ledger independently reconstructs beginning/end snow
ice, liquid, cold content, enthalpy, precipitation, vapor, melt/refreeze,
terminal liquid, and every admitted energy component; it cannot feed either
the solve or acceptance residual.

`Ran:` the final affected LSE suite passed 66/66 and the final affected
orchestrator suite passed 818/818 with one configured skip. Five affected
authority binaries passed 36/36; anti-evasion and required-suite guards passed
(3/3); science-contract admission returned `A0_ADMITTED` for 49 contracts and
30 changed science surfaces; the final frost profile passed 422/422. The mixed
open/covered rainy fixture proves
positive throughfall and stemflow, destination exclusion, parcel-set
mass/advection, rain-on-snow refreeze, independent ledger closure, direct
receipt poisons, and exact parent rollback at ledger, precipitation-rejection,
snow--soil-rejection, accepted-subslab, and final-owner-join seams. Existing
V8 producer tests prove positive initial and second drainage; the parcel and
manifest tests prove exact custody for both routes. A forced live drainage
configuration was not claimed because its LSE state correctly rejected as
singular/outside-domain.

`Ran / assurance:` typed source adoption for the amended snow-energy contract
created transaction `607a5e68b2ff34cdd18b8ac83c4ad9748e5721ea369741373dbe6b742686584c`,
generation `bddd853e1e723231ce6bf9ec8b9be863eafbf277b98ab991031dc6ed6c4d3274`.
`validate --all` passed and the retained-genesis verification passed all 90
transitions. The generator, not a hand edit, updated the identity and review
locks.

`Static / QA:` warnings-denied orchestrator Clippy remains non-green on the
documented baseline, with no net-new finding (91 preceding diagnostics at the
current tree versus 92 at the clean base). Touched production files are below
the hard 3,000-line ceiling: attachment 2,976, Stage-3 solver 2,907, and real
consumer 2,995 lines. Same-module includes preserve private scope; further
decomposition is deferred to avoid mixing an architectural refactor into the
custody correction.

`Disposition:` the bounded persistent physical-custody checkpoint is `PASS`.
Child 1 remains `EXECUTING / HOLD`; terminal chronology, runner-owned 48-
support construction, exact-once terminal liquid, additive restart, remaining
matrices, and terminal package closure remain open.

`Ran / terminal implementation gate:` exact clean SHA
`43cc9bbea2fbf5fe6ab6596cee4162de75cef999` passed the full workspace
3,300/3,300 with seven slow tests and six configured skips.

# 2026-08-24 terminal-chronology WIP HOLD intake

`Static:` exact pushed WIP `3fda26f0df866504ed4b2f8a26d167e48420ae29`
is preserved as forward-corrected work. It is not terminal chronology PASS
evidence. The prior exact-clean physical implementation
`43cc9bbea2fbf5fe6ab6596cee4162de75cef999` remains the last qualified
physical checkpoint.

`Ran:` the unchanged focused interior-terminal regression failed `0/1` with
`snow.cloud_forcing_unavailable` because covered candidate discovery crossed
the raw Stage-3 carrier boundary. Command:
`nix develop --command cargo nextest run -p openwepp-hillslope-orchestrator interior_terminal_event_runs_covered_event_and_snow_free_remainder --no-fail-fast`.
No runner, terminal-liquid receiver, restart, selector/default, activation, or
cutover work is authorized by this WIP. Disposition remains `EXECUTING / HOLD`.

# 2026-08-24 terminal-chronology contract-first authority

`Static:` coordinated candidate successors now bind the three closed execution
modes, immutable per-trial covered carrier reconstruction, terminal-specific
snow--soil custody, dormant endpoint plus canonical V4 pending parcels,
single coupled event-ordinal authority, exact mutation sets, independently
reconstructed nonempty terminal ledgers, and acyclic framed receipt lineage.
Both independent reviewers and both mandatory verification passes returned
`GO`; retained finding dispositions and review chronology are under
`artifacts/science-contracts/terminal-chronology/`.

`Ran:` the new two-test structural guard passed, then the successor guard plus
six affected historical contract binaries passed 36/36. `git diff --check`
passed. These executions validate contract structure and lifecycle only. The
focused runtime terminal test remains at its recorded WIP failure until the
now-authorized terminal implementation correction is made.
