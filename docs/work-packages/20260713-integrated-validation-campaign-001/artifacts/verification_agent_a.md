# Independent Verification A

Status: `PASS-HOLD-VERIFICATION`

Evidence class: `Ran` lightweight integrity checks and `Static` artifact,
source, and command-log inspection. Heavy gates were not rerun.

## Verdict

Verdict: `PASS`

The integrated-validation campaign remains correctly on `HOLD`. The release
lane failed with exit 101, its remaining required gates were blocked, no
artifact presents a false terminal `PASS`, and the successor package correctly
requires a full campaign restart after the release-runner defect is repaired.

Independent verification A certifies that `INTV-A-01`, `INTV-A-02`, and
`INTV-A-03` are fully disposed.

## Review-finding verification

| Finding | Result | Verification |
| --- | --- | --- |
| `INTV-A-01` | `PASS` | `package.md`, `hold-legitimacy-audit.md`, and `disposition.md` now consistently distinguish the directly rerun missing-coefficients guard from the two source-related mutual-exclusion failures. None claims that all three guards passed independent focused reruns. |
| `INTV-A-02` | `PASS` | The six affected evidence records are explicitly classified as partial pre-fix or blocked. They bind their supporting command IDs, identify command 13 for the baseflow-external-once evidence, record the missing complete H2637 groundwater and selected snow reconstructions/output hashes, and prohibit reuse as restart closure evidence. |
| `INTV-A-03` | `PASS` | The successor defect-closure package now records the exact no-skip release invocation, canonical cohort/watchlist paths, expected suite counts, pinned external-repository commit and input hashes, pre-run verification, mismatch-to-HOLD behavior, and the mandatory full campaign restart. |

## Pinned stability-input integrity

The successor package pins `/workdir/wepp-forest` at
`375ccc296ed1ea491f599ff1b1a25b415d494a2a`. Read-only checks confirmed that
repository HEAD and that both input files match the pinned commit.

| Input | SHA-256 | Expected rows | Observed file lines |
| --- | --- | ---: | ---: |
| `defect_seeds.csv` | `42b7d827d842ecbe75843175a80ab4f67a097784156658df8fb849161eb98958` | 1,166 | 1,167 including header |
| `hillslope_watchlist.csv` | `42214345a228d27a0536b771dd73068dc897d369f54cb8a197457dea675e26ab` | 19 | 20 including header |

These values are mutually coherent with the successor command's
`--expect-suite wb05b_1166=1166` and
`--expect-suite release_gate_watchlist=19` arguments.

## HOLD and restart integrity

- Command 16 is recorded as a genuine release failure with exit 101 after the
  three H2637 tests failed under threaded `cargo test --workspace`.
- Required release/authority, build, lint, and stability work after that
  failure is recorded as blocked, not passed or waived.
- Focused and broad pre-fix evidence is explicitly non-terminal and non-reusable
  for restart closure.
- The successor defect-closure package repairs the release-runner isolation
  defect; it does not convert the failed campaign into a pass.
- The campaign must restart from the beginning on the required frozen source
  and exact pinned stability inputs after that repair.

No evidence supports weakening the campaign `HOLD`. The accepted corrections
do not alter the release failure, the successor repair boundary, or the
full-restart requirement.
