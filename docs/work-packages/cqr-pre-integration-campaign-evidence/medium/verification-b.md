# Medium Terminal Review And Independent Verification B

Evidence class: **Ran + Static independent review**

Review source: committed HEAD
`3475d41d06327ba65154f45de67645727d508251`.

Overall disposition: `PASS`.

## Terminal Review

### Scope, Selection, And Checkpoints

The fresh start census contains 32 deduplicated production rows above CRAP 30
across 25 modules. The accepted Medium ledger binds 19 of those rows across the
fixed M-01 through M-13 cohort. Both selection reviews reproduced the live
cohort; their three formatter disagreements were resolved conservatively to
`E-PRODUCTION`. The resulting ledger has 19 actionable rows and no retained
`R-*` or `X-*` exception.

All thirteen module records are `MODULE-PASS`. Each records classification,
obligations, focused tests and consumers, coverage/CRAP disposition, line-count
governance, independent review, and a committed checkpoint. The reconciled
checkpoint ledger is:

| Modules | Checkpoint commits |
| --- | --- |
| M-01 through M-04 | `3a4f7f45`, `3dcb28aa`, `ee54bdca`, `15921b1b` |
| M-05 through M-07 | `2b111ba7`, `6f690359`, `2d570697` |
| M-08 through M-10 | `b58bfb33` + `43289099`, `4ba55de9`, `81efc1cd` |
| M-11 through M-13 | `d8bca733`, `d8069ee7`, `69822725` |

The earlier pending-SHA concern is closed by documentation reconciliation
commit `7d366395`; all named commits exist and precede review HEAD. M-08's
follow-up changes only a test-local Clippy allowance. Its checkpoint artifact
hash therefore need not equal the later source hash, and the final workspace
measurement is the controlling CRAP authority. M-01 and M-02 do not have
separate focused artifact directories, but their compact records preserve the
run results required by the revised contract and the final mechanical census
independently removes every one of their start identities. This is not a gate
deferral.

Every recorded touched Rust file remains below the 2,000-line warning
threshold. Current independent `wc -l` replay also found the largest touched
production files at 1,599 lines (`registries.rs`), 1,565 lines (`lib.rs`), and
1,551 lines (`openwepp-management-schema/src/lib.rs`); no line-count exception
is required.

### Behavior And Module Evidence

Static review of the module records and implementation commits found no
unresolved review finding. Tests-only modules add characterization at their
real parser, schema, runner, release, and migration consumers. The production
decompositions preserve ordered blocks:

- M-01 and M-02 retain parser grammar, invariant priority, error construction,
  and numeric expression order while extracting private helpers.
- M-10 moves the final-storage statements verbatim into `final_storage_m2` and
  binds the complete hydrograph and result fields bitwise.
- M-13 preserves CLI option, authority, validation, publication, and first-error
  order; detached pre-decomposition characterization passed 7/7 and the final
  binary-plus-unit cohort passed 12/12.

M-08 binds dynamic operands with real routing/canopy/melt consumers and
anti-alias cases. M-10's focused science cohort passed 17/17. The M-10 broad
coverage diagnostic failures were in two unchanged process-global audit-count
assertions, neither of which calls the target oracle/helper; ordinary
same-source quick execution passed. No module record defers a focused coverage,
CRAP, consumer, review, or line-count requirement to a later module.

### Final Authority And Gates

The final measurement is source-bound to `69822725`. The final ordinary closure
source is `553647f0`; the only intervening Rust changes are test-only:
`replacement.to_owned()` becomes `replacement.clone_into(...)`, and a narrow
`clippy::too_many_lines` allowance is attached to the cohesive M-07 branch
matrix. No production source changed. HEAD after `553647f0` adds only Medium
documentation and evidence.

The first all-target Clippy attempt failed on exactly those two test lints. The
archived authoritative rerun passed. The remaining terminal gates also have
direct exit-zero evidence: formatting; quick nextest 1,851/1,851; full nextest
1,930/1,930; deny; the exact 65-file campaign Markdown lint; and
`git diff --check`. No required terminal gate is deferred.

