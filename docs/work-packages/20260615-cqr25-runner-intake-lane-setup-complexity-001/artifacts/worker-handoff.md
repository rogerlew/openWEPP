# Worker Handoff

Status: complete.

Static: CQR25 production work is complete.

Ran: final target `execute_hillslope_run` CRAP is `12.4198250729`.

Ran: no target-file CRAP row is above `30`.

Ran: cargo gates passed:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`

Next steps:

- Commit the package write set without staging unrelated `AGENTS.md`.
- Push `main`.
- Only after push, update `docs/work-packages/cqr-burndown-execplan.md` for
  CQR25 with package path, pushed commit SHA, branch, date, and final CRAP.
