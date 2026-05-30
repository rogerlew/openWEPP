# HPHYS0205 Review Findings — Claude Code

Reviewer: Claude Code
Date (UTC): 2026-05-29
Scope: layer-authoritative FC/WP correction wiring in
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
and the supporting tests, contracts, and 39-hillslope rerun evidence.

Evidence classes: **Static** (read source / contract / artifact) and **Ran**
(command executed). Runs over package outputs were produced by the package
executor and analyzed by this reviewer; reviewer-issued commands are marked.

This artifact records observations and evidence only. It does not propose an
implementation approach or a disposition.

---

## I-1 — Correction now fires; FC/WP residual fell ~12× but did not reach tolerance

Ran (reviewer, duckdb over executor outputs under
`/tmp/hphys0205_20260530T022235Z/parity/` and the prior
`/tmp/hphys0202_20260530T003833Z/parity/`), hillslope 1, row 0:

| Column | HPHYS0202 | HPHYS0205 | legacy (derived) |
|---|---|---|---|
| ProfileFCStore | 320.326 | 107.459 | ~113.6 |
| ProfileWPStore | 118.581 | 29.663 | ~31.4 |
| ProfilePorosityCap | 1092.710 | 1092.710 | 1092.710 |

Ran (reviewer, `reports/semantic/H1.semantic.json`), per-day constant offset:

| Column | mean_abs_diff (mm) | max_rel_diff | fail / points | tol (abs / rel) |
|---|---|---|---|---|
| ProfileFCStore | 6.1807 | 0.0544 | 1461 / 1461 | 0.1 / 0.02 |
| ProfileWPStore | 1.7368 | 0.0553 | 1461 / 1461 | 0.1 / 0.02 |
| ProfilePorosityCap | 0.0002 | 0.0000 | 0 / 1461 | 0.1 / 0.02 |

The injected correction changed the published values substantially (FC residual
~64.5% → ~5.4%, WP ~73.5% → ~5.5%). The fail count is unchanged because the
residual still exceeds the 2% relative tolerance.

## I-2 — Disposition / evidence report fail-counts, not residual movement

Static. `hphys0205_disposition.md` records "FC/WP semantic residual did not
improve" with `ProfileFCStore 39 -> 39`, `ProfileWPStore 39 -> 39`, and
`27 -> 39` / `1 -> 39` vs the HPARITY02 baseline. These are fail-hillslope
counts. The column is saturated at 39/39, so the count cannot reflect the
per-value residual change recorded in I-1 (mean_abs_diff 206.7 → 6.18 mm for FC;
87.2 → 1.74 mm for WP). The disposition and gap-matrix evidence do not record
the residual-magnitude delta.

## I-3 — Porosity and FC/WP are computed on different layer sets

Static. `ProfilePorosityCap` (which matches legacy to 7 digits, I-1) is produced
by `compute_wb13_profile_symbols_from_legacy_seed`
(`02_soil_slope.rs`), which operates on `legacy_expand_soil_layers_to_200mm`
(line 620 → `legacy_normalize_layers_to_200mm`, line 726). The corrected
per-layer `thetfc`/`thetdr` are produced by the new
`compute_corrected_layer_theta_symbols`, which operates on
`legacy_source_layers_from_seed_depths` (line 659) and does **not** pass through
`legacy_normalize_layers_to_200mm`. The same per-layer correction function
(`legacy_correct_layer_moisture`) is applied in both paths; the layer set
differs. The exact-match column uses the normalized set; the ~5.5%-residual
columns use the non-normalized set.

## I-4 — Corrected values map positionally from a different layer collection

Static. In the emission loop, the corrected pair is selected by
`corrected_layer_theta_symbols.as_ref().and_then(|layers| layers.get(layer_position))`,
where `layer_position` indexes `ofe.layers`, while the corrected vector is built
from `legacy_source_layers_from_seed_depths(&seeds, …)`. The mapping assumes a
1:1 positional correspondence between the source-layer collection and
`ofe.layers`. This correspondence is not asserted in the code path.

## I-5 — Corrected path requires measured inputs; raw path does not

Static. `compute_corrected_layer_theta_symbols` requires `layer.fc_measured?`
and `layer.wp_measured?` (returns `None` if either is absent). The raw per-layer
values it falls back to use `layer.fc_rosetta.or(layer.fc_measured)` /
`layer.theta_r_rosetta.or(layer.wp_measured)`. For a soil layer with Rosetta
values but no measured FC/WP, `compute_corrected_layer_theta_symbols` returns
`None` and the emission falls back (`map_or((raw_layer_thetfc, raw_layer_thetdr),
…)`) to the uncorrected raw values with no error or diagnostic. The 39-hillslope
test set carries measured values, so the corrected path was exercised here; the
fallback divergence (per I-1, ~64%) would re-enter silently for a measured-absent
layer.

## I-6 — Seed FC/WP publication source was changed

Static. `wb13_profile_fc_store_mm` / `wb13_profile_wp_store_mm` are now set from
`ofe_corrected_fc_store_mm` / `ofe_corrected_wp_store_mm` — sums over `ofe.layers`
of the corrected per-layer values (I-3/I-4 layer set) — rather than from
`Wb13ProfileSymbols { fc_store, wp_store }`, which were removed from that struct.
The seed FC/WP and the per-layer aggregation now derive from the same
non-normalized collection. Per the HPARITY02 disposition, the prior seed
(`compute_wb13_profile_symbols_from_legacy_seed`, normalized) corresponded to
ProfileWPStore failing 1/39 and ProfileFCStore failing 27/39.

## I-7 — FC carries residual beyond the FC/WP-vs-porosity layer-set difference

Static. The HPARITY02 disposition records, under the normalized seed,
ProfileWPStore failing 1/39 while ProfileFCStore failed 27/39. The FC-only
excess over WP is present independent of the publication-path and layer-set
observations above (I-3/I-6); WP under the normalized computation was within
tolerance on 38/39 hillslopes.

## I-8 — Gates pass; working tree uncommitted

Ran (reviewer, from `/home/workdir/openWEPP`): `cargo fmt --check`,
`cargo clippy --workspace --all-targets -- -D warnings`,
`cargo test --workspace`, `cargo deny check` all exit 0. Static: HPHYS0205
changes are unstaged in the working tree; `git log` HEAD is `9d438df`
(HPHYS0202 hold closeout + HPHYS0205 scaffold); no HPHYS0205 commit exists.
