# Stage-A Heavy Gates

Run ID: 20260812T154346Z
Command log root: /home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T154346Z
TMPDIR (absolute): /tmp/c3_v2_heavy_gates.pf3Vx2WJ

## Hardware

- `uname -a`: `Linux forest 6.8.0-136-generic #136-Ubuntu SMP PREEMPT_DYNAMIC Wed Jul  1 21:53:05 UTC 2026 x86_64 x86_64 x86_64 GNU/Linux`
- CPU summary (`lscpu`): `x86_64`, `Address sizes: 46 bits physical, 48 bits virtual`

## Worktree status

- Head: `02631ae92af6b073ed7957592fef4bad68dcf77f`
- `git status --short --branch` was captured in run logs

## Commands

- `cargo clippy --workspace --all-targets -- -D warnings` -> `FAIL`
  - start: `2026-08-12T15:43:47Z`
  - end: `2026-08-12T15:44:03Z`
  - exit: `101`
  - log: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T154346Z/cargo_clippy.attempt1.log`
- `cargo nextest run --workspace --profile full` -> `FAIL`
  - start: `2026-08-12T15:44:03Z`
  - end: `2026-08-12T16:00:44Z`
  - exit: `100`
  - log: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T154346Z/cargo_nextest.attempt1.log`
- `cargo test --doc --workspace` -> `PASS`
  - start: `2026-08-12T16:00:44Z`
  - end: `2026-08-12T16:00:52Z`
  - exit: `0`
  - log: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T154346Z/cargo_test_doc.attempt1.log`
- `cargo deny check` -> `PASS`
  - start: `2026-08-12T16:00:52Z`
  - end: `2026-08-12T16:00:55Z`
  - exit: `0`
  - log: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T154346Z/cargo_deny.attempt1.log`
- `cargo fmt --all -- --check` -> `PASS`
  - start: `2026-08-12T16:00:55Z`
  - end: `2026-08-12T16:00:57Z`
  - exit: `0`
  - log: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T154346Z/cargo_fmt.attempt1.log`
- `git diff --check` -> `PASS`
  - start: `2026-08-12T16:00:57Z`
  - end: `2026-08-12T16:00:57Z`
  - exit: `0`
  - log: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T154346Z/git_diff_check.attempt1.log`

## Run-ID: 20260812T160818Z

- TMPDIR: /tmp/c3_v2_heavy_gates.m0UaCqmUzZ
- Hardware file: /home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T160818Z/hardware.txt
- Head: `02631ae92af6b073ed7957592fef4bad68dcf77f`
- Worktree status file: /home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T160818Z/git_status.txt
- Command log: /home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T160818Z/command-log.json
- Command summary: /home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T160818Z/command-summary.tsv

### Commands

- `cargo_clippy --workspace --all-targets -- -D warnings` (attempt 1) -> `PASS`
  - start: `2026-08-12T16:08:18Z`
  - end: `2026-08-12T16:08:20Z`
  - exit_code: `0`
  - log: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T160818Z/cargo_clippy.attempt1.log`
- `cargo nextest run --workspace --profile full` (attempt 1) -> `FAIL`
  - start: `2026-08-12T16:08:20Z`
  - end: `2026-08-12T16:38:32Z`
  - exit_code: `100`
  - log: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T160818Z/cargo_nextest.attempt1.log`
- `cargo test --doc --workspace` (attempt 1) -> `PASS`
  - start: `2026-08-12T16:38:32Z`
  - end: `2026-08-12T16:38:40Z`
  - exit_code: `0`
  - log: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T160818Z/cargo_test_doc.attempt1.log`
- `cargo deny check` (attempt 1) -> `PASS`
  - start: `2026-08-12T16:38:40Z`
  - end: `2026-08-12T16:38:40Z`
  - exit_code: `0`
  - log: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T160818Z/cargo_deny.attempt1.log`
- `cargo fmt --all -- --check` (attempt 1) -> `PASS`
  - start: `2026-08-12T16:38:40Z`
  - end: `2026-08-12T16:38:43Z`
  - exit_code: `0`
  - log: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T160818Z/cargo_fmt.attempt1.log`
- `git diff --check` (attempt 1) -> `PASS`
  - start: `2026-08-12T16:38:43Z`
  - end: `2026-08-12T16:38:43Z`
  - exit_code: `0`
  - log: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T160818Z/git_diff_check.attempt1.log`

## Run-ID: 20260812T165618Z

- TMPDIR: `/tmp/c3_v2_heavy_gates.inbiXQEUFK`
- Head: `02631ae92af6b073ed7957592fef4bad68dcf77f`
- Worktree status file: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T165618Z/command-log.json`
- Command log root: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T165618Z`
- Command summary: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T165618Z/command-summary.tsv`

### Commands

- `cargo nextest run --workspace --profile full` (attempt 1) -> `FAIL`
  - start: `2026-08-12T16:56:18Z`
  - end: `2026-08-12T17:33:13Z`
  - exit_code: `100`
  - log: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T165618Z/cargo_nextest.attempt1.log`
  - command-log: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T165618Z/command-log.json`
  - command-result: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T165618Z/command-result.txt`
  - done-marker: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T165618Z/done.txt`
  - blocker: `No space left on device` during temp dir creation and linker `collect2` failures in compile/link step (exit 100)

## Run-ID: 20260812T173601Z

- TMPDIR: `/home/workdir/c3_v2_heavy_gates.mvOgYZN5Oe`
- Head: `02631ae92af6b073ed7957592fef4bad68dcf77f`
- exact command: `cargo nextest run --workspace --profile full`
- attempt 1: launcher failed before command execution; empty log preserved
- attempt 2: `PASS`
  - start: `2026-08-12T17:37:43Z`
  - end: `2026-08-12T18:32:15Z`
  - exit_code: `0`
  - result: `2422 tests run: 2422 passed (54 slow), 33 skipped`
  - log: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T173601Z/cargo_nextest.attempt2.log`
  - command log: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T173601Z/command-log.json`
  - command summary: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T173601Z/command-summary.tsv`

## Final Heavy-Gate Verdict

`PASS`. The original Clippy failure, two external-interrupt nextest failures,
the root-filesystem exhaustion failure, and the malformed detached-launch
attempt remain preserved. The stable exact-byte gates are workspace Clippy
PASS, full nextest `2422/2422` PASS, doc tests PASS, `cargo deny` PASS,
formatting PASS, and diff hygiene PASS.

## Run-ID: 20260812T173601Z

- TMPDIR: /home/workdir/c3_v2_heavy_gates.mvOgYZN5Oe
- Head: `02631ae92af6b073ed7957592fef4bad68dcf77f`
- Command log root: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T173601Z`
- Command summary: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T173601Z/command-summary.tsv`
- Command log: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T173601Z/command-log.json`
- Command result: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T173601Z/command-result.txt`
- Done marker: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T173601Z/done.txt`

### Commands

- `cargo nextest run --workspace --profile full` (attempt 2) -> `PASS`
  - start: `2026-08-12T17:37:43Z`
  - end: `2026-08-12T18:32:15Z`
  - exit_code: `0`
  - log: `/home/workdir/openWEPP/docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/artifacts/comparator-gate-logs/20260812T173601Z/cargo_nextest.attempt2.log`
