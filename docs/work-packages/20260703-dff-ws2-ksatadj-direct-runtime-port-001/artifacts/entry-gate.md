# WS-2 Entry-Gate Verification — `ksatadj` state at HEAD

Evidence: **Static** (source read of `main` @ `4328446b`, 2026-07-03) via an
Explore code-state sweep. Resolves the WS-2 entry gate before any kernel edit.

| # | Question | Verdict |
|---|----------|---------|
| 1 | `ksatadj` still parsed? | **YES** — all three `DisturbedPolicy` variants. |
| 2 | `ksatadj` projected into runtime? | **Projected into a typed struct, then DROPPED** (no consumer). |
| 3 | Any `ksatadj` adjustment in the direct production conductivity? | **NO** — WB14 is frost + base only. |
| 4 | `ksflag = 0 → frost off` coupling live? | **NO — already decoupled.** No work. |
| 5 | `INV-SUBHYD-032` state | Authored, `hard-fail`, contract `in_review`; `HOLD`-until-implemented posture. |

## 1. Parse (present)
`crates/openwepp-input-contract/src/parsers/soil.rs:236-260` — `DisturbedPolicy`:
- `V9002`: `ksatadj: bool, luse, stext, ksatfac_mm_h, ksatrec_per_day`
- `V9003`: `ksatadj, luse, burn_code, stext, lkeff_mm_h`
- `V9005`: `ksatadj, luse, burn_code, stext, texid_enum, uksat_mm_h, lkeff_mm_h`

Parsers `soil.rs:654-757`; round-trip tests `soil.rs:1394-1477`; `lkeff` sentinel
`POLICY_LKEFF_SENTINEL`. No `keffflag` symbol anywhere.

## 2. Projection (present but dead)
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs:76-79,244-297`
projects `TypedSoilWb11RuntimeProjection { ksatadj: bool, ksatfac_mm_h:
Option<f64>, ksatrec_per_day: Option<f64>, lkeff_mm_h: Option<f64> }`
(V9002 → ksatfac/ksatrec; V9003/V9005 → lkeff; None → all None/false).
**No downstream consumer** — whole-tree search for
`ksatadj|ksatfac|ksatrec|lkeff|uksat` hits only the parser + this projection.
Dropped: `uksat_mm_h`, `burn_code`, `texid_enum`, `luse`, `stext`.

## 3. Direct WB14 conductivity (no ksatadj — the port site)
`crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs:3291-3316`
(`DirectProductionInfiltrationAuthority::inputs`): `effective_conductivity_m_s` =
`frost_infcap_m_s` → frost-seeded `self.effective_conductivity_m_s`
(`:1536-1544`) → `layers.first().conductivity_m_s`. Consumed by the Green-Ampt
solver `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:1595-1636`.
No `ksatadj`/`sat_frac`/`lkeff`. (WB18 percolation conductivity
`direct_runtime/subsurface.rs:1317-1418` is separate, also no ksatadj.)
Deleted kernel: `…/hydrology_phase_lateral_drainage/02_ksat_adjustment.rs`
(677 lines, removed in `a381702b`, 2026-06-30). Recover:
`git show a381702b^:crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage/02_ksat_adjustment.rs`.

## 4. `ksflag → frost` — already decoupled (entry gate clear)
`ksflag` occurs only in `soil.rs:216,352-361,438` (parse) and
`lane_setup_helpers.rs:118` (carry). **Never read to gate anything.** Frost
activation: `00a_snow_frost_authority_impl.rs:347-350` keys off
`frost_wint_red_enabled` (from `frost_projection.wint_red`,
`00_builders_and_authority.rs:1314`, ⟵ `04_snow_frost_irrigation.rs:164`
`frost.wint_red == 1`), not `ksflag`. FQ-4 (`docs/work-packages/README.md:2666-2685,
2718-2729`) closed this and explicitly notes it is separate from the forest
`ksatadj` model. **WS-2 keeps frost on; no coupling to remove.**

## 5. `SC-SUBHYD-001` / `INV-SUBHYD-032`
`docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md` — front-matter
`status: in_review`. `INV-SUBHYD-032` (line 297, `hard-fail`): source-intent
`ksatadj` effective-conductivity from `REF-SUBHYD-KSATADJ-INTENT`.
`BR-SUBHYD-KSATADJ-EXECUTE` (line 257): trigger `ksatadj = 1`; inputs `solwpv`,
`ks`, top-two `dg`/`por`/`cpm`/`thetfc`/`thetdr`/`wb18_perc_theta`/`wb18_perc_ul`,
+ `ksatfac`/`ksatrec` (9001) or `lkeff` (9003). `BR-SUBHYD-KSATADJ-GUARD`
(line 260): typed hard-fail **or contract `HOLD` until the source-intent operand
lineage is implemented**. Algorithm lines 163-204 + REFINTENT001 addendum
854-877; **anti-surrogate clause** (871-877) + conformance obligation (468-470,
874-877): a two-layer vector where `avsat/(avpor*avcpm)` differs from
`Σst_i/Σul_i`. Open gaps: `GAP-SUBHYD-001` (comparator vectors, promotable-with-
risk), `GAP-SUBHYD-002` (runtime-field aliases, **non-promotable** — WS-2 lands
the `Keff_ksatadj` alias map).

## Net
WS-2 is a genuine fresh port into the direct lane: parse + (dead) projection
exist; runtime conductivity formation, the alias map, and conformance vectors do
not. The `ksflag→frost` gate is already clear, which removes a whole limb the
strategy anticipated.
