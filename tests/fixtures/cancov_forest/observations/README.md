# Canopy-Stratified Snow Observations

Normalized observation tables for the `tests/fixtures/cancov_forest/` canopy
gradient. These are external-authority observations for SNOWDENSITY-10.3.3 and
later snow/frost fidelity work.

## Files

- `sites/harvard_hf237_strata.csv`: Harvard Forest HF237 daily open,
  hardwood, and hemlock snow depth/SWE/density.
- `profiles/harvard_hf237_density_profiles.csv`: Harvard HF237 vertical density
  profile observations.
- `sites/marcell_rds_2021_0016_points.csv`: Marcell RDS-2021-0016 point-level
  snow depth/SWE observations.
- `sites/marcell_rds_2021_0016_snowcourse_means.csv`: Marcell snowcourse means.
- `sites/marcell_rds_2021_0016_stratum_means.csv`: Marcell conifer,
  deciduous, and open stratum means.

## Binding Notes

- Harvard `open` binds to `harvard_open_ma`.
- Harvard `hardwood` binds to `harvard_deciduous_ma`.
- Harvard `hemlock` is installed but remains unbound because the current
  Harvard fixture set has no pure hemlock/conifer hillslope.
- Marcell `conifer`, `deciduous`, and `open` bind to
  `marcell_conifer_mn`, `marcell_deciduous_mn`, and `marcell_open_mn`.
- Marcell snowcourse `S53` is retained as `unknown` in point/snowcourse files
  but excluded from stratum means because it is not described in the RDS
  metadata or coordinate supplement.

## Regeneration

```sh
.venv/bin/python tools/snowfreeze_observed/cancov_stratified_observations.py
```
