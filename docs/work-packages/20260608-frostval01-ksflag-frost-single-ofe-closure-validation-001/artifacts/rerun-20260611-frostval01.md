# FROSTVAL01 Rerun Supplement

Status: complete-after-follow-ons
Evidence mode: Ran + Static

## Context

This supplement records a fresh FROSTVAL01 execution after the documented
follow-ons in the frost queue closed the original blockers:

- FQ-1 removed the `HS-RUNTIME-E-062` soil-coverage population blocker.
- FQ-3 restored runoff and annual-crop ET/canopy engagement on the substrate.
- FQ-4 corrected standard `ksflag` frost activation and folded in the FQ-2
  full-WAT closure-ledger fix.

The original FROSTVAL01 `executed-hold` artifacts remain historical evidence for
the first run. This supplement supersedes the original hold verdict for current
package disposition.

## Commands

Ran:

- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
- `openwepp-cli-hill --run-dir /wc1/runs/al/algebraic-radium/wepp/runs --run-file <generated-wrapper.toml> --output-dir <prefix-output-dir> --policy compat`

Fresh run root:

- `/tmp/frostval01_rerun_20260611T020951Z`

Generated evidence:

- `/tmp/frostval01_rerun_20260611T020951Z/run_status.tsv`
- `/tmp/frostval01_rerun_20260611T020951Z/off/run_status.tsv`
- `/tmp/frostval01_rerun_20260611T020951Z/reports/activation_summary.csv`
- `/tmp/frostval01_rerun_20260611T020951Z/reports/frost_off_summary.csv`
- `/tmp/frostval01_rerun_20260611T020951Z/reports/frost_on_off_deltas.csv`
- `/tmp/frostval01_rerun_20260611T020951Z/reports/annual_closure_residuals.csv`
- `/tmp/frostval01_rerun_20260611T020951Z/reports/summary.json`

Execution note:

- The first parallel frost-on pass returned one transient nonzero exit for `p4`
  after producing WAT/pass outputs, caused by an empty release metadata sidecar
  parse (`RELMD-E-003`). A serial rerun of `p4` succeeded and the final
  `run_status.tsv` records `43/43` clean exits.

## Activation Gate

Ran:

- Frost-on population prefixes: `43`
- Frost-on failures: `[]`
- Frost-off paired population failures: `[]`
- WAT outputs: `43`
- `frsoil.active=false`: `[]`
- Zero-`frozwt` prefixes: `[]`
- `max(frozwt)` range: `27.499999999999993..31.000000000000007 mm`
- `Q` total range: `377.4596796181721..932.4370340257644 mm`

Paired frost-on/off evidence:

- `43/43` prefixes have nonzero `sum_frozwt` delta.
- `43/43` prefixes have nonzero `Q` delta.
- `43/43` prefixes have nonzero `latqcc` delta.
- Maximum absolute deltas:
  - `Q`: `570.94468755901 mm`
  - `Dp`: `55.27367886777205 mm`
  - `latqcc`: `345.8282278819506 mm`

Verdict: Milestone 1 is satisfied. Frost is active across the full single-OFE
population, and the frost-off paired run proves the gate changes hydrology.

## Closure Gate

Ran:

- Annual closure rows: `258` (`43` prefixes x years `2..7`)
- Corrected identity:
  `RM + Irr - Interception - Q - Ep - Es - Er - Dp - latqcc - Tile - delta(SoilWaterTotal)`.
- Initial annual storage uses the previous day's ending `SoilWaterTotal`; final
  storage uses the last day in the evaluated year.
- Max absolute annual residual: `3.2173375075217336e-11 mm`
- Mean absolute annual residual: `1.2140145908367492e-11 mm`
- Worst row: `p39`, year `6`, residual `-3.2173375075217336e-11 mm`

Verdict: closure-under-frost is satisfied at numerical noise. The original
FROSTVAL01 `frost-break` classification remains withdrawn as a defective-ledger
artifact.

## Disposition

FROSTVAL01 is complete after rerun on the repaired substrate. No new defect-shaped
follow-on is required for standard `ksflag` frost activation or single-OFE
closure-under-frost.

Residual boundaries remain as documented by FQ-4 and later frost packages:

- Frost depth magnitude/parity is separate from activation and closure and is
  handled by FDMC01/FDHP01.
- MOFE/routing remains outside this single-OFE package.
- Forest `ksatadj` remains outside this standard-`ksflag` validation.
