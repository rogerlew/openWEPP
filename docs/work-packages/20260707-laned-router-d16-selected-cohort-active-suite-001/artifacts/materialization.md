# Materialization Evidence

Status: EXECUTED. Evidence mode: Static + Ran.

## Command

Ran:

```text
/home/workdir/wepppy/.venv/bin/python docs/work-packages/20260707-laned-router-d16-selected-cohort-active-suite-001/artifacts/materialize_selected_cohort.py
```

Result:

```json
{"manifest": "/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-d16-selected-cohort-active-suite-001/artifacts/selected-cohort-materialization.json", "members": 4}
```

## Generated Inputs

Generated run directories:

- `artifacts/selected-cohort-runs/h2637`
- `artifacts/selected-cohort-runs/mn_corn_h4`
- `artifacts/selected-cohort-runs/n_idaho_forest_h1`
- `artifacts/selected-cohort-runs/wa_cascades_forest_h1`

Machine-readable manifest:

- `artifacts/selected-cohort-materialization.json`

Each member has separate mode-specific runfiles:

- `*.plain.run.toml` writes to `output-plain/`.
- `*.hybrid.run.toml` writes to `output-hybrid/`.
- `snow.txt` is copied as a legacy-discovered sidecar and is not emitted as an
  invalid string key in the TOML runfile; explicit `[inputs.snow]` inline
  values are not invented by this package.

## Source Authority Chain

External members:

1. Read `<root>/landuse/landuse.parquet`.
2. Selected the lowest `wepp_id` with `_map = disturbed` and the package's
   target `disturbed_class`.
3. Copied source sidecars from `<root>/wepp/runs/` into the package-local run
   directory.
4. Read the source legacy management with WEPPpy `managements.py`.
5. Loaded the WEPPpy Disturbed route-coefficient table from
   `/home/workdir/wepppy/wepppy/nodb/mods/disturbed/route_coefficients.py`.
6. Generated native `ow-lanuse-1` management text with
   `Management.as_openwepp_native_cropland(...)`.

This avoids importing the full Disturbed NoDb controller because that import
attempts Redis connections in this local shell. The route-coefficient values and
validation still come from the canonical WEPPpy Disturbed coefficient module.

## Selected Native Route Coefficients

| Member | Class | Values |
|---|---|---|
| `mn_corn_h4` | `agriculture crops` | `480.0, 0.25, 0.010, 0.050, 0.12` |
| `n_idaho_forest_h1` | `forest` | `410.0, 0.95, 0.060, 0.200, 0.75` |
| `wa_cascades_forest_h1` | `forest` | `410.0, 0.95, 0.060, 0.200, 0.75` |

H2637 reuses the prior D16 package-local native fixture with
`500.0, 0.0, 0.0, 0.0, 0.0`, already accepted as H2637 diagnostic timing
evidence and not used as Disturbed production policy.

## Negative Proof

- No `/wc1/runs/*` input root was mutated.
- No legacy WEPP row/ridge/random-roughness fields were used to compute route
  coefficients.
- Generated external `.man` files start with `ow-lanuse-1` and carry explicit
  `routing_coefficients` markers.
- Generated runfiles are package-local TOML bindings; the inventory manifests
  under `tools/owcmp/suites/` remain unchanged.