The instrumented workspace run had four coverage-environment-only failures:
three unchanged H2637 selector tests affected by shared process environment and
one unchanged R3C process-global audit-count assertion. The same H2637 family
was present at Medium start, the R3C family was reproduced by the M-10
diagnostic, and none is in a Medium production edit. The ordinary full profile
then passed all 1,930 tests, including those four. The apparent `FAILED` strings
from two tamper fixtures occur inside passing tests and are not hidden failures.

### Protocol Deviations And Findings

| ID | Finding | Disposition |
| --- | --- | --- |
| MTR-B-01 | Quick checkpoints were recorded after M-05, M-07, and M-10, rather than literally after M-03, M-06, and M-09. | `accepted`: this is an iteration-cadence deviation, not a missing acceptance gate. The runs are cumulative, and the authoritative final quick and full profiles cover M-01 through M-13 on the closure source. No terminal evidence is deferred. |
| MTR-B-02 | The instrumented final run began with thirteen documentation-only checkpoint reconciliations in the worktree. | `accepted`: commit `7d366395` proves the dirty paths changed only module records, not instrumented Rust/test input. Closure gates were subsequently source-bound to `553647f0`. |
| MTR-B-03 | Default virtual-workspace report commands emitted empty reports, so reporting expanded the 18 metadata workspace members into explicit `--package` arguments. | `accepted`: both corrected reports reused the existing profile without another test run; the package list is durable, contains 18 unique members, and the resulting JSON reports nonzero workspace totals. |
| MTR-B-04 | The instrumented run used `--ignore-run-fail`. | `accepted`: all four underlying failures are individually attributed and contradicted as regressions by the clean 1,930/1,930 ordinary full run. |
| MTR-B-05 | M-08 and final all-target lint corrections use narrow `too_many_lines` allowances. | `accepted`: both annotate exhaustive test matrices only; no production function, coverage target, obligation, or identity is waived. |

Terminal review verdict: `PASS`. There is no blocking or undispositioned
finding. The irregular cadence and measurement corrections are recorded
process deviations with direct terminal superseding evidence, not reasons to
repeat expensive workspace runs.

## Independent Verification

I independently replayed the durable evidence rather than relying on the
summary prose alone.

### Artifact And Metric Replay

- SHA-256 and byte-size replay exactly matched all five values in
  `final-metrics.md`: LCOV `3c6037a...` / 4,547,311 bytes, LLVM JSON
  `469e0550...` / 19,975,824, CRAP JSON `08d34162...` / 2,957,096, filtered
  census `95019608...` / 2,656, and identity diff `f94a7126...` / 2,902.
- Reapplying the binding jq production filter to `final-crap.json` produced a
  byte-identical `final-production-over30.json` with the same SHA-256.
- The LLVM JSON independently reports 109,878 instrumented lines, 96,512
  covered, and 87.835599% line coverage.
- All authoritative `.time` files report exit zero. The retained initial
  Clippy attempt alone reports exit 101 and is correctly labeled superseded.

### Identity Closure And Residual Cohort

Independent set comparison by `(file, function)` reproduced:

| Census | Rows | Modules |
| --- | ---: | ---: |
| Medium start | 32 | 25 |
| Medium final | 13 | 12 |
| Removed | 19 | 13 target modules |
| Added | 0 | 0 |
| Retained | 13 | 12 |

The 19 removed identities exactly equal the accepted M-01 through M-13 ledger;
no fixed identity remains and no new production identity appears. The 13
retained identities exactly match the Low L-01 through L-12 fixed ledger,
including both `snowbench_coe_melt.rs` rows. Their changed numeric coverage in
an untouched backlog does not create a new identity or a Medium regression.

### Verification Verdict

The CRAP authority, target identity closure, no-new-identity ratchet, residual
Low cohort, focused module non-deferral, test-only lint response, failure
attribution, line-count governance, and final ordinary gates all independently
verify.

Independent verification verdict: `PASS`. Medium is eligible for terminal
transition once the peer terminal review/verification reaches the same
disposition; this artifact does not pre-claim that separate peer result.
