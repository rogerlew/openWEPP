# Independent verification B

Status: `PASS — TERMINAL HOLD VERIFIED`

Evidence mode: `Static + Ran + Expected-red`

Terminal disposition: `HOLD` is legitimate. This is not package completion,
release approval, exact-workspace correctness, or performance qualification.

## Findings

No blocking or non-blocking finding.

## Terminal state consistency

The package, catalog, roadmap, active kickoff, gate results, finding
disposition, final disposition, and hold-legitimacy audit consistently report
an executed terminal `HOLD` with bounded correctness retained and
release/performance qualification failed. None reports the work package as
complete or the current source as release-qualified.

The component candidate's exact release run 1 exited `101` before JSON because
the real aggregate lacked an authentic completed `N=2,S=6` sweep with
`58/14/16/28` accounting. Runs 2/3 remain explicitly
`NOT_RUN_AFTER_FAILED_CONJUNCT`. The entire candidate was reverted. The failed
log, unchanged candidate source identity, and binary identity remain forensic
evidence and are not promoted to passing or retained-production evidence.

## Gate truthfulness

The terminal records preserve all unresolved results at their actual strength:

- the historical exact 1/10/19 matrix passes science/scaling but fails its
  wall/RSS budgets;
- revision 61 fails its conjunctive per-run RSS gate and is reverted;
- revision 31 fails its authentic audit conjunct on run 1, leaves runs 2/3 not
  run, and is reverted;
- year, century, and 5,000-hillslope workloads remain not run/nonqualifying;
- broad orchestrator `clippy -D warnings` remains failed; and
- exact-workspace correctness remains failed or unestablished.

The retained one-OFE timing is labeled diagnostic only. Bounded `140/140` LSE
correctness, affected crate checks, formatting, and diff hygiene are not used
to relabel broader failed or unrun promotion gates.

## Revert and forensic reconciliation

The current 179-path Rust manifest independently reproduces
`2813f6e8faabb9408bac5e59b9271626ff5bcdc7fe49ab6dda810d3a1c3eee0d`.
Reversing only the 23 authorized post-baseline authority-test patches in memory
reproduces frozen aggregate
`78d756be1fa11ed85ee92b7d19e6c04427b01b122efaf7804d1b55d60536bbbe`.
This is a forensic reconciliation, not a false exact-workspace restoration
claim.

All seven revision-31 production/test-support/audit declarations are absent,
all six extracted candidate files are absent, and the focused structural seam
is expected red on exactly those seven names. Static search finds no semantic
component-replay residue. The LSE suite passes `140/140` after reversion.

The terminal Rust census now correctly records 179 existing changed/untracked
Rust paths, 46 files in the 2,000--2,999-line WARN band, and zero files at or
above the 3,000-line hard ceiling.

## Review and sequencing disposition

Both final implementation reviews close all eleven accepted candidate findings
at exact manifest
`edc3f0b94c393e537b0115403548b779754f33308e53a26b1041932e9915be71`.
Those closures are accurately scoped to the rejected candidate and do not
claim retained v31 connectivity after reversion.

The package preserves, without backdating or relabeling, all four MEDIUM
write-set/contract-first sequencing defects:

1. eight paths were edited before their retrospective package listing;
2. `v11_vegetation_consumer.rs` was edited before exact write-set entry;
3. the extracted LSE and runner component-replay paths were edited before
   exact-path listing; and
4. the vapor/emax production correction preceded capture of its intended
   expected red.

The earlier rejected-HOLD review row is explicitly closed/superseded by the
continued revision-61 and v31 execution; its history is not erased.

## Checks run

- Current sorted Rust-manifest reconstruction: `PASS`, 179 entries, exact
  `2813f6e8...ee0d`.
- Frozen-manifest forensic reconstruction: `PASS`, exact
  `78d756be...bbbe`.
- Candidate, baseline, and revision-61 evidence-log SHA-256 verification:
  `PASS`.
- Seven-symbol and broader component-replay residue searches: `PASS`.
- Exact structural seam: `EXPECTED_RED`, all seven declarations absent.
- Full LSE nextest suite after revert: `PASS 140/140`.
- `cargo fmt --all -- --check`: `PASS`.
- `git diff --check`: `PASS`.

No heavy release command was rerun. Existing heavy FAIL and NOT RUN evidence
was audited statically against its raw logs and remains unchanged.

## HOLD legitimacy

The first proposed HOLD was correctly rejected while safe authorized work
remained. Execution then continued through revision 61 and corrected v31, in
addition to the documented bounded cache, custody, validation, topology,
terminal-package, and physical-evidence routes. Each was retained or fully
reverted under prospectively fixed science, materiality, timing, RSS, and
authenticity gates. The remaining performance gap is orders of magnitude and
cannot be closed by weakening validation, solver cadence, audit authenticity,
or science authority.

Further execution now needs a new owner decision: revise the workload/budget
premise and qualification protocol, or authorize a materially different
canonical solver/evaluation architecture with new science-contract and work-
package authority. That is a genuine authority boundary. Terminal `HOLD` is
therefore legitimate after the corrected revision-61 and v31 cycles exhausted
the credible bounded routes.

QA pass statement: terminal evidence, statuses, failed/non-run gates,
reversions, sequencing defects, and the HOLD boundary are internally
consistent. Verdict `PASS`; disposition `HOLD`.
