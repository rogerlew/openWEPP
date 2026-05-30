# HPHYS0211 Residual Gap Matrix

Status: completed  
Evidence mode: Static + Ran

## Family matrix
Sources:
- `/tmp/hphys0208_20260530T155837Z/parity/reports/hillslope_semantic_summary.json`
- `/tmp/hphys0209_20260530T171007Z/parity/reports/hphys0209_profilewp_focus_summary.json`

| Family/Column | Fail hillslopes | Mean abs diff avg | Status |
| --- | --- | --- | --- |
| `ProfileDepth` | `0/39` | `0.0000` | closed |
| `ProfilePorosityCap` | `0/39` | `0.0209` | closed |
| `ProfileWPStore` | `1/39` (`H7`) | `0.0573` | bounded |
| `ProfileFCStore` | `27/39` | `2.0527` | open |
| `Dp` | `39/39` | `40.1559` | open |
| `latqcc` | `39/39` | `173.2285` | open |
| `Total-Soil` | `39/39` | `116.0649` | open |
| `SoilWaterTotal` | `39/39` | `116.0649` | open |

## Diagnostic signatures (ran)
- `ProfileFCStore` split pattern is structural, not intermittent:
  - `27` hillslopes fail all `1461/1461` compared rows,
  - `12` hillslopes fail `0/1461` rows.
  - Evidence:
    `/tmp/hphys0211_20260530T203603Z/analysis/hphys0211_profilefc_fail_pattern_summary.txt`
- `Dp` fails `1461/1461` rows for all `39` hillslopes.
- `Dp` top-row candidate values cluster discretely by hillslope:
  `24.172774` (`2`), `33.301038` (`7`), `39.525469` (`19`),
  `40.128282` (`1`), `50.080803` (`10`) mm/day-equivalent publication lane.
  Evidence:
  `/tmp/hphys0211_20260530T203603Z/analysis/hphys0211_dp_toprow_candidate_clusters.tsv`
- `latqcc` fails all hillslopes with per-hillslope mean abs diff range
  `62.761330 .. 258.735663`.
- `Total-Soil` and `SoilWaterTotal` fail all hillslopes with identical
  per-hillslope ranges (`80.366098 .. 138.987183`), indicating shared lineage.

## Root-cause decomposition ledger
| Gap ID | Family | Canonical lineage | Implementation anchors | Root-cause decomposition | Owner follow-on |
| --- | --- | --- | --- | --- | --- |
| `HP211-RC-001` | `Dp` | `D -> Dp` (`SC-WATBAL-001` lines `955`, `1116-1138`; `SC-PERC-001` lines `276-318`) | `execute_scheduler_kernel_lifecycle` reseeds WB11 every day: `crates/openwepp-runner/src/hillslope/mod.rs:2795`; seed overwrites `wb18_perc_theta/fc/ul` and `wb11_soil_water`: `1792-1871`; WB18 percolation consumes seeded layer state: `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs:679-833`; WB13 publishes `D` as `Dp`: `crates/openwepp-runner/src/hillslope/mod.rs:3935-3952` | Daily lifecycle currently re-applies initialization-seed formulas instead of preserving prior-day WB18 mutable state, producing persistent high percolation-loss regimes and fully saturated Dp failure. | `HPHYS0212` |
| `HP211-RC-002` | `latqcc` | `q -> latqcc` (`SC-WATBAL-001` line `964`; `SC-SUBHYD-001` lines `346-356`, `374-382`) | Seed hard-codes WB19 parameters each day: `crates/openwepp-runner/src/hillslope/mod.rs:1898-1915`; WB19 lateral flux uses anisotropy/Ke/slope: `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs:942-1016`; WB13 publishes `q` as `latqcc`: `crates/openwepp-runner/src/hillslope/mod.rs:3941-3967` | Lateral-loss publication is dominated by fixed daily-seeded WB19 controls plus reseeded layer state, yielding persistent high `q` magnitudes. Additionally, WB13 row assembly currently hard-codes `Tile=0` and `SubRIn=0`, preventing contract-consistent decomposition checks against `Qd = latqcc + Tile`. | `HPHYS0212` |
| `HP211-RC-003` | `ProfileFCStore` | normalized-profile storage authority (`SC-WATBAL-001` lines `1071-1083`; `SC-PERC-001` lines `293-300`) | Profile storage seed uses normalized corrected-layer aggregate: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs:511-537`; authoritative layer symbols are mapped back to parser layers: `631-701`; WB13 publishes `wb13_profile_fc_store_mm`: `402-413`, consumed at `crates/openwepp-runner/src/hillslope/mod.rs:3845-3850` and `3977` | Residual signature is static lane split (`all rows fail` or `all rows pass`) and aligns with depth-domain authority choices (normalized-profile aggregate vs parser-layer mapped thresholds). This remains unresolved authority adjudication/remediation for `27` hillslopes. | `HPHYS0212` (close with Dp/latqcc root fixes) |
| `HP211-RC-004` | `Total-Soil`, `SoilWaterTotal` | `wb11_soil_water -> Total-Soil`, `Total-Soil + frozwt -> SoilWaterTotal` (`SC-WATBAL-001` lines `965-966`; `SC-SOIL-001` `INV-SOIL-013`) | WB13 publication reads `wb11_soil_water`: `crates/openwepp-runner/src/hillslope/mod.rs:3870-3879`; WB19 phases mutate `wb18_perc_theta_*` but do not update `wb11_soil_water`: lateral `1041-1054`, drainage `1360-1373`; daily execution re-seeds `wb11_soil_water`: `2795` and `1843-1871` | Aggregate publication lineage is coupled to reseeding and stale aggregate state risk after WB19 withdrawals. Shared fail pattern with `Dp/latqcc` indicates this is downstream of WB11/WB19 lifecycle defects, not an independent publication bug. | `HPHYS0213` (after HPHYS0212 rerun) |

## Gap register
| Gap ID | Description | Evidence | Status |
| --- | --- | --- | --- |
| `HP211-GAP-001` | Coupled residual blockers remain unresolved and prevent hold-lift (`ProfileFCStore`, `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`). | Ran + Static: matrix + code/contract anchors above. | open |
| `HP211-GAP-002` | WB11 daily lifecycle currently conflates initialization seeding with mutable-state carry and is the dominant shared defect owner for `Dp`, `latqcc`, and aggregate soil-water publication residuals. | Static: `execute_scheduler_kernel_lifecycle` + seed functions; Ran: discrete Dp clusters and full-row fail saturation. | open |
| `HP211-GAP-003` | `ProfileFCStore` residual is structurally partitioned and requires explicit normalized-profile vs parser-layer authority adjudication under the existing HPHYS0207 policy. | Ran: `27/39` all-row fail vs `12/39` zero-row fail split. | open |
