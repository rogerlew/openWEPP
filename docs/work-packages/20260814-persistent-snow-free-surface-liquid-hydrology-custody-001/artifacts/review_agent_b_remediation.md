# Review Agent B Remediation — Hydrology And Ownership

Evidence class: `Static exact-byte + Ran focused tests`

Reviewed commit: `a4138bee2ae2e0316f9dffb2e5d8ce7a45d4f5e5`

Verdict: `HOLD / two material implementation findings remain / no authority HOLD`.

The review read the complete version-5 `SC-SURFACELIQUID-001`, package,
historical review, disposition, production WB14 transition, persistent owner,
ingress/closure modules, real-hydrology adapter, unified LSE bridge, and focused
tests. It ran:

```text
cargo nextest run -p openwepp-hillslope-orchestrator surface_liquid --profile quick
25 passed / 507 skipped

cargo nextest run --test land_surface_energy_real_hydrology_shadow_contract --profile quick
13 passed / 0 skipped
```

## Material findings

### B-REMEDIATION-HIGH-001 — Actual receivers are mutated, but their ending equations are not independently reconstructed

`apply_production_infiltration()` now correctly calls the shared production
`apply_direct_same_pass_infiltration()` transition on the cloned bound lane and
recomputes aggregate soil water
(`land_surface_energy_shadow/mod.rs:681-704`). The bridge also credits the named
soil-thermal layer and retained LSE tile
(`land_surface_energy_shadow/mod.rs:605-678`). Those corrections genuinely add
the previously missing real recipients.

The acceptance boundary nevertheless stops after applying the receipt-derived
amounts. `apply_ingress_to_real_receivers()` returns immediately after the
production helper (`land_surface_energy_shadow/mod.rs:448-476`), without
independently reconstructing the ordered beginning-to-ending soil-layer deltas
or validating the soil-thermal and retained-LSE ending equations. The separate
closure validator consumes only the surface owner, frozen ingress, ending
surface state, and producer receipts
(`surface_liquid_closure.rs:174-184,356-368`); it does not consume the ending
`DirectRunFrame`, soil-thermal candidates, or LSE tile candidates. The focused
receiver test checks only that the first soil layer increased and that energy
credits equal sums of those same producer receipts
(`land_surface_energy_real_hydrology_shadow_contract.rs:1035-1070`).

Consequently a wrong distribution among production soil layers, or an
incorrect receiver ending value that is self-consistent with a producer
receipt, is not rejected by an independent owner reconstruction. This leaves
the version-5 requirement to independently reconstruct the resulting ordered
soil-layer mass deltas and validate every owner join incomplete. Add frozen
beginning/ending receiver operands and independent mass/enthalpy equations,
with poisons that distinguish wrong-layer distribution, omitted/doubled
thermal credit, and omitted/doubled retained tile credit.

### B-REMEDIATION-HIGH-002 — The canonical `SURFACELIQUID-E-001..011` failure contract is not implemented end to end

The new failure payload type exists, but the generic error mapping reaches only
`E-001`, `E-002`, `E-003`, `E-005`, `E-006`, and `E-010`
(`surface_liquid_owner.rs:154-171`). `E-004`, `E-007`, `E-008`, and `E-011`
have no production construction site; they appear only in the enum/string
table and an enumeration test. In particular:

- cadence/continuation failures are returned as generic `Identity` or `Domain`
  and therefore become `E-002` or `E-003`, although the canonical guard table
  assigns them to `E-008` (`surface_liquid_ingress.rs:442-477,745-754`);
- receiver/rollback-envelope failures return
  `LandSurfaceEnergyShadowError::Identity`, not a contextual
  `SURFACELIQUID-E-011` payload
  (`land_surface_energy_shadow/mod.rs:478-550`);
- the ingress wrapper always reports `attempted_owner_sha256=None` and only the
  transaction context (`surface_liquid_ingress.rs:273-295`), even after a
  candidate clone exists; the regression test explicitly blesses the missing
  attempted hash (`surface_liquid_ingress.rs:1658-1679`); and
- applicable OFE/tile/surface/source/parcel context remains absent from these
  public failures.

This does not satisfy the accepted finding requiring exact canonical error
identity and rollback context. Route each guard branch to its specified code
and phase, preserve available typed identities, compute the attempted owner
hash from the isolated candidate when available, and translate unified
receiver/envelope failures to `E-011`. Add behavioral tests for every reachable
guard code; enumerating the variants is not evidence that the runtime emits
them.

## Confirmed corrections

- Strict initial versus accepted restart combinations are enforced during
  state validation, including null-lineage interval-zero/zero-carry and
  nonnull-lineage interval `1..=48` rules.
- Configuration and the unified bridge bind topology rank, production lane
  index/ID, exact ordered soil-layer identities, first thermal receiver, run,
  bitwise OFE area, day, interval, and 1800-second cadence.
- Surface `D/A/F` preserves exact typed source identity, uses one immutable
  beginning store, authorizes proportionally, and debits finalized use only.
- Signed condensation is credited before capacity overflow; overflow becomes
  a timed mass/enthalpy parcel rather than being clipped.
- The surface-store equation explicitly reconstructs
  `W0 - F/f_t + C/f_t - overflow/f_t + retained/f_t`, and parcel total
  mass/enthalpy plus unequal-area routing are reconstructed from frozen ingress
  operands rather than ledger residuals.
- Production WB14 and the 1800-second continuation enter one shared interval
  transition. The shadow uses zero legacy depression custody by construction.
- All work remains clone/candidate-only. The reviewed code adds no runner
  selector, production dispatch, default, publication, or production-state
  mutation path.

The remaining findings are in-scope implementation defects. They do not
justify a new authority package or a new HOLD boundary.
