# Kernel Profile Compliance Checklist

Status: completed/HOLD
Evidence mode: Static + Ran

Static:
- [x] Contract-first authority updated in canonical unit governance and
  unit-safe boundary contract docs.
- [x] Contract-derived tests added for helper direction and guard behavior.
- [x] Production code uses named helpers in first-wave high-risk paths.
- [x] Invalid conversion inputs surface typed errors; no silent defaults or
  clamping were added.
- [x] Baseline provenance recorded for formulas/constants.
- [x] Raw literal exceptions use explicit `UNIT-CONVERSION-ALLOW:` rationale.
- [x] Follow-up HOLD recorded for remaining all-production raw conversion
  inventory.

Ran:
- `cargo fmt --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo deny check`: pass with existing duplicate/unmatched-license warnings.
- `cargo test --workspace`: fails only known SIMIMPL18 ET-domain tests.

Profile outcome: completed/HOLD.
