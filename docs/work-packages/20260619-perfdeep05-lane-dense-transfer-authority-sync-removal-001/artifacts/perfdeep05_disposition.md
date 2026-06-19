# PERFDEEP05 Disposition

Evidence class: Ran + Static.

## Verdict

`NO-GO - sync hotspot removed, endpoint still fails activation gate`.

PERFDEEP05 achieved the local implementation goal: the PERFDEEP04
`sync_from_writeback_surface` hotspot is removed from the PERFDEEP03 opt-in
H2637 daily hot loop, direct dense transfer updates work, and final H2637
identity passes.

It did not achieve the performance goal. Final-code opt-in H2637 measured
`911.11 s`, which is:

- `+241.14 s` versus the PERFDEEP01 `669.97 s` reference;
- `+209.16 s` versus the final default-disabled `701.95 s` run;
- `1.298x` slower than final default-disabled execution.

No default activation is allowed.

## Acceptance Matrix

| Criterion | Result |
|---|---|
| `sync_from_writeback_surface` absent from opt-in daily scheduler hot loop | PASS |
| Direct dense transfer update tests | PASS |
| Default-disabled path remains available | PASS |
| HBP byte identity | PASS |
| WAT byte identity | PASS |
| PASS Arrow equivalence | PASS |
| Final opt-in endpoint measured against `669.97 s` | PASS, but gate failed |
| PERFDEEP04 hotspot re-profiled or disproven | PASS: old sync absent; new dense-edge costs identified |
| Full Rust gates | PASS |
| Markdown lint | See `perfdeep05-gate-results.md` |

## Decision

Keep the PERFDEEP05 implementation as an opt-in diagnostic/performance
experiment. Do not revert to the old full resync path, because it was measured
at `33.49%` inclusive in PERFDEEP04 and is now gone without breaking identity.

Do not expand the island or activate by default inside this package. The
remaining profile shows that the partial dense island is still paying
compatibility-edge costs:

- `refresh_cached_slots_from_writeback_surface`: `16.20%` children,
  `9.07%` self;
- `apply_kernel_writeback_payload`: `10.47%` children;
- `SymbolRegistry::id_of`: `7.72%` children, mostly under dense logical
  writeback apply;
- `flush_dirty_to_writeback_surface`: `6.72%` children.

## Follow-On Recommendation

Open a new package before more implementation:

```text
PERFDEEP06 - Lane-Dense Edge Cost Removal / Indexed Dense Writeback Authority
```

Candidate objective: remove the remaining measured dense-edge costs by moving
daily refresh and writeback application away from logical symbol lookup, or
prove that the partial island cannot become endpoint-flat without the larger
kernel-body array-native rewrite.

PERFDEEP06 should start from this profile, not from a revert.
