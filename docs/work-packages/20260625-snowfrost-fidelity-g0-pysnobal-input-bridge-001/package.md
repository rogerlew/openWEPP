# SNOWFROST-FIDELITY-G0 PySnobal Input Bridge

Status: queued

Package type: diagnostic bridge and harness implementation.

Primary gap: `GAP-SNOWFREEZE-002`.

Objective: add an offline Rust tool that reads WEPP/openWEPP hillslope inputs
and emits PySnobal-ready forcing/config artifacts, then add a thin Python
harness that runs PySnobal against those artifacts and validates that SWE and
physical snow-depth outputs are sane enough to inform SNOWFROST-FIDELITY
snow-depth work.

This package follows `AGENTS.md`, `docs/work-packages/AGENTS.md`,
`docs/codex_exec_plans.md`, `docs/standards/kernel-work-package-preparation.md`,
`docs/specifications/unit-governance.md`, `crates/AGENTS.md`, `tests/AGENTS.md`,
`docs/specifications/science-contracts/AGENTS.md`,
`docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`,
`docs/specifications/wepp-input-files/specs/climate-file.spec.md`,
`tools/snowfreeze_observed/README.md`, and SNOWFROST-FIDELITY-A through F.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only science-review, harness-review, and
verification subagents for PySnobal forcing-lineage review, anti-alias review,
and final package verification. Expected outputs are compact Markdown findings
summarized into `artifacts/review_agent_a.md`,
`artifacts/review_agent_b.md`, `artifacts/verification_agent_a.md`, and
`artifacts/verification_agent_b.md`; subagents may not edit files.

## Purpose

SNOWFROST-FIDELITY-E/F proved that current openWEPP and pinned legacy WEPP both
fail physical snow-depth control at the paired observed sites, while current
openWEPP SWE is already close to legacy SWE. Before changing snow-depth
producer/carry/input/settlement logic, this package asks a diagnostic question:
given the same WEPP/openWEPP meteorological inputs and snow-partition lineage,
can PySnobal produce physically plausible snow depth and SWE for the pilot
sites?

The package must not make PySnobal a correctness authority. PySnobal output is
hypothesis evidence only. Observed physical snow depth and
`SC-SNOWFREEZE-001#INV-SNOWFREEZE-048` remain the snow-control authority.

## Non-Goals

- Do not change production snow, frost, hydrology, erosion, storage, or
  publication physics.
- Do not tune openWEPP snow-depth, frost-depth, heat-flow, frozen-K, SFCC,
  impedance, or migration/fringe parameters.
- Do not default-activate direct runtime or delete compatibility runtime.
- Do not classify an openWEPP snow or frost defect from PySnobal agreement
  alone.
- Do not require PySnobal as a build/test dependency for normal Rust workspace
  gates.
- Do not silently install PySnobal, Cython, NumPy, pandas, or other Python
  dependencies from the harness.

## Authority Envelope

In scope:

- WEPP `.run`, `.cli`, `.slp`, `.sol`, `snow.txt`, `frost.txt`,
  `pmetpara.txt`, and fixture-local manifests needed to build a hillslope
  snow diagnostic run.
- Existing openWEPP parser/runtime input projection for daily climate,
  hourly winter air temperature, hourly radiation, cloud fraction, rain/snow
  partition, snow controls, slope/aspect, and station elevation.
- PySnobal input schema from `/workdir/pysnobal/pysnobal/defaults.py`:
  `net_solar_Wm-2`, `downwelling_thermal_Wm-2`, `temp_air_degC`,
  `temp_ground_degC`, `vapor_pressure_Pa`, `wind_speed_ms-1`,
  `precip_mass_mm`, `precip_temp_degC`, `snow_precip_fraction`, and
  `snow_precip_density_kgm-3`.
- PySnobal outputs `specific_mass_snow_kgm-2` and `thickness_snow_m` as the
  diagnostic SWE and physical snow-depth surfaces.
- Constant ground-temperature sensitivity lanes:
  `Tg=-2.5 degC at z_g=0.10 m`, `Tg=-0.5 degC at z_g=0.10 m`, and
  `Tg=0.0 degC at z_g=0.10 m`.
