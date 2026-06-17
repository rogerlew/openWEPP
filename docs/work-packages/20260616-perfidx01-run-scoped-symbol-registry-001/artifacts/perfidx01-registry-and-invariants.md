# PERFIDX01 Registry And Invariants

Status: PASS 2026-06-16
Evidence mode: **Static** + **Ran**

## Implementation

PERFIDX01 added the Stage 1 registry surfaces required by ADR-0022 without
flipping runtime storage authority:

- `SymbolId(u32)` in `openwepp-kernel-contract`.
- Frozen `SymbolRegistry` with sorted `Vec<BoundarySymbol>` and
  `BTreeMap<BoundarySymbol, SymbolId>` reverse lookup.
- `BTreeMap` to registry export adapter that returns entries in id order and
  fails closed on unknown symbols.
- Thread-local validation hook behind
  `OPENWEPP_SYMBOL_REGISTRY_AUDIT_PATH`.
- Hillslope runner audit wiring that is inactive unless the env var is set.

The authoritative runtime storage remains the existing `BTreeMap` surface.

## Sorted-Id Invariant

Static: `SymbolRegistry::from_symbols` sorts and deduplicates all symbols before
assigning `SymbolId` by vector index. Therefore id order is the sorted
`BoundarySymbol::as_str()` order for every registry built through the only public
constructor.

Ran:

```text
cargo test -p openwepp-kernel-contract symbol_registry_assigns_ids_in_sorted_symbol_order
```

Result: PASS as part of `cargo test --workspace`.

The same constructor built every real-run registry in the completeness audit
cohort below.

## Equality Adapter Invariant

Ran:

```text
cargo test -p openwepp-kernel-contract symbol_registry_export_surface_matches_btreemap_order_after_sort
```

Result: PASS as part of `cargo test --workspace`.

This validates that registry id-ordered export preserves the current sorted
`BTreeMap` key order for registered surfaces.

## No Lazy Interning / Fail-Closed Invariant

Static:

- `SymbolRegistry::id_of` returns `SymbolRegistryError::UnknownSymbol` for
  absent symbols.
- `export_surface_in_id_order` calls `id_of` for every surface key and therefore
  fails instead of lazily interning.
- The validation hook records every `BoundarySymbol::new` while active and
  returns a typed `HillslopeCliError::RuntimeSurfaceFailure` if any post-freeze
  symbol is missing.

Ran:

```text
cargo test -p openwepp-kernel-contract symbol_registry_audit_records_post_freeze_unknowns
```

Result: PASS as part of `cargo test --workspace`.

## Real-Run Completeness Audit

All real-run audits were rerun with the same compatibility sidecar flags used by
the PERFOPT01 anchor:

```text
--policy compat --legacy-sidecar-discovery
```

Reports were written to `/tmp/perfidx01/audit/*.json`.

| Case | Registry symbols | Constructed symbols | Unknown symbols | Timing |
|---|---:|---:|---:|---|
| `ofe1` | 18219 | 2134 | 0 | `PERFIDX01_AUDIT_COMPAT case=ofe1 elapsed_s=9.01 user_s=8.98 sys_s=0.01 maxrss_kb=24576` |
| `ofe2` | 34071 | 2217 | 0 | `PERFIDX01_AUDIT_COMPAT case=ofe2 elapsed_s=19.75 user_s=19.71 sys_s=0.03 maxrss_kb=28760` |
| `ofe3` | 49904 | 2228 | 0 | `PERFIDX01_AUDIT_COMPAT case=ofe3 elapsed_s=29.84 user_s=29.78 sys_s=0.04 maxrss_kb=31516` |
| `ofe4` | 66295 | 2583 | 0 | `PERFIDX01_AUDIT_COMPAT case=ofe4 elapsed_s=49.37 user_s=49.34 sys_s=0.02 maxrss_kb=26764` |
| `ofe5` | 81600 | 2250 | 0 | `PERFIDX01_AUDIT_COMPAT case=ofe5 elapsed_s=47.21 user_s=47.15 sys_s=0.05 maxrss_kb=29952` |
| `h2637` | 1699798 | 3616 | 0 | `PERFIDX01_AUDIT_COMPAT case=h2637 elapsed_s=1580.81 user_s=1579.02 sys_s=1.49 maxrss_kb=427360` |
| `h2637_with_ui` | 1699798 | 3616 | 0 | `PERFIDX01_AUDIT_COMPAT case=h2637_with_ui elapsed_s=1578.75 user_s=1576.16 sys_s=2.32 maxrss_kb=428128` |

Audit summary command:

```text
h2637: registry=1699798 constructed=3616 unknown=0
h2637_with_ui: registry=1699798 constructed=3616 unknown=0
ofe1: registry=18219 constructed=2134 unknown=0
ofe2: registry=34071 constructed=2217 unknown=0
ofe3: registry=49904 constructed=2228 unknown=0
ofe4: registry=66295 constructed=2583 unknown=0
ofe5: registry=81600 constructed=2250 unknown=0
```

Disposition: PASS. No post-freeze unknown symbols were observed across H2637
both UI variants and the OFE1-5 ladder.

