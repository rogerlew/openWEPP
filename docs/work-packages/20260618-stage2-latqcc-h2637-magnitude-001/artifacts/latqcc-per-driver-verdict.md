# H2637 `latqcc` Per-Driver Verdict

Package:
`20260618-stage2-latqcc-h2637-magnitude-001`

## Evidence

Static:

- Verdict taxonomy follows the package and ADR-0017:
  `CORRECT`, `OPENWEPP-DEFECTIVE`, `LEGACY-DEFECTIVE`, `UNRESOLVED`,
  `CONTRACT-GAP`.

Ran:

- H2637 selected-day WB19 trace and recomputation from `/tmp/stage2_latqcc/diag3`.

## Driver Verdicts

| Driver | Verdict | Basis |
| --- | --- | --- |
| WB19 equation implementation | `CORRECT` | `latqcc == q`; `q == sum(substeps)`; recomputed Eq [6.2.4] potential residual <= `4.163336342344337e-17 m`. |
| Target/cap/withdrawal accounting | `CORRECT` | `target == min(potential, available_pool, capacity)`; `q == target`; `q == sum(layer withdrawals)`; `Qd == q + Qdd`. |
| Conductivity driver | `UNRESOLVED` for absolute magnitude, no defect found | `wb19_lateral_ssh == wb18_perc_ssc`; no inflation override observed; no external absolute bound for H2637 conductivity magnitude. |
| Drainable thickness / active depth | `CORRECT` for bounds, `UNRESOLVED` for absolute magnitude | Layer/substep operands are finite and bounded; no storage-cap defect found; no absolute H2637 magnitude benchmark exists. |
| `drfc` threshold / FC lineage | `CORRECT` for formula lineage, `UNRESOLVED` for absolute magnitude | `drfc = fc + (1-coca) * dg`; withdrawals are bounded above threshold; no FC-lineage defect found. |
| Legacy comparator | `UNRESOLVED` flag only | Like-for-like no-UI legacy flags a partition difference; ADR-0017 forbids treating that as the target. |

## Overall Verdict

`CONTRACT-GAP`.

The H2637 `latqcc` magnitude is an honest output of the current WB19 equation
and traced operands. No openWEPP kernel equation defect, withdrawal defect,
conductivity override defect, active-depth defect, or `drfc` formula defect was
found.

The package cannot honestly mark the magnitude `CORRECT` in the absolute
physics sense because the available external-authority suites validate
response, branch, and cap behavior, not an absolute lateral-flow magnitude for
H2637 forest soil. The missing authority is an absolute-magnitude benchmark or
contracted acceptance envelope for the lateral-flow driver stack.
