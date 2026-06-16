# CQR33 Kickoff

Execute CQR33 from `docs/work-packages/cqr-burndown-execplan.md`.

Target:

- `crates/openwepp-input-contract/src/parsers/watershed_structure.rs`
- Snapshot row: rank 27, original CRAP `240`, CC `15`, coverage `0%`

Protected boundaries:

- no public parser API changes;
- no parser grammar, token order, compatibility mode, typed error, error ID,
  error variant, field name, output shape, or runtime-facing semantic changes;
- behavior-preserving decomposition only.
