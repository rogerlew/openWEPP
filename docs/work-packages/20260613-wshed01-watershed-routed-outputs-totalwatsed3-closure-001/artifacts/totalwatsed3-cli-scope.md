# totalwatsed3 CLI Scope

Status: T-A executed

Evidence mode: Static + Ran

## Scope Decision

Build `openwepp-cli-totalwatsed3` as a dedicated openWEPP-native producer. It
is not part of `openwepp-cli-watershed`, does not use channel routing, and must
not depend on wepppyo3 `wepp_interchange`. wepppy remains the authoritative
semantic reference for this package, not a code dependency.

The T-arc supersedes the W-C/W-D watershed-CLI `totalwatsed3` path. The W-B and
W-C watershed fixes remain valid for the separate channel-output follow-on, but
`totalwatsed3` closure is hillslope-only:

```text
P - (Runoff + Lateral Flow + ET + Percolation + Interception) - DeltaStorage
```

`Runoff` is the independent PASS `runvol` operand. It is not WAT `Q`, and
`runvol == Q * Area / 1000` is a self-consistency smell unless the producer can
prove those operands came from independent surfaces.

## Authoritative Semantics Read

wepppy producer:

- `/home/workdir/wepppy/wepppy/wepp/interchange/totalwatsed3.py:37-57`
  names PASS, WAT, soil, and element optional columns.
- `totalwatsed3.py:72-131` defines the legacy-compatible schema. Exact
  hydrology fields (`runvol`, `P`, `RM`, `Q`, `Dp`, `latqcc`, `QOFE`, `Ep`,
  `Es`, `Er`) are volumes in `m^3`; depth aliases are in `mm`.
- `totalwatsed3.py:583-610` aggregates PASS by day and sums `runvol`,
  `sbrunv`, sediment masses, detachment, and deposition.
- `totalwatsed3.py:613-676` aggregates WAT by day with `depth_mm * Area *
  0.001` volume weighting. When an OFE selector exists, `latqcc` contributes
  only from the max/outlet OFE per `wepp_id` and day.
- `totalwatsed3.py:679-808` optionally aggregates `TSMF`, `QRain`, and `QSnow`
  from soil/element files by joining through WAT area.
- `totalwatsed3.py:919-1006` merges PASS and WAT, derives depth aliases from
  volumes, computes baseflow diagnostics, and derives `Streamflow`.

wepppy audit:

- `/home/workdir/wepppy/tools/totalwatsed3_daily_closure_audit.py:32-69`
  requires the date, area, exact volume, and storage fields; reported depth
  aliases and profile terms are optional diagnostics.
- `totalwatsed3_daily_closure_audit.py:90-145` reconstructs depths from exact
  volumes and computes the primary closure with storage.
- `totalwatsed3_daily_closure_audit.py:136-153` uses total precipitation as
  the primary closure input and retains Rain+Melt closure only as a diagnostic.

openWEPP current seam:

- `crates/openwepp-hillslope-output/src/contracts.rs:62-68` requires
  `outputs.pass` as `.hbp`; optional WAT/soil/element outputs are parquet.
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs:259-305`
  writes the HBP event payload, but the six event volume slots are currently
  zero and no PASS `runvol` parquet is emitted from this path.
- `crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs:145-166`
  consumes those six volume slots without exposing them.
- `crates/openwepp-runner/src/watershed_wat.rs:216-253` uses pass filenames
  only to locate sibling WAT parquet files, then aggregates WAT. This is the
  superseded W-D self-consistency path.

Ran schema sample:

```text
/home/workdir/wepppy/.venv/bin/python - <<'PY'
from pathlib import Path
import pyarrow.parquet as pq
base = Path('/wc1/runs/ar/arboreal-dendrite/wepp/output/interchange')
for name in ['H.pass.parquet', 'H.wat.parquet', 'H.soil.parquet', 'H.element.parquet']:
    path = base / name
    schema = pq.read_schema(path)
    pf = pq.ParquetFile(path)
    print(name, len(schema.names), pf.metadata.num_rows)
