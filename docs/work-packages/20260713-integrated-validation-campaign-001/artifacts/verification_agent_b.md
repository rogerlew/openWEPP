# Independent Verification B

Status: `PASS-HOLD-VERIFICATION`

Evidence class: **Ran** for lightweight hash, repository-identity, line-count,
Markdown, and diff checks; **Static** for corrected evidence, successor package,
and terminal-disposition inspection. No heavy validation gate was rerun.

Verdict: `PASS`

The campaign's `HOLD-INTEGRATED-VALIDATION` result remains correct and no false
terminal PASS is present. The pinned successor release inputs are coherent and
INTVAL-B-01 and INTVAL-B-02 are fully disposed.

## Accepted-correction verification

### INTVAL-B-01: verified

The six previously pre-run evidence records now truthfully distinguish partial
pre-fix bindings from blocked release/stability evidence:

- `operand-lineage.md`, `consumer-path-evidence.md`, and
  `conservation-reconstruction.md` bind named command groups while disclaiming
  terminal or reusable post-fix closure;
- `publication-identity.md` and `fail-closed-results.md` claim only partial
  pre-fix PASS; and
- `comparator-delta-review.md` correctly remains `BLOCKED-PRE-FIX` because
  command 16 stopped before stability.

The assessment likewise records missing complete H2637 groundwater and snow
numeric operands/output hashes and forbids reuse after the restart. The
scenario matrix now labels H2637 numeric groundwater reconstruction incomplete
and limits that row's PASS to active-owner publication and the three routing-
authority selections. It binds the explicit two-channel
external-baseflow-once assertion to watershed command 13. This resolves the
remaining attribution defect without expanding the evidence claim.

### INTVAL-B-02: verified

The successor package now binds an exact no-skip invocation with both stability
CSV paths, expected suite counts, source commit, SHA-256 values, and a mismatch
HOLD rule. Independent lightweight checks found:

| Input | Pinned SHA-256 | Local structure |
| --- | --- | --- |
| `defect_seeds.csv` | `42b7d827d842ecbe75843175a80ab4f67a097784156658df8fb849161eb98958` | 1,167 lines: header plus 1,166 expected records |
| `hillslope_watchlist.csv` | `42214345a228d27a0536b771dd73068dc897d369f54cb8a197457dea675e26ab` | 20 lines: header plus 19 expected records |

`/workdir/wepp-forest` resolves to the pinned source commit
`375ccc296ed1ea491f599ff1b1a25b415d494a2a`. The release script accepts the
specified cohort, watchlist, and repeated suite-expectation arguments and
requires both CSVs when stability is not skipped. Thus the successor command is
executable as written and can test the intended no-skip release/stability path.
This verification does not claim that the queued successor command has passed.

## Terminal disposition and restart

The release row remains `FAIL`, the final closure gates remain `BLOCKED`, and
HOLD-only Markdown/diff checks are not represented as Phase 6 substitutes.
Partial domain PASS records are explicitly pre-fix and non-transferable.
Therefore `HOLD-INTEGRATED-VALIDATION` remains the only legitimate terminal
result for this source freeze.

The restart rule remains exact: close `INTVAL-REL-001` with its pinned no-skip
release acceptance, freeze the correction commit, and rerun every integrated-
validation phase from Phase 0 through Phase 6. No pre-fix result may satisfy the
restarted campaign.
