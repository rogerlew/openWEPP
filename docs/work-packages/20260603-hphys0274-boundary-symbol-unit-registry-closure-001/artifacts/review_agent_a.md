# Review Agent A

Status: completed
Evidence mode: ran-review

Static: Review scope included registry code, tests, docs, gate wrapper, package
artifacts, and package compliance.

Ran: independent `rust_code_reviewer` sub-agent review completed during
HPHYS0274 execution.

## Findings And Disposition

### A1: `stmdur` unit mismatch and missing `timem_####`

- Severity: high.
- Evidence: `crates/openwepp-climate-runtime-adapter/src/lib.rs` converts
  `stmdur` and `timem_####` to seconds before runtime publication.
- Disposition: accepted.
- Fix: changed `stmdur` registry unit to `s`, added `timem_####`,
  hillslope-prefixed timing/intensity aliases, and tests in
  `tests/integration/sim_contract_boundary_unit_registry.rs`.
- Closure impact: resolved.

### A2: Cross-unit publication alias ownership

- Severity: high.
- Evidence: runtime rows such as `prcp` and `snow.runtime_swe` used publication
  aliases with different units from their runtime unit.
- Disposition: accepted.
- Fix: removed cross-unit publication ownership from runtime rows and added
  duplicate/ambiguous publication alias validation and tests.
- Closure impact: resolved.

### A3: Missing WB13 profile runtime aliases

- Severity: high.
- Evidence: WB13 publication/runtime guards require
  `wb13_profile_porosity_cap_mm`, `wb13_profile_fc_store_mm`,
  `wb13_profile_fc_tail_mm`, and `wb13_profile_wp_store_mm`.
- Disposition: accepted.
- Fix: added WB13 profile runtime aliases to the registry and required-alias
  manifest.
- Closure impact: resolved.

### A4: Template lookup divergence

- Severity: high.
- Evidence: unit registry template lookup returned the first matching template
  rather than rejecting ambiguous concrete aliases.
- Disposition: accepted.
- Fix: changed lookup to collect matching canonicals and return
  `AmbiguousBoundaryAlias` for overlapping templates; added bad-template and
  ambiguous-template tests.
- Closure impact: resolved.

### A5: Gate too narrow

- Severity: medium.
- Evidence: the first gate only ran a small focused test and missed concrete
  high-risk aliases.
- Disposition: accepted.
- Fix: added `hphys0274_required_boundary_aliases()`, expanded tests to validate
  the full touched-scope manifest and WAT schema metadata, and added focused
  clippy to `tools/release/check_unit_registry.sh`.
- Closure impact: resolved.

### A6: Package artifacts not closed

- Severity: high.
- Evidence: review, disposition, verification, and gate artifacts were queued
  while implementation was still in progress.
- Disposition: accepted.
- Fix: final artifacts were completed after implementation, gate execution, and
  review disposition.
- Closure impact: resolved.

## Conclusion

No undispositioned Review Agent A findings remain.