PY
```

Observed substrate shape:

| File | Rows | Key columns |
|---|---:|---|
| `H.pass.parquet` | `78912` | `wepp_id`, date fields, `runvol`, `sbrunv`, `peakro`, `tdet`, `tdep`, `sedcon_1..5` |
| `H.wat.parquet` | `271808` | `wepp_id`, `ofe_id`, date fields, `OFE`, WAT flux/storage fields, `Area`, profile terms |
| `H.soil.parquet` | `271808` | `wepp_id`, `ofe_id`, date fields, `OFE`, `TSMF` |
| `H.element.parquet` | `74380` | `wepp_id`, `ofe_id`, date fields, `QRain`, `QSnow` |

## CLI Input Contract

`openwepp-cli-totalwatsed3` should consume an interchange directory and emit
one `totalwatsed3.parquet`.

Required inputs:

- PASS event rows with `wepp_id`, date fields, `runvol`, `sbrunv`, `tdet`,
  `tdep`, and sediment concentration columns. T-B must provide this as an
  openWEPP-native parquet reader/writer surface. Current openWEPP HBP-only
  output is insufficient until the HBP/PASS lineage exposes nonzero `runvol`.
- WAT rows with `wepp_id`, date fields, `Area`, `P`, `RM`, `Q`, `Dp`,
  `latqcc`, `QOFE`, `Ep`, `Es`, `Er`, `Total-Soil Water`, `frozwt`, and
  `Snow-Water`.

Optional inputs:

- WAT optional terms: `SoilWaterTotal`, `ProfileDepth`,
  `ProfilePorosityCap`, `ProfileFCStore`, `ProfileWPStore`, `Interception`,
  and `InterceptionStorage`.
- Soil rows for `TSMF`.
- Element rows for `QRain` and `QSnow`.

Selector rules:

- `sim_day_index` is canonical; `day` may be accepted as a compatibility alias
  only if tests cover it.
- OFE selector accepts `ofe_id` or `OFE`.
- `wepp_id` is required for multi-hillslope input. Empty `wepp_id` selectors
  are only valid for single-surface test fixtures with an explicit contract
  reason.

Area rules:

- WAT `Area` is the primary area source for hydrology weighting.
- If a future CLI option supplies an external area lookup, it must be checked
  against WAT `Area` where both exist; nonpositive, missing, or divergent area
  must fail closed instead of silently defaulting.

## Aggregation Semantics

Group key:

```text
year, sim_day_index, julian, month, day_of_month, water_year
```

PASS aggregation:

- `runvol`, `sbrunv`, `tdet`, and `tdep`: sum by group.
- `seddep_i`: sum `sedcon_i * runvol` by group.
- `sed_del`: sum `seddep_1..5`.
- `sed_vol_conc`: compute from summed sediment mass and summed `runvol`;
  zero runoff produces zero concentration.

WAT aggregation:

- `Area`: sum by group.
- Exact volume fields: sum `depth_mm * Area / 1000.0`.
- `Q`: WAT runoff volume diagnostic, not the closure Runoff operand.
- `latqcc`: if no OFE selector exists, sum normally. If OFE selector exists,
  include only rows whose OFE equals the max OFE for that `wepp_id` and day.
- `QOFE`: sum by area-weighted volume; do not substitute for `runvol`.
- `UpStrmQ`, `SubRIn`, storage/profile/interception depth fields: publish as
  area-weighted depths after summing volume equivalents.
- Missing `Interception` publishes `0.0 mm`; missing profile/storage optional
  terms publish null/absent-equivalent diagnostics, not invented values.

Soil/element optional aggregation:

- `TSMF`: area-weighted mean using WAT area for matching
  `(wepp_id, ofe_id, date)`.
- `QRain`/`QSnow`: area-weighted depths using WAT area for matching
  `(wepp_id, ofe_id, date)`.

Baseflow/output-only diagnostics:

- `Baseflow`, `Aquifer losses`, `Reservoir Volume`, and `Streamflow` are not
  part of the closure audit. T-B may implement deterministic zero defaults or
  explicit baseflow options, but it must record the policy and test it.

## Output Schema

The openWEPP-native schema must at minimum satisfy the audit-required surface
and keep the W-D repairs:

- Date fields: `year`, `sim_day_index`, `julian`, `month`, `day_of_month`,
  `water_year`.
- Exact volumes in `m^3`: `runvol`, `sbrunv`, `P`, `RM`, `Q`, `Dp`, `latqcc`,
  `QOFE`, `Ep`, `Es`, `Er`.
- Required storage/profile depths in `mm`: `Total-Soil Water`, `frozwt`,
  `Snow-Water`; optional `SoilWaterTotal`, `ProfileDepth`,
  `ProfilePorosityCap`, `ProfileFCStore`, `ProfileWPStore`.
- Depth aliases in `mm`: `Precipitation`, `Rain+Melt`, `Runoff`,
  `Lateral Flow`, `Percolation`, `Transpiration`, `Evaporation`, `ET`,
  `Interception`.

Water-balance acceptance does not require ash columns. If the existing
watershed writer schema is reused, ash fields may publish deterministic zeros
when no ash input is configured, but they are not T-B/T-C closure operands.

## Closure Gate

Primary daily residual:

```text
residual_mm =
  Precipitation
  - (Runoff + Lateral Flow + ET + Percolation + Interception)
  - DeltaStorage
