# Low And Campaign Terminal Verification A

Evidence class: **Ran + Static**

Verification source: `27bd7b13a87c43438b396f42e42289d75561c6ec`

Verdict: `PASS`

## Independent Reconstruction

The committed final evidence is internally reproducible. This verification did
not rerun workspace coverage or the full Rust gates; it inspected their durable
commands, logs, timings, exit statuses, artifacts, and source ancestry as
required by the terminal verification assignment.

| Artifact | Recomputed bytes | Recomputed SHA-256 | Result |
| --- | ---: | --- | --- |
| `final/final.lcov` | 4,552,212 | `acf5635539695b70d82593d908549b0d2c89b470c8bd13a3aaba434dfb64faad` | PASS |
| `final/final.json` | 20,005,639 | `df7493ddfc4c62e75c011d249f64efaf919c2ff6d8ab5f493faca2d04dc086df` | PASS |
| `final/final-crap.json` | 2,957,059 | `0f66b37412fbaa7b692f831b3aa1f39fe77f69a0523ddddb5ae1d360c9558a3a` | PASS |
| `final/final-production-over30.json` | 380 | `a9c356cb7109e7253d7770b22557216f22c0cf593984147daeeb24f8f81c6f26` | PASS |
| `final/final-actionable-over30.json` | 3 | `37517e5f3dc66819f61f5a7bb8ace1921282415f10551d2defa5c3eb0985b570` | PASS |
| `final/report-packages.txt` | 440 | `773e707aa9a39077a4efb4479d1a52ac253d3ce156e4f8b277f8d4e70844a690` | PASS |

Applying the execution contract's exact production filter directly to the
9,544 entries in `final-crap.json` produces byte-equivalent JSON with two rows
across two modules: `MeteorologyError::fmt` at CRAP 56 and
`SymbolAliasRegistryError::fmt` at CRAP 90. Removing exactly those two
dual-reviewed dispositions produces byte-equivalent `[]` actionable output.
The final LLVM JSON independently reports 97,163/110,035 covered lines, or
88.3019039397%.

## Cohort And Module Accounting

The fixed baseline ledger independently parses as 67 rows across 45 module
paths. The final assessment's original-module table independently parses as 45
modules whose baseline column sums to 67 and final column sums to two. The
tranche census sequence is 67/45, 54/35, 32/25, 13/12, and finally two raw rows
across two modules with zero actionable row. Thus 65 raw identities are absent
and every original module has an explicit terminal disposition.

The High-B table's `Removed = 21` denotes its fixed target cohort, while the
54-to-32 census change is 22: High-B terminal evidence identifies the
additional non-target removal as `GwcoeffParseError::fmt`. This is an explained
cohort-versus-census distinction, not a missing identity or arithmetic gap.

All twelve Low records were inspected. L-01 through L-07, L-09, L-10, and L-12
are `MODULE-PASS`; each records its tier, exact target slice, all eight A-H
families, function floor/aggregate/CRAP evidence, consumers, and an independent
review disposition. L-08 and L-11 are `DISPOSITIONED-NO-ACTION` with exact
symbol-level, denominator-retained `R-OBSERVABILITY` dispositions accepted by
both selection reviewers. No module finding is deferred or left open. L-03's
explicit draft PMET contract gaps remain outside its formatter/error-publication
slice and receive no false closure claim.

Current source binding for the two retained rows also reproduces:

- `crates/openwepp-meteorology/src/error.rs`: 79 lines, SHA-256
  `216b42dd308bd50c55a84091d0e629be275c3aaf06e7ca49a1d63d6a5eaf5c06`;
- `crates/openwepp-sim-contract/src/symbols.rs`: 698 lines, SHA-256
  `13a475dc0c7376072b91a48f9eaded2f36925022533def98fe296dc98e8fc9cd`.

The ten actionable module evidence directories each contain the expected
LCOV, LLVM JSON, and CRAP artifact triplet. Their recorded independent module
reviews and the terminal empty actionable filter agree; no Low target reopens
above CRAP 30.

## Source Delta And Gate Evidence

