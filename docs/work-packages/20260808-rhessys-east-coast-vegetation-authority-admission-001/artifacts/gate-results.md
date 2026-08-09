# Gate Results

Status: `PASS`

Evidence mode: `Ran on 2026-08-08`

Working directory was `/home/workdir/openWEPP`. Root `/tmp` had no free space,
so Rust executions used a dedicated `mktemp` directory on `/dev/shm` through
`TMPDIR`; test profile, selection, concurrency, and assertions were unchanged.

## Critical Correctness Gate

`TMPDIR=<dedicated /dev/shm directory> cargo nextest run --workspace --profile full`
returned exit code zero. The generated terminal JUnit receipt records:

- report: `openwepp-nextest-full`;
- tests: `2324`;
- failures: `0`;
- errors: `0`;
- reported test time: `2188.255 s`;
- UUID: `5696fd5f-158c-409e-b15b-ce8a128d96e1`; and
- receipt SHA-256:
  `cb4591e8b45686fdbe808072c233987db370e729cda192de0a43aa4dbbfb4d7d`.

The exact-terminal-byte rerun completed at `2026-08-08 23:19:49 -0700`. The PTY
renderer did not retain the aggregate console summary, so the counts above are
taken from the command-produced `target/nextest/full/junit.xml`, not from
historical evidence.

## Focused And Documentation Gates

All exact commands below returned zero on terminal bytes:

```text
markdown-doc lint --path docs/work-packages/20260808-rhessys-east-coast-vegetation-authority-admission-001 --format plain
markdown-doc lint --path docs/work-packages/20260808-rhessys-east-coast-coupled-vegetation-slice-001 --format plain
markdown-doc lint --path docs/backlog/20260806-rhessys-derived-vegetation-crate.md --format plain
markdown-doc lint --path docs/backlog/TRACKER.md --format plain
markdown-doc lint --path docs/ROADMAP.md --format plain
markdown-doc lint --path docs/work-packages/README.md --format plain
markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md --format plain
markdown-doc lint --path docs/specifications/science-contracts/index.md --format plain
bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md
.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md
cargo fmt --all -- --check
TMPDIR=<dedicated /dev/shm directory> cargo nextest run --test vegetation_boundary_authority_contract
git diff --check
```

The admission-package lint validated 40 files and the successor-package lint
validated 34 files; all single-file lints reported zero errors/warnings. Unit
compliance passed, strict Binding Exposure passed `3/3`, and the focused suite
passed `9/9` with zero skipped. Direct assertions also passed the 71-row
selected ledger, archived-prompt hash, clean external worktrees, and pinned
commit identities. The focused suite, contract checks, package lint, and diff
check were rerun after the terminal lifecycle and version-3 wording corrections.

## Diagnostic And Invalidated Runs

- The first focused compile failed before tests because root `/tmp` could not
  create a Rust temp directory. The RAM-backed retry compiled successfully.
- The first new-test execution then failed one line-wrap-sensitive documentary
  literal. The assertion was corrected without weakening the obligation; all
  subsequent focused executions passed `9/9`.
- An earlier full-profile run was intentionally interrupted after Reviewer B
  found contract/test truthfulness defects. It is invalidated and not claimed.
  The complete passing run above used the corrected final contract/test bytes.

No production Rust, Cargo file, runtime, external-authority suite, comparator,
calibration, or publication gate changed or is claimed.
