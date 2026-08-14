# Terminal Verification A — Hydrology And Ownership

Status: `PASS / GO`

Evidence class: `Static + Ran`

Verifier role: independent terminal hydrology/ownership verifier

Reviewed identity: local `main` at
`3f1cf8ee32855a501d7d5b07ac3459d8a3fc8cc3` plus the current uncommitted
Child-2 worktree diff. The required campaign base resolves to
`0db1960129ad4f8fc4e292b20574dfe7229d5fe1`.

## Verdict

`PASS / GO` for the declared Child-2 endpoint:

```text
COMPLETE / V7 vegetation-real-hydrology arbitration shadow implemented /
production unchanged
```

This is a bounded, default-off, single-OFE root-water endpoint. It is not
evidence for routed multi-OFE execution, partial-frost execution, ground
evaporation, a land-surface-energy runtime, a real scheduler consumer, selector
activation, publication or production cutover.

No unresolved material hydrology, ownership, identity, rollback, selector or
gate finding remains inside that boundary.

## Independent Static Verification

### Actual production state and owner extraction

- `RealHydrologyShadowAdapter::try_from_day_start()` accepts an immutable
  `DirectRunFrame`, retains a complete clone, and calls the same
  `DirectRunFrame::seed_day_frame()` constructor used by the production direct
  executor.
- Layer supply, configured layer order, aggregate water, transfer state,
  scheduler identity and frozen facts are extracted from those freshly seeded
  production `DirectDayFrame` values. The implementation does not substitute
  `DiagnosticWaterState`, `ProportionalWater` or an untyped inventory map.
- Production-owned `authorize_direct_layer_withdrawals()` consumes the actual
  seeded `DirectDayFrame` layer state. It delegates only the equal-status
  proportional allocation arithmetic to the one shared kernel-contract
  primitive.
- Production native root uptake and the shadow candidate both call
  `apply_direct_finalized_layer_liquid_debit()`. Aggregate soil-water
  reconstruction is likewise centralized and shared by ET, subsurface and the
  shadow adapter.
- The bounded canonical snapshot includes run, hillslope, day, transaction,
  owner, interval, OFE/lane topology and area, water and transfer operands,
  exact configured layer order and all twelve layer fields. The package does
  not mislabel this projection as serialization of the whole production
  runtime. The complete cloned frame is separately protected by structural
  equality.

### Immutable same-snapshot V7 transaction

- Before the V7 solve, the public bridge requires exact interval,
  immediately-next transaction, layer cardinality/order, beginning layer
  liquid and frozen-state equality between vegetation forcing and the real
  hydrology snapshot.
- Root accessibility is derived from the joined vegetation forcing; it is not
  accepted as a second independent caller assertion.
- Every V7 water request is wrapped with exact transaction, interval,
  occupancy, OFE/lane, layer and stand-ground amount-basis identity.
- Duplicate source/requester identities, stale transactions, wrong intervals,
  wrong lanes, wrong layers and wrong bases fail before a candidate is
  exposed.
- All positive eligible occupancies sharing a source are arbitrated from one
  immutable beginning supply. Zero demand, rooting exclusion, frozen
  exclusion, zero/nonzero storage limitation and true eligible competition
  have distinct validated reasons.
- The arbiter permits one authorization only. The V7 public water phase then
  performs its accepted potential request, fixed-authorization rebuild and
  finalized-use sequence without a second authorization.

### `D/A/F`, debit and reason preservation

- The shared protocol and the real-owner candidate independently require exact
  request/authorization/use identity and `0 <= F <= A <= D`.
- Canonical source sums are used by allocation, receiving-owner validation and
  ending-state reconstruction. Regression vectors cover finite-input derived
  overflow, per-request overauthorization, aggregate one-ULP overdraw, request
  order, zero supply and signed zero.
- Only finalized uses enter `debit_amounts_by_source`; maximum authorization is
  never substituted for use. Unused authorization therefore remains in the
  staged owner inventory.
- The debit is applied to the exact configured vector index, not lexical layer
  order. Full depletion has an explicit exact mass/depth path that avoids a
  residual binary64 store.
- The resulting V7 `WaterOwnerCandidate` is reconstructed from the same
  finalized uses while the real candidate debits the cloned production frame.
  The public integration vector proves both sides are produced by the actual
  V7 phase.

### Candidate isolation and production invariance

- Authorization is read-only. Candidate construction starts from
  `self.beginning_frame.clone()` and mutates only that owned clone.
- Validation, identity, protocol and debit failures return no candidate.
- Focused tests compare the original complete `DirectRunFrame` structurally
  after success and after rejected identity, basis, layer and overuse cases.
  They also join identical canonical beginning bytes across adapter,
  arbitration and candidate.
