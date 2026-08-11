# Finding Disposition

Status: `reopened — implementation corrections complete; terminal re-review pending`

Evidence mode: `Static + Ran`

## 2026-08-11 external review disposition

| Finding | Decision | Current closure |
|---|---|---|
| HIGH: WAT5 failure can clobber ordinary WAT/PASS | accepted | Complete requested output set now stages to same-directory transaction paths; preexisting targets are backed up only at commit, outputs publish with rollback, and the manifest publishes last. Existing-WAT5, day-2 source, forced-close, forced-link, missing-manifest, and success regressions pass. |
| MEDIUM: public rows omit depression storage | accepted | Schema v2 adds per-bin `depression_storage_retention_depth_mm`, renames raw generation to `raw_wb14_post_depression_generation_depth_mm`, and independently reconstructs a positive-storage Parquet fixture. |
| MEDIUM: terminal evidence not bound to committed SHA | accepted | Old working-tree receipts are historical only. Exact-clean replacement-commit full workspace, doctests, and reviews remain pending. |
| MEDIUM: enabled performance unmeasured | accepted | A 45-year p1-geometry/source-complete workload measured elapsed, CPU, RSS, rows, compressed bytes, active support, and throughput; user docs carry the 2.15 GB/301 GB planning warning. |
| LOW: public writer can serialize invalid rows | accepted | Writer-boundary finite/nonnegative, key/clock, duration, raw closure, closing identity, rate, and hourly closure validation now fails before publication. |
| Run-file-only opt-in | accepted user direction | Presence of `outputs.wat_subhourly` remains the sole user-facing selector and is recorded explicitly in the manifest. |

The scientific erosion `NO_ADOPTION` decision and the decision not to open the
Topanga outcome cohort remain accepted without modification.

## Science reviews

| Finding | Decision | Closure |
|---|---|---|
| historical `REVIEW-A-MAJOR-001` premature intake HOLD | accepted | Completed lawful package-local feasibility; all fixed exponents rejected before outcome access. |
| `REVIEW-A-MAJOR-001` / `WAT5-B-001` whole-interval apportionment | accepted | Split source intervals at exact 300-second boundaries before chronological Green-Ampt advancement; delayed-ponding vector and real rows verify timing. |
| `REVIEW-A-MAJOR-002` / `WAT5-B-002` tolerance used as source classifier | accepted | Exact-positive/zero predicates now control source, support, and sparse presence; sub-tolerance positive vectors pass/fail as contracted. |
| `REVIEW-A-MAJOR-003` / `WAT5-B-004` missing composed closure | accepted | Added raw event with depression storage, composed-hour, and day `WAT5-E-004` guards plus negative vectors. |
| `WAT5-B-003` hourly-reset bin index | accepted | Published the canonical day-relative bin `0..287`; boundary and real-consumer keys verify it. |
| historical B exponent/reduction/Topanga scope findings | accepted | Rejected fixed-hour reduction and every exponent, narrowed the screen to one-sided rejection, and retained the Topanga outcome embargo. |

The earlier Review A/B `PASS/GO` applies only to the superseded 2026-08-10
implementation. Reopened Review A retained `PASS/GO`; reopened Review B found
writer chronology, exact-code, and real positive-storage publication gaps.
Those corrections now pass focused gates and await fresh exact-SHA re-review.

The first exact-head terminal campaign at `dff48b554` was interrupted and is
non-admitted after Rust review identified three additional writer/transaction
findings. Explicit manifest-construction rollback composition, strictly
advancing cross-day calendar keys, finite derived arithmetic, and distinct
typed `WAT5-E-003` domain versus `WAT5-E-004` closure failures now pass focused
regressions. A replacement exact-SHA campaign and re-review remain pending.

## Rust correctness review

