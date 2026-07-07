# Route-Coefficient Authority Audit

Status: EXECUTED-HOLD-ROUTE-COEFFICIENT-AUTHORITY. Evidence mode: Static + Ran.

## Contract Authority

Static:

- `openwepp-management-lanuse-authority-contract.md` defines native
  `ow-lanuse-1` forest/cropland route coefficients as explicit management-file
  authority and says they are not inferred from row width, ridge spacing,
  random roughness, canopy cover, or other legacy cropland fields.
- `SC-INFILE-MANAGEMENT-001` binds the optional `routing_coefficients` /
  `routing_coefficients_v1` marker under native `landuse=3/4` only.
- `SC-OFEROUTE-001` requires active or activation-candidate paths to consume
  source-authorized friction operands or fail closed. It explicitly rejects
  all-lane `k_o=500`, `I=0`, `LAI=0`, or `h_c=0` missing-source production
  defaults.

## Current Parser/Runtime Shape

Static:

- Parser function:
  `crates/openwepp-input-contract/src/parsers/management.rs::parse_optional_routing_coefficients`
  accepts only `routing_coefficients` / `routing_coefficients_v1` followed by
  five real values when the native forest/cropland extension is allowed.
- Active runtime builder:
  `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
  fails closed when any scheduled lane lacks complete, schedule-consistent
  `route_*` authority symbols.

## External Cohort Inventory

Ran:

```text
find /wc1/runs/al/algebraic-radium/wepp/runs -maxdepth 1 -name '*.man' | wc -l
44

find /wc1/runs/un/unpalatable-rind/wepp/runs -maxdepth 1 -name '*.man' | wc -l
40

find /wc1/runs/ar/arboreal-dendrite/landuse -maxdepth 1 -name '*.man' | wc -l
36

rg -l "routing_coefficients" /wc1/runs/al/algebraic-radium/wepp/runs /wc1/runs/un/unpalatable-rind/wepp/runs /wc1/runs/ar/arboreal-dendrite/landuse -g '*.man' | wc -l
0

rg -l "^ow-lanuse-1$" /wc1/runs/al/algebraic-radium/wepp/runs /wc1/runs/un/unpalatable-rind/wepp/runs /wc1/runs/ar/arboreal-dendrite/landuse -g '*.man' | wc -l
0
```

Interpretation: selected external roots contain `120` management files, none
with native datver and none with a route-coefficient extension.

## Repo Fixture Inventory

Ran:

```text
find tests/fixtures -name '*.run.toml' | sort
tests/fixtures/dff_ws1_native_forest/hjandrews_conifer_forest/p2.run.toml
tests/fixtures/disturbed_burn/forest_high_severity_clay_loam/p4.run.toml
tests/fixtures/disturbed_burn/forest_high_severity_loam/p313.run.toml
tests/fixtures/laned_shadow_h2637/p2637.run.toml

rg -n "routing_coefficients" tests/fixtures -g '*.man'
<no matches, exit 1>
```

The repo fixtures are useful for parser/runtime tests, but no committed fixture
already supplies the active Lane-D route extension.

## External Root Snapshot

Ran: `artifacts/external-root-snapshot.md` records compact list and content
digests for the inspected external management file sets. This does not make
the mutable `/wc1` roots permanent authority, but it improves provenance for
the count-based zero-match audit.

## Audit Result

The hold is not caused by a missing command invocation. It is a missing input
authority surface: no selected external cohort member currently carries the
native management data the active Lane-D runtime is required to consume.
