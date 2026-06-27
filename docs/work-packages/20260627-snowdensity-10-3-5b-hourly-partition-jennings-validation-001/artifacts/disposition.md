# Disposition

Status: complete
Evidence mode: Static/Ran

Final disposition:
`COMPLETE-10-3-5B-HOURLY-PARTITION-JENNINGS-VALIDATED`.

## Summary

The package implemented an opt-in Harder-Pomeroy hourly precipitation-phase
partition at the direct-runtime hourly winter forcing seam, preserved default
`legacy_rst`, and validated the candidate against the Jennings et al. observed
phase corpus without site calibration.

## Closure Evidence

- Contract amendment: `SC-SNOWFREEZE-001` v92 with `INV-SNOWFREEZE-065` and
  `OBL-SNOWFREEZE-P-040`.
- Direct consumer proof: real direct snow partition consumer receives the
  selected phase model; frost remains pinned to `LegacyRst`.
- Jennings validation: `11,711,058` scored rows across `6,883` stations;
  Harder-Pomeroy hourly accuracy `0.903141` versus legacy `RST` 0 C accuracy
  `0.858331`.
- Default rollback: absent/empty selector remains `legacy_rst`; no
  parser/runfile/user CLI selector; invalid selector values fail closed.
- Scope control: no public WAT/HBP/PASS schema changes, no fixture edits, no
  density/melt/canopy/radiation/frost physics changes, no compatibility-runtime
  deletion, no default activation.

## Gates

All required gates passed:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo test --test auth11_required_suite_obligation_guards_contract`
- `wctl doc-lint --path docs/work-packages`

## Follow-On

Proceed to snow-depth impact adjudication using the opt-in candidate against the
maritime and mixed/deciduous canopy strata identified by the 10.3 sequence.
Candidate promotion/default activation remains out of scope until coupled
snow-depth signatures justify it.
