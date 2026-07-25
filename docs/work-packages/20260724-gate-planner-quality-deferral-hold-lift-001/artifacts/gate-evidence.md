# Gate Evidence

Evidence class: Ran.

## Edit-Loop Gates

- Clean scaffold reproduction under `--cfg coverage`: seven selected tests,
  three passed and four failed.
- First independent focused rerun exposed deterministic node-order drift in the
  mutation fixture.
- Corrected isolated mutation identity under `--cfg coverage`: 1 passed,
  179 skipped, `75.146s`.
- Rustfmt: PASS.
- Ordinary owning-crate Nextest: 175 passed, 14 coverage-only tests skipped.
- Owning-crate all-target Clippy with warnings denied: PASS.
- TESTGATE alignment and CI executor source contracts: 22 passed.
- Package documentation lint: 7 files, 0 errors, 0 warnings.
- Diff whitespace check: PASS.

## Terminal Gates

Executed head:
`0342b9f8c6611d2ba7e1a95ea35b213179dcef3d`.

Initial and final `git status --porcelain` were empty.

| Gate | Result | Metrics |
|---|---|---|
| Exact seven under `--cfg coverage` | `PASS` | 7 passed, 6 slow, 173 skipped; `1852.497s`; exit 0 |
| Complete coverage-configured gate-planner lib | `PASS` | 178 passed, 9 slow, 2 skipped; `2635.919s`; exit 0 |
| Full workspace profile | `PASS` | 2,267 passed across 194 binaries, 11 slow, 43 skipped; `519.233s`; exit 0 |

Durable evidence root:
`/home/workdir/openWEPP-quality-history/20260724-hold-lift-0342b9f8`.

Logs and exit-code files:

- `logs/phase1-exact-seven.{log,exit-code}`
- `logs/phase2-gate-planner-lib.{log,exit-code}`
- `logs/phase3-workspace-full.{log,exit-code}`

The prerequisite correctness gates authorized the Order-3 quality transition.

## Snapshot-Isolation Correction

- Exact public-audit consumer under `--cfg coverage`: `PASS`, `107.072s`.
- Exact clean-root repeat: `PASS`, `106.686s`.
- Exact consumer with deliberate untracked shared-root probe: `PASS`,
  `105.465s`; probe removed.
- Committed/dirty observation negative guard: `PASS`.
- Gate-planner lib/tests warnings-denied Clippy: `PASS`.
- Rustfmt and diff validation: `PASS`.
- Independent implementation review: `PASS`, no findings.
- Independent security review: `PASS`, no findings.

## Order-3 Hold Lift

- Attempt 13 exact head:
  `32022d8cc4bd3e56c62233552b5886bf2648c1bd`.
- Full profile: `PASS`, 2,279 / 2,279.
- Science-manual profile: `PASS`, 36 / 36.
- Canonical workspace inventory: exact 2,315-test disjoint union.
- Merged snowbench ledger: `PASS`, 18 / 18 science-manual contributions.
- CRAP debt: `PASS`, 2 raw, 2 adjudicated, 0 actionable.
- Publication: 11 files, 1,421,222 bytes.
- Evidence ID:
  `f641feeda798047dac30ad7ef760bbadc31b71265e32415353be71b53e8b5544`.
- Independent terminal verifiers A and B: both `PASS`, no findings.

The initial verifier runs were infrastructure-blocked by ENOSPC before issuing
a verdict. Both exact reruns passed after redirecting only `TMPDIR` to an empty
`/home` scratch root. The repository and publication remained unchanged.
