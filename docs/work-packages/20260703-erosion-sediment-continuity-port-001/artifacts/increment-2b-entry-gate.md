# Increment 2b (ROADMAP §E.2, ADR-0036) — Hydrograph Substrate: Entry Gate

Author: Claude Code, 2026-07-04. Evidence: **Static** (direct-runtime WB14/DC01
source, the HBP writer/parser pair, the watershed routing kernel — the
watershed/HBP half via a read-only survey agent with file:line citations,
verified at the load-bearing points). Execution record appends below as the
increment runs.

Status: executed in the same pass (operator: "scaffold and execute E.2", scope
confirmed contracts + code + tests). Executor: Claude Code (ADR-0035/0036
pattern). Authority: [ADR-0036](../../../decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md)
(Accepted 2026-07-04); stage 2b of
[`increment-2-entry-gate.md`](increment-2-entry-gate.md) §7.

## 1. Load-bearing recon resolutions

### 1.1 The hourly-shape authority already exists: the DC01 transfer weights

ADR-0036 D4 requires `Σ V_h = runvol` exactly. Raw `wb14_hourly_excess_m`
**cannot** provide that: by construction
(`compute_wb14_infiltration_depression_with_profile`, `runoff.rs:1466-1572`)
its sum is `total_rainfall − cumulative_infiltration` — **pre**
depression-storage (`Δdep` withheld at `runoff.rs:1558-1559`), **pre** frost
retention, and **without** the saturation-excess component that
`q_runoff_m = partition_runoff_m + surface_saturation_runoff_m`
(`runoff.rs:745`) adds.

The reconciliation rule is **already ratified production behavior**:
`dc01_surface_transfer_weights` (`03_executor.rs:529-556`) builds the
unit-normalized hourly distribution from
`excess_h + hourly_saturation_carry_h` (uniform fallback when the day has
runoff without a shape; all-zero without runoff), and the MOFE transfer
publication already pairs the **exact daily total** with that distribution
(`INV-RUNOFFPART-031`/M2 lineage, `03_executor.rs:616-625`).

**Decision:** the DC01 weights become the single **hourly-runoff shape
authority**, shared by all three consumers:

- serialized hydrograph: `V_h = runvol · w_h` (m³) — `Σ w_h = 1` by
  construction ⇒ the D4 closure ties exactly (f64-rounding hygiene only);
- the per-hour erosion solve: `rate_h = (q_runoff_m · w_h) / 3600 s` (m/s
  depth-rate into the operand slot `peakro` fills today);
- downstream runon admission (already consumes them — unchanged).

One shape, three consumers — the solve, the interchange, and the routing all
see the *same* hydrograph. The pure function is extracted crate-visible and
computed for **every** lane (today it only runs for lanes with a downstream
neighbor); the existing transfer call keeps byte-identical results (same pure
computation).

Timing caveat, recorded: the proportional shape absorbs depression-storage /
frost-retention timing (it scales the whole profile rather than front-filling
the early hours). That is the **already-ratified M2 semantics** — E.2 adopts
it rather than inventing a second distribution rule; onset-timing refinement
is a separate future contract item if field evidence demands it.

### 1.2 HBP extension mechanics (verified against writer + parser)

- Writer `build_hbp_event_payload`
  (`02_output_and_climate_helpers.rs:191-237`); parser
  `parse_runoff_event_payload` (`payload_validator.rs:257-295`). The payload
  is directory-length-prefixed but **strict-consumption**
  (`validate_payload_consumed`, `payload_validator.rs:615`): additive fields
  must be explicitly read, gated on `payload_schema_minor`.
- **Mechanism:** bump `SUPPORTED_MINOR_V1` 0→1 (`hbp/mod.rs:44`); write the
  new surfaces behind minor ≥ 1; parse conditionally on
  `header.payload_schema_minor >= 1`. Minor-0 (legacy) shards stay readable;
  old readers **reject** minor-1 payloads loudly (`validate_payload_minor`,
  `payload_validator.rs:212`) — fail-closed forward compatibility, no silent
  mis-parse.
- New fields (count-prefixed f64 arrays, the existing `:225-228` pattern),
  placed before the trailing reserved `2 × i64`: `V_h[24]` (m³),
  `S_h[24]` (kg). The per-class E.1 deferral rides the same bump: `npart`
  un-hardcoded from 1 → 5 (`append_hbp_common_prefix`,
  `02_output_and_climate_helpers.rs:271`) with real `sedcon_i`/`frcflw_i`
  arrays.
- `HbpLatestEventPayload` (`hbp/types.rs:104`) gains
  `hourly_runoff_volume_m3` / `hourly_sediment_mass_kg`.

### 1.3 The peak-unit defect is live in the watershed today

The HBP `peak_runoff_m3_s` field is fed the WB16 **depth-rate** (m/s
misnomer). The watershed then computes contributor "volume" as
`peak · duration` (`kernel/direct.rs:627`) and **sums those across
contributors** (`:638-640`) — dimensionally a *depth*, so inlet volume
aggregation loses area weighting across hillslopes of different areas (the
Rational branch `volume/duration · gain` partially cancels the error for a
single contributor, but multi-contributor sums do not). **D2's unit fix is
therefore a live-defect fix, not just hygiene**: serialize true
`peak_runoff_m3_s = depth_rate × hillslope_area` at the same minor bump, and
`V_h` in true m³ makes inlet volume aggregation exact where the hourly path
is taken.