| Finding | Decision and closure |
|---|---|
| `WAT5-RCR-001` no-clobber publication | accepted; existing targets fail typed and completed files publish with atomic no-replace hard links plus Drop cleanup |
| `WAT5-RCR-002` unbounded day clock | accepted; WAT5 source support is bounded to `[0, 86400] s` with `WAT5-E-003` tests |
| `WAT5-RCR-003` missing raw depression-storage closure | accepted; replay carries the storage delta and validates the raw identity |
| `WAT5-RCR-004` duplicated conservation helpers | accepted; overlap allocation and earliest removal use shared slice helpers |
| `WAT5-RCR-005` resident frame-size regression | accepted; optional event is boxed and the existing layout guard passes |
| `WAT5-RCR-006` raw unit arithmetic | accepted; named typed depth/rate/signed-residual conversions and domain vectors pass |
| `WAT5-RCR-007` output-target aliases | accepted; optional targets are pairwise checked after lexical normalization before writers open |
| `WAT5-RCR-008` unclassified output errors | accepted; stable `OHOUT-WAT5-E-001..003` codes bind to `WAT5-E-005` |
| `WAT5-RCR-009` missing completed-file validation | accepted; physical row count and full schema/metadata are checked before publication |

The earlier Rust correctness `PASS` applies only to the superseded 2026-08-10
implementation. Reopened review found final-name serialization, a concurrent
WAT5 race, chronology, and real-chain evidence gaps. Those corrections now
pass focused gates and await fresh exact-SHA re-review. Symlink aliasing remains
a documented filesystem-policy residual; direct and lexical aliases are
rejected.

## Rust QA review

| Finding | Decision | Closure |
|---|---|---|
| `WAT5-RQA-CRITICAL-001` false boundary replay | accepted | Same chronological split fix plus analytic delayed-ponding regression. |
| `WAT5-RQA-HIGH-001` incomplete behavioral/roundtrip matrix | accepted | Added boundary, hour, multi-interval, rain+saturation, invalid-domain, exact-positive, raw/hour/day, and real typed 26-field Parquet roundtrip vectors. |
| `WAT5-RQA-HIGH-002` missing HBP/routing exclusion gate | accepted | Added named source guard over HBP assembly, OFE routing, watershed orchestration/output, and runner watershed consumers. |
| `WAT5-RQA-HIGH-003` stale post-fix real evidence | accepted | Rebuilt release runner; refreshed p61 closure/on-off identity and p102 typed rejection under `/home/workdir/openwepp-wat5-terminal`. |

The earlier Rust QA `PASS` applies only to the superseded implementation.
Reopened QA found rollback-cleanup reporting, chronology validation,
real-chain evidence, stale temporary-file filtering, and evidence-identity
gaps. The implementation corrections pass focused gates; exact-SHA re-review,
full-workspace, doctest, exact-diff, and terminal-verifier receipts remain
closure gates.

## Terminal authority and usability findings

| Finding | Decision | Closure |
|---|---|---|
| `WAT5-VA-HIGH-001` missing one-command user path | accepted | Added the exact `[outputs] wat_subhourly` runfile entry, CLI invocation, no-clobber note, and retained manifest/binary provenance. |
| Verifier B: base=head A0 receipt observed zero dirty science surfaces | accepted | Added explicit tracked/untracked `--worktree` admission; 17 science paths are now observed and fingerprinted. |
| Verifier B: four shared paths carried stale Plant-only custody | accepted | Added atomic WAT5 bindings while retaining and validating the independent Plant bindings and both A1 gates. |
| Verifier B: approved contract described rename rather than hard-link publication | accepted | Changed the canonical WAT5 algorithm/guard text to atomic no-replace hard-link publication. |
| `WAT5-B-A0-001` parser registry omitted from authority fingerprint | accepted | Added `input-surface-registry.md` to the digest set and a source-level complete-input regression; fresh A0 hash is `6f95845b...`. |
| Verifier B: pre-repair full receipt not terminal-exact | accepted | The 2,379-test pass is superseded for closure; fresh run `b920db77-...` passed 2,380/2,380 and post-repair doctests reported zero failures. |
