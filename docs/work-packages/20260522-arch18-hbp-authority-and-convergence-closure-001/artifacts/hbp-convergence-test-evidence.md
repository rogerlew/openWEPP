# HBP Convergence Test Evidence

Static: convergence test design and authority intent reviewed.
Ran: parser + bridge convergence suites executed.
Status: complete.

## Added/Updated Convergence Coverage

File:

- `tests/integration/infile_hbp_parser_contract.rs`

New tests:

1. `parser_and_bridge_share_hbp_w_001_warning_id`
2. `strict_policy_rejects_legacy_forms_across_parser_and_bridge`
3. `compatibility_policy_accepts_legacy_forms_with_hbp_w_001`

## Command Evidence

1. `cargo test --test infile_hbp_parser_contract`
- Result: pass
- Evidence: `14 passed; 0 failed`
- Includes the three new convergence tests above.

2. `cargo test --manifest-path crates/openwepp-legacy-bridge/Cargo.toml`
- Result: pass
- Evidence: `13 passed; 0 failed`
- Confirms bridge-local strict/compat HBP gate behavior remains green.

3. `cargo test --manifest-path crates/openwepp-input-contract/Cargo.toml`
- Result: pass
- Evidence: crate unit/doc tests pass.

## Behavioral Convergence Assertions

- Shared warning ID lock:
  - parser `HbpWarningCode::HbpW001 -> "HBP-W-001"`
  - bridge `HbpWarningCode::LegacyMagicAliasApplied -> "HBP-W-001"`
- Strict-mode legacy rejection is enforced in both surfaces.
- Compatibility-mode legacy acceptance requires warning emission in both surfaces.

## Notes

Workspace-wide test/format/clippy gates are currently blocked by concurrent
ARCH17 in-progress files under orchestrator runtime-input surfaces; ARCH18
convergence evidence is therefore recorded with both:

- positive in-scope HBP convergence runs (above), and
- full gate blocker details in `gate-results.md`.