### 1.4 Watershed routing scope (bounded by what exists)

The watershed consumes **one latest runoff EVENT per hillslope shard**
(`hbp/mod.rs:76-82` overwrite loop; intake at
`openwepp-cli-watershed.rs:336-411`) — routing is a single-event summary
machine today. E.2's D3 upgrade is therefore scoped to: **hour-resolved
superposition of that event** at the inlet
(`assemble_direct_incoming_peak_partition`, `kernel/direct.rs:607`:
`incoming_peak = max_h(Σ_contributors V_h/3600)` replacing the scalar peak
sum; volumes from `Σ V_h`), with the `Ws11IpeakBranch::Rational`
reconstruction (`direct.rs:185-190`) conditional per amended
`INV-ROUTE-005` — triangular/rectangular fallback for minor-0 shards.
Per-day watershed routing (consuming every event, not the latest) is a
**separate pre-existing gap**, explicitly out of E.2's write set.

## 2. Per-hour solve operand table (D1 realization)

| Operand | Per-hour basis | Note |
|---|---|---|
| `peakro_m_s` slot | `rate_h = q_runoff_m · w_h / 3600` | mean depth-rate of hour `h`; same slot/units as today |
| `runoff_depth_m` | `q_runoff_m · w_h` | the hour's depth share |
| `effdrn_s` | `3600.0` | the hour IS the flow duration (quasi-steady hour) |
| `effint`/`effdrr` | `erosion_effective_intensity` on the **hour's** excess/rainfall intervals | existing producer, hour-filtered input |
| `beta` | `0.5` if `wb14_hourly_rainfall_m[h] > 0` else `1.0` | per-hour resolution of the `param.for` rule |
| `qin_m2_s`, `strldn` | `0` (single-OFE scope) | per-hour `qin_h` operands are E.3's handoff |
| rill width | sequential growth: hour `h` seeds hour `h+1`'s width; end-of-day persists to the lane carry | `shears` growth is monotone between disturbances; hour order is deterministic |
| `kiadjf`/`kradjf`/`tcadjf`, consolidation, frost regime, cover/roots/residue | **daily** (computed once, shared by all hours) | soil/cover state is a daily surface |
| activation | day-level `passby` event gate first (legacy `contin.for` event-size semantics), then per **hydraulically-active hour** (`w_h > 0`; `∨ qin_h > 0` when E.3 lands) | ADR-0036 D1 |
| `S_h` | the hour solve's exported mass: `exported_kg_m(h) · fwidth` | `Σ S_h` = the day's exported sediment by construction |

Daily aggregates published as today: `tdet/tdep = Σ_h`, toe concentration
recomputed at day level (`Σ exported / (q_runoff · efflen)` — preserving the
E.1 `tdet = Σ sedcon × runvol` reconstruction identity on zero-deposition
days), per-class split per E.1 (GAP-SED-007 basis).

**Comparator arm (D1):** the daily peak-based solve is retained behind an
opt-in flag (`erosion_peak_form_comparator` class) for one transition window
— cross-check surface, never publication authority once the hourly flip
lands.

## 3. Stage plan (each stage gated; Codex review per push)

- **2b-1 — contract amendments (no code):** SC-SED-001 (active-hour basis,
  per-hour closure invariants, shape-authority + timing GAP rows);
  SC-INFILE-HBP-001 + `hbp-file.spec.md` (minor-1 EVENT extension:
  `V_h`/`S_h`, npart=5 per-class, true-m³/s peak); SC-ROUTE-001
  (conditional `INV-ROUTE-005`, paired intake, fallback).
- **2b-2 — hillslope hourly solve:** shared weights extraction; per-hour
  assembly/solve/aggregation in the erosion span; day-frame `V_h`/`S_h`
  surfaces; per-hour in-solve conservation gates + day-level Σ gate; the
  recession acceptance case (crafted `qin_h > 0` falling-limb payload at
  solver level must deposit without any clamp).
- **2b-3 — HBP minor-1:** writer + parser + `HbpLatestEventPayload`;
  round-trip test on real shards (p61 + a WS3 cell); minor-0 shards
  parse-stable; intake closure validation (`Σ V_h = runvol`,
  `Σ S_h = mass`) in `validate_latest_event_vectors`.
- **2b-4 — watershed consumption:** contribution struct + intake forwarding;
  hour-resolved inlet superposition behind the surface-presence condition;
  Rational fallback byte-stable for minor-0 shards (existing watershed
  fixture suite must hold).
- **2b-5 — re-baselines + full gates:** p61/DFF-WS3 re-baselined (hourly
  numbers differ from peak-form; acceptance = conservation + directional law
  + reconstruction identity); full AGENTS battery; branch push, no
  self-merge.

## 4. Hold criteria

1. The weight-based closure fails materially on real fixtures
   (`|Σ V_h − runvol|` beyond rounding) — would falsify the shared-shape
   premise; stop and report.
2. The recession acceptance case cannot produce deposition without a clamp —
   falsifies the D1 form; the ADR's recorded fallback (step-hydrograph) is a
   redesign, not an inline pivot.
3. HBP minor-1 round-trip instability on real shards, or any silent
   mis-parse path found for minor-0 readers.
4. Per-hour solve trips the flux-closure discretization gate broadly on
   production fixtures (not just crafted-stiff instruments) — cost/noise
   falsification per ADR-0036 D1.
