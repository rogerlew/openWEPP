# Independent Terminal Verification A

Status: `PASS`

Evidence class: **Ran + Static**. Frozen source:
`de520f1ff867ca5c65b1f82dfe32a19c213ae18c`.

Verdict: `PASS-INTEGRATED-VALIDATION`. No current-scope blocker or unresolved
Review A/B finding remains. This artifact is Verification A only and does not
replace the separately required Verification B or final clean-worktree commit.

## Fixed-Source And Evidence Identity

- **Ran:** `git rev-parse HEAD` returned the full frozen-source hash above.
  `git status --short` and the changed-file inventory show documentation and
  evidence changes only; no production, test, contract, fixture, registry, or
  release-tool source differs from the frozen commit.
- **Ran:** every `final-*.exit` record in the integrated campaign, including
  `final-00` through `final-16` and all five focused reconstruction exit
  records, is exactly `0`.
- **Static:** the first `restart-*` evidence and earlier nonzero candidates are
  explicitly classified as invalidated or diagnostic provenance in the
  command ledger, scenario matrix, gate results, assessment, and disposition.
  No terminal count or reconstruction operand is imported from those runs.
- **Ran:** the frozen manifest records Rust 1.92.0, nextest 0.9.138, external
  `/workdir/wepp-forest` commit `375ccc296ed1ea491f599ff1b1a25b415d494a2a`,
  cohort SHA-256 `42b7d827d842ecbe75843175a80ab4f67a097784156658df8fb849161eb98958`,
  and watchlist SHA-256
  `42214345a228d27a0536b771dd73068dc897d369f54cb8a197457dea675e26ab`.
  Fresh checks of both input files reproduce those hashes.

## Exact Release Verification

**Static + Ran:** direct inspection of `logs/final-16-release-candidate.log`,
its timing record, the frozen release script, and the retained generated
artifacts confirms the exact unskipped invocation recorded in the scenario
matrix.

| Release obligation | Independently verified result |
| --- | --- |
| workspace closure loop | script invokes format, Clippy with `-D warnings`, full-profile nextest, and deny; log reports 1,960/1,960 passed with three configured skipped tests and no skip argument |
| required authority | every required hard-fail lane passes; the checksum warning is the deliberate injected-drift negative vector inside a passing SOILAUTH03 test |
| release artifacts | binary build, sidecar emission, and release-directory lint pass |
| stability | `wb05b_1166` 1,166/1,166 and `release_gate_watchlist` 19/19 |
| timing | exit `0`, elapsed 50:29.56, maximum RSS 210,360 KB |
| stability JSON | SHA-256 `6e855d94a5d1035c58db2942dbf2668e315d861a1bf1dd6de9a4d4daf5dee6ea` |
| authority report | SHA-256 `b6a3605bd899590e8d85f2a52e938ba518bbb2320832fe68910d4b53369dddea` |

The two artifact hashes were recomputed directly from
`/tmp/openwepp_release_260713ci_7gM53o`; they match
`final-release-artifact-hashes.md` exactly.

## Scenario-Matrix And Consumer Verification

The corrected scenario matrix contains, for every terminal lane, an exact
command or named selection, fixture/output surface, producer-to-real-consumer
handoff, required evidence, observed count, and final log binding. Direct log
inspection reproduces H2637 positive 1/1 and three fail-closed selections 1/1
each; p61 and p102 1/1 each; erosion 368/368; frost 320/320; W7R 1/1; MT3 7/7;
totalwatsed3 17/17; watershed hourly 30/30; runner 214/214; and watershed
129/129.

The final independent reconstruction uses fresh manifest, HBP, pass, WAT, and
Parquet consumer surfaces. It publishes and reconstructs H2637 `S0`, `SN`,
`QbN`, and `QsN`; closes P61/P102 water, sediment, and particle classes;
checks production snow/frost storage; compares all 14 serial/parallel semantic
products; and exercises the W11B CLI plus typed same-grid/baseflow-once
consumers. Rejected latest-event, raw-export, physical-depth, scalar,
diagnostic, and zero-fill aliases prevent producer-tautology closure.

## Review-Finding And Administrative Audit

- Review A is `PASS` with no correction finding and a nonblocking disposition
  for three touched 2,000-line warning-band files; none reaches 3,000 lines.
- Review B's `INTVAL-RB-001` is accepted and corrected: the scenario matrix now
  retains the required commands/selections, fixtures/outputs, real consumers,
  evidence, and logs. `INTVAL-RB-002` is accepted and corrected: the release
  row says `no skip flags` while truthfully recording three configured skips.
  Neither review defers or carries a current-scope follow-up.
- The work-package catalog identifies both packages and the frozen source with
  a completed `PASS-INTEGRATED-VALIDATION` result. The completed
  `INTVAL-20260713` entry is absent from the forward-only roadmap. Package
  status remains `IN_REVIEW` until both terminal verifications and the final
  commit are recorded, which is consistent with the live administrative state.

## Independently Run Gates

| Command | Result |
| --- | --- |
| `cargo fmt --check` | `PASS` (exit 0) |
| `bash tools/release/check_authority_suite_antievasion.sh` | `PASS` (exit 0) |
| `cargo nextest run --test auth11_required_suite_obligation_guards_contract` | `PASS` (3/3, zero skipped) |
| `cargo deny check` | `PASS` (advisories, bans, licenses, sources) |
| `markdown-doc lint --path docs/work-packages/20260713-dc-intval-finalization-001 --path docs/work-packages/20260713-integrated-validation-campaign-001 --path docs/work-packages/README.md --path docs/ROADMAP.md` | `PASS` (42 files, 0 errors, 0 warnings) |
| `git diff --check` | `PASS` (exit 0) |

The exact terminal release log is accepted as the already-run evidence for
workspace Clippy, full-profile nextest, release artifacts, required authority,
and both stability cohorts; this verifier inspected the script and log rather
than rerunning those expensive gates.

## Verification Disposition

`PASS`: the terminal claims are fixed-source, exact-command, independently
reconstructable, real-consumer bound, fail-closed, non-evasive, and
administratively consistent. Verification A found no reason to hold closure.
