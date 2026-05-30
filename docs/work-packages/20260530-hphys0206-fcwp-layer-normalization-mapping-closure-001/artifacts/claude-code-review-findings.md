# HPHYS0206 Review Findings — Claude Code

Reviewer: Claude Code
Date (UTC): 2026-05-30
Scope: FC/WP layer-normalization + parser-layer mapping change in
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
and `00_core_types.rs`, plus supporting tests, contracts, and the 39-hillslope
rerun under `/tmp/hphys0206_20260530T032538Z/parity/`.

Evidence classes: **Static** (read source / contract / artifact) and **Ran**
(command executed). Runs over package outputs were produced by the package
executor and analyzed by this reviewer; reviewer-issued commands are marked.

This artifact records observations and evidence only. It does not propose an
implementation approach or a disposition.

---

## I-1 — Prior structural observations were addressed

Static. Relative to HPHYS0205:
- The corrected per-layer thetfc/thetdr are now computed on the
  200 mm-normalized layer set (`compute_normalized_corrected_layer_theta_symbols_from_legacy_seed`
  → `legacy_expand_soil_layers_to_200mm`), the same set the matching porosity
  column uses.
- The silent measured-input fallback is removed. Absent
  `bulk_density_g_cm3` / `fc_measured` / `wp_measured` now hard-fail with typed
  errors (`HS-RUNTIME-E-060/061/062`) rather than reverting to raw values.
- The positional `get(layer_position)` map is replaced by a depth-overlap remap
  (`map_corrected_layer_theta_symbols_to_parser_layers`) with a per-parser-layer
  coverage guard (`covered_depth_mm == layer_thickness_mm`, tol `1e-9`).
- `hphys0206_disposition.md` now reports residual-magnitude deltas, not only
  fail-hillslope counts.

## I-2 — Residual did not improve; it slightly worsened

Ran (reviewer, `reports/semantic/H1.semantic.json`, and disposition averages):

| Metric | HPHYS0205 | HPHYS0206 |
|---|---|---|
| H1 ProfileFCStore mean_abs_diff (mm) | 6.18 | 6.50 |
| H1 ProfileWPStore mean_abs_diff (mm) | 1.74 | 2.00 |
| 39-avg ProfileFCStore mean_abs_diff (mm) | 6.49 | 7.22 |
| 39-avg ProfileWPStore mean_abs_diff (mm) | 1.89 | 2.24 |

Fail counts unchanged: ProfileFCStore 39/39, ProfileWPStore 39/39. The
normalization change moved FC/WP slightly further from legacy.

## I-3 — Porosity/ProfileDepth aggregate on the normalized grid; FC/WP aggregate on the parser grid; the grids span different depths

Static + Ran. For H1 (`p1.sol`), the parser soil layers bottom at 159 cm =
1590 mm (6 layers). The published `ProfileDepth` = 1800.0 mm and matches legacy
exactly (Ran: `H1.semantic.json` ProfileDepth fail 0/1461), i.e. the
legacy/normalized profile is 9 × 200 mm = 1800 mm. `ProfilePorosityCap` is
aggregated over that normalized set (`compute_wb13_profile_symbols_from_legacy_seed`)
and matches legacy to 7 digits (Ran: mean_abs_diff 0.0002 mm).

FC/WP are published by the runner as `Σ(thetfc_#### · dg_####)` over the
**parser** per-layer symbols (`crates/openwepp-runner/src/hillslope/mod.rs`
WB13 builder; `dg_####` is the parser layer thickness emitted at
`02_soil_slope.rs`). HPHYS0206 computes corrected thetfc/thetdr on the
normalized grid and then remaps them onto the parser layers, so the FC/WP
aggregation extent is the parser depth (1590 mm for H1), while porosity /
ProfileDepth / legacy use the normalized depth (1800 mm).

Direct evidence of the consequence on WP (which carried no FC-style base
residual — see I-6): Ran, H1 legacy ProfileWPStore = 31.40 mm (derived from
candidate 29.40 + mean_abs_diff 2.00, and max_rel_diff 0.0636 = 2.00/31.40),
candidate = 29.40 mm, a −2.0 mm deficit. Under the normalized seed (HPARITY02)
ProfileWPStore matched legacy on 38/39 hillslopes; on the parser-grid path
(HPHYS0205 29.66 mm, HPHYS0206 29.40 mm) it is ~2 mm low. A 200 mm normalized
layer at the wilting floor (~0.01) contributes ~0.01 × 0.2 × 1000 = 2 mm.

## I-4 — Coverage guard verifies parser-layer coverage but not normalized-layer consumption

Static. `map_corrected_layer_theta_symbols_to_parser_layers` builds normalized
intervals as uniform 200 mm slabs from depth 0 and requires each **parser**
layer to be fully covered by them (`covered_depth_mm == layer_thickness_mm`).
It does not assert that every **normalized** interval is consumed by some parser
layer. When the normalized profile is deeper than the parser profile (H1:
normalized 1800 mm vs parser 1590 mm), the normalized interval(s) below the
parser bottom overlap no parser layer and are dropped without error or
diagnostic. The normalized-slab reconstruction (`WB13_PROFILE_LAYER_THICKNESS_M
= 0.2`) matches the geometry emitted by `legacy_normalize_layers_to_200mm`
(uniform 200 mm layers), so the slab thickness itself is consistent; the
unconsumed-tail behavior is the gap.

## I-5 — Fail-closed change alters projection behavior for measured-absent soils

Static. The corrected path is now mandatory: a soil layer with Rosetta values
but no measured `fc_measured`/`wp_measured`/`bulk_density_g_cm3` now returns
`HS-RUNTIME-E-060/061` and hard-fails the whole runtime-surface build, where the
prior path projected (raw) and continued. The 39-hillslope set carries measured
values, so this path was not triggered in the rerun.

## I-6 — FC carries a residual independent of the grid/extent observation

Static. Under the normalized seed (HPARITY02 disposition) ProfileWPStore failed
1/39 while ProfileFCStore failed 27/39. The FC-only excess is present in the
direct normalized aggregation, independent of the parser-grid remap (I-3) and the
unconsumed-tail behavior (I-4); WP under the normalized aggregation was within
tolerance on 38/39.

## I-7 — Gates; working tree uncommitted

Ran (reviewer, from `/home/workdir/openWEPP`, HPHYS0206 working tree):
`cargo fmt --check` → exit 0; `cargo deny check` → exit 0;
`cargo clippy --workspace --all-targets -- -D warnings` → exit 0;
`cargo test --workspace` → exit 0. Static: HPHYS0206 changes are unstaged;
`git log` HEAD is `130c384` (HPHYS0206 scaffold); no HPHYS0206 implementation
commit exists.
