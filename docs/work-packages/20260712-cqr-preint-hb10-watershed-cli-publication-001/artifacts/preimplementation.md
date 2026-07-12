# HB-10 Preimplementation Record

Evidence class: **Static + existing campaign measurement**

## Intake

- Applicable instructions: root `AGENTS.md`, `crates/AGENTS.md`, root plus
  `docs/work-packages/AGENTS.md` for package artifacts.
- Fixed source: `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`.
- Start SHA/lines: `5e388b7781e2850411bd7c70e92695c06316c07fba99f89dcdfdc0fa4fc92f54`,
  `2,350`; WARN below blocker.
- Source authority read: runfile contract; `SC-SYSTEM-001`
  `INV-SYSTEM-001..006/028..036`; manifest/publication callers and executable
  watershed CLI tests.

## Fixed Rows And Floor Audit

The committed High-B ledger binds six rows: `run` 58/77.446%/96.596;
`hillslope_area_m2_from_source_runfile` 11/45.614%/30.465;
`parse_watershed_runfile` 66/69.369%/191.186;
`validate_manifest_publication_metadata` 18/53.846%/49.854;
`validate_manifest_per_ofe_wb13_publication_policies`
11/45.714%/30.357; and `validate_manifest_mofe_hourly_carry_metadata`
17/51.724%/49.515.

The High-B start CRAP artifact additionally reports these distinct production
floors below 75%:

| Function | Coverage | CRAP |
| --- | ---: | ---: |
| `build_topology_from_watershed_structure` | 74.699% | 24.847 |
| `default_hillslope_binary` | 61.538% | 3.512 |
| `project_channel_crfrac_from_watershed_soil` | 59.649% | 12.205 |
| `resolve_structure_contributor_local_id` | 59.091% | 3.616 |
| `validate_manifest_mofe_hourly_carry_inactive_single_ofe` | 61.905% | 3.498 |
| `validate_manifest_mofe_hourly_carry_required_arrays` | 60.606% | 6.528 |
| `validate_manifest_mofe_hourly_carry_totals` | 60.870% | 6.498 |
| `validate_manifest_per_ofe_wb13_publication_counts` | 50.769% | 25.438 |
| `validate_manifest_per_ofe_wb13_publication_keys` | 54.545% | 11.602 |
| `validate_watershed_runfile_applicability` | 58.140% | 8.641 |
| `watershed_groundwater_authority_from_gwcoeff` | 28.000% | 19.437 |

`print_help` is 0%/CRAP 2 and is a candidate literal-glue exclusion, not a
silent omission. All other rows require coverage or a reviewed eligibility
disposition. A fresh implementation-start report must supersede these
coordinates because HB09 changed the shared workspace.

## Mechanical Seams And Protected Order

- `run`: argument parsing, filesystem/runfile intake, pass supervision,
  topology/routing, typed publication conversion and output/timing write.
- `parse_watershed_runfile`: document validation, applicability, required
  paths, hillslope blocks, sidecars and output resolution.
- Manifest validators: common schema/count/area, per-OFE policies/counts/keys,
  hourly carry policy/shape/arrays/totals.
- Area helper: read/TOML/path/slope parse, native area sum, unit conversion and
  finite-positive rejection.

Extraction must retain first-error priority, exact `CLIWAT-E-*` text/path,
iteration order, unit factors and the distinction between absent, malformed,
inactive and valid-zero metadata. No boolean accumulator or generic JSON-field
helper may obscure field-specific error provenance.

## Existing Consumer Map

The executable behavior contract already exercises 30+ scenarios including
jobs/arguments, generated/reuse paths, stale pass inventory, worker failure,
typed frames, baseline EBE, sediment/jobs identity, applicability, no-event
groundwater, legacy discovery, MOFE metadata failures, valid multi-OFE area
and per-OFE policy. The execution phase must inventory branch coverage rather
than add redundant success fixtures. Publication closure must name the chain:
CLI runfile -> pass inventory/manifest intake -> topology/network frame ->
typed publication frame -> watershed writer -> Parquet readback.

## Stop Conditions

Stop and open a DC package if cover-first evidence exposes a valid-input
failure, contract mismatch, changed manifest semantics, wrong area/unit basis,
publication/conservation error, or required compatibility fallback. Do not
hide such a defect inside CQR extraction. No production or test edit occurred
during this kickoff.
