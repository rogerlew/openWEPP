# Constitutive Fixture Root

Status: Active
Last updated: 2026-05-31

This tree stores external-authority constitutive suite fixtures.

## Layout

- `tests/fixtures/constitutive/<suite_id>/`
  - `manifest.*` (optional)
  - one or more fixture payload files used by suite tests

## Naming

- `<suite_id>` must match the suite definition `suite_id` in
  `docs/specifications/external-authority/suites/`.
- Fixture filenames should be deterministic and scenario-oriented, for example:
  - `nominal_case_a.json`
  - `boundary_case_fc_threshold.json`
  - `invalid_missing_symbol.json`

## Requirements

1. Fixture units basis must be documented in suite definition metadata.
2. Invalid-domain fixtures must reflect fail-closed expectations.
3. If fixtures are transformed from upstream references/datasets, record the
   transform/provenance in suite notes or a local fixture manifest.
