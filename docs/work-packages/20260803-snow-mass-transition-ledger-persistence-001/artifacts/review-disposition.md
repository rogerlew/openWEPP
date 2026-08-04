# Review Finding Disposition

Status: `all current-scope findings accepted and remediated`

Evidence mode: `Static + Ran`

| Source | Severity | Finding | Decision and action |
|---|---|---|---|
| A | High | constructor/live-frame layout ceilings exceeded | accepted; boxed constructor ledger and already-optional shadow record; layout guard passes at `4112/15552 B` |
| A+B | Medium | typed ledger errors erased at two seams | accepted; added typed wrapper variants, source/display/code mapping, and tests |
| A | Medium | producer/persisted thresholds could drift | accepted; centralized both mass-boundary constants and reused `WB11_ZERO_THRESHOLD` |
| A | Low | public by-value downstream signature changed | accepted; restored the signature and retained a narrow compatibility lint rationale |
| B | Medium | disabled-vs-verbose test missed real melt/Stage 3 | accepted; replaced with warm layered, nonzero-melt Stage-3 fixture |
| B | Medium | signed operands and failure variants under-tested | accepted; added negative raw/retained valid case and category cases |
| B | Medium | unchecked constructor/public allocation helper debt | accepted; narrowed authoritative constructor and removed unused public helper |
| B | Medium | trace writer argument/line Clippy failures | accepted; extracted request/row context and retained one ordered-schema formatter rationale |
| B | High | avoidable deep clone at by-value seam | accepted; authoritative state is stored, then the local is moved |
| A+B | High | comparator, allocation, line-count, API, and gate artifacts stale | accepted; rebuilt exact candidate, expanded paired protocol, refreshed terminal inventory/API/allocation evidence, and reran required gates |

No finding was rejected or deferred. The 48-byte residual headroom at both
layout ceilings is recorded as non-blocking debt rather than hidden.
