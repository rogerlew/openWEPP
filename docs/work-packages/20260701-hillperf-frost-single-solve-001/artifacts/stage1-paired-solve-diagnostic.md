# Stage 1 — Paired Frost-Solve Diagnostic (H2637)

Evidence class: Ran (instrumented capture + analysis; commands below).
Verdict: **MATERIALLY DIVERGENT** → Stage 2 adjudication memo required
(operator yes/no per the package's execution model).

## Method

Both frost-partition call sites — the runner day-input builder
(`frost_day_context`) and the executor R4A span
(`reconcile_r4a_frost_runtime`) — emit one JSONL row per solve when
`OPENWEPP_WP2_FROST_PAIR_TRACE_PATH` is set (shared formatter
`write_wp2_frost_pair_trace`, exported from the orchestrator so the two
sides cannot drift; inert when unset — hooks-inert H2637 identity gate
CLEAN against the frozen WP-1 baseline hashes). One traced H2637 run
(60.92 s with tracing) captured 471,922 rows; paired and diffed by
(lane, day) with `wp2_pair_analysis.py`.

## Structural findings

- **Perfect 1:1 pairing:** 235,961 pairs, zero builder-only, zero r4a-only,
  zero duplicates — every OFE-day solves exactly twice.
- **The solve runs on every OFE-day, not only winter days.** 471,922 total
  solves = 2 × the full OFE-day count. F1's cost basis is therefore the
  whole run, not a winter subset; the single-solve saving is a full half of
  the dominant frost block in the exit profile.
- **Prior frost state is bit-identical on both sides** (`in_prior_dfrost_m`
  and `in_prior_ws_frz_m` deltas exactly 0 on all 235,961 pairs) — both
  solves start from the same carried frost history; divergence is driven
  entirely by the liquid/layer state.

## Divergence (r4a − builder, absolute)

| Field | max | p99 | # ≠ 0 | # > 1 mm |
|---|---:|---:|---:|---:|
| `in_soil_water_m` (input) | 1.44e-01 | 5.99e-02 | 235,961 | 207,625 |
| `out_frost_depth_after_m` | **2.45e-01** | 6.81e-03 | 42,619 | 8,483 |
| `out_frozen_water_after_m` | 1.80e-02 | 5.55e-05 | 42,634 | 661 |
| `out_thdp_after_m` / `out_dthaw_after_m` | 2.45e-01 | 1.46e-03 | 23,738 | 2,996 |
| `out_infcap_frz_m_s` | 1.67e-05 (full frozen↔unfrozen range) | 5.63e-07 | 42,136 | — |
| `out_frwatc_net_liquid_delta_m` | 7.81e-02 | 4.94e-04 | 49,088 | 1,647 |
| `out_fgthwd_flag_after` (flag flips) | 1.0 | — | 389 | 389 |
| `out_total_fine_layer_count` | 0 | 0 | 0 | 0 |

Worst cases are qualitative disagreements, not noise: lane 18 day 8396 —
builder frost depth 0.002 m vs r4a 0.247 m; lane 1 day 4464 — builder
0.202 m vs r4a 0.0, with `infcap_frz` flipped between fully-frozen
(1.67e-10 m/s) and unfrozen (1.67e-5 m/s) on the same day.

## The hybrid-consumption finding

Production today consumes **both** disagreeing trajectories on the same
day: the builder's outcome gates that day's infiltration
(`infcap_frz_m_s`) and seeds the storage-liquid pre-projection, while the
R4A outcome is what mutates the frame, feeds publication projection, and
commits to carry. On the 42k divergent lane-days, the day's infiltration
behavior and its published/carried frost state come from two solves that
disagree — an internal-consistency defect independent of which solve is
correct.

## Authority evidence for Stage 2

- **Legacy ordering (Ran — grep of the pinned 260430 baseline):**
  `contin.for` daily sequence is `call soil` (→ `frsoil`, soil.for:656) at
  :812 → `call winter` :849 → `call irs` (infiltration) :964 →
  `call watbal` (ET/percolation) :1067. Legacy solves frost **once per
  day, on start-of-day state, before infiltration**; a post-ET re-solve
  has no legacy analogue.
- **Contract (Static):** `SC-SNOWFREEZE-001#INV-SNOWFREEZE-012` binds the
  frost water-state handoff to **`frwatc(1)` once at active-day hour-1
  ingress** and `frwatc(0)` at day-end, explicitly prohibiting re-applied
  daily water-balance deltas. The twice-daily solve has no contract
  authority; the once-at-ingress language supports the start-of-day single
  solve.

## Reproduction

```bash
OPENWEPP_WP2_FROST_PAIR_TRACE_PATH=<out.jsonl> target/release/openwepp-cli-hill \
  --run-dir <h2637 runs> --run-file <h2637.run> --output-dir <out> \
  --policy compat --legacy-sidecar-discovery
python3 wp2_pair_analysis.py <out.jsonl>   # script archived in this artifacts dir
```
