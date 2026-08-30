# Review D — QA, anti-evasion, and qualification

Status: **HOLD**

Review date: `2026-08-29`

Scope: independent read-only review of the exact current worktree and package
evidence. This review changed no production, test, contract, assurance, or
other package artifact.

## Evidence reviewed

Static:

- Read `package.md`, the package gate/disposition/implementation evidence,
  owner amendments, calibration matrix, worker handoff, review/verification
  placeholders, and the current tracked plus untracked source set.
- The worktree has 282 changed or untracked paths. The tracked terminal diff is
  196 files with 48,840 insertions and 19,248 deletions. The package-owned file
  manifest and terminal-diff reconciliation are still queued, so this review
  cannot prove that every changed path is owned, intentional, or covered by the
  selected gates.
- Inspected the production runner chain from
  `execute_direct_publication_stream` through committed-day staging, durable
  archive append/digest verification, archive acknowledgement, retained direct
  publication validation, WB13 provenance construction, staged output writes,
  and output/manifest transaction publication. The compatibility rollback is
  absent and the publication source is the direct publication frame.
- Inspected the one-day qualification body. It validates the committed
  snapshot, exact 48-parent chronology, adaptive receipt count, rejection-count
  reconciliation, independent mass/energy ledger bounds, receipt-reseal bounds,
  the covered fixed-point support, and existence of final PASS/loss/manifest
  outputs.
- Inspected adaptive telemetry/audit declarations and searched restart and
  committed-archive sources for those types. The telemetry is default-off,
  process-local/thread-local, and has no serde wire. No adaptive telemetry,
  fixed-point audit, comparison audit, or physical-closure audit field was
  found in restart or committed archive schemas. This satisfies the narrow
  owner requirement that the diagnostic counters not be persisted, although
  the opt-in diagnostic APIs remain compiled production APIs.
- The package correctly marks provisional 600-ms floor-dependent evidence as
  `SUPERSEDED`, and the main gate ledger keeps failed compacted-day and paused
  seasonal runs as historical negative evidence rather than PASS evidence.

Ran:

- `bash tools/release/check_authority_suite_antievasion.sh` — PASS.
- `nix develop --command cargo nextest run --test
  auth11_required_suite_obligation_guards_contract` — PASS, 3/3, run
  `5af5c00f-dcfe-445f-9a49-acc769d24df4`.
- `nix develop --command cargo nextest run -E
  'binary(snow_stage3_adaptive_compositional_contract)'` as part of run
  `0793df6a-7fb0-46c5-b662-f440acf03919` — PASS, 4/4 contract tests.
- Runner negative-path/source guards
  `r7c_direct_production_source_excludes_compatibility_entrypoints`,
  `compatibility_runtime_deletion_removes_obsolete_transition_modes`, and
  `stage3_v11_runner_streams_rows_and_durably_archives_each_day_without_batch_retention`
  — PASS, 3/3, run `506f35e4-c24a-4af9-a194-76729953a20d`.
- Orchestrator guard
  `retired_snow_stage3_shadow_is_not_compiled_or_exported_in_production` —
  PASS, 1/1, run `cfbe1961-24e2-4201-8251-e8f99caf55c7`.
- `git diff --check` — PASS.
- `nix develop --command cargo fmt --all -- --check` — **FAIL** at
  `tests/integration/surface_liquid_hydrology_custody_authority_contract.rs:189`.
- Counted every changed/untracked Rust file: 43 are at least 2,000 lines and
  `crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow.rs` is
  3,012 lines, above the package's nonexempt 3,000-line closure limit.
- Inspected `/tmp/adaptive_microstep_amendment/one-day-final-v7-opt.log` and
  `.time`: 48 telemetry rows and 48 phase rows; final cumulative publication
  counts 975 supports/61 events; 497 accepted and 206 rejected trials; 1,578
  independently checked ledgers; maximum residuals
  `1.77635683940025046e-15 kg m^-2` and
  `1.39698386192321777e-9 J m^-2`; receipt-reseal maxima
  `9.98625182546675205e-10 J m^-2` and
  `4.37694325228221714e-12 K`; test PASS in 357.55 s; complete Cargo command
  exit 0 in 561.39 s with compilation-inclusive peak RSS 5,894,016 KiB.

## Findings

### D-1 — Closure-blocking: no exact-current terminal correctness gate

