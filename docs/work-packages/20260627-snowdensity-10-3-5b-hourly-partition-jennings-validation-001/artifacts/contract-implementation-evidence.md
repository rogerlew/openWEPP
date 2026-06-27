# Contract Implementation Evidence

Status: complete
Evidence mode: Static

## Contract Amendment

Updated `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
from `contract_version: 91` to `contract_version: 92`.

Added authority:

- `snow_phase_partition_model` variable with accepted values `legacy_rst` and
  opt-in `harder_pomeroy_hourly`.
- `harder_pomeroy_hourly_normalized_relative_humidity` variable documenting the
  exact-saturation normalization seam for supersaturated dewpoint-derived RH.
- `INV-SNOWFREEZE-065` authorizing only a package-bound direct-runtime hourly
  partition opt-in, preserving default `legacy_rst`, requiring active-hour
  precipitation reconstruction, and forbidding parser/runfile/user activation,
  defaults, fixture edits, output-schema changes, and unrelated physics edits.
- `OBL-SNOWFREEZE-P-040` binding direct-consumer proof, default-vs-opt-in
  evidence, humidity-normalization evidence, and Jennings validation artifacts.
- `SNOWDENSITY-10.3.5b Opt-In Hourly Partition And Jennings Validation
  Addendum`.

No production code was changed before this contract amendment.
