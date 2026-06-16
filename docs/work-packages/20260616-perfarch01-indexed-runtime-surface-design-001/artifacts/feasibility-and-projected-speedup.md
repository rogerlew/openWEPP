# PERFARCH01 Feasibility And Projected Speedup

Status: COMPLETE 2026-06-16
Evidence mode: **Ran** (prototype) + **Static** (PERFOPT01/PERFHO02 attribution)

## Prototype

Prototype source:

```text
docs/work-packages/20260616-perfarch01-indexed-runtime-surface-design-001/artifacts/prototypes/indexed_surface_microbench.rs
```

Ran:

```text
rustfmt docs/work-packages/20260616-perfarch01-indexed-runtime-surface-design-001/artifacts/prototypes/indexed_surface_microbench.rs
rustc -O docs/work-packages/20260616-perfarch01-indexed-runtime-surface-design-001/artifacts/prototypes/indexed_surface_microbench.rs -o /tmp/perfarch01_indexed_surface_microbench
/tmp/perfarch01_indexed_surface_microbench
```

Result:

```text
symbols=6396
lookup_ops=1944
sorted_id_order_matches_string_sort=true
clone_btreemap_ns_per=464296.16
clone_dense_ns_per=4226.56
clone_speedup=109.85x
lookup_btreemap_format_ns_per_op=303.65
lookup_dense_precomputed_ns_per_op=1.39
lookup_speedup=219.16x
update_btreemap_clone_insert_ns_per_batch=815942.34
update_dense_clone_set_ns_per_batch=7048.14
update_batch_speedup=115.77x
```

The prototype models 6,396 symbols across static, climate, WB18/WB19, frost,
PL, and MOFE transfer families. It compares the current physical pattern
(`BTreeMap<String, f64>` plus formatted lookup strings) with sorted-id dense
`Vec<Option<f64>>` storage and pre-resolved ids. It is not a full openWEPP
benchmark, but it directly measures the storage operations PERFHO02 identified.

## Interpretation

The storage primitive is fast enough. Clone, lookup, and update batches were
roughly 110x, 219x, and 116x faster in the prototype. The sorted-id invariant
also held, so string-order compatibility is practical rather than speculative.

PERFOPT01 H2637 after-time is the current performance anchor:

```text
h2637 elapsed_s=849.86
```

The legacy high-OFE comparator is approximately 10 seconds for the same class of
run, so a <=10x target means about <=100 seconds. From 849.86 seconds, that
requires about an 8.5x total speedup. A <=5x target means about <=50 seconds and
requires about 17x total speedup.

Using Amdahl's law with 50-100x accelerated storage operations:

| Migrated elapsed share | 75x accelerated total speedup | Projected H2637 time | Legacy ratio if legacy is 10 s |
|---:|---:|---:|---:|
| 80% | 4.75x | 179 s | 17.9x |
| 85% | 6.20x | 137 s | 13.7x |
| 90% | 8.93x | 95 s | 9.5x |
| 93% | 12.14x | 70 s | 7.0x |
| 96% | 18.94x | 45 s | 4.5x |

With a 50-100x primitive speedup, <=10x requires migrating roughly 89-90% of the
current elapsed time out of string-keyed surface mechanics. <=5x requires roughly
95-96%.

## Feasibility Verdict

The indexed runtime surface is feasible and should be the next architecture
track. It is the first evidence-backed path that can plausibly reach <=10x
because the measured primitive speedups are two orders of magnitude and the hot
surface touches almost every OFE-day phase.

The <=10x target is plausible but conditional. PERFHO02 shows 96.24% children
under the scheduler lifecycle and broad dominance of symbol access, dynamic
symbol formatting, guard scans, and writeback application, but it does not prove
that all 96.24% is removable map/string overhead. Stage implementation must
measure the migrated share after each step.

The <=5x target is not credible as a storage-only promise. It requires >=95%
effective migration at 50-100x speedup, or additional non-storage wins after the
indexed surface is in place. Treat <=5x as an aspirational second target after
the full indexed surface is measured.

## Decision For Follow-On

Proceed to staged implementation. The first implementation package should build
and freeze the run-scoped registry, prove sorted-id/string-order equivalence, and
add equality adapters without changing production behavior.
