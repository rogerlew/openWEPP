# Unit Registry Audit

Status: completed/HOLD
Evidence mode: ran

Static: HPHYS0279 reads the executable boundary-symbol unit registry from
`crates/openwepp-sim-contract/src/units.rs` and uses it for contract-scoped
cross-checks.

Registry audit behavior:

- canonical symbols and boundary aliases are parsed from
  `BoundaryUnitEntry::new(...)`,
- publication aliases are parsed from the registry entry publication alias
  list,
- missing or unparseable registry source is a hard lint failure
  (`SCUNIT-E-010`),
- unit checks are scoped to the registry entry's owning `contract_id`,
- alias rows are checked only when the symbol/alias belongs to the current
  contract,
- registered entries owned by a contract must appear by canonical symbol in
  that contract's `Variables and Units` section,
- registered boundary/API and publication aliases owned by a contract must
  appear in that contract's `Symbol Alias Map`.

Ran:

- `cargo test --test hphys0279_sc_unit_compliance_lint_contract -- --nocapture`:
  pass, including registry-mismatch, missing-registry, unparseable-registry,
  missing-alias, and alias-only variables fixtures.
- `tools/release/check_sc_unit_compliance.sh --format json`: fail/HOLD with
  current registry/contract gaps.
