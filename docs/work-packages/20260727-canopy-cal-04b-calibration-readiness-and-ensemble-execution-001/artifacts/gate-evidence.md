# Gate Evidence

Status: `PRE-HEAVY GATES PASS / NATIVE PRODUCTION GATE FAIL`

Evidence class: `Ran + Static`

Result-bearing gates: `NATIVE PROOF FAILED / HUBBARD NOT RUN`; Harvard remains
`NOT AUTHORIZED / SEALED`.

| Gate | Result | Evidence |
|---|---|---|
| Daymet custody | `PASS` | `sha256sum -c SHA256SUMS`; 10 entries OK |
| scaffold validator | `PASS` | `findings=14 holdout=SEALED result_ledgers=empty` |
| package Markdown | `PASS` | prospective scope; 24 files; 0 errors; 0 warnings |
| diff whitespace | `PASS` | no findings |
| scientific review A | `PASS` | `prospective-review-agent-a.md` |
| scientific review B | `PASS` | `prospective-review-agent-b.md` |
| scaffold verification A | `PASS` | `scaffold-verification-agent-a.md` |
| scaffold verification B | `PASS` | `scaffold-verification-agent-b.md` |

The table above is scaffold-only evidence. The final pre-heavy section below
supersedes its earlier execution hold for Hubbard work only.

## Final Pre-Heavy Gates

Status: `PASS / HUBBARD EXECUTION AUTHORIZED`

Evidence class: `Static + Ran`

- Primary implementation review: `PASS`.
- QA implementation review: `PASS`.
- Science implementation review: `PASS`.
- Rustfmt, 22 Rust tests, Clippy with warnings denied, and dependency policy:
  `PASS`.
- Eleven Python control tests and executor validation: `PASS`.
- Daymet custody, Markdown lint, diff check, output ownership, and clean
  external object root: `PASS`.
- Rust line-count governance: `PASS`; maximum 1,082 lines, no warning or
  exception.
- Harvard: `SEALED`; durable opening token absent.

This section authorizes only the observed Hubbard/pre-freeze DAG. It does not
claim result-bearing completion.

## Observed Execution Gate

Status: `FAIL / TERMINAL HOLD`

Evidence class: `Ran`

- Attempt 004 `prepare`: `PASS`.
- Attempt 004 `build_executor`: `PASS`.
- Attempt 004 `build_production_runner`: `PASS`.
- Attempt 004 native-default real-consumer proof: `PASS`, 16,437 days.
- Attempt 004 frozen interior `GSI-5557` real-consumer proof: `FAIL` at lane 1,
  day 11,186 because LAI was positive while post-growth canopy height was
  missing/non-positive.
- Synthetic, Hubbard population, reconstruction, readiness, freeze, terminal
  result validation, and Harvard holdout: `NOT RUN` after the required stop.

The retained observed receipt has exit code 1 and state `FAIL`.
`execution-incident-004.md` binds its hashes and preserved object root.

## Terminal Closure Gates

Status: `PASS FOR HOLD DISPOSITION`

Evidence class: `Ran`

- Terminal scaffold validator:
  `PASS ... lifecycle=TERMINAL_HOLD`.
- Package Python controls: 15 tests `PASS`, including four terminal-HOLD
  lifecycle regressions.
- Executor validator: `PASS`.
- Markdown: 39 CAL-04B files, 25 hold-lift scaffold files, and the two catalog
  files; 0 errors and 0 warnings.
- CSV parseability: 25 CAL-04B CSV files `PASS`.
- Daymet custody and `git diff --check`: `PASS`.
- Dual terminal scientific review and dual terminal verification: `PASS` after
  accepted findings were corrected and rerun.
