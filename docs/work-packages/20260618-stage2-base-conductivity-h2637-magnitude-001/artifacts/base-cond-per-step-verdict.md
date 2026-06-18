# Base Conductivity Per-Step Verdict

Evidence class: Static + Ran

Overall package verdict: `OPENWEPP-DEFECTIVE`.

The defect is narrow and upstream: openWEPP currently computes vertical
`wb18_perc_ssc_####` with arithmetic 200 mm layer averaging. Baseline source
intent computes vertical `ssc1` by inverse-conductivity accumulation and
separately computes hourly lateral `ui_ssh1` arithmetically from
`ssc2 * ui_anisrt`.

## Verdict Table

| Component | Verdict | Reason |
|---|---|---|
| Sensitivity gate | PASS | `ksat_x0.9` changed WAT/PASS checksums, aggregate `latqcc`, PASS `runvol`, and peak WAT `latqcc`. |
| Raw H2637 `.sol` conductivity | CORRECT | Parsed `ksat` and anisotropy match `SC-INFILE-SOIL-001` input mapping. |
| `ksatadj` involvement | NOT ACTIVE | H2637 has `ksatadj=0`; REFINTENT001 remains correct but byte-inert for this fixture. |
| Vertical 200 mm `ssc` normalization | OPENWEPP-DEFECTIVE | H2637 layer 3 should be `117.955408163210 mm/h`; openWEPP publishes `270.8259 mm/h`. |
| Hourly lateral 200 mm `ui_ssh` normalization | CORRECT for H2637 | With anisotropy `1.0`, source-intent hourly `ui_ssh` layer 3 is `270.8259 mm/h`. |
| Runtime hourly lateral conductivity consumer | CORRECT | H2637 hourly lane consumes `wb19_lateral_ssh`, consistent with `SC-SUBHYD-001` HPHYS0257. |
| `wb18_perc_ssc == wb19_lateral_ssh` equality | DEFECTIVE AS A PROJECTION RESULT | The two surfaces must be distinct for split layers; equality is not source-authoritative. |
| WB19 equation and withdrawal | NOT REOPENED | STAGE2-LATQCC already closed equation/capacity/withdrawal checks for selected H2637 high-magnitude rows. |

## FARPOINT01 Disposition

FARPOINT01 is not resolved by this package.

The H2637 71% flag cannot be closed as `CORRECT` while the vertical
percolation conductivity surface is source-intent defective. It also should not
be routed to a direct WB19 lateral equation fix: the hourly lateral surface for
H2637 matches the arithmetic `ui_ssh` source intent.

Required next step:

- Defect-closure ExecPlan for vertical `ssc` 200 mm normalization, preserving
  hourly `ui_ssh` arithmetic normalization and proving the two surfaces can
  diverge on split-layer inputs.

After that closure lands, rerun H2637 and re-dispose the remaining magnitude
flag. Possible outcomes after the fix are:

- FARPOINT01 resolves if source-intent conformance explains the flag.
- FARPOINT01 remains a contract/authority gap if the high magnitude persists
  with all source-intent conductivity surfaces correct.
- A new defect localizes if rerun evidence identifies a different
  source-intent mismatch.
