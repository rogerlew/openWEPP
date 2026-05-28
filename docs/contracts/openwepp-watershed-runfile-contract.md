# openWEPP Watershed `.run` Contract

Status: `draft-normative`

This contract defines the schema-versioned watershed `.run` file for
`openwepp-cli-watershed`.

## Scope

- Declarative watershed run configuration in TOML.
- Required watershed legacy core file bindings (`pw0.*`) and hillslope
  pass-shard bindings (`hillslopes_block`).
- Sidecar discovery parity mode through `--legacy-sidecar-discovery`.
- Output-path configuration for watershed interchange parquet emissions.

## Schema ID and Versioning

Required schema ID:

```text
openwepp-watershed-runfile-v1
```

Versioning rules:

- breaking changes require a new schema ID (`v2`, `v3`, ...),
- additive backward-compatible keys remain in `v1`.

## File Format

- extension: `.run`
- encoding: UTF-8
- syntax: TOML 1.0
- one watershed run definition per file

Legacy line-oriented WEPP stdin recipes are out of contract for this surface.

## Execution Modes

`openwepp-cli-watershed` supports two sidecar modes:

1. Default mode (no legacy discovery flag):
   - sidecar controls come from `.run` optional keys.
2. Legacy compatibility mode (`--legacy-sidecar-discovery`):
   - runtime discovers legacy sidecars in the run directory.
   - discovered sidecars are authoritative.
   - `.run` optional sidecar keys (`inputs.chaninp`, `inputs.tcr`) are ignored.

## Input Contract

Top-level required keys:

- `schema` (exactly `openwepp-watershed-runfile-v1`)
- `run_name` (human-readable run label)
- `unit_system` (exactly `metric`)

`[inputs]` required keys:

- `pw0_str` (`.str` path)
- `pw0_chn` (`.chn` path)
- `pw0_imp` (`.imp` path)
- `pw0_man` (`.man` path)
- `pw0_slp` (`.slp` path)
- `pw0_cli` (`.cli` path)
- `pw0_sol` (`.sol` path)
- `hillslopes_block` (array of hillslope-pass bindings; must be non-empty)
- `applicability` (Chapter-13 applicability selector table; fail-closed)

`[inputs.applicability]` required keys:

- `chapter13_small_watershed_intent` (`true` required)
- `allow_partial_area_response` (`false` required)
- `allow_headcutting` (`false` required)
- `allow_bank_sloughing` (`false` required)
- `allow_perennial_streams` (`false` required)

Applicability selector guard semantics:

- missing required applicability selectors are typed hard-fail errors
  (`CLIWAT-E-040`),
- any disallowed selector value (`allow_* = true` or
  `chapter13_small_watershed_intent = false`) is a typed hard-fail error
  (`CLIWAT-E-040`),
- no implicit defaults/coercion are allowed for this table.

`inputs.hillslopes_block[]` required keys:

- `hillslope_id` (`u32`)
- `pass_file` (`.hbp` path)

`inputs.hillslopes_block[]` optional keys:

- `unit_system` (`"M"` or `"metric"` only when present)
- `use_existing_pass_file` (`true` only when present)
- `manifest_file` (hillslope run manifest JSON path; required when contributor
  pass shard reports `nofe > 1`)

`inputs.hillslopes_block[].manifest_file` metadata contract:

- source schema: `openwepp-hillslope-run-manifest-v1`
- required fields under `wb13_publication`:
  - `publication_ofe_policy`
  - `contributor_ofe_count`
  - `area_policy`
  - `publication_area_m2`
- required consistency checks at watershed intake:
  - `contributor_ofe_count == hbp.nofe`
  - `publication_ofe_policy == "single-row-canonicalized-hillslope-aggregate"`
  - `area_policy == "sum-ofe-geometry-area"`
  - `publication_area_m2` finite and `> 0`
- failures are typed hard-fail intake errors; no implicit defaults/coercion.

`[inputs]` optional sidecar keys:

- `chaninp` (`chan.inp` path)
- `tcr` (`tcr.txt` path)

Legacy template lineage for required `pw0.*` inputs and hillslope pass block:

- `/home/workdir/wepppy/wepp_runner/templates/watershed.templatec`

## Legacy Discovery Compatibility Mode