The only package-recorded full-workspace critical regression is explicitly
FAIL (`8ec6202e-fafa-454a-8fc9-f9f2e621d149`: 107 failures and 10 timeouts).
No exact-current replacement full-workspace PASS is recorded. The referenced
`target/nextest/full/junit.xml` has also been overwritten: it currently reports
UUID `31e1c9a6-4973-46b0-8d88-c5acf272c267`, 87 tests, and 42 failures, not the
3,582-test run described by the ledger. Thus the live path is not durable
evidence for the historical row, and neither report can close a Critical
terminal gate.

Disposition required: retain the historical report under an immutable
package-local path, run the exact-current critical regression selected by the
testing standard after source freeze, and record a direct PASS or a truthful
package HOLD. This finding blocks GO.

### D-2 — Closure-blocking: terminal diff and ownership are unreconciled

`owned-file-manifest.md` and `terminal-diff-reconciliation.md` are queued while
the worktree contains 282 changed/untracked paths spanning production,
contracts, assurance identities, fixtures, tests, and multiple older package
artifacts. A review cannot infer package ownership or gate selection from this
unreconciled tree.

Disposition required: freeze source, generate the exact owned-file manifest,
reconcile every terminal path against declared intent, and select gates from
that exact manifest. This finding blocks GO.

### D-3 — Closure-blocking: current mechanical gates fail or are absent

Current rustfmt fails, and `v11_covered/open_snow.rs` is 3,012 lines. The line
count artifact is still queued despite 43 touched Rust files at or above the
2,000-line WARN threshold. Package governance makes a nonexempt 3,000+ file a
hard closure block.

Disposition required: mechanically split the 3,012-line file below 3,000,
record WARN rationales/split posture for every touched 2,000+ file, run rustfmt
successfully, and update `line-count-governance.md`. This finding blocks GO.

### D-4 — Closure-blocking: required independent closure artifacts are absent

Reviews A, B, and C, both terminal verifications, and the consolidated finding
disposition are all queued. The worker handoff still says implementation is in
progress. The package explicitly requires four GO reviews, disposition of
every finding, and dual PASS verification.

Disposition required: complete those independent reviews/verifications only
after the source and terminal evidence are frozen, then disposition every
finding. This finding blocks GO.

### D-5 — High: package artifacts contain mutually stale current-state claims

The main gate ledger and disposition now report a passing final optimized
one-day, but several required artifacts still describe an unresolved-liquid
failure as current:

- `owner-amendment-60-second-floor.md:34-36` says the latest one-day remains
  failed;
- `calibration-readiness-matrix.md:7` says qualification is incomplete because
  the one-day publication gate still fails;
- `worker-handoff.md:6-9` assigns resolution of that already-superseded error;
- `package.md:221-227` leaves checkpoints D-F open and describes the older
  1,435-support/52-event posture rather than the final 975/61 evidence.

The superseded pre-implementation artifact is appropriately historical, but
the files above are not labeled frozen historical snapshots. Their conflicting
present-tense claims make the package disposition non-canonical.

Disposition required: reconcile all current-status artifacts while preserving
old failures as explicitly historical evidence. This finding blocks GO until
the package has one consistent terminal disposition.

### D-6 — High: one-day evidence is real but not exact-current snapshot-bound

The optimized one-day log is legitimate real-consumer evidence and supports
the stated 497/206 and 975/61 measures. It runs the production runner through
the downstream publication/output path; it is not a producer-only or shadow
test. However, the artifact records no test-binary hash or source manifest,
and two integration contract files were modified after the log timestamp.
Those later edits do not invalidate the observed production computation, but
they do prevent calling the run an exact-current terminal-worktree gate.

Disposition required: bind the accepted log to an exact source/test-binary
manifest, or rerun the required qualification after terminal source freeze.
Do not rerun merely to replace a valid functional proof; rerun only where the
canonical gate requires exact-current evidence.

### D-7 — No blocking finding on persisted diagnostics or retired paths

Current static and executed guards support the negative claims: the retired
Stage-3 shadow/compatibility runtime is not compiled into production, the
runner uses the direct streaming/archive consumer, and the adaptive audits are
not serialized or retained in restart/publication evidence. No production
diagnostic persistence or selector-evasion finding was found in this review.

## Terminal decision

**HOLD.** The owner-amended one-day performance and real downstream-consumer
objective has persuasive positive evidence, and the anti-evasion/persistence
posture reviewed here is sound. Package closure and atomic cutover are not yet
legitimate because the exact terminal diff is unreconciled, current formatting
and line-count gates fail, the exact-current Critical workspace regression is
absent, required reviews/verifications are queued, and current-status artifacts
contradict one another.

GO may be reconsidered after D-1 through D-6 are closed on one frozen source
snapshot without weakening the exact-60, receipt/custody, event, rollback,
ledger, or downstream-consumer gates.
