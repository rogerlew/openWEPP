# Hillslope direct runtime: sub-5× performance assessment

State: `concept` — evidence-ranked refactor assessment; promotable to a
work-package series.
Author: Claude Code, 2026-07-01.
Evidence class: **Ran** for all timings and the profile (commands in §6);
**Static** for source-mechanism attribution (file:line cited per finding).
Disposition of each finding (fix shape, sequencing, contract adjudication)
belongs to the executing package, per the review-artifact convention.

## 0. Summary

The H2637 single-hillslope direct endpoint measured **~71.4 s median
(70.16 / 72.63 s)** against **~9.65 s legacy (9.63 / 9.67 s, wepp_260430_hill,
same host, same day)** → **7.40× legacy**. The `<=5x` bar on this host is
**≤48.2 s**: ~23 s must come out.

A DWARF call-graph `perf` profile (15,549 samples) shows the remaining gap is
**not** spread across the runtime — it is concentrated in the **winter
(snow/frost) subsystem, ~64% of wall**, and inside it three overhead classes
that are not physics:

1. the hourly frost partition is **solved twice per winter OFE-day**
   (§F1, ~34% + ~34% inclusive);
2. guard **symbol `String` construction on the success path** of the winter
   kernel (§F2, ~10–16%);
3. same-day **duplicate hourly-forcing construction** and per-solve
   re-derivation of cacheable curves (§F3, §F5).

The previously-hypothesized costs are measured **non-problems**: the per-day
`DirectDayFrame` lifecycle (seed/apply/commit/drop) totals **~3%**, and the
streaming publication sink does not register. The `<=5x` target is reachable
by either F1 alone plus small findings, or F2–F7 without F1; both lanes
together project to **~3.5–4.5×** (§4).

## 1. Measured baseline (this host, 2026-07-01)

Host: dual Xeon E5-2697 v2 @ 2.70 GHz (the wshedperf01 host). Binary:
`target/release/openwepp-cli-hill` built from `main@50b38d77` (worktree carried
uncommitted **watershed-only** changes; hillslope surface unaffected — verified
via `git diff --stat`). Fixture: H2637 (19 OFE × 12,419 days = 235,961
OFE-days), staged from the canonical wepp-forest WB05A replay inputs.

| Run | Wall | Max RSS |
|---|---:|---:|
| openWEPP direct, rep 1 | 70.16 s | 82,144 KiB |
| openWEPP direct, rep 2 | 72.63 s | 83,248 KiB |
| legacy `wepp_260430_hill`, rep 1 | 9.63 s | 4,608 KiB |
| legacy `wepp_260430_hill`, rep 2 | 9.67 s | 4,608 KiB |

Ratio ≈ **7.40×** (median/median). Budgets on this host: `<=10x` = 96.5 s
(passing), `<=5x` = **48.2 s** (gap ≈ 23 s), per-OFE-day ≈ 302 µs now vs
204 µs at 5×.

Both openWEPP runs exercised the production default path
(`--policy compat --legacy-sidecar-discovery`, no runtime selector — direct is
the only runtime per ADR-0031), all five outputs enabled, exit 0.

## 2. Where the time goes (profile)

`perf record --call-graph dwarf,16384 -F 199` over a full run; 15,549 samples.
Inclusive (children) percentages; overlapping — they do not sum.

