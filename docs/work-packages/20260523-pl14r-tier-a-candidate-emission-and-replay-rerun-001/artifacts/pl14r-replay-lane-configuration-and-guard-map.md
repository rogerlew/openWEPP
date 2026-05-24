# PL14R Replay Lane Configuration and Guard Map

Status: `complete`
Evidence mode: `Static + Ran`

## Replay Lane Configuration

- Comparator run root: `/tmp/pl14r_tiera_cmp_20260523`
- Baseline lane:
  - root: `/tmp/pl14r_tiera_cmp_20260523/baseline`
  - source fixture: `/workdir/wepp-forest_260430_baseline/tests/fixtures/delicate_game_pw0/runs`
  - execution binary: `/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill`
- Candidate lane:
  - root: `/tmp/pl14r_tiera_cmp_20260523/candidate`
  - source surface: WB13 openWEPP candidate
    `wb13-h5-wat-candidate-sample.dat` staged as `output/H5.wat.dat`
- Output subdir: `output`
- Include surfaces (strict Tier-A replay lane):
  - `H5.wat.dat`
  - `H5.plot.dat`
- Tolerances:
  - `abs_tol = 0`
  - `rel_tol = 0`

## Guard Map

| Guard ID | Trigger | Detection surface | Failure posture |
|---|---|---|---|
| `PL14R-G-001` | Missing required single-OFE routing metadata | `pl14r_tier_a_replay_rerun_contract` (`route_comparator_tier_metadata`) | Typed hard failure (`MissingRequiredMetadata`) |
| `PL14R-G-002` | Missing required replay include surface in candidate lane (`H5.plot.dat`) | `h5_plot_comparator.json` (`raw.only_baseline_count=1`, `raw.only_baseline_examples=["H5.plot.dat"]`) | Strict comparator failure (`strict_pass=false`) + explicit `HOLD` |
| `PL14R-G-003` | Structural mismatch in `H5.wat.dat` under strict tolerance | `h5_wat_comparator.json` (`raw.status_counts.structure_diff=1`) | Strict comparator failure (`strict_pass=false`) + explicit `HOLD` |
| `PL14R-G-004` | Missing reproducibility hashes for binaries/tools/outputs/artifacts | `pl14r-comparator-run-provenance-manifest.md` | Hard-fail / `HOLD` by `INV-SYSTEM-014` |
| `PL14R-G-005` | Fallback artifact substitution for required include surfaces | PL14R contract + test vector obligations (`INV-WATBAL-014`) | Prohibited; violation forces hard-fail / `HOLD` |

## Explicit No-Fallback Posture

- No synthetic `H5.plot.dat` candidate artifact was created.
- Missing candidate include-surface coverage is surfaced explicitly in
  comparator JSON and dispositioned as `HOLD`.
- No clamping/defaulting/fallback substitution was applied to strict comparator
  outcomes.
