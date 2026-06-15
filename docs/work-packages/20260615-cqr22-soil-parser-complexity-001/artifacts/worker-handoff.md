# Worker Handoff

Status: complete pending package commit and push.

Current branch: `main`.

Package path:
`docs/work-packages/20260615-cqr22-soil-parser-complexity-001/`.

Scoped target:
`crates/openwepp-input-contract/src/parsers/soil.rs::parse_policy_row`.

Final target CRAP: `5.0`.

New helper CRAP maximum: `8.004096`.

Required gates passed:

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr22-soil-parser-complexity-001 --format json
git diff --check
```

Next actions:

1. Commit CQR22 package write set, excluding the unrelated `AGENTS.md`
   worktree modification.
2. Push `main`.
3. Only after package push succeeds, check off CQR22 in
   `docs/work-packages/cqr-burndown-execplan.md` with package path, pushed
   commit SHA, branch, date, and final target CRAP.
