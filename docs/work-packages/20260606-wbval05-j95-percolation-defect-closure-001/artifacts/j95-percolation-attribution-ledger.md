# J-95 Percolation Attribution Ledger

Status: complete

Evidence mode: static+ran

Purpose: reproduce and attribute `WBVAL05-J95-HKERNEL-WB11-PERC-E-003` for
`p7`, `p11`, `p18`, and `p20`.

Required content:

- Reproduction command and exit status for each target.
- Exact typed failure evidence, including `sim_day_index`, calendar year,
  Julian day, phase, and message ID.
- Per-layer WB18/WB11 domain surfaces at or immediately before failure.
- Named mechanism or legitimate branch-out boundary.
- Seven-gate result for any in-envelope mechanism.

Static:

- Contract authority: `SC-PERC-001` now clarifies WB18 same-pass ingress
  consumption order. A finite non-negative published `wb12_infiltration`
  surface is the authoritative WB18 `fin/xfin` source; WB18 should only
  reconstruct WB14/WB12 liquid partition when that surface is absent.
- Code attribution: pre-fix WB18 percolation recomputed WB14 infiltration
  whenever `management.initial.params.tillay2_m` existed. On J-95 the runtime
  already had `wb12_infiltration=0`, but recomputation validated stale
  `snow.runtime_swe=-0.006171157610042402` and surfaced it as
  `HKERNEL-WB11-PERC-E-003`.
- Per-layer WB18 inputs at failure were not the root cause. Final diagnostic
  summary for p7 before correction showed `layer_count=8`,
  `lane_substeps=24`, `infiltration=0`, `tillay2=0`, `slflag=1`,
  `kslast=0.0000000009`, `ui_bdrkth=10`, and `invalid_layers=none`.
- Named in-envelope mechanism: WB18 incorrectly re-ran runoff/snow liquid
  partition validation despite an already-published zero same-pass infiltration
  lineage.
- Named branch-out boundary after correction: WB14 runoff reconciliation now
  fails first on the same invalid snow state with
  `HKERNEL-WB14-RUNOFF-E-003`, which is outside WBVAL05 percolation authority.

Ran:

- Reproduced pre-fix p7/p11/p18/p20 J-95 failure with
  `openwepp-cli-hill --run-dir /wc1/runs/in/indispensable-presenter/wepp/runs
  --run-file /tmp/wbval05_j95_perc_20260606T000000Z/generated_runfiles/<p>.toml
  --output-dir /tmp/wbval05_j95_perc_20260606T000000Z/outputs/<p>
  --policy compat`; all four returned RC 1, emitted no WAT, and failed at
  `HKERNEL-WB11-PERC-E-003`.
- Temporary local attribution print, removed before final validation, exposed:
  `HKERNEL-WB11-PERC-E-003: ... state symbol
  snow.runtime_swe=-0.006171157610042402 outside [Some(0.0), None]`.
- Post-fix final validation used final release binary and outputs under
  `/tmp/wbval05_j95_perc_20260606T000000Z/outputs_final/`; p7/p11/p18/p20
  no longer fail at WB18 percolation and instead stop at
  `HKERNEL-WB14-RUNOFF-E-003`.

Seven-gate result:

| Gate | Result | Evidence |
|---|---|---|
| Reproduction | true | Four pre-fix CLI repros. |
| Mechanism | true | Published zero infiltration was ignored; WB18 recomputed snow partition. |
| Ownership | true for WB18 misattribution; false for remaining snow state | Code fix in WB18; remaining negative SWE is WB14/snow boundary. |
| Authority | true | `SC-PERC-001` v29 amendment. |
| Safety | true | No percolation guard loosening; invalid snow still fails closed upstream. |
| Testability | true | New `wbval05_wb18_percolation...` regression. |
| Validation | partial | WB18 PERC failure cleared; WAT still blocked by WB14 snow guard. |
