# Review Agent B

Status: complete

Evidence mode: static+ran

Static:

- Reviewed package scope, contract-test, baseline instrumentation patch, paired
  lineage runner, full-suite metrics, paired-lineage summary/ledger, and related
  gate/disposition artifacts.
- Reviewed generated artifact names against required package deliverables and
  acceptance gates.

Ran:

- `cargo fmt --check` -> pass.
- `cargo test --test hphys0298_paired_lineage_partition_contract -- --nocapture`
  -> pass (`3 passed; 0 failed`).
- `.venv/bin/python -m py_compile docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py`
  -> pass.
- `git -C /workdir/wepp-forest_260430_baseline status -sb` -> `## HEAD (no branch)`.
- Ledger spot-check: all nine windows carry raw-snow forcing deltas greater than
  the declared `2.0 mm` window tolerance, but the classifier does not compare raw
  snow before assigning later cut-points.

## Findings

- **High - First-divergence verdicts are not reliable because the classifier skips
  an upstream forcing term and checks cut-points out of contract order.** The
  package defines the ordered cut-points as winter gate, hourly forcing, then raw
  hourly melt in `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/package.md:110-121`, and the tolerance is `2.0 mm` in
  `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py:35`. The
  implementation checks raw melt before forcing and only checks raw rain in
  `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py:598-609`, while raw snow is
  collected at `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py:707-724` but never
  compared. The generated H7 first-2013 row shows
  `baseline_raw_snow_sum_mm=71.5815` and `openwepp_raw_snow_sum_mm=7.158219696969699`
  in `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/paired-lineage-ledger.json:318-330`; raw melt/rain deltas are
  within tolerance at `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/paired-lineage-ledger.json:345-346`, yet the
  row is classified as `negative-melt-correction` in
  `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/paired-lineage-ledger.json:353-355`. This can assign later
  cut-points while upstream hourly forcing is already open, so the nine
  `OPENWEPP-DEFECTIVE` source-partition verdicts should remain `HOLD`.

- **High - The generated ledger does not satisfy the required per-symbol
  provenance schema.** The package requires each ledger row to include first
  divergent canonical symbols, baseline/openWEPP values, source lines, `Q` and
  WB13 identity status, final verdict, and next action in
  `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/package.md:356-367`; the canonical WATBAL amendment repeats source-line
  provenance and independent correctness requirements in
  `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md:246`. The
  runner initializes aggregate window sums only in
  `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py:649-693`, and the Markdown
  summary publishes only aggregate columns in
  `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/paired-lineage-summary.md:26-38`. Static search found no
  per-row `canonical_symbol`, first-divergent symbol list, or baseline/openWEPP
  source-line fields in `paired-lineage-ledger.json`. Without those fields, an
  independent reviewer cannot reproduce the first-divergence assignment from the
  artifact alone.

- **Medium - Required canonical evidence artifacts remain queued, and observe
  identity omits the specified three-lane check.** The package requires release
  comparator, instrumented observe-off, and instrumented observe-on lanes before
  trace use in `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/package.md:311-321`, but the runner only executes the
  release lane and observe-on lane in
  `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py:499-508`. The required
  `paired-observe-identity-evidence.md` artifact is still `not-run` in
  `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/paired-observe-identity-evidence.md:1-8`, and the required
  `partition-ledger.md` artifact is still `not-run` in
  `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/partition-ledger.md:1-7`, while alternate outputs are written from
  `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py:878-929`. Consumers following
  the package-specified artifact names will see no completed identity or ledger
  evidence.

- **Medium - Required closure gates and hold/disposition artifacts are not yet
  complete.** The package requires `cargo fmt`, `clippy -D warnings`,
  `cargo test --workspace`, `cargo deny check`, anti-evasion guards, and doc lint
  in `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/package.md:391-399`, but
  `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/gate-results.md:1-17` remains `not-run`. The kernel-profile
  checklist remains unchecked in
  `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/kernel-profile-compliance-checklist.md:1-13`, and final
  disposition/handoff remain queued in
  `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/disposition.md:1-8` and
  `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/worker-handoff.md:1-9`. This is acceptable only for an active
  `HOLD`; it blocks any non-HOLD closure.

## Non-Blocking Debt / Follow-Ups

- The metrics and lineage summaries cite candidate HEAD
  `2e626969f7d0789ed80b2a3b4666fb6dc7689de8`, but the worktree contains
  uncommitted changes. Add a status/diff manifest or commit SHA plus dirty-state
  provenance before treating `/tmp/hphys0298_full_20260605T000000Z` as
  reproducible evidence.
- `package.md` still reports `Status: queued` and unchecked progress for phases
  with completed evidence; update the living plan after the blocking evidence
  issues are corrected.

## QA Statement

QA pass not granted. Keep HPHYS0298 in `HOLD` until the first-divergence
classifier follows the canonical cut-point order, ledger artifacts include
required per-symbol provenance, canonical evidence files are populated, and the
required gates/disposition are complete.
