# Kernel-Profile Compliance Checklist

Status: hold

Evidence mode: static + ran

Static:
- Checked against
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.

Ran:
- `cargo fmt --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `cargo deny check`: passed with warnings.
- `bash tools/release/check_authority_suite_antievasion.sh`: passed.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`:
  passed `2` tests.

Checklist:
- [x] Canonical `SC-*` authority is amended before production code edits.
- [ ] Contract-derived tests are added before production code edits.
  - Gap: tests were added and run, but not all were landed before the related
    production edit.
- [x] Pre-implementation contract gate is recorded.
- [x] Pinned legacy provenance is cited for equations, constants, guards, and
  invariants.
- [x] Runtime aliases preserve legacy WEPP symbol continuity.
- [x] Typed guards reject missing, non-finite, or out-of-domain required
  surfaces for touched trigger/capacity paths.
- [x] No heuristic/proxy process-physics substitutions are introduced.
- [x] Disposition remains `HOLD` because semantic parity, dual review, and
  dual verification are incomplete.
