# Review Agent B Final — Hydrology And Ownership

Evidence class: `Static exact-byte + Ran focused tests`

Reviewed commit: `c0d5da743099a6dc760d5a231236543d0354d967`

Verdict: `HOLD / two material receiver-envelope defects remain / no authority HOLD`.

The final review reread the version-5 contract, historical findings and
dispositions, exact remediation diff, surface owner/ingress/closure modules,
unified real-hydrology bridge, shared production infiltration transition, and
focused tests. It ran:

```text
cargo nextest run -p openwepp-hillslope-orchestrator surface_liquid --profile quick
30 passed / 507 skipped

cargo nextest run --test land_surface_energy_real_hydrology_shadow_contract --profile quick
15 passed / 0 skipped
```

## Material findings

### B-FINAL-HIGH-001 — The sealed receiver envelope still accepts extra or nonfinite thermal state and three forged rollback owners

The remediation correctly seals `UnifiedLseFinalization`, the ingress
candidate, and the unified candidate, and it now requires an exact five-kind
rollback cardinality. It does not yet validate the exact complete receiver
bytes promised by the package:

- `UnifiedLseFinalization::try_new()` checks only that every thermal tile has a
  nonempty, duplicate-free layer vector
  (`land_surface_energy_shadow/mod.rs:218-277`).
- `validate_receiver_sets()` requires the configured infiltration layer only
  at vector position zero and uniqueness, but does not require exact layer
  cardinality/order or finite values for every returned layer
  (`land_surface_energy_shadow/mod.rs:887-934`).
- Receiver closure freezes and validates only the one named infiltration layer
  (`land_surface_energy_shadow/mod.rs:1305-1358,1447-1463`), and unified
  candidate validation likewise joins only that named layer
  (`land_surface_energy_shadow/mod.rs:450-470`).

Therefore a callback can return a valid top receiver followed by an extra
unique layer containing arbitrary or nonfinite beginning/ending values; the
sealed constructor, receiver-set validator, independent closure, and unified
candidate validator all accept it and publish it in
`soil_thermal_candidates()`.

The rollback set is also exact only by kind, not by owner identity. The bridge
validates hydrology owner identity and the soil-thermal owner/digest, but does
not validate the `LandSurfaceEnergy`, `Vegetation`, or `Biogeochemistry`
owner IDs or beginning digests (`land_surface_energy_shadow/mod.rs:937-980`).
One row of each required kind with arbitrary identities for those three owners
passes. The focused poison loop covers duplicate, extra, missing, and changed
`after_sha256`, but not substituted owner IDs/digests or an extra unique
thermal layer (`land_surface_energy_real_hydrology_shadow_contract.rs:1251-1321`).

This is the unclosed portion of the accepted Rust receiver-envelope finding
and the requested exact receiver/rollback-set check. Bind the exact expected
thermal layer vector and every owner identity/beginning digest before the
callback result is accepted; validate finiteness/domain for every carried
receiver value; add extra-layer, nonfinite-layer, wrong-owner, and wrong-digest
poisons.

### B-FINAL-HIGH-002 — Canonical E004/E007/E011 failures still omit identities available at their guard sites

The remediation genuinely makes `E004`, `E007`, `E008`, `E009`, `E010`, and
`E011` reachable and preserves beginning/attempt hashes through the ingress
and receiver paths. Cadence and WB14 transition failures now correctly emit
`E008`; candidate versus independent-closure failures are separated; and
receiver-envelope failures emit `E011`.

The payload remains incomplete where the offending identity is known:

- `validate_native_shadow_domain()` uses whole-frame `any()` predicates and
  emits E004/E007 with only the transaction context
  (`land_surface_energy_shadow/mod.rs:628-674`). The offending production lane
  and its configured OFE are available by indexed iteration but are discarded.
- `UnifiedLseFinalization::try_new()` emits a generic E011 with no OFE/tile or
  owner context even for a concrete wrong/duplicate tile or thermal receiver
  (`land_surface_energy_shadow/mod.rs:218-270`).

The contract requires OFE/tile/surface/source/parcel identity when applicable,
with typed absence reserved for genuinely unavailable identity. The new entry
tests assert only transaction and hash presence for E004/E007 and therefore
bless the missing OFE context
(`land_surface_energy_real_hydrology_shadow_contract.rs:1006-1072`). Iterate to
the first canonical offending receiver/lane, retain its exact available
identity, and add exact context assertions. No numerical or ownership behavior
change is required.

## Confirmed closed findings

- The production receiver now uses the shared
  `apply_direct_same_pass_infiltration()` transition on the cloned exact lane.
- Frozen receiver operands independently reconstruct every ordered production
  layer, aggregate soil water, infiltration thermal credit, ending thermal
  enthalpy, and retained LSE tile enthalpy. Wrong-layer redistribution and
  omitted/doubled thermal/LSE credits have focused poisons.
- Surface `W0-F+C-overflow+retained`, parcel mass/enthalpy, routed-area, D/A/F,
  signed condensation, and strict restart/cadence invariants remain intact.
- The proportional authorization envelope is sealed and independently
  reconstructed before finalized debit.
- Canonical configuration/state persistence, ingress/resource/unified
  candidate sealing, and duplicated ending-state joins are materially
  corrected.
- Production state remains clone-only, and no runner selector, production
  dispatch/default, activation, or publication path was introduced.

The remaining findings are bounded in-scope implementation defects. They do
not justify a new contract, package, model identity, or authority HOLD.
