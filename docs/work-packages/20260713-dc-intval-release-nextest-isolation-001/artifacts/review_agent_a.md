# Independent Review A

Status: `HOLD-VALID-WITH-FINDING`

Evidence class: **Ran** for lightweight repository identity, hash, row-count,
line-count, Markdown, and diff-integrity checks; **Static** for source/test
comparison, release-log attribution, protected-boundary review, and successor
authority review. No heavy gate was rerun.

## Verdict

Verdict: `HOLD-VALID-WITH-FINDING`

The one-line release correction is in envelope, preserves every downstream
lane, and replaces only threaded workspace libtest with the canonical full
nextest profile. The exact no-skip release run proves full process-isolated
workspace closure at 1,945/1,945 and then fails closed on an independent,
protected required-authority provenance defect. `HOLD-INTVAL-REL-001`, the
blocked-gate classifications, and transition to `INTVAL-AUTH-PROV-001` are
legitimate and non-deferred.

One evidence-record finding requires disposition before verification closure.
It does not undermine the implemented isolation correction or justify lifting
the HOLD.

## Findings

### `INTVAL-REL-A-01` — focused pre/post run claims lack persisted command evidence

Severity: `MEDIUM` — evidence truthfulness and reproducibility.

`intake.md` and `implementation-evidence.md` classify the source regression's
pre-fix failure, its post-fix 1/1 pass, and the focused three-guard 3/3 pass as
`Ran`. The package artifact directory contains only the pinned-input and exact
release logs; it contains no focused pre-fix/post-fix command log, exit record,
or command-result table.

Static comparison does establish that the regression discriminates the
intended change: at intake commit `1a6a0349`, the release script contains the
exact standalone `cargo test --workspace` command and lacks
`cargo nextest run --workspace --profile full`; the current script has the
opposite state. The exact release log also provides strong post-fix evidence
because profile `full` has `default-filter = "all()"`, the three named H2637
guards are not ignored, and all 1,945 selected tests pass. It does not preserve
the claimed historical focused executions themselves.

Disposition required: either bind the original focused commands, exits, and
logs if they exist, or revise the affected artifacts to distinguish static
pre-fix guard proof and full-release post-fix proof from unpersisted focused
runs. Do not rerun a heavy gate to resolve this documentation finding.

## Isolation correction and protected-boundary audit

- The source diff is exactly one release-script replacement:
  `cargo test --workspace` becomes
  `cargo nextest run --workspace --profile full`.
- The new 14-line Rust contract test requires the canonical nextest command and
  rejects the exact stale threaded command. At 120 total lines, the touched
  `.rs` file is below the 2,000-line warning and 3,000-line refactor gates.
- No production crate, H2637 assertion, selector, fixture, lock, science
  contract, threshold, authority posture, stability behavior, or skip behavior
  changed in the reviewed diff.
- The exact release invocation contains both canonical stability input paths
  and both expected-suite arguments. It contains no skip, limit, retry, or
  waiver flag.
- Full nextest uses `default-filter = "all()"`, `fail-fast = false`, and zero
  retries. Its raw log records 1,945 tests passed, three pre-existing skipped
  tests, four slow tests, and no failures.
- Dependency advisories, bans, licenses, and sources pass before authority
  evaluation begins.

## Authority-failure attribution

The raw release log stops immediately after authority fixture-integrity begins
and identifies suite `cas_l4_subhyd_watyld_fcwp_consistency_001`, fixture
`wb19_fcwp_coca_watyld_cases.json`, missing required provenance keys, and a
missing `schema_version`. Static inspection narrows the missing per-item keys
to `source_repo` and `source_commit`; the other required per-item fields are
present. The suite is registered as `required` / `hard-fail`, so exit 1 is the
correct result.

No remaining required authority suite, release binary build/staging/lint, or
stability lane ran after that fail-fast error. Their `BLOCKED` status is
truthful. The separately required post-release formatting, Clippy, full
nextest, deny, Markdown, and diff closure loop is also correctly blocked by the
package's exit-zero sequencing rule; partial results from inside the failed
release invocation do not satisfy that final loop.

## HOLD legitimacy and successor authority

The provenance file, fixture bytes, lock, suite/registry posture, validator,
and thresholds are protected by `INTVAL-REL-001`. Repairing them in this
package would exceed its declared write set; weakening or skipping the
integrity lane would violate acceptance. The queued
`20260713-dc-intval-authority-provenance-001` package provides a finite,
provenance-only correction boundary and requires the exact release command to
restart from the beginning.

Independent lightweight checks confirmed:

| Authority item | Result |
| --- | --- |
| Source commit `9aa4c3d61549ab30da665a4dc109bab811522fe9` | Exists, is an ancestor of intake commit `1a6a0349`, and contains the current fixture bytes |
| Fixture SHA-256 | `a1c50a82cd1e497875cb034481c4b2ef710c319907480b0f584fde30f48fae5e`, matching both the successor package and `fixtures.sha256` |
| Fixture/lock/provenance against intake | Unchanged |
| Stability repository HEAD | `375ccc296ed1ea491f599ff1b1a25b415d494a2a`, exact pinned match |
| Cohort CSV | Pinned SHA-256 matches; 1,167 lines, comprising header plus 1,166 expected records |
| Watchlist CSV | Pinned SHA-256 matches; 20 lines, comprising header plus 19 expected records |

The successor prohibits fixture-byte, lock, suite-authority, registry,
validator, threshold, and skip changes. Its proposed metadata binds the
existing bytes to a verified Git source rather than inventing new authority.

## Roadmap, catalog, and restart posture

The roadmap now identifies `INTVAL-AUTH-PROV-001` as the active blocker without
claiming release or integrated-validation PASS. The work-package catalog
records `INTVAL-REL-001` as executed-HOLD and lists the bounded queued
successor. The handoff consistently requires successor closure, a fresh exact
release run through all lanes, and then a full integrated-validation restart.
No result from this failed release attempt may be carried into terminal release
or integrated-validation acceptance.

Lightweight scoped Markdown lint and repository diff-integrity checks pass.