```

Storage basis:

- Basic audit storage: `Total-Soil Water + frozwt + Snow-Water`.
- Enriched audit storage when present: `SoilWaterTotal + Snow-Water`.

Acceptance requirements:

- `Runoff` must derive from PASS `runvol / Area * 1000`, independently from
  WAT `Q`.
- Reported depth aliases must match volume-reconstructed depths within the
  configured tolerance.
- Whole-run and daily residuals must close at the established noise floor. T-C
  should use `SC-WATBAL-001` `TOL-WATBAL-008` (`<= 1e-9 mm`) for the
  area-weighted hillslope-total residual unless T-B first amends contract
  authority for a totalwatsed3-specific tolerance.
- Exact all-zero residuals on the real arboreal-dendrite cohort are a
  tautology hold, not acceptance.

## Red Tests for T-B

Contract/schema tests:

- `openwepp-cli-totalwatsed3` binary exists and is listed by the runner crate.
- CLI rejects missing PASS or WAT input with a typed error.
- CLI rejects nonpositive area and mixed-null required operands.
- CLI emits audit-required columns with exact volume/depth units.

Operand-lineage tests:

- A fixture where PASS `runvol` differs from WAT `Q * Area / 1000` must publish
  `runvol` and `Runoff` from PASS, while retaining WAT `Q` as a diagnostic.
- A MOFE fixture with nonzero internal `latqcc` on non-outlet OFEs must publish
  outlet-only `latqcc`.
- A MOFE fixture must sum `QOFE` by area-weighted volume and must not clone
  aggregate rows across OFEs.
- Missing `Interception` publishes `0.0`; present interception is
  area-weighted from WAT.

Real-run gates:

- The CLI produces `totalwatsed3.parquet` from the arboreal-dendrite
  `H.pass.parquet` + `H.wat.parquet` + optional soil/element inputs.
- Output rows match the WAT date domain, sorted by
  `(year, julian, sim_day_index)`.
- The wepppy audit can read the output without schema repair.

## T-B/T-C Breakdown

T-B implementation:

- Add the dedicated CLI entrypoint.
- Add an openWEPP-native PASS parquet reader/writer or HBP-to-PASS adapter that
  exposes canonical `runvol` and companion PASS metrics.
- Implement area-weighted aggregation and openWEPP-native parquet writing.
- Add focused unit/integration tests for schema, PASS-vs-WAT independence,
  MOFE outlet lateral handling, and optional fields.
- Remove or relocate the superseded `build_watershed_daily_rows_from_wat` path
  so `openwepp-cli-watershed` no longer owns totalwatsed3 production.

T-C closure:

- Run the dedicated CLI on arboreal-dendrite.
- Run `totalwatsed3_daily_closure_audit.py` against the emitted parquet.
- Accept only nonzero-at-noise independent closure.
- On pass, update the package disposition, remove the ROADMAP deferral, and
  name `WATERSHED-CHANWB-ROUTED-OUTPUT` as the decoupled follow-on.
