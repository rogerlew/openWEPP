# Gate Results

Evidence class: `Ran`

Candidate authority commit: `663fe049e18caa0177d2235b2ec7d293d37419ac`.
TESTGATE was not run.

## Focused And Governance Gates

- focused evaluation-shadow authority: PASS, `4/4`;
- all 36 v127-pinned integration binaries: PASS, `151/151` (independent Rust
  reviewer);
- strict Binding Exposure: PASS, nine rows fully consolidated;
- assurance validation: PASS, three DRAFT reports and zero public;
- Markdown lint: PASS, zero findings;
- formatting, warnings-denied Clippy, and doctests: PASS;
- `cargo deny check`: PASS with inherited unmatched `MIT-0` allowance warning;
- diff check: PASS.

## Scoped Diagnostic

`check_sc_unit_compliance.sh --path SC-SNOWFREEZE-001.md` reports one inherited
missing `snow.routed_melt_m` Symbol Alias Map entry. The same alias registration
and omission predate v127; this package adds no unit, alias, conversion, or
runtime surface and does not claim a clean unit-lint result.

## Critical Exact-Head Gates

Pending the read-only heavy runner at clean candidate `663fe049`:

- quick profile;
- frost profile; and
- default/full workspace.

The superseded clean candidate `c77296df` passed static gates but its focused
target failed `3/4` because the test helper searched only backticked table IDs.
No authority defect was hidden: `663fe049` corrected the helper, focused `4/4`
passes, and the heavy suite restarted from the new clean identity.
