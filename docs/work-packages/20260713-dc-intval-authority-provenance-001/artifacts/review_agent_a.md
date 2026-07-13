# Independent Review A

Status: `PASS-HOLD-REVIEW`

Evidence class: **Ran** for lightweight hash, Git-object, repository-alias,
line-count, Markdown, and diff-integrity checks; **Static** for provenance/test
diff, release-log, gate-classification, protected-boundary, binding-inventory,
finalizer, roadmap, and catalog review. No heavy gate was rerun.

## Verdict

Verdict: `PASS`

The metadata-only correction is authoritative and independently accepted by
the real release fixture-integrity consumer. The terminal exact release run
then fails on a distinct protected required-authority binding defect, so
`HOLD-INTVAL-AUTH-PROV-001` is legitimate and contains no false PASS.

The revised `INTVAL-FINAL-001` successor is adequate. It replaces piecemeal
single-gate successors with one governed iterative DC loop, begins with the
complete seven-suite/five-target binding family, owns later correctable
in-repository blockers, preserves heavy-gate cadence, and requires exact
release plus a full frozen-source integrated-validation restart. No review
finding remains.

## Provenance correction and regression audit

The production fixture JSON and lock are unchanged from intake. The correction
only:

- adds `schema_version: 1`;
- removes contradictory legacy top-level source claims; and
- adds per-item `/workdir/openWEPP` and exact commit
  `9aa4c3d61549ab30da665a4dc109bab811522fe9` authority.

Independent checks confirm `/workdir/openWEPP` resolves to this repository,
the source object is a Git commit and an ancestor of intake, and its fixture
object hashes to
`a1c50a82cd1e497875cb034481c4b2ef710c319907480b0f584fde30f48fae5e`.
That value equals the current fixture, lock, per-item `sha256`, and
`source_sha256`.

The new regression is not tautological. It hard-codes the expected repository
and source commit rather than accepting values parsed from provenance, requires
the exact schema and absence of legacy top-level claims, reads the independent
lock entry, hashes current fixture bytes, verifies the Git object type, reads
the fixture object at the pinned commit, and requires all byte hashes to agree.
Static application to intake fails on the missing schema and contradictory
top-level claims. The archived release consumer independently advances through
fixture integrity after the correction.

The touched Rust test is 400 lines, below the 2,000-line warning and 3,000-line
refactor thresholds. No fixture result, fixture lock, suite posture, validator,
threshold, science contract, production path, dependency, or skip behavior
changed.

## Release attempt and gate-truth audit

| Attempt | Verified result | Disposition |
| --- | --- | --- |
| 01 | Exact command exits 1 after 2.13 seconds because `cargo fmt --check` prints only deterministic rustfmt changes in the new regression | Valid preserved red gate |
| 02 | After the in-envelope mechanical formatting correction, the exact command restarts from the beginning, passes 1,946/1,946 full-profile tests, dependency policy, and fixture integrity, then exits 1 on the first missing required Cargo target | Terminal package evidence |

The second run is a legitimate correction/rerun, not retry-until-green: attempt
01 identifies a deterministic in-scope formatting defect, the source is
formatted, and the unchanged exact command restarts at its first lane. Neither
attempt uses a skip, limit, waiver, alternate authority registry, or relaxed
suite expectation.

Attempt 02 proves the provenance consumer acceptance because the release
script completes fixture hash/provenance verification before invoking the
required lane. It then requests
`auth05_level4_constitutive_authority_hardening_contract`; Cargo lists current
targets, reports that target absent, and the required hard-fail lane exits 1.

The release-lane classifications are accurate:

- workspace formatting/check/Clippy, full nextest, dependency policy, and
  fixture integrity pass in attempt 02;
- required authority fails on the missing AUTH05 target;
- later required authority, binary build/staging/lint, and stability are
  blocked by fail-fast ordering; and
- the separately required post-release formatting, Clippy, full-nextest, deny,
  Markdown, and diff loop remains blocked because release never exits zero.

Partial pass results inside the failed command do not satisfy terminal release
or integrated-validation acceptance.

## Required-binding attribution

Static registry inventory confirms seven active `required` / `hard-fail`
suites refer to five absent integration targets. Three suites share AUTH05;
the remaining four bind HPHYS0224 through HPHYS0227. Commit
`a381702beca580fa10e71456a897f1a6a705a968` deleted all five while removing the
symbol-map kernel-boundary runtime. The raw failure names AUTH05 because it is
the first required target reached; HPHYS0227 was independently discovered but
not reached in the terminal command.

This is one coherent binding family, not evidence for five serial successors.
Registry bindings, authority-test restoration, suite posture, and the retired
runtime are protected by the metadata-only provenance package, so stopping
here is an out-of-envelope HOLD rather than diagnostic deferral.

## Iterative finalizer review

`INTVAL-FINAL-001` provides the necessary broader authority without making it
unbounded:

- its first correction batch restores all five executable current-authority
  targets and adds an all-active-required-bindings guard;
- deleted tests are assertion provenance only, and the deleted symbol-map
  runtime, wrappers around it, production fallbacks, and old-path revival are
  prohibited;
- every newly exposed blocker requires reproduction, mechanism, ownership,
  authority, safety, red regression, measurable acceptance, and a prior
  intended-write-set revision;
- production or kernel changes require the seven-gate DC bar, contract-first
  authority, baseline provenance, and science-contract governance;
- skips, threshold/tolerance loosening, fixture-result edits, suite
  deactivation, retry-until-green, surrogate physics, and silent
  canonicalization are prohibited;
- related fixes are batched, focused/profile tests carry local iteration, and
  expensive exact-release/full-workspace work occurs at coherent candidate
  boundaries rather than after every file edit;
- every nonzero release result is retained in one cumulative defect ledger,
  corrected in-envelope when authorized, and rerun from the exact command's
  beginning; and
- once release passes every lane, the candidate is frozen and all integrated-
  validation phases restart from Phase 0. A new defect returns to the same
  loop, creates a new frozen candidate, and forces another full restart without
  mixing evidence.

Its HOLD rule is appropriately strict: only unavailable external evidence,
missing or contradictory canonical authority, invalid upstream inputs, or
ownership outside openWEPP may terminate the broad campaign. Another failed
gate, deleted test, effort, or need for source reading cannot.

## Roadmap, catalog, and handoff

The provenance package, HOLD audit, disposition, and handoff consistently name
`INTVAL-FINAL-001` as the non-piecemeal continuation. The roadmap records
provenance acceptance at 1,946/1,946 and the seven-suite/five-target starting
inventory without claiming release PASS. The catalog records this package as
executed-HOLD and the iterative finalizer as queued. All require exact release
completion followed by the complete integrated campaign restart; no prior
partial evidence is reusable.

Scoped Markdown lint and repository diff-integrity checks pass.
