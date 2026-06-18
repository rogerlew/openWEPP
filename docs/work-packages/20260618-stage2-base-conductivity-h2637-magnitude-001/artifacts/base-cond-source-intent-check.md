# Base Conductivity Source-Intent Check

Evidence class: Static + Ran

Overall verdict: `OPENWEPP-DEFECTIVE`, narrowly scoped to vertical
`wb18_perc_ssc_####` 200 mm normalization. The package does not find a WB19
equation defect or a reason to make hourly `wb19_lateral_ssh_####` harmonic.

## Per-Step Check

| Step | Evidence | Verdict |
|---|---|---|
| Driver liveliness | `ksat_x0.9` completed and changed WAT/PASS checksums, all-OFE `latqcc`, PASS `runvol`, and peak WAT `latqcc`. | PASS |
| `ksatadj` exclusion | H2637 has `ksatadj = 0`; REFINTENT001 was byte-inert on H2637. | CORRECT; not the driver |
| Raw soil `ksat` parse | `SC-INFILE-SOIL-001` maps `.sol` `ksat` to `soil.ofe[i].layers[j].ksat_mm_h`; H2637 rows carry `60, 330.2755, 33.0275, 33.0275 mm/h`. | CORRECT |
| Source-layer anisotropy parse | H2637 `ui_anisrt = 1.0` for all source layers; openWEPP multiplies lateral source conductivity by that ratio. | CORRECT for H2637 |
| 200 mm vertical `ssc` normalization | Baseline source accumulates vertical `ssc` as inverse conductivity and finalizes `ssc1 = slayth / ksinv`; openWEPP arithmetically averages `ksat` into `ssc_m_s`. Layer 3 is `270.8259 mm/h` current vs `117.955408163210 mm/h` source intent. | OPENWEPP-DEFECTIVE |
| 200 mm hourly `ui_ssh` normalization | Baseline source separately accumulates `ui_ksari += thickness * ssc2 * ui_anisrt` and finalizes `ui_ssh1 = ui_ksari / slayth`. With H2637 anisotropy `1.0`, layer 3 hourly horizontal intent is `270.8259 mm/h`, matching current `wb19_lateral_ssh_0003`. | CORRECT for H2637 hourly lateral |
| Runtime lateral consumer | `00_lateral_transfer.rs:168-170` loads hourly `wb19_lateral_ssh` when the lane is not daily and `solwpv >= 7778`; the trace shows H2637 `solwpv=9002` and `lane_substeps=24`. | CORRECT |
| `ssh == ssc` as an invariant | Trace shows equality today, but `SC-SUBHYD-001` HPHYS0257 requires separate hourly `ui_ssh`; `SC-PERC-001` owns vertical `ssc`. Equality is accidental for non-split layers and wrong for split-layer vertical `ssc`. | OPENWEPP-DEFECTIVE if retained |
| WB19 equation/capacity/withdrawal | STAGE2-LATQCC already recomputed high-magnitude rows and found no equation, withdrawal, active-depth, or `drfc` defect. | Not reopened |

## Peak-Row Relevance

Ran:

- The selected peak diagnostic row sums WB19 lateral `q` to
  `71.62409876710505 mm`.
- On that row, layers 2-9 are conductivity-active for all 24 substeps and
  layer 1 is active for 15 substeps.
- Layer 3, the split layer where vertical `ssc` is inflated, is active for all
  24 lateral substeps.

Interpretation:

- The H2637 high-magnitude lateral rows are genuinely sensitive to the
  conductivity lineage.
- The direct hourly lateral conductivity consumed on those rows is `ui_ssh`,
  whose arithmetic split-layer value is source-intent consistent for
  anisotropy `1.0`.
- The vertical `ssc` defect still invalidates the package's broader
  base lateral/percolation conductivity lineage because WB18 percolation and
  daily vertical consumers receive the inflated `wb18_perc_ssc_0003`.

## Source-Intent Boundary

Static:

- `input.for` owns the split between vertical `ssc1` and hourly horizontal
  `ui_ssh1`.
- `SC-PERC-001` owns `wb18_perc_ssc_####` as per-layer vertical saturated
  conductivity.
- `SC-SUBHYD-001` owns hourly `wb19_lateral_ssh_####` as the modern
  `ui_ssh(i)` conductivity surface.

Therefore, the defect closure must not simply make every conductivity surface
harmonic. It must separate vertical harmonic `ssc` from hourly horizontal
arithmetic `ui_ssh`.
