# Independent Review B

Evidence class: `Ran` for the command-log replay and lightweight documentation
checks; `Static` for source, package, and release-script inspection.

Verdict: `HOLD-VALID-WITH-FINDINGS`

The integrated campaign's HOLD is legitimate. The focused selections passed,
the required release command failed, the remaining release and final gates did
not run, and no partial PASS may be inferred. Two accepted-fix documentation
findings prevent terminal review closure; neither finding changes the HOLD or
authorizes a production/science change.

## Independent command replay

I recomputed the result counts, exit status, elapsed time, and peak RSS directly
from logs `00` through `18` and their `.time` companions.

| ID | Result | Elapsed | Peak RSS (KiB) |
| --- | ---: | ---: | ---: |
| 00 | exit 0 | 0:00.06 | 12,672 |
| 01 | 2 passed; exit 0 | 0:01.37 | 48,384 |
| 02 | 1 passed, 9 skipped; exit 0 | 7:21.69 | 196,060 |
| 03 | 1 passed, 9 skipped; exit 0 | 0:01.14 | 48,000 |
| 04 | 1 passed; exit 0 | 0:00.80 | 48,384 |
| 05 | 1 passed; exit 0 | 0:00.59 | 48,000 |
| 06 | 1 passed; exit 0 | 0:16.56 | 204,372 |
| 07 | 1 passed; exit 0 | 0:31.60 | 205,904 |
| 08 | 367 passed, 1,580 skipped; exit 0 | 2:28.08 | 209,544 |
| 09 | 320 passed, 1,627 skipped; exit 0 | 9:17.45 | 198,528 |
| 10 | 1 passed, 28 skipped; exit 0 | 0:43.76 | 742,388 |
| 11 | 7 passed; exit 0 | 0:07.82 | 458,224 |
| 12 | 17 passed; exit 0 | 0:03.24 | 736,888 |
| 13 | 30 passed, 99 skipped; exit 0 | 0:00.69 | 45,312 |
| 14 | 213 passed; exit 0 | 1:46.30 | 748,312 |
| 15 | 129 passed; exit 0 | 0:01.14 | 45,312 |
| 16 | 3 failed; exit 101 | 8:50.55 | 382,244 |
| 17 | 29 files, 0 findings; exit 0 | 0:00.02 | 9,984 |
| 18 | exit 0 | 0:00.02 | 15,360 |

The focused nextest selections in IDs `01` through `15` total 1,092 passed.
Across all 19 commands, 18 exited zero and ID `16` exited 101. Before the
failing target, the release log contains 74 test-result summaries totaling 657
passed, 3 failed, and 3 ignored. Its H2637 target reports 5 passed, 3 failed,
and 2 ignored. The package ledger's counts and timings agree with this replay.

## Release failure and HOLD audit

The failure mechanism is established rather than inferred from the exit code
alone. `tests/integration/laned_shadow_h2637.rs` declares that its
process-environment mutations require nextest process-per-test isolation. Its
negative cases remove and set shared H2637 selector variables. The release
script still invokes `cargo test --workspace`, whose threaded execution lets
those mutations collide. ID `16` consequently returned successful reports in
the three cases that expected missing-coefficient or mutually-exclusive-selector
errors. The same missing-coefficient case passed under nextest in ID `03`.

Because the release script is fail-fast, `cargo deny`, required authority,
release-surface, and stability work after that point did not run. Phase 6 final
gates are therefore correctly `BLOCKED`, not failed or passed. A serial retry,
skip, environment override, or partial PASS would weaken the default release
contract. Routing the harness correction through `INTVAL-REL-001` is the proper
bounded disposition.

## Consumer and reconstruction audit

Static inspection supports the substantive direct-consumer claims: H2637 reads
the emitted HBP and Parquet products and checks active/default byte identity;
P61 and P102 independently reconstruct volume and sediment operands; W7R reads
the production CLI products and compares decoded output rows; MT3 exercises
shape sensitivity and watershed closure. The watershed-hourly selection in ID
`13`, not the H2637 selection in ID `02`, contains the explicit two-channel
"external once" baseflow assertion. Broad profile success alone is not a
substitute for binding these named consumers and reconstructions to their
command IDs.

## Findings

### INTVAL-B-01 — accepted fix required: evidence artifacts remain pre-run

`operand-lineage.md`, `consumer-path-evidence.md`,
`conservation-reconstruction.md`, `publication-identity.md`,
`fail-closed-results.md`, and `comparator-delta-review.md` still declare
`ACTIVE` or `ACTIVE-STATIC-BOUND` and describe current-run confirmation as
pending. That conflicts with the package progress checkbox and assessment claim
that these evidence products were published from the passing lanes.

Before dual review and verification can close, reconcile each artifact to the
evidence actually obtained, bind the relevant command ID and named test, and
retain `BLOCKED` for any comparator/release/stability claim that depends on work
after ID `16`. Bind the baseflow-once claim to ID `13`; do not attribute it to
the H2637 lane. This is a truthfulness and evidence-binding defect, not a reason
to rerun the passing commands.

### INTVAL-B-02 — accepted fix required: successor release acceptance is not executable as written

`INTVAL-REL-001` correctly bounds its production write set to the release
harness and proposes replacing the stale workspace cargo-test lane with the
full nextest profile plus a source guard. However, its acceptance language asks
the "default release command" to run through stability without binding the
required stability inputs. The integrated campaign's no-argument ID `16`
command cannot do that: after the test-harness correction, the release script
exits 2 unless both cohort-seed and watchlist CSV arguments are supplied (or
stability is skipped, which this campaign forbids).

Amend the successor package to bind the exact no-skip release invocation and
the canonical cohort/watchlist paths, provenance, and hashes. Its acceptance
must prove the required authority and stability lanes rather than merely
reaching the next argument check. The existing protected write set, prohibition
on skip/retry/serial workarounds, and requirement for a full integrated-campaign
restart should remain unchanged.

## Restart rule

After both findings are corrected and `INTVAL-REL-001` closes with its exact
acceptance command passing, freeze the correction commit and rerun every
integrated-campaign phase from Phase 0 through Phase 6. No result from this
pre-correction source freeze can satisfy terminal PASS for the restarted
campaign.
