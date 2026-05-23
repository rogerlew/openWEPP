# PL14 Replay Lane Configuration and Guard Map

Status: `complete`
Evidence mode: `Static + Ran`

## Replay Lane Configuration

- Comparator run root: `/tmp/pl14_tiera_cmp_20260523`
- Baseline lane:
  - root: `/tmp/pl14_tiera_cmp_20260523/baseline`
  - source fixture: `/workdir/wepp-forest_260430_baseline/tests/fixtures/delicate_game_pw0/runs`
  - execution binary: `/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill`
- Candidate lane:
  - root: `/tmp/pl14_tiera_cmp_20260523/candidate`
  - source surface: WB13 openWEPP candidate
    `wb13-h5-wat-candidate-sample.dat` staged as `output/H5.wat.dat`
- Output subdir: `output`
- Include surfaces:
  - `H5.wat.dat`
  - `H5.plot.dat`
- Tolerances:
  - `abs_tol = 0`
  - `rel_tol = 0`

## Guard Map

| Guard ID | Trigger | Detection surface | Failure posture |
|---|---|---|---|
| `PL14-G-001` | Missing required single-OFE routing metadata | `pl14_tier_a_candidate_replay_contract` (`route_comparator_tier_metadata`) | Typed hard failure (`MissingRequiredMetadata` / `SingleOfeCountMismatch`) |
| `PL14-G-002` | Missing required WB13 replay symbol | `pl14_tier_a_candidate_replay_contract` (`Wb13DailyWaterBalanceRow::from_surface`) | Typed hard failure (`MissingRequiredOutputSymbol`) |
| `PL14-G-003` | Missing required comparator artifact in candidate lane (`H5.plot.dat`) | `h5_plot_comparator.json` (`only_baseline_count=1`) | Strict comparator failure (`strict_pass=false`), explicit hold signal for PL15 disposition |
| `PL14-G-004` | Structural schema mismatch in compared surface | `h5_wat_comparator.json` (`status=structure_diff`) | Strict comparator failure (`strict_pass=false`), no fallback schema rewriting |

## Explicit No-Fallback Posture

- No candidate fallback artifact was synthesized for `H5.plot.dat`.
- Missing candidate artifact surfaced explicitly in comparator JSON.
- No clamping/defaulting was applied to strict comparator outcomes.
