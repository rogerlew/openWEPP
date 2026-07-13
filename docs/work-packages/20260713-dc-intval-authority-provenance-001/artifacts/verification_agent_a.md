# Independent Verification A

Status: `PASS-HOLD-VERIFICATION`

Evidence class: **Ran** for lightweight fixture/Git-object hashes, current line
count, Markdown, and diff-integrity checks; **Static** for guard, release-log,
review-disposition, HOLD, gate, finalizer, roadmap, and catalog inspection. No
heavy gate was rerun.

## Verdict

Verdict: `PASS`

The provenance correction, attempt-02 terminal evidence,
`HOLD-INTVAL-AUTH-PROV-001`, blocked-gate classifications, and user-directed
`INTVAL-FINAL-001` strategy all verify. The three accepted Review B findings
are substantively disposed, no false release/integrated PASS is present, and
the finalizer remains autonomous and governed while preserving every original
acceptance obligation.

Independent Verification A reaches terminal PASS. All accepted review and
verification findings are fixed; no deferred or follow-up finding remains.

## Verification-finding disposition

`INTVAL-AUTH-PROV-VA-01` is fixed. The final Rust regression is 442 lines and
`implementation-evidence.md` now records that exact count. The 2,000-line
warning and 3,000-line refactor thresholds remain satisfied.

## Correction and guard verification

The final metadata posture is precise:

- `schema_version: 1` is present;
- contradictory top-level source claims are absent;
- exactly one target fixture item names `/workdir/openWEPP`, commit
  `9aa4c3d61549ab30da665a4dc109bab811522fe9`, and the exact canonical source
  path; and
- fixture bytes, lock, item hash, source hash, and the Git object all equal
  `a1c50a82cd1e497875cb034481c4b2ef710c319907480b0f584fde30f48fae5e`.

The final regression is non-tautological. It requires exactly one matching
item and exactly one occurrence of every authority field, verifies exact
repository/commit/path values, derives the lock digest independently, hashes
the current fixture, checks the Git object type, and uses the verified YAML
`source_path` for `git show`. Thus the accepted source-path and duplicate-item
finding is fully disposed.

Unarchived focused red/green and AUTH06 5/5 results are consistently classified
as supporting only. Archived attempt 02 remains terminal consumer evidence for
the unchanged provenance correction.

## Attempt-02 and HOLD verification

Attempt 02 restarts the literal no-skip pinned-input release command from its
beginning after the deterministic attempt-01 formatting correction. Its raw
log/time record exit 1 after 9:44.72 and establishes:

| Lane | Verified result |
| --- | --- |
| Workspace formatting/check/Clippy | PASS |
| Full nextest | 1,946/1,946 passed; three skipped; four slow |
| Dependency policy | Advisories, bans, licenses, and sources PASS |
| Fixture integrity | PASS; corrected provenance consumed |
| Required authority | FAIL on absent AUTH05 Cargo target |
| Remaining authority | BLOCKED |
| Binary build/staging/release lint | BLOCKED |
| Stability suites | BLOCKED |

The corrected wording now accurately says no unchanged retry or
retry-until-green occurred. The mechanical formatting fix and complete restart
are legitimate. Partial pass results inside the failed release command do not
satisfy the separately required final gate loop or any terminal release or
integrated-validation acceptance.

Static inventory confirms seven active required suites bind the five targets
deleted together by `a381702b`; AUTH05 is the first missing target reached in
the raw log. Registry/test restoration and the retired symbol-map runtime are
protected by this metadata-only package. The HOLD is therefore a genuine
out-of-envelope boundary here, not a deferred in-envelope correction.

## Review-disposition verification

| Finding | Verification |
| --- | --- |
| `INTVAL-AUTH-PROV-B-01` | PASS: retry wording is accurate; supporting focused evidence and archived terminal evidence are consistently distinguished. |
| `INTVAL-AUTH-PROV-B-02` | PASS: exact source path, unique target item/fields, and path-driven Git-object verification are load-bearing. |
| `INTVAL-AUTH-PROV-B-03` | PASS: finalizer states the narrow precedence override, includes the literal command, pins commit/hashes, and prohibits intermediate successors. |

No accepted review or verification finding is deferred or follow-up.

## Finalizer acceptance-preservation verification

`INTVAL-FINAL-001` is autonomous and non-piecemeal without becoming
unbounded:

- its routing rule supersedes only the original instruction to open a separate
  package for each newly exposed semantic defect;
- every original scenario, consumer/conservation reconstruction, fixed-source
  restart, gate, review, verification, and terminal outcome remains binding;
- the seven-suite/five-target family is one coherent first correction batch,
  with deleted tests serving only as assertion provenance and no symbol-map
  runtime/wrapper revival permitted;
- every later blocker requires the seven-gate DC record and a prior intended-
  write-set amendment; kernel changes additionally require contract-first,
  baseline-authoritative governance;
- focused tests support local iteration, while exact release/full-workspace
  runs occur at coherent candidate boundaries and every nonzero result is
  retained in the cumulative ledger;
- the literal exact release command, pinned external commit, both input hashes,
  and expected suite counts are embedded in the package;
- release must restart from the beginning after each correction and pass
  authority, binaries, lint, and stability before a candidate freeze;
- the full integrated campaign then restarts from Phase 0 on one frozen source,
  and any new in-repository defect returns to the same loop with another full
  restart and no evidence mixing; and
- HOLD remains limited to a proven external/authority boundary outside the
  broad repository-owned envelope. Effort, another failed gate, deleted tests,
  or further source reading cannot end the campaign.

Skips, threshold/tolerance loosening, fixture-result edits, suite deactivation,
retry-until-green, surrogate physics, silent canonicalization, production
fallbacks, and weaker failure posture remain prohibited.

Roadmap, catalog, package disposition, HOLD audit, and handoff consistently
route the executed provenance HOLD into the queued iterative finalizer without
claiming release PASS. Scoped Markdown lint and diff-integrity checks pass.