- No commit or production-state replacement exists in this Child-2 API. The
  result contains an uncommitted vegetation water phase and a real hydrology
  shadow candidate only.

### Legacy ET and production exclusion

- The shadow begins from the pre-hydrology seeded day frame and does not invoke
  either native R4N surface ET or native root uptake. It does not execute
  legacy ET and subtract it afterward.
- The unchanged production executor still invokes its ordinary production
  spans. The only shared production change is extraction of the exact layer
  debit and soil-water aggregation mechanics into common production-owned
  functions.
- Recursive source inspection finds no reference to
  `execute_v7_real_hydrology_water_shadow`, `RealHydrologyShadowAdapter` or a
  shadow selector in `openwepp-runner` or production direct-runtime dispatch.
  No runner, selector, default, output or publication path is changed by the
  terminal diff.
- The shadow neither donates denied canopy demand to ground evaporation nor
  creates a ground requester. That later joint owner set remains outside this
  child.

### Bounded exclusions are explicit and enforced

- `execute_v7_real_hydrology_water_shadow()` rejects any adapter with more than
  one OFE/lane or a selected lane other than index zero. Lower-level tests prove
  OFE/layer identity separation without claiming routed execution.
- Extraction accepts exactly unfrozen or exactly fully frozen production
  layers. A partially frozen layer returns the typed operand error
  `partially frozen layer requires a future typed forcing surface`; no
  whole-layer approximation or silent availability rule is used.
- Package artifacts consistently assign routed scheduler coordination and the
  exhaustive consumer rollback matrix to Child 4. They do not use the
  low-level multi-lane test as real-consumer evidence.

## Reviews, Dispositions And Terminal Evidence

- Independent hydrology review: `GO`; `HYD-REV-001..007` and
  `HYD-REREV-001` are closed.
- Independent Rust correctness review: `GO`; `RUST-REV-001..005` and the
  documented intake findings are closed.
- The finding-disposition artifact marks every material finding accepted and
  corrected. It contains no silent rejection or unresolved current-boundary
  deferral.
- Exact-terminal comparator rerun: `PASS`, including 507/507 orchestrator
  quick tests, all four affected checks and strict Clippy gates, vegetation
  authority and implementation suites, AUTH11, admission, anti-evasion,
  formatting, diff hygiene and package Markdown lint.
- Line-count governance truthfully records the 2,118-line `WARN`, a bounded
  co-location rationale and a mandatory decomposition before Child 4 extends
  the module. It is below the 3,000-line closure block.

## Ran Evidence

Executed independently against the current worktree:

- `cargo nextest run --test vegetation_real_hydrology_shadow_contract
  --profile quick` — PASS, 3/3.
- `cargo nextest run -p openwepp-hillslope-orchestrator -E
  'test(vegetation_real_hydrology_shadow)' --profile quick` — PASS, 13/13.
- `cargo nextest run -p openwepp-vegetation -E 'test(water_phase)'
  --profile quick` — PASS, 6/6.
- `bash tools/release/check_science_contract_admission.sh --base-ref
  0db196012 --worktree` — PASS; 45 admitted contracts, nine changed science
  surfaces, authority SHA-256
  `ac829c7b73c92022e269823a2f88c3329efcc4785e4c8cd10caef6dfb455e5af`.
- `bash tools/release/check_authority_suite_antievasion.sh` — PASS.
- `cargo nextest run --test
  auth11_required_suite_obligation_guards_contract` — PASS, 3/3.
- `cargo fmt --all -- --check` — PASS.
- `git diff --check` — PASS before writing this report.
- Recursive selector/dispatch source scan — PASS; no shadow API reference in
  the runner or direct-runtime production dispatch.

## Residual Boundaries

The following are explicit later-child obligations and are not evidence gaps
for this bounded endpoint:

- coordinated routed multi-OFE execution;
- a scientifically admitted partial-frost source split;
- joint root and ground evaporation requests in one batch;
- land-surface-energy and soil/frost thermal owner candidates;
- a real scheduler consumer and exhaustive phase-injection matrix;
- runtime selection, publication or production cutover.

## Final Verification Disposition

`PASS / GO`. The exact Child-2 bytes use real production hydrology state and
production-owned candidate mechanics, preserve immutable same-snapshot V7
request/authorization/finalized-use identity, debit finalized use only, retain
typed reasons and source identity, leave production state unchanged, isolate
legacy ET, fail closed at the declared single-OFE and partial-frost boundaries,
and remain unreachable from production selection or dispatch.