When launched with `--legacy-sidecar-discovery`, runtime checks for these
optional sidecars in the run directory:

- `chan.inp`
- `tcr.txt`

If `inputs.chaninp` or `inputs.tcr` are provided in this mode, they are
ignored.

## Unit System Contract (Metric Only)

Watershed `.run` execution is metric-only.

- `unit_system` must be present and set to `metric`.
- alternate unit selectors are out of contract.
- no implicit or automatic unit conversion is performed at this boundary.

## Output Contract

`[outputs]` required keys (`.parquet` paths):

- `ebe_pw0` -> `ebe_pw0.parquet`
- `chan_out` -> `chan.out.parquet`
- `chanwb` -> `chanwb.parquet`
- `chnwb` -> `chnwb.parquet`
- `soil_pw0` -> `soil_pw0.parquet`
- `totalwatsed3` -> `totalwatsed3.parquet`
- `loss_hill` -> `loss_pw0.hill.parquet`
- `loss_chn` -> `loss_pw0.chn.parquet`
- `loss_out` -> `loss_pw0.out.parquet`
- `loss_class_data` -> `loss_pw0.class_data.parquet`
- `loss_all_years_hill` -> `loss_pw0.all_years.hill.parquet`
- `loss_all_years_chn` -> `loss_pw0.all_years.chn.parquet`
- `loss_all_years_out` -> `loss_pw0.all_years.out.parquet`
- `loss_all_years_class_data` -> `loss_pw0.all_years.class_data.parquet`

`pass_pw0.*` outputs are out of contract for this watershed CLI surface.

If any required outputs are missing after run completion, execution must fail.

## Path Semantics

All path values in this contract accept:

- absolute paths, or
- relative paths.

Relative-path rule:

- input paths resolve relative to the directory containing the `.run` file.
- output paths resolve relative to `--output-dir`.

Safety rules:

- no shell interpolation is performed,
- no glob expansion is performed,
- missing required input paths are hard errors,
- missing required output parent directories must hard-fail if runtime cannot
  create them.

## Minimal Example

```toml
schema = "openwepp-watershed-runfile-v1"
run_name = "oak-creek-ws-001"
unit_system = "metric"

[inputs]
pw0_str = "runs/pw0.str"
pw0_chn = "runs/pw0.chn"
pw0_imp = "runs/pw0.imp"
pw0_man = "runs/pw0.man"
pw0_slp = "runs/pw0.slp"
pw0_cli = "runs/pw0.cli"
pw0_sol = "runs/pw0.sol"
chaninp = "runs/chan.inp"
tcr = "runs/tcr.txt"

[inputs.applicability]
chapter13_small_watershed_intent = true
allow_partial_area_response = false
allow_headcutting = false
allow_bank_sloughing = false
allow_perennial_streams = false

[[inputs.hillslopes_block]]
hillslope_id = 7
pass_file = "output/H7.hbp"
manifest_file = "output/openwepp_hillslope_run_manifest.json"
unit_system = "metric"
use_existing_pass_file = true

[outputs]
ebe_pw0 = "output/interchange/ebe_pw0.parquet"
chan_out = "output/interchange/chan.out.parquet"
chanwb = "output/interchange/chanwb.parquet"
chnwb = "output/interchange/chnwb.parquet"
soil_pw0 = "output/interchange/soil_pw0.parquet"
totalwatsed3 = "output/interchange/totalwatsed3.parquet"
loss_hill = "output/interchange/loss_pw0.hill.parquet"
loss_chn = "output/interchange/loss_pw0.chn.parquet"
loss_out = "output/interchange/loss_pw0.out.parquet"
loss_class_data = "output/interchange/loss_pw0.class_data.parquet"
loss_all_years_hill = "output/interchange/loss_pw0.all_years.hill.parquet"
loss_all_years_chn = "output/interchange/loss_pw0.all_years.chn.parquet"
loss_all_years_out = "output/interchange/loss_pw0.all_years.out.parquet"
loss_all_years_class_data = "output/interchange/loss_pw0.all_years.class_data.parquet"
```

## Legacy Compatibility Launch Example

```bash
openwepp-cli-watershed \
  --run-dir /path/to/runs \
  --run-file pw0.run \
  --output-dir /path/to/output \
  --legacy-sidecar-discovery
```
