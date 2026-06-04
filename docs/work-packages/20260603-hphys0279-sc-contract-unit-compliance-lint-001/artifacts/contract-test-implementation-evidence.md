# Contract Test Implementation Evidence

Status: completed
Evidence mode: ran

Static: HPHYS0279 contract-derived tests were added in
`tests/integration/hphys0279_sc_unit_compliance_lint_contract.rs`.

Ran:

- `cargo test --test hphys0279_sc_unit_compliance_lint_contract -- --nocapture`:
  pass, 9 tests.

Fixture coverage:

- compliant contract fixture passes,
- contract missing `Variables and Units` and `Symbol Alias Map` fails,
- alias row with placeholder `Units check` fails,
- registry unit mismatch fails with `SCUNIT-E-004`,
- missing registry source fails closed with `SCUNIT-E-010`,
- unparseable registry source fails closed with `SCUNIT-E-010`,
- missing registered boundary/publication alias rows fail with `SCUNIT-E-011`,
- alias-only `Variables and Units` coverage fails with `SCUNIT-E-012`,
- empty `Variables and Units` table for registered symbols fails with
  `SCUNIT-E-009`.