| Cost center | Inclusive | Self | Notes |
|---|---:|---:|---|
| `compute_direct_winter_frost_partition` (both call sites) | **63.6%** | 0.3% | THE cost center |
| — via runner day-input builder `frost_day_context` | 34.3% | | start-of-day lane state |
| — via executor `run_r4a_runoff_partition_span_with_winter_frost` | 33.9% | | evolved frame state |
| `compute_active_frost_hourly_state` (hourly loop body) | 23.0% | 7.4% | inside both solves |
| `alloc::fmt::format_inner` (String formatting, total) | **17.8%** | 1.5% | §F2 — guard symbol names |
| `legacy_tmpadj_surface_temperature_from_typed` | 16.8% | 2.0% | per-hour; 7 symbol Strings per call |
| `derived_frost_depths_from_fine_state` | 9.8% | 4.8% | + `round` 4.3% incl (3.6% self) |
| `fit_legacy_tmpcft_curve` | 8.0% | 1.3% | fitted curve, re-derived per solve |
| kernel-mode (page faults / syscalls) | 7.9% | 7.9% | tracks allocator churn + output writes |
| allocator machinery (`malloc`/`free`/`realloc`/`raw_vec`) | — | **~15.8%** | large share under format/frost paths |
| `memmove`/`memcpy` (clones) | — | ~8.2% | |
| `build_simimpl28_hourly_winter_forcing_typed` | 6.0% | 0.3% | **two** builder call sites/day (§F3) |
| — inside it: Harder–Pomeroy psychrometric phase | 3.3% | | iterative solve per hour |
| `snow_liquid_partition` (builder) | 5.9% | 0.1% | |
| `run_r4o_subsurface_compute_span` / `run_r4m_percolation_span` | 4.3% / 4.3% | 0.3% / 2.1% | the biggest non-winter phases (`run_lateral` 3.5%) |
| transcendentals (`sincos`/`cos`/`sin`/`exp`/`pow`/`log`) | — | ~10% | winter forcing + frost energy terms |
| float **parsing** at runtime (`dec2flt`) | — | ~1.1% | §F7 |
| **day-frame lifecycle** (`seed_day_frame` + `DayFrame::seed` + `apply_publication_day_input` + `commit` + `drop`) | **~3.1%** | | measured non-problem |

## 3. Findings (ranked by recoverable wall time)

### F1 — The winter frost partition is solved twice per winter OFE-day (~up to 21 s)

**Ran:** the two inclusive call chains above are near-identical in weight
(34.3% vs 33.9%; the `round` leaf splits 1.69% vs 1.60% across them with the
same interior chain).

**Static:** the runner's day-input authority solves the full hourly frost
partition from **start-of-day lane state** to pre-project day inputs —
frozen infiltration capacity `infcap_frz_m_s`, `storage_liquid_delta_m`,
same-day hydrology layers
(`crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs:287`,
consumed at `:145-151`). The executor's R4A runoff span then **re-solves the
same day** — same `DirectWinterFrostComputeInputs` (controls, thermal, hourly
forcing array, passed through the day input) — against the frame's
**post-ET/percolation evolved layer state**
(`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:916-944`),
and that outcome is what mutates the frame and commits to lane carry.

Snow shows the opposite, single-solve pattern: the builder solves the snow
liquid partition once and the frame's snow-coupling span **consumes** the
precomputed outputs. The frost double-solve is a residue of the R7G retrofit
lineage, not a deliberate two-pass design anywhere in contract.

**Candidate directions** (executing package to adjudicate):
(a) single frame-internal solve early in the day whose operands feed both the
infiltration inputs and R4A (mirrors the snow pattern; legacy runs frost once
per day before infiltration — static claim, verify against the pinned
baseline before relying on it);
(b) carry the builder outcome into the frame and let R4A consume-and-adjust
instead of re-solving.

**Yield / risk:** eliminating one solve ≈ up to −21 s (−30%). **Identity-
affecting unless the surviving solve is the one whose outputs currently feed
publication and carry** (that is the R4A solve). Whether a start-of-day or
evolved-layer liquid state is the contract-correct partition input is a
science-contract question (`SC-SNOWFREEZE-*`), not a mechanical one — this is
the finding that needs contract adjudication, and it is also the single
largest lever. Note the two solves differ *only* in soil-liquid/layer inputs;
controls, thermal, and the 24-hour forcing are byte-identical per day.

### F2 — Guard symbol `String`s constructed on the success path (~7–11 s now; ~4–6 s after F1)

**Ran:** `format_inner` 17.8% inclusive; `Display for usize` 4.75% (the
`{hour:04}` suffix); `String` growth/realloc chains under it;
`record_constructed_boundary_symbol` alone 1.27% self (a thread-local
side-effect of every `BoundarySymbol` construction).

