# Carrier parent-static and same-map validation-once contract cycle

Evidence mode: `Static + Ran + Expected-red`

Base commit under review: `a28c55c2d0f06e0c4aab58642f1009f70f82b3d3`.
This authorized uncommitted contract-first increment amends
`SC-LANDSURFACEENERGY-001` to version 30, maps to existing
`INV-LANDSURFACEENERGY-159`, and adds `OBL-LANDSURFACEENERGY-C-019`. It creates
no invariant, solver version, production runtime, or performance claim.

## Source-real authority

Static: the parent supplied a measured `103059 us` target: `39740 us` parent
static/forcing work plus `63319 us` repeated native validation. Source inspection
found three distinct opportunities and boundaries:

- the current first exact forcing validation precedes V8 and may authorize only
  V8's later validation of that pointer-identical forcing;
- V8 validates structural LSE/surface objects, not the distinct resident V3 LSE
  and V2 surface objects; fallible ingress derivation remains between V8 and the
  native projector; and
- `FrozenLitterV3Resident` already carries a private validated revision covering
  the exact resident objects and history lineage. A borrowed non-Clone map proof
  may be minted from that revision only at the native-validation position and
  consumed immediately to omit the two repeated resident validations.

The existing resident and revision derive `Clone`; v30 allows this only as an
inseparable private clone of the exact whole immutable resident. The new parent
plan, forcing proof, and borrowed resident map proof remain non-Clone/non-wire.
Every successor is fully validated before the revision advances atomically;
restart reconstructs full validated custody. No dynamic state/result cache,
digest-only admission, transfer, reuse, public API, or fallback is authorized.

The expected-red population requires authentic `1/52/52` parent-static,
forcing, and dynamic-validation counts; per-regime applicable role/path parity;
ordinary zero-native use; exact source order; distinct structural/native and
proof-custody poisons; dynamic vegetation/surface/soil-hydrology,
solver/residual, and output poisons; paired first-error precedence through all
of those later boundaries in vegetation -> surface -> soil/hydrology ->
solver/residual -> output order; and rollback. Its source
guard covers the intended new owner plus the actual carrier, V8, ingress,
resident, and native seams and is supplemental to executable evidence.

## Reproducible validation

Ran: strict BEI lint:

```text
.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md
```

Result: `PASS`, 14 fully consolidated rows.

Ran: contract unit compliance:

```text
bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md
```

Result: `PASS`, no findings.

Ran: scoped whitespace gate:

```text
git diff --check -- docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md docs/specifications/science-contracts/index.md tests/integration/land_surface_energy_balance_authority_contract.rs crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_adaptive_production_tests.rs docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/artifacts/science-contracts/carrier-parent-static-validation-once/contract_ref.md docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/artifacts/science-contracts/carrier-parent-static-validation-once/readiness-matrix.md docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/artifacts/science-contracts/carrier-parent-static-validation-once/disposition.md
```

Result: `PASS`.

Ran: focused contract-derived assertion:

```text
env RUST_MIN_STACK=67108864 nix develop -c cargo nextest run --test land_surface_energy_balance_authority_contract version_thirty_binds_parent_static_and_same_map_validation_once_to_existing_invariant
```

Result: `PASS`, 1/1.

Expected-red: focused orchestrator compile:

```text
env RUST_MIN_STACK=67108864 nix develop -c cargo test -p openwepp-hillslope-orchestrator --lib carrier_parent_static_and_same_map_validation_once_has_authentic_1_52_52_counts --no-run
```

Result: `EXPECTED_RED`; compilation fails on the absent intended owning module
and absent real validation-once typestate/audit/oracle APIs. This is the required
pre-implementation classification, not a runtime failure or passing evidence.

Command output was observed in this worktree but no durable log file is claimed;
the exact commands above are the reproduction authority.

## Ordered manifest

The manifest covers, in this exact order:

1. `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`
2. `docs/specifications/science-contracts/index.md`
3. `tests/integration/land_surface_energy_balance_authority_contract.rs`
4. `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_adaptive_production_tests.rs`

Recipe:

```text
sha256sum <the four paths above in the listed order> | sha256sum
```

Ordered manifest SHA-256:
`f6bd360c2711b5ec4fce212a56dd4dc1167567cfeca863fd4c156c1a33a748be`.

The procedure-compliant finding ledger is `disposition.md`; it covers
A-001 through A-006, B-01 through B-04, and B-FINAL-01.

Production implementation remains gated on independent re-review and
verification of this corrected authority.
