# tests/AGENTS.md
> Agent playbook for openWEPP tests.

## Authorship
**This document and all AGENTS.md documents are maintained by GitHub Copilot / Codex / Claude Code, which retain full authorship rights for all AGENTS.md content revisions. Agents may author and revise AGENTS.md documents when and where they see fit.** Revisions must preserve applicable user direction, package scope, review expectations, and higher-precedence governance.

## Mission Snapshot
- Own integration and regression coverage for openWEPP contracts, guards, comparators, and CLIs.
- Keep tests aligned with canonical authority locations after documentation refactors.
- Use tests to enforce contract-derived behavior, not to pin duplicated narrative in indexes or package artifacts.
- Preserve deterministic, local-only execution.

## Primary Assets / Key Files
- `tests/integration/` — contract, package, comparator, CLI, and authority tests.
- `crates/*/src/**/tests.rs` and module-local test files — crate unit tests.
- `docs/specifications/science-contracts/contracts/SC-*.md` — canonical assertion targets for process authority.
- `docs/specifications/science-contracts/index.md` — lifecycle registry assertion target only.
- `docs/work-packages/*/artifacts/` — package evidence assertions when a package owns an artifact contract.

## Standard Workflow
1. Read root `AGENTS.md` and this file before editing tests.
2. For tests touching canonical contracts, read `docs/specifications/science-contracts/AGENTS.md`.
3. For tests tied to a package artifact, read the package-local `package.md` and relevant artifact.
4. Assert behavior, typed guards, invariant IDs, and canonical authority in the owning contract or source file.
5. Avoid asserting long duplicated registry notes; assert lifecycle registry structure in `index.md` and detailed authority in `SC-*` contracts.
6. Keep test reconciliation path/structure-only when docs are deduplicated.

## Validation Checklist
- Focused test for changed area when practical: `cargo nextest run --test <integration_test_name>` or `cargo nextest run -p <package>`.
- Fast local loop: `cargo nextest run --workspace --profile quick`.
- Snow/frost fidelity loop: `cargo nextest run --workspace --profile frost`.
- Erosion sediment/routing loop: `cargo nextest run --workspace --profile erosion`.
- Full handoff gate when package requires it: `cargo nextest run --workspace --profile full`.
- Full Rust closure loop when package or implementation scope requires it: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo nextest run --workspace --profile full`, `cargo deny check`.
- Fall back to `cargo test --workspace` only when validating libtest-specific behavior or when a package explicitly requires the legacy harness.
- Expensive fixture families are scheduled in `.config/nextest.toml` groups: `snowbench`, `cli-fixture`, `frost-fixture`, and `runner-fixture`.
- Use `docs/standards/local-ci-gate-selection.md` for gate-tier selection and
  `tools/local_ci/nextest_timing.py` to record persistent local timing evidence
  for expensive nextest runs.
- For external-authority suite posture/cohort/required-case binding edits: `bash tools/release/check_authority_suite_antievasion.sh` and `cargo nextest run --test auth11_required_suite_obligation_guards_contract`.

## Doc-Coupled Test Rules
- Registry tests should verify row existence, paths, lifecycle fields, and compact governance pointers.
- Contract tests should verify invariant IDs, obligations, guard posture, provenance anchors, and authority text.
- Work-package tests should verify package autonomy, required artifacts, truthfulness labels, and closure status only when the package owns that artifact contract.
- If a documentation dedup moves authority from registry to contract, update tests to the canonical contract target without weakening the asserted obligation.

## Common Pitfalls
- Do not pin a fact to `docs/specifications/science-contracts/index.md` merely because it used to be duplicated there.
- Do not remove a failing assertion by broadening it to a vague substring; move it to the canonical authority location.
- Do not use tests to bless heuristic/proxy process physics.
- Do not classify comparator residuals as openWEPP defects without unit and lineage-stage proof plus independent correctness authority.

## References
- Rust crates: `crates/AGENTS.md`.
- Science contracts: `docs/specifications/science-contracts/AGENTS.md`.
- Work packages: `docs/work-packages/AGENTS.md`.