- Diagnostic longwave, net-shortwave, and roughness assumptions required only
  to make PySnobal run, each explicitly labeled as a proxy in lineage output.

Out of scope:

- Production process-physics edits.
- Observation tolerance changes.
- New external observation acquisition.
- Reanalysis, HRRR/WRF/Noah, NLDAS, ERA5-Land, SHAW, or full soil-temperature
  model coupling.
- Runtime coupling from PySnobal back into openWEPP.
- PySnobal vendoring or dependency management.

## Required Design Decisions

1. Implement Rust input generation in `openwepp-runner`, not in Python.
   Rust must reuse openWEPP parser and runtime projection code for WEPP inputs,
   daily-to-hourly air temperature/radiation, and rain/snow partition. Python
   may run PySnobal and compare outputs, but it must not recreate CLIGEN or
   WEPP daily-to-hourly algorithms.
2. Add a new runner binary named `openwepp-snowbench` with a subcommand or mode
   named `export-pysnobal`.
3. Emit complete, uniform hourly PySnobal forcing rows. If existing winter
   forcing helpers suppress warm/no-snow days, add a diagnostic-only projection
   path that reuses the same SIMIMPL28 calculations but explicitly forces
   export. This must not alter production winter-trigger behavior.
4. Treat PySnobal `temp_ground_degC` as soil temperature at depth `z_g`, not as
   snow-surface temperature. `frost.hourly.surface_temp_c_####`,
   `surtmp(hour)`, and any openWEPP adjusted surface temperature are invalid
   `temp_ground_degC` sources for this package.
5. Convert snowfall depth to precipitation mass before feeding PySnobal:
   `snow_mass_mm = snowfall_depth_m * snow_density_kg_m3`. This is valid
   because `1 kg m^-2 = 1 mm` water equivalent. Do not treat
   `snow.hourly.snowfall_m_####` as millimeters of water.
6. Label every exported field with source class:
   `mechanical`, `deterministic-derived`, or `diagnostic-proxy`.
7. PySnobal sanity gates may support SNOWFROST-FIDELITY routing, but they may
   not close `GAP-SNOWFREEZE-002` and may not authorize production snow/frost
   physics changes.

## Intended Write Set

