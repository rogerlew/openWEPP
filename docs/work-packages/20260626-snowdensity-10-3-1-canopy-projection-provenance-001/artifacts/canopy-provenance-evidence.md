# Canopy Provenance Evidence

Status: complete.

Evidence class: Static + Ran.

## Commands

Static extraction:

```bash
for d in tests/fixtures/cancov_forest/*; do
  [ -d "$d" ] || continue
  site=$(basename "$d")
  man=$(ls "$d"/*.man)
  awk 'NR==15{p15=$0} NR==16{p16=$0} NR==35{p35=$0} END{print site,p15,p16,p35}' site="$site" "$man"
done
```

Runtime evidence:

```bash
cargo run -q -p openwepp-runner --bin openwepp-snowbench -- \
  coe-melt --run-dir tests/fixtures/cancov_forest/<fixture> \
  --output-dir docs/work-packages/20260626-snowdensity-10-3-1-canopy-projection-provenance-001/artifacts/coe_melt_runtime_surface/<fixture> \
  --model legacy_coe
```

The first attempted `export-pysnobal` route was stopped because the CLI includes
the full openWEPP snow projection and generated oversized intermediate forcing.
The retained `coe-melt` route calls the same `export_pysnobal_inputs` runtime
surface extraction and writes `constants.canopy_cover_fraction` in
`coe_melt_summary.json`. Generated forcing bridges and daily snow CSVs were
deleted after the summaries were retained to keep the package small.

## Upstream wepppy seasonal projection authority

Source:
`/home/workdir/wepppy/docs/work-packages/20260626_deciduous_mixed_forest_managements/artifacts/winter-cancov-validation.md`.

Final-year winter mean `Cancov`:

| Class | Projected winter mean |
|---|---:|
| Evergreen | `0.90000` |
| Mixed | `0.44446` |
| Deciduous | `0.06653` |

Important limitation: this is upstream wepppy/WEPP trajectory evidence by
management class, not current openWEPP snowbench runtime evidence.

## Per-Fixture Canopy Table

Raw values are from the committed fixture `.man` files. `initial cancov` is the
field openWEPP currently seeds into the runtime surface. `plant reference` is the
line-16 canopy/reference value used to identify management class intent; it is
not the snowbench runtime value when the initial condition differs.

| Fixture | Class | Plant reference | Initial `cancov` | wepppy projected winter mean | openWEPP runtime `cancov` | Runtime source |
|---|---|---:|---:|---:|---:|---|
| `berthoud_conifer_co` | RAP_TS conifer | `1.00000` | `0.05000` | `0.90000` | `0.05000` | `generated_openwepp_runtime_surface.cancov` |
| `harvard_mixed_ma` | mixed | `0.55000` | `0.55000` | `0.44446` | `0.55000` | `generated_openwepp_runtime_surface.cancov` |
| `hjandrews_conifer_or` | conifer | `1.00000` | `0.90000` | `0.90000` | `0.90000` | `generated_openwepp_runtime_surface.cancov` |
| `hubbardbrook_deciduous_nh` | deciduous | `0.20000` | `0.20000` | `0.06653` | `0.20000` | `generated_openwepp_runtime_surface.cancov` |
| `marcell_mixed_mn` | mixed | `0.55000` | `0.55000` | `0.44446` | `0.55000` | `generated_openwepp_runtime_surface.cancov` |
| `morescreek_conifer_id` | RAP_TS conifer | `1.00000` | `0.82000` | `0.90000` | `0.82000` | `generated_openwepp_runtime_surface.cancov` |
| `sleepers_pasture_vt` | pasture/ag | `0.90000` | `0.50000` | n/a | `0.50000` | `generated_openwepp_runtime_surface.cancov` |
| `tenderfoot_conifer_mt` | conifer | `1.00000` | `0.90000` | `0.90000` | `0.90000` | `generated_openwepp_runtime_surface.cancov` |

## Runtime Artifact Index

Generated summary artifacts retained:

- `artifacts/coe_melt_runtime_surface/berthoud_conifer_co/coe_melt_summary.json`
- `artifacts/coe_melt_runtime_surface/harvard_mixed_ma/coe_melt_summary.json`
- `artifacts/coe_melt_runtime_surface/hjandrews_conifer_or/coe_melt_summary.json`
- `artifacts/coe_melt_runtime_surface/hubbardbrook_deciduous_nh/coe_melt_summary.json`
- `artifacts/coe_melt_runtime_surface/marcell_mixed_mn/coe_melt_summary.json`
- `artifacts/coe_melt_runtime_surface/morescreek_conifer_id/coe_melt_summary.json`
- `artifacts/coe_melt_runtime_surface/sleepers_pasture_vt/coe_melt_summary.json`
- `artifacts/coe_melt_runtime_surface/tenderfoot_conifer_mt/coe_melt_summary.json`

