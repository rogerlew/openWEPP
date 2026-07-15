# ASSURE-02 Gate Results

Evidence class: Ran unless marked Static

Execution date: 2026-07-15 UTC

## Documentation And Repository Gates

| Gate | Result | Evidence |
| --- | --- | --- |
| `markdown-doc lint` | PASS | 17 canonical/catalog paths plus a final self-including 20-file package-tree pass; zero reported errors or warnings |
| `markdown-doc validate` | PASS | Same scope; every invocation reported zero errors |
| Local Markdown targets | PASS | Read-only resolver checked 137 relative link occurrences; zero missing paths |
| `git diff --check` | PASS | Final scoped tree |
| `uk2us` preview | PASS | 35 scoped files unchanged; only the newly changed catalog lines were checked because the 3,000+ line historical catalog has unrelated pre-existing conversions |
| Prompt lifecycle | PASS | Kickoff prompt moved with `markdown-doc mv --no-backup` from `prompts/active/` to `prompts/archived/` |
| Write-set boundary | PASS | No changed/untracked `.rs`, `assurance/`, `usersum/assurance/`, release script/workflow, or WEPPcloud vendor file |
| Rust/CRAP gates | N/A | Documentation-only package; see `line-count-governance.md` |

The `wctl` documentation wrapper could not start in this environment because
its Python environment lacks `typer`. Parent and independent verifiers used the
direct canonical `markdown-doc` commands; this is recorded rather than calling
the wrapper a pass.

## Groundwater Prototype Evidence Gates

| Gate | Result | Evidence |
| --- | --- | --- |
| Retained identities | PASS | All seven SHA-256 values in the claim matrix matched the current retained files |
| Frozen-to-intake path currency | PASS | Exact twelve-path `git diff --quiet` returned exit `0` with empty output |
| Focused current-tree confirmation | PASS | Nextest run `565647ce-f955-47f9-b79a-65f943d3b8ce`: 7/7 passed, 1930 skipped, profile `quick` |
| Analytical arithmetic | PASS | Maximum two-day binary64 residual `1.7763568394002505e-15 m3` is below `1.0e-12 m3` |
| H2637 recurrence reconstruction | PASS | Residual `-4.249045559845399e-11 m3` is within `1.2601452784040276e-7 m3` |
| H2637 post-export reconstruction | PASS | Residual `-4.250466645316919e-11 m3` is within `1.2097394672678664e-7 m3` |
| Claim envelope | PASS | Prototype limits conclusion to formulation/code/domain/consumer/ledger verification; no current empirical or application-fitness claim |

## Review And Verification Gates

| Gate | Result | Evidence |
| --- | --- | --- |
| Independent Review A | Initial HOLD; disposition complete | Five findings accepted and remediated in `review-disposition.md` |
| Independent Review B | Initial HOLD; disposition complete | Six findings accepted and remediated; live release conflict named rather than hidden |
| Independent Verification A | PASS after one metadata correction cycle | All five original findings and `VA-001` closed; evidence/tests independently recomputed |
| Independent Verification B | PASS, including terminal recheck | All six findings closed at the documentation boundary; release remains prohibited |
| External scientific peer review | NOT CLAIMED | Future production report requirement, not supplied by coding-agent review |
| User/scientific-steward acceptance | HOLD | Required terminal direction decision cannot be self-issued by an agent |

## Release Safety Disposition

`ASSURE03-REL-001` remains open. Current aggregate release automation still
snapshots/uploads the prohibited v1 candidate. ASSURE-02 truthfully documents
and prohibits that route but does not claim executable closure or modify release
code. Therefore the package is `EXECUTED-HOLD-USER-ACCEPTANCE`, ASSURE-03 is
blocked pending acceptance, and openWEPP release-candidate assembly is
separately prohibited.