- `crates/openwepp-runner/Cargo.toml`
- `crates/openwepp-runner/src/bin/openwepp-snowbench.rs`
- `crates/openwepp-runner/src/hillslope/**` or a new
  `crates/openwepp-runner/src/snowbench/**` module for exporter internals
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/**` only if a
  diagnostic hourly projection helper is needed
- `tools/snowfreeze_observed/pysnobal_compare.py`
- `tools/snowfreeze_observed/README.md`
- `tests/integration/snowfrost_fidelity_g0_pysnobal_bridge_contract.rs`
- `docs/work-packages/20260625-snowfrost-fidelity-g0-pysnobal-input-bridge-001/**`
- `docs/work-packages/README.md`

Any write outside this set requires package amendment before implementation.

## Deliverables

- Rust CLI binary `openwepp-snowbench export-pysnobal`.
- Per-site/per-lane export artifacts:
  - `forcing.csv` with the exact PySnobal required columns.
  - `config.yaml` with `io`, `z`, `params`, `init`, and `defaults`.
  - `lineage.json` describing source class, units, conversion, and provenance
    for every PySnobal input column.
  - `openwepp_snow.csv` containing current openWEPP daily `Snow-Water` and
    `Snow-Depth` comparison surfaces where available.
  - `audit.json` and `audit.md` with mass/alias/unit checks.
- Python harness `tools/snowfreeze_observed/pysnobal_compare.py`.
- All-five-site PySnobal result artifacts under this package:
  - `artifacts/pysnobal_site_summary.json`
  - `artifacts/pysnobal_site_summary.md`
  - per-site/per-lane compact CSV/JSON references or summaries.
- Contract/regression tests that prevent the known alias and proxy mistakes.
- Review, verification, line-count governance, gate results, disposition, and
  worker-handoff artifacts.

## PySnobal Export Schema

The Rust exporter must emit CSV rows indexed by timestamp and columns in this
order:

1. `net_solar_Wm-2`
2. `downwelling_thermal_Wm-2`
3. `temp_air_degC`
4. `temp_ground_degC`
5. `vapor_pressure_Pa`
6. `wind_speed_ms-1`
7. `precip_mass_mm`
8. `precip_temp_degC`
9. `snow_precip_fraction`
10. `snow_precip_density_kgm-3`

Column rules:

- `temp_air_degC`: openWEPP hourly winter air temperature surface.
- `vapor_pressure_Pa`: saturation vapor pressure at dew point, converted from
  kPa to Pa, with the dew point held/interpolated according to the documented
  climate forcing convention.
- `wind_speed_ms-1`: WEPP daily wind speed repeated hourly unless a stronger
  openWEPP hourly surface exists in scope.
- `precip_mass_mm`: hourly rain water equivalent plus snowfall water
  equivalent. Rain uses `rain_m * 1000`. Snow uses
  `snowfall_depth_m * snow_density_kg_m3`.
- `snow_precip_fraction`: `snow_mass_mm / precip_mass_mm` when precipitation
  mass is positive, otherwise `0.0`.
- `snow_precip_density_kgm-3`: `snow.txt` `newsnw` value, or the parser's
  explicit default when `snow.txt` is absent.
- `precip_temp_degC`: hourly air temperature proxy unless a stronger
  precipitation temperature surface is introduced and documented.
- `net_solar_Wm-2`: hourly incoming shortwave converted from
  `MJ m^-2 h^-1` to `W m^-2`, multiplied by a documented fixed diagnostic
  net-shortwave factor. This is a proxy and must be labeled as such.
- `downwelling_thermal_Wm-2`: diagnostic longwave estimate derived from
  existing openWEPP hourly air temperature and cloud fraction. This is a proxy
  and must be labeled as such.
- `temp_ground_degC`: constant lane value for G0. The lane value and
  `z.soil_temp_m` must match and be recorded in `lineage.json`.

## Phase Plan

### Phase 0: Scaffold, Authority, and PySnobal Environment Probe

- Fill `artifacts/required-reading-map.md`.
- Record the local PySnobal availability probe:
  - path to `/workdir/pysnobal`;
  - whether the compiled extension imports under `PYSNOBAL_PYTHON`;
  - exact Python executable used for package validation.
- Record source evidence for PySnobal required columns and `T_g`/`z_g`
  semantics.
- Record why constant `T_g` lanes are the G0 strategy and why seasonal or
  external soil-temperature models are deferred.

Exit criteria:

- `artifacts/pre-implementation-evidence.md` exists and labels every claim as
  `Static:` or `Ran:`.
- The package either identifies an importable PySnobal runner or closes in
  `HOLD-PYSNOBAL-UNAVAILABLE` before code changes.

### Phase 1: Rust Exporter Skeleton and Schema Tests

- Add `openwepp-snowbench` to `crates/openwepp-runner/Cargo.toml`.
- Implement `openwepp-snowbench export-pysnobal --run-dir <path>
  --output-dir <path>`.
- Start with one site and the three constant `T_g` lanes.
- Emit `forcing.csv`, `config.yaml`, `lineage.json`, `audit.json`, and
  `audit.md` without running PySnobal yet.
- Add focused tests for schema, lane config, and fail-closed missing input
  behavior.

Exit criteria:

- `cargo build -p openwepp-runner --bin openwepp-snowbench` passes.
- Focused tests prove the CSV column set and lane config.
- The exporter refuses to emit rows with NaN, non-finite values, non-uniform
  timestamps, or negative precipitation mass.

### Phase 2: Anti-Alias and Unit-Lineage Closure

- Add explicit regression tests for the known failure classes:
  - snowfall depth is not water equivalent;
  - WAT `Snow-Water` is not physical snow depth;
  - `frost.hourly.surface_temp_c_####` is not `temp_ground_degC`;
  - raw daily `rad` in langleys/day is not written as hourly `W m^-2`;
  - missing proxy lineage for longwave/net-shortwave/ground temperature blocks
    export acceptance.
- Record `artifacts/schema-and-lineage.md` with every PySnobal field, unit,
  source, conversion, source class, and rejected aliases.

Exit criteria:

- `cargo test --test snowfrost_fidelity_g0_pysnobal_bridge_contract` passes.
- `artifacts/schema-and-lineage.md` proves every required PySnobal column has a
  source class and rejected-alias entry.

### Phase 3: Python Harness and One-Site PySnobal Run

- Add `tools/snowfreeze_observed/pysnobal_compare.py`.
- The harness must:
  - fail clearly if PySnobal cannot be imported;
  - read one or more Rust export directories;
  - call `pysnobal.pysnobal.run_snobal`;
  - write `pysnobal_output.csv`, `pysnobal_summary.json`, and
    `pysnobal_summary.md`;
  - compare PySnobal `specific_mass_snow_kgm-2` and `thickness_snow_m` against
    observed snow-depth rows where present and current openWEPP WAT surfaces
    where exported;
  - report route labels without declaring openWEPP defects.
- Run the harness for Site 1 first.

Exit criteria:

- `.venv/bin/python -m py_compile tools/snowfreeze_observed/pysnobal_compare.py`
  passes.
- A one-site PySnobal run exits successfully and produces finite, non-negative
  SWE/depth series for all three `T_g` lanes.
- The harness summary reports max SWE, max snow depth, peak date, observed
  paired snow-depth count, mean absolute observed-depth residual where present,
  and lane spread.

### Phase 4: All-Site SNOWFROST-FIDELITY Diagnostic Run

- Export PySnobal inputs for all five
  `tests/fixtures/snowfreeze_observed` pilot sites.
- Run PySnobal for all sites and all three constant `T_g` lanes.
- Generate compact package-level summaries:
  - site/lane success status;
  - max SWE and snow depth;
  - observed-depth residual summaries where paired rows exist;
  - openWEPP-vs-PySnobal snow-depth/SWE comparisons where openWEPP outputs are
    available;
  - sensitivity to constant ground-temperature lanes.
- Preserve raw outputs under `target/` and copy only compact summaries into the
  package artifacts.

Exit criteria:

- All five sites produce metric-bearing PySnobal summaries or a fail-closed
  site-specific blocker with root cause.
- `artifacts/pysnobal_site_summary.json` and
  `artifacts/pysnobal_site_summary.md` exist.
- The summary states whether PySnobal is reasonable enough to use as a
  diagnostic comparator for SNOWFROST-FIDELITY-G, or whether ground
  temperature/energy-forcing proxies dominate and must be improved first.

### Phase 5: Review, Verification, and Disposition

- Run focused and full validation commands.
- Complete dual reviews, finding disposition, dual verification, line-count
  governance, owned-file manifest, gate results, final disposition, and worker
  handoff.
- Update `tools/snowfreeze_observed/README.md` with the exporter and harness
  usage.

Exit criteria:

- Every gate in `package.md` is `PASS`, or the package closes in a named
  `HOLD-*` state with blocker evidence.
- No accepted review finding remains unfixed.
- The package states the next authorized route: proceed to
  SNOWFROST-FIDELITY-G snow-depth producer/carry/input/settlement DC, improve
  PySnobal forcing first, or hold because PySnobal is unavailable/inconclusive.

## Validation Commands

Run from `/home/workdir/openWEPP`.

- `cargo build -p openwepp-runner --bin openwepp-snowbench`
- `cargo run -p openwepp-runner --bin openwepp-snowbench -- export-pysnobal --run-dir tests/fixtures/snowfreeze_observed/site1_sleepers_south_field_vt --output-dir target/snowfrost_fidelity_g0/site1`
- `cargo run -p openwepp-runner --bin openwepp-snowbench -- export-pysnobal --run-dir tests/fixtures/snowfreeze_observed/site2_sleepers_w9_hardwood_vt --output-dir target/snowfrost_fidelity_g0/site2`
- `cargo run -p openwepp-runner --bin openwepp-snowbench -- export-pysnobal --run-dir tests/fixtures/snowfreeze_observed/site3_scan_mandan_nd --output-dir target/snowfrost_fidelity_g0/site3`
- `cargo run -p openwepp-runner --bin openwepp-snowbench -- export-pysnobal --run-dir tests/fixtures/snowfreeze_observed/site4_ggd498_morris_mn --output-dir target/snowfrost_fidelity_g0/site4`
- `cargo run -p openwepp-runner --bin openwepp-snowbench -- export-pysnobal --run-dir tests/fixtures/snowfreeze_observed/site5_reynolds_creek_us_rls_id --output-dir target/snowfrost_fidelity_g0/site5`
- `.venv/bin/python -m py_compile tools/snowfreeze_observed/pysnobal_compare.py`
- `PYSNOBAL_PYTHON=/tmp/pysnobal-build-venv/bin/python .venv/bin/python tools/snowfreeze_observed/pysnobal_compare.py --input-root target/snowfrost_fidelity_g0 --output-json docs/work-packages/20260625-snowfrost-fidelity-g0-pysnobal-input-bridge-001/artifacts/pysnobal_site_summary.json --output-md docs/work-packages/20260625-snowfrost-fidelity-g0-pysnobal-input-bridge-001/artifacts/pysnobal_site_summary.md`
- `cargo test --test snowfrost_fidelity_g0_pysnobal_bridge_contract`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `git diff --check`
- `rg -n "qwet|Qwet|frzftp" crates || true`

The PySnobal command may use a different Python executable, but the exact
executable must be recorded in `artifacts/pysnobal-run-evidence.md`. If no
working PySnobal executable exists, close in `HOLD-PYSNOBAL-UNAVAILABLE`
instead of adding fallback wrappers.

## Sanity Gates

The Python harness must fail a site/lane when any of the following occurs:

- PySnobal exits non-zero or returns no output rows.
- Required PySnobal columns are missing.
- Output SWE or snow depth contains NaN, infinity, or negative values beyond
  a clearly documented numerical tolerance.
- Derived bulk density from `specific_mass_snow_kgm-2 / thickness_snow_m` is
  non-finite for positive-depth rows or exceeds a package-defined physical
  sanity ceiling.
- A lane with positive snow precipitation never produces positive SWE or snow
  depth.
- The Rust `lineage.json` lacks source class or rejected-alias evidence for a
  required forcing field.

These are sanity gates only. Passing them does not prove snow-depth fidelity.

## HOLD Boundaries

Close as `HOLD` only at one of these named boundaries:

- `HOLD-PYSNOBAL-UNAVAILABLE`: PySnobal cannot be imported or executed from any
  explicit local Python environment.
- `HOLD-RUST-FORCING-SURFACE-ABSENT`: openWEPP cannot produce continuous
  hourly forcing without production-behavior changes outside this package.
- `HOLD-PYSNOBAL-SCHEMA-MISMATCH`: PySnobal requires additional inputs or
  semantics not represented by its checked local schema.
- `HOLD-FORCING-PROXY-DOMINATES`: the constant `T_g`, longwave, or net
  shortwave proxy lanes dominate results so strongly that PySnobal snow-depth
  comparisons would be misleading without a stronger forcing model.
- `HOLD-UNIT-ALIAS-RISK`: any anti-alias gate cannot prove snowfall
  depth-vs-SWE, WAT SWE-vs-depth, radiation, or ground-temperature semantics.

## Review Requirements

Dual reviews must check:

- Gate Evidence Non-Deferral compliance.
- Rust exporter reuses openWEPP runtime input machinery rather than rebuilding
  daily-to-hourly algorithms in Python.
- PySnobal `temp_ground_degC` is not sourced from `frost.hourly.surface_temp_c`.
- Snowfall depth-to-mass conversion is correct and rejects depth-as-SWE alias.
- Proxy forcing fields are labeled and cannot become production authority.
- No production physics, constants, runtime activation, or observation
  tolerances changed.
- Line-count governance for all touched `.rs` files.

Every finding must be dispositioned as `accepted`, `rejected`, `deferred`, or
`follow-up` before closure.
