# Independent Review B

Evidence class: **Ran** for lightweight log replay, hashes, Git-object checks,
line counts, Markdown, and diff checks; **Static** for source, package,
protected-boundary, and successor inspection. No heavy gate was rerun.

Verdict: `HOLD-VALID-WITH-FINDINGS`

The `HOLD-INTVAL-REL-001` disposition is legitimate. The intended process-
isolation defect is corrected, but its exact no-skip release acceptance did not
exit zero: the unchanged required-authority integrity gate failed closed on a
protected provenance record. Two accepted-fix evidence/package findings block
terminal review closure; neither authorizes a fixture-value, production,
authority-validator, or release-lane change.

## Command and log audit

The two archived command records reconcile exactly:

| ID | Command/result | Exit | Elapsed | Peak RSS (KiB) |
| --- | --- | ---: | ---: | ---: |
| 00 | pinned `/workdir/wepp-forest` commit and two SHA-256 values verified | 0 | 0:00.02 | 3,840 |
| 01 | exact no-skip release command; authority provenance failure | 1 | 9:43.01 | 209,716 |

ID 01 contains the exact package invocation with cohort and watchlist paths and
both `--expect-suite` counts; it contains no skip, periodic/manual substitution,
retry, or waiver. Its progression proves formatting and all-target Clippy
completed, full-profile nextest ran 1,945 tests across 175 binaries with 1,945
passed and three configuration-skipped, and `cargo deny` passed all four policy
classes. The log then records both validator errors for
`cas_l4_subhyd_watyld_fcwp_consistency_001` and exits 1. Fail-fast ordering
correctly leaves the remaining authority suites, binary build/staging/sidecars,
release lint, and stability `BLOCKED`. The separate final closure gates are
also truthfully `BLOCKED`, not converted to PASS by results embedded earlier in
the failed release command.

The pinned input hashes in ID 00 match the current files. Their data counts are
1,166 cohort records plus a header and 19 watchlist records plus a header, which
cohere with the exact expectations. `/workdir/wepp-forest` resolves to the
stated commit `375ccc296ed1ea491f599ff1b1a25b415d494a2a`.

## Correction and protected-boundary audit

The implementation is direct and bounded:

- the release script replaces only `cargo test --workspace` with
  `cargo nextest run --workspace --profile full`;
- the README documents full-profile nextest process isolation; and
- the 14-line Rust source guard requires that canonical command and rejects the
  exact stale workspace-libtest command.

The source header for the H2637 integration target requires nextest process-per-
test isolation for process-global selector mutations. The full nextest release
lane completes without the former three failures, so the original harness
mechanism no longer blocks the workspace lane. No production crate, physics,
science contract, H2637 selector/assertion/threshold, fixture payload/lock/
provenance, authority registry, validator, skip behavior, or dependency was
edited. The touched Rust test is 120 lines, below both line-count thresholds.

The new failure is independently real. The authority validator requires a
top-level schema marker and per-fixture `source_repo`, `source_commit`,
`source_path`, `source_sha256`, and transform note. The protected record lacks
the schema marker and its fixture item lacks the first two provenance fields.
The JSON and lock both hash to
`a1c50a82cd1e497875cb034481c4b2ef710c319907480b0f584fde30f48fae5e`;
the same path at Git commit
`9aa4c3d61549ab30da665a4dc109bab811522fe9` has that exact hash. The release
gate therefore rejected incomplete metadata without identifying a fixture-byte
or physics defect.

The provenance record, payload, lock, registry posture, validator, and
thresholds are protected by this package. Weakening validation, skipping
authority, or editing those files here would violate the correction envelope.
The finite `INTVAL-AUTH-PROV-001` successor is the appropriate ownership
boundary, so this is not an improper diagnostic relay.

## Findings

### INTVAL-REL-B-01 — accepted fix required: focused execution claims lack archived evidence

`intake.md`, `implementation-evidence.md`, and the package state that the new
guard failed before correction, passed 1/1 afterward, and the three former
H2637 collisions passed together 3/3. The package artifact set contains only
IDs 00 and 01; it has no command/log/time record for either the guard's focused
pre/post execution or the focused three-test H2637 selection.

ID 01 directly proves the post-correction guard and nonignored H2637 tests as
members of a successful full workspace nextest run, but its compact summary
does not prove the claimed focused selections/counts. It cannot prove a pre-fix
execution. Before review closure, either archive the actual focused command
transcripts/timings, or reclassify the unsupported pre-fix and focused-count
claims as Static reconstruction/full-workspace inclusion with exact source and
ID 01 bindings. Do not present unarchived execution as `Ran`.

### INTVAL-REL-B-02 — accepted fix required: successor permits contradictory provenance metadata

The successor correctly limits implementation to provenance metadata plus a
narrow guard, pins the fixture-producing Git commit and locked fixture hash,
protects fixture bytes/lock/authority/validator/thresholds, requires exact
release and final gates, and preserves the full integrated-campaign restart.
However, the current provenance file also contains a top-level
`source_commit: working-tree-hphys0256`. The successor says to add the pinned
per-item commit but does not require removal or normalization of that stale,
non-Git placeholder. The resulting file could contain two conflicting source
commit claims while satisfying the current nonempty-field validator.

Amend the successor envelope and source guard to require one unambiguous,
truthful provenance posture: remove the obsolete top-level source fields if
they are outside the canonical schema, or normalize them to the same verified
repository/commit and explicitly prove that representation is canonical. The
guard must require `schema_version: 1`, the exact per-item 40-hex source commit,
and unchanged fixture/source hashes rather than mere field presence.

## HOLD and restart disposition

There is no partial release or integrated-validation PASS. Roadmap and catalog
entries truthfully mark this correction as executed HOLD, name the provenance
successor, and retain the sequence: close `INTVAL-AUTH-PROV-001`, rerun the
exact pinned-input release command from its beginning, then restart integrated
validation from Phase 0 at the correction commit. No result from this failed
release attempt may satisfy either later acceptance.