Coverage was measured at `9145d288809935a79ec78143758a0d8de1c2ffd7`.
The ordinary closure commit `8e0f7367fad57a9ec03e8855727a6bfd64560ca0`
changes only two test files: four exact floating-point assertions in the L-10
private test module become epsilon assertions, and the exhaustive PMET test
receives one narrow `clippy::too_many_lines` allowance. Production source,
test selection, exercised inputs, equations, and publication behavior are
unchanged. The historical L-10 whole-file checkpoint hash therefore differs
at closure, while its 926-line production-prefix binding remains unchanged.

The retained logs and `.time` files show exit zero for final LCOV, JSON, CRAP,
formatting, all-target warnings-denied Clippy, full nextest, deny, exact
Markdown lint, and diff check. The superseded Clippy attempt exits 101 and
contains exactly the five test-only diagnostics corrected above. The passing
ordinary closure evidence records 1,944/1,944 full-profile tests with three
skipped, six package gates totaling 411 tests, the 14-test PMET integration
selection, and the five-test L-10 private matrix.

The instrumented `--ignore-run-fail` log contains the three known H2637
process-environment selector failures and the known R3C process-global audit
counter failure. Their identities match prior tranche evidence, no Low target
owns them, and both measurement-source and corrected-source ordinary full
profiles pass all 1,944 selected tests. Expected fixture tamper output that
contains the word `FAILED` is not misclassified as a Rust test failure.

## Campaign Exit Conditions

| Exit condition | Independent result |
| --- | --- |
| Original High-A and High-B eligible rows absent | PASS; both transitions and both A/B verifications are terminal PASS, and final accounting retains none |
| Original Medium eligible rows absent | PASS; Medium transition and both A/B verifications are terminal PASS, and the final filter retains none |
| Executed modules have tier, target coverage/floor, obligations, and consumers | PASS; Low records were audited directly and prior tranche terminal verifications attest their complete module sets |
| Every Low row closed or currently dual-dispositioned | PASS; eleven eligible identities are absent and the only two raw rows match current L-08/L-11 source hashes |
| No new actionable identity or touched-module regression | PASS; exact final production filter has only the two baseline-retained rows and the actionable filter is empty |
| No unresolved defect, conservation/publication gap, evidence gap, or dirty overlap | PASS; module and terminal records contain no deferred finding, and commit `27bd7b13` was clean before this verification artifact was created |
| Full final gate set | PASS from durable command, log, timing, and exit-status evidence |
| Final assessment committed and roadmap synchronized | PASS; `27bd7b13` contains only terminal evidence/documentation beyond the corrected closure source and records the exact recommendation |

High A, High B, and Medium transitions and their dual verification records were
inspected and all report PASS with their expected residual censuses and full
gate results. Their terminal commits are predecessors of the Low measurement
and assessment history; Low does not rewrite or contradict their evidence.

## Findings And Disposition

| Finding | Disposition |
| --- | --- |
| High-B fixed-row removal is 21 while its total census reduction is 22 | `accepted-fixed`: the corrected final assessment now states 22 census removals as 21 fixed plus the explicitly named non-target baseline `GwcoeffParseError::fmt`; it records `13 + 21 + 1 + 19 + 11 = 65`, matching the 45-module table and final filter exactly |
| L-10's historical post-module whole-file hash predates the lint-only assertion correction | `accepted-explained`: the production-prefix hash is unchanged, the complete two-file delta is test-only, and authoritative ordinary gates pass after correction |
| Instrumented coverage has four nonzero test failures under `--ignore-run-fail` | `accepted-attributed`: all four are prior, source-unchanged shared-environment failures and the ordinary full profile passes the same tests |

No blocking, deferred, or follow-up finding remains. Verification A is `PASS`.
The independently authored Verification B also returns review and verification
`PASS`. The corrected assessment, transition, roadmap, and Low ExecPlan now
record both terminal PASS dispositions without changing any metric, gate,
source, or recommendation evidence. The evidence supports the exact terminal
recommendation `GO-INTEGRATED-VALIDATION`.
