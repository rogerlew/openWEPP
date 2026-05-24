# CLI04 Contract Implementation Evidence

Status: completed (Phase A)
Evidence mode: Static

## Static
- Amended canonical contract/spec authority surfaces for CLI04 Phase A:
  - `docs/contracts/openwepp-runner-contract.md`
  - `docs/contracts/openwepp-hillslope-runfile-contract.md`
  - `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
  - `docs/contracts/README.md`
- Ratified shared output-boundary target and transition posture:
  - target crate path: `crates/openwepp-output/`,
  - transition predecessor path: `crates/openwepp-hillslope-output/`.
- Added explicit `outputs.wat` metadata parity requirements for
  `H.wat.parquet`:
  - field metadata keys: `units`, `description`,
  - dataset metadata keys:
    `dataset_version`, `dataset_version_major`,
    `dataset_version_minor`, `schema_version`.
- Added CLI04 parquet dependency authority for new implementation work:
  - required `arrow-rs` stack: `parquet` + `arrow-array` + `arrow-schema`,
  - `arrow-schema` treated as companion crate within `arrow-rs`,
  - `arrow2` adoption prohibited for new implementation work in this package.
- Added explicit WAT authority provenance exception:
  - default comparator baseline remains
    `/workdir/wepp-forest_260430_baseline` @
    `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`,
  - WAT output semantics for closure parity (including optional
    `InterceptionStorage`) follow post-`wepp_260430` consumer-lineage authority.

## Ran
- Not run (Phase A is contract/spec + artifact authoring only).
