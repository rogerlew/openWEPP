# Review Agent A — Exact-Commit Final Rust Pass

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `848e60358cfc98d2de6a6dcdc6c45779a3255228`

The shared checkout advanced after review intake, so static inspection and all
commands below used an isolated `git archive` of the reviewed commit. Later
worktree bytes are excluded.

Verdict: `HOLD / NO-GO`.

## Findings

### High — `E011` still substitutes the first receiver for the actual offending receiver or rollback owner

`UnifiedLseFinalization::try_new()` combines empty, duplicate, mismatched,
nonunique-layer, rollback-cardinality, and changed-hash checks into one
predicate at
`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs:292-310`.
On any failure it reports the hydrology owner and the first ending LSE tile at
lines 311-326. A failure in a later soil-thermal tile or a specific rollback
row therefore reports an unrelated OFE/tile and the wrong implicated owner.

The second validation boundary repeats the defect. `validate_receiver_sets()`
and `validate_rollback_joins()` detect exact thermal-vector and ordered-owner
violations, but `apply_ingress_to_real_receivers()` discards their identity at
`land_surface_energy_shadow/mod.rs:1008-1040` and rebuilds every failure from
the first configuration record plus the hydrology owner. For example, a wrong
second-tile thermal layer is labeled with the first tile, while a wrong LSE or
soil-thermal rollback row is labeled as hydrology and receives an unrelated
surface/source identity.

This does not satisfy the accepted final-review requirement to preserve the
first canonical offending row, nor
`SC-SURFACELIQUID-001.md:475-481`, which requires applicable owner/OFE/tile
identity in the canonical public failure. The failure is fail-closed and the
rollback hashes are present, but its typed diagnostic context is materially
false for multi-receiver envelopes.

The focused poison loop at
`tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs:1353-1433`
uses only one tile and asserts only `E011` plus production-frame equality. It
cannot reject misattribution to the first receiver and does not assert the
offending rollback owner.

Required correction: have the sealed-finalization, receiver-set, and rollback
validators return the first ordered violation with its applicable owner and
OFE/tile identity; preserve that context when constructing `E011`. Add a
multi-tile poison where only a non-first thermal receiver fails and exact
ordered LSE/Hydrology/SoilThermal rollback poisons that assert owner identity
and typed absence for non-applicable tile/surface fields.

## Confirmed closures

- Canonical configuration/state persistence is strict and configuration-bound.
  The root state is not raw-serde serializable; `canonical_bytes()` validates
  structure, lineage, configuration identity and self-digest before emitting
  bytes, and the strict parser rejects noncanonical bytes.
- Arbitration, resource, ingress, finalization and unified candidates are
  externally sealed. Resource validation re-derives proportional authorization
  from immutable `W0 + D`, retains exact D/A identities, and checks
  `0 <= F <= A <= D` before finalized-use debit.
- Current ingress is evaluated only after the resource candidate. Ingress and
  receiver work occurs on clones; the returned candidate is validated before
  exposure, and the supplied production frame remains unchanged on success and
  failure.
- `UnifiedReceiverExpectations` is independent of the fixed-cap callback and
  binds LSE/soil-thermal owners and beginning digests. Receiver validation
  requires the exact ordered tile vector, exact ordered thermal-layer vectors,
  all carried thermal numerics finite, and the configured infiltration layer
  first.
- Rollback acceptance is exactly three ordered rows—LandSurfaceEnergy,
  Hydrology, SoilThermal—with exact owner IDs and exact unchanged beginning
  digests. Extra, missing, duplicate, reordered and forged rows reject.
- Production receiver closure now matches the production aggregate formula:
  each layer contributes `theta_m + residual_theta * max(depth_m -
  frozen_depth_m, 0)`. Ordered infiltration deltas, aggregate beginning/end,
  soil-thermal enthalpy, and retained LSE enthalpy are independently
  reconstructed.
- `E004` and `E007` select the first offending production lane and carry
  transaction, hydrology owner, OFE and the configured surface identity.
  `E011` is reachable with beginning/attempted hashes, subject to the exact
  offender-context defect above.
- The owner is absent from both normal `DirectRunFrame` constructors, no runner
  references the unified bridge or configures the state, and no production
  selector/default/publication path was added.
- Exact line counts match the governance artifact: `runoff.rs` 2,852;
  `00_core_frames.rs` 2,783; `surface_liquid_owner.rs` 2,818;
  `vegetation_real_hydrology_shadow.rs` 2,157;
  `surface_liquid_ingress.rs` 1,959;
  `land_surface_energy_shadow/mod.rs` 2,273;
  `surface_liquid_closure.rs` 769; and `surface_liquid_wb14.rs` 303. No affected
  Rust file reaches the mandatory 3,000-line threshold; all 2,000+ files have
  explicit WARN rationale.

## Residual risk and missing tests

- The residual-water runtime test uses nonzero `residual_theta` with
  `frozen_depth_m = 0`. The exact frozen-depth operand and production-equivalent
  `max(depth-frozen, 0)` arithmetic are statically present, but no focused
  vector combines nonzero residual storage with nonzero admitted frozen depth.
- No multi-OFE/multi-tile test asserts exact `E011` offender identity. The
  existing wrong-owner/digest/order poisons assert the code only.
- Full-workspace nextest was not rerun at this exact commit. The package retains
  a prior failed full-workspace attempt followed by affected-crate/focused
  passes, so critical package closure still needs exact-head workspace evidence
  after this finding is corrected.

## Ran against the isolated exact commit

- `cargo nextest run --profile quick --test surface_liquid_hydrology_custody_authority_contract --test land_surface_energy_real_hydrology_shadow_contract` — 25 passed.
- `cargo nextest run --profile quick -p openwepp-hillslope-orchestrator -E 'test(/surface_liquid/)'` — 30 passed; 507 skipped by the filter.
- `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings` — PASS.

## Approval statement

`NO-GO`: exact commit `848e60358` is not acceptable for dependency-package
closure. Numerical custody, receiver reconstruction, serialization, sealing,
rollback atomicity and non-activation are materially closed, but the accepted
canonical `E011` exact-offender context requirement remains open and requires a
fresh exact-byte review after correction.
