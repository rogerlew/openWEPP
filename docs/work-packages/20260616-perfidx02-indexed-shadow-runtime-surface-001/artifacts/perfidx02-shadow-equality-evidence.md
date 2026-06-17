# PERFIDX02 Shadow Equality Evidence

Status: PASS 2026-06-16
Evidence mode: **Ran**

The indexed shadow is env-gated by `OPENWEPP_INDEXED_SHADOW_REPORT_PATH`.
With the hook enabled, the runner builds `IndexedWritebackSurface` from the
authoritative BTreeMap surfaces, exports it back through the frozen registry,
and records mismatches. The BTreeMap remains authoritative.

## Cohort

| Case | Registry symbols | Clone-source observations | Equality checks | State entries checked | Flux entries checked | Max clone-source entries | Mismatches | Sparse clone win |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| OFE1 | 2,847 | 2,192 | 2,192 | 4,764,754 | 81,104 | 1,747 | 0 | 55.491x |
| OFE2 | 3,327 | 4,384 | 2,192 | 5,159,332 | 81,104 | 2,404 | 0 | 54.623x |
| OFE3 | 3,788 | 6,576 | 2,192 | 5,174,612 | 81,104 | 2,411 | 0 | 58.900x |
| OFE4 | 4,807 | 8,768 | 2,192 | 5,926,576 | 107,408 | 2,762 | 0 | 57.785x |
| OFE5 | 4,740 | 10,960 | 2,192 | 5,231,712 | 81,104 | 2,441 | 0 | 61.928x |
| H2637 without UI | 44,746 | 235,961 | 12,419 | 49,999,262 | 608,531 | 4,087 | 0 | 69.882x |
| H2637 with UI | 44,746 | 235,961 | 12,419 | 49,999,262 | 608,531 | 4,087 | 0 | 54.096x |

Reports:

```text
/tmp/perfidx02/shadow/ofe1.json
/tmp/perfidx02/shadow/ofe2.json
/tmp/perfidx02/shadow/ofe3.json
/tmp/perfidx02/shadow/ofe4.json
/tmp/perfidx02/shadow/ofe5.json
/tmp/perfidx02/shadow/h2637.json
/tmp/perfidx02/shadow/h2637_with_ui.json
```

## Registry Completeness

The tightened production registry also passed the no-lazy-interning audit:

| Case | Registry symbols | Constructed symbols | Unknown symbols |
|---|---:|---:|---:|
| OFE1 | 2,847 | 2,847 | 0 |
| OFE2 | 3,327 | 3,327 | 0 |
| OFE3 | 3,788 | 3,788 | 0 |
| OFE4 | 4,807 | 4,807 | 0 |
| OFE5 | 4,740 | 4,740 | 0 |
| H2637 without UI | 44,746 | 44,746 | 0 |
| H2637 with UI | 44,746 | 3,616 | 0 |

Compared with PERFIDX01's H2637 registry capacity of about 1.7M symbols, the
reachable production registry is now 44,746 symbols for H2637.