**Static:** the winter kernel's typed guards materialize a heap `String`
symbol name *before* performing the range check, on every check, every hour,
every solve. Exhibit:
`legacy_tmpadj_surface_temperature_from_typed`
(`crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs:1518-1595`)
builds **7 `BoundarySymbol` Strings per call** — 3 × `hourly_symbol`
(`format!("{root}_{hour:04}")`) + 4 × `BoundarySymbol::from(&str)` (heap
alloc) — and it runs per hour × 2 solves ≈ ~336 allocations per winter
OFE-day in this one function. The same pattern recurs across the winter
guards via `typed_boundary.rs` (`hourly_symbol`, `wb18_perc_state_symbol`,
`wb19_*_symbol`, `frost_layer_symbol`, `frost_fine_layer_symbol` — all
`format!`-based) and `require_state_range` (String per check).

**Precedent:** R7H already fixed exactly this class once —
`require_frost_fine_state_range` checks first and formats only on failure —
and that single fix took the endpoint from 112.99 s to 61.40 s. The remaining
guards never got the same treatment.

**Direction:** check-first / format-on-failure across the winter guard
surface; a symbol type that can carry `&'static str` (or the typed enum)
until an error actually needs a formatted name. Behavior-preserving,
H2637-identity-preserving (error-path strings unchanged), CQR-shaped.

### F3 — 24-hour winter forcing built twice per day, with an iterative psychrometric solve per hour (~2–4 s)

**Ran:** `build_simimpl28_hourly_winter_forcing_typed` 6.0% inclusive, over
half of it the Harder–Pomeroy hydrometeor-temperature iteration (3.3%).

**Static:** two production call sites, both in the runner day-input builder —
frost hourly forcing (`00a_snow_frost_authority_impl.rs:168`) and the snow
liquid partition (`:357`). On days where both snow and frost are active, the
same (day, lane) forcing derivation runs twice. **Verification required**
before deduplication: confirm the two `DirectWinterHourlyContext` argument
sets are identical field-for-field (the like-for-like rule) — if they are,
build once per (lane, day) and share; identity-preserving.

### F4 — `derived_frost_depths_from_fine_state` recomputation and `round` (~2–4 s)

**Ran:** 9.8% inclusive, 4.8% self, plus `round` at 4.3% inclusive called
essentially only from here (and its thaw-feedback re-entry).

**Static:** the fine-layer → depth-summary derivation walks the full
fine-layer array (with per-element rounding) **per hour, per solve**, while
hourly freeze/thaw typically changes one boundary layer. Directions:
incremental depth tracking across hours; establish whether the `round` is a
contract/parity requirement or incidental quantization. Contract-gated
(`INV-SNOWFREEZE-*` frost-depth invariants); higher risk than F2/F3.

### F5 — `fit_legacy_tmpcft_curve` re-derived per solve (~2–4 s)

**Ran:** 8.0% inclusive. **Static:** the fitted temperature curve is
re-derived inside every partition solve; its inputs appear to be day-of-year,
monthly normals, and static thermal parameters — if confirmed, it is
cacheable per (lane, day) — or per day-of-year — instead of twice per day.
Verify input set first; identity-preserving if the cache key covers all
inputs.

### F6 — Clone/allocation tax outside the format paths (~3–5 s aggregate)

**Ran:** allocator ~15.8% self + memmove/memcpy ~8.2% total; a large share
sits under F1/F2 paths and falls with them; the rest is a long tail of
mechanical clone patterns. **Static** inventory (each small, all
behavior-preserving):

- Trace events constructed **before** the disabled-check, cloning two layer
  `Vec`s per OFE-day with tracing off:
  `subsurface.rs:375` + `diagnostic_events.rs:214-215`.
- `erosion_inputs.clone()` (contains `wave2.classes: Vec`) **before** the
  enabled-check: `erosion.rs:387-388`.
- Span pattern clones state twice and clones the just-built shadow projection
  into the span report instead of moving it (e.g. `subsurface.rs:386,396,415`;
  same shape in ET/storage/runoff/erosion spans).
- `seed_day_frame` re-collects the subsurface layer `Vec` twice per day
  (`00_core_frames.rs:638-647`); `commit_day` clones it back
  (`:1000/1013/1022`); `latest_r4a_frost_layers` clones it again
  (`runoff.rs:951-973`) plus `to_vec` at `:979`.
- Day-input builder allocates ~4–5 fresh 24-slot hyetograph `Vec`s per
  OFE-day plus `hyetograph.clone()`
  (`00_builders_and_authority.rs:2126`, `01_frost_and_layer_helpers.rs`).
- `apply_publication_day_input` re-clones ~9 `Option` inputs the builder
  just built (`03_executor.rs:413-462`) — a build→clone-in round trip.

Direction: move-not-clone, construct-behind-the-gate, reuse buffers held by
the builder/frame. The kernel-mode 7.9% (page-fault/heap churn) shrinks as a
side effect.

### F7 — `usize → String → f64` conversion in the hot diagnostic path (~0.5–0.8 s)

**Ran:** `dec2flt`/`parse_number` ~1.1% self — the binary is *parsing floats*
at simulation time. **Static:** `diagnostic_count_to_f64`
(`typed_boundary.rs:162-164`) does `value.to_string().parse::<f64>()` where
`value as f64` is exact for any realistic count. Trivial, identity-preserving.

### F8 — Telemetry micro-costs (~0.5–1 s)

~120–140 relaxed `AtomicU64` RMWs per OFE-day (`DIRECT_AUDIT`, every span
records 5–6) plus 14 `DirectPhaseView` constructions per OFE-day whose results
are discarded (executor `03_executor.rs:310-315` — counter-only loop).
Direction: single-threaded `Cell` counters or per-run aggregation; delete the
vestigial view loop. Small, free, zero-risk.

## 4. Composite path to `<=5x`

Overlaps accounted (F2/F3/F5 partially nest inside F1's duplicate):

| Lane | Findings | Projected endpoint | Ratio |
|---|---|---:|---:|
| Mechanical only (no contract adjudication) | F2 + F3 + F5 + F6 + F7 + F8 | ~50–54 s | ~5.2–5.6× |
| F1 only + trivia | F1 + F7 + F8 | ~48–50 s | ~5.0–5.2× |
| **Both** | F1 + F2..F8 | **~34–43 s** | **~3.5–4.5×** |

Neither lane alone reliably clears 48.2 s; together they clear it with
margin. Sequencing note: F2/F6/F7/F8 are CQR-shaped (behavior-preserving,
H2637 byte-identity gate per package); F1 is the contract-adjudicated
centerpiece and should be its own package with SC-SNOWFREEZE review in the
entry gate; F3/F5 need a like-for-like input verification step before the
dedup is legal; F4 is the highest-risk/lowest-certainty item — take it last,
only if the margin is still short.

## 5. Measured non-problems (do not spend effort here)

- **`DirectDayFrame` per-day lifecycle** — seed + apply + commit + drop ≈
  3.1% total. The ~200-field frame rebuild per OFE-day is *not* the
  bottleneck; frame pooling would be premature.
- **Publication** — the streaming sink, row construction, and parquet
  row-group writes do not register above noise; RSS is run-length-flat
  (~82 MiB).
- **Growth / decomposition phases** — allocation-free, arithmetic-only.
- **Non-winter hydrology** — percolation + subsurface + lateral ≈ 8–9%
  combined; real physics, correctly shaped; only worth touching via F6's
  clone patterns.

## 6. Reproduction

```bash
# staging: copy the H2637 WB05A replay inputs (p2637.{sol,man,slp,cli},
# pmetpara.txt, snow.txt, …) into a scratch run-dir; author a
# openwepp-hillslope-runfile-v1 TOML pointing at them, all five outputs on.
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
/usr/bin/time -f '%e s\t%M KiB' target/release/openwepp-cli-hill \
  --run-dir <runs> --run-file <h2637.run> --output-dir <out> \
  --policy compat --legacy-sidecar-discovery
# legacy anchor, same inputs:
(cd <runs> && /usr/bin/time -f '%e s\t%M KiB' \
  /home/workdir/wepppy/wepp_runner/bin/wepp_260430_hill < p2637.run)
# profile:
perf record --call-graph dwarf,16384 -F 199 -- target/release/openwepp-cli-hill …
perf report --stdio --children   # and: --symbol-filter=<fn> -g caller
```

perf.data (246 MB) is session-scratch, not committed; the report excerpts in
§2–§3 are the durable evidence. A promoted package should re-record and keep
its flat/children text dumps as artifacts (the perfdeep05 convention).
