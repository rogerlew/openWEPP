# Local CI Timing Tools

`nextest_timing.py` records local nextest timing evidence under
`target/local-ci-history/` (already ignored by git). Use it when a gate run is
expensive enough that the wall time and slow-test list should be durable.

## Common Commands

Record an existing JUnit file:

```bash
python tools/local_ci/nextest_timing.py summarize \
  --label full-existing \
  --profile full
```

Run a gate and record it:

```bash
python tools/local_ci/nextest_timing.py run \
  --label quick \
  --profile quick \
  -- cargo nextest run --workspace --profile quick
```

`run` and `sweep` delete the selected JUnit file before executing and require a
fresh JUnit file afterwards. Use `summarize` when intentionally recording an
existing JUnit file. When `--junit` is omitted, the path defaults to
`target/nextest/<profile>/junit.xml`.

Benchmark a nextest test-group cap without editing the committed config:

```bash
python tools/local_ci/nextest_timing.py sweep \
  --group cli-fixture \
  --caps 2,3 \
  --profile full \
  --filterset 'binary(/^(cli01_runner_hillslope_integration|cli03_runner_contract_derived_tests|cli04_runner_wat_parquet_contract_derived_tests)$/)'
```

The latest summary is written to `target/local-ci-history/latest.md`; the full
append-only log is `target/local-ci-history/nextest-runs.jsonl`.

## CQR Quality-Evidence Intake

Before CQR target selection, validate the exact quality-observatory publication
with `cqr_quality_evidence.py inspect`. Supply its complete control receipt and
expected `quality_evidence_id`; retain the canonical output in the batch
package. Only `CURRENT` may select modules.

The tool reconstructs registry adjudication and actionable module ranking from
exact compact rows and launches no measurement. Fresh acquisition requires
`authorize-recollection`, a retained `STALE`/`INVALID` receipt, and the explicit
operator CQR directive.

## CQR Aggregate Admission

Before the first implementation edit in a multi-package CQR batch, commit an
aggregate scaffold with its package-local batch manifest. The manifest must
enumerate the master ExecPlan, every module package, and all required batch
paths. Bind that authority in each module scaffold, commit the module scaffold,
then validate it against the earlier aggregate scaffold:

```bash
python tools/local_ci/check_cqr_aggregate_admission.py \
  --repo . \
  --aggregate-package docs/work-packages/<aggregate-id>/package.md \
  --aggregate-scaffold <aggregate-scaffold-commit> \
  --module-package docs/work-packages/<module-id>/package.md
```

The command fails unless the aggregate package existed with `ACTIVE` or
`READY` status at the named commit, its declared write set remains unchanged,
it predates the module scaffold, the module binds the exact package/commit, and
the immutable batch manifest and module scaffold jointly bind the master plan,
complete module list, and intended write set. Duplicate headings/fields,
non-canonical paths, late bindings, and post-scaffold write-set changes fail
closed. Retain the PASS JSON before production/test implementation.

## Retired TESTGATE Interface

TESTGATE, its controller, planner transitions, receipts, ledgers, recovery, and
forest1 workflow were deleted by ADR-0043 roadmap Order 4. They have no live
command or workflow interface. Do not reconstruct or invoke them for
prospective work.

Use the direct commands in
`docs/standards/local-ci-gate-selection.md` and record exact commands/results in
the owning package. Historical evidence inspection is read-only and confers no
prospective authority. Defunct Omarchy records are historical metadata, not
live queue occupancy.

Existing receipts, attempts, and verdicts retain their original bytes and
meaning. Generation-17 policy identity is pinned in
`gate-policy/history/adr0039-generation17.json` to an immutable Git object and
digest, never the current live testing standard.

## Assurance Amendment Receipts

For a typed report-data-only amendment, build the assurance binary once and run
the receipt-authorized focused gate:

```bash
cargo build --release -p openwepp-assurance
.venv/bin/python tools/local_ci/run_assurance_amendment.py \
  --receipt assurance/v2/transactions/<receipt-id>.json
```

The runner validates the canonical receipt and current generation, rejects
non-focused paths or gate IDs, performs one named build/check realization per
affected report, runs the pinned `assurance-amendment` nextest manifest without
a shell, and writes untracked evidence under
`target/local-ci-history/assurance-amendment/`.

## Release CLI Evidence

For timing, comparator, or package evidence that invokes a release runner CLI,
build the exact runner binary target first. A generic workspace
`cargo build --release` can leave non-default runner bins stale.

Broad runner build:

```bash
cargo build --release -p openwepp-runner --bins
```

Narrow hillslope-only build:

```bash
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
```

Record binary provenance before accepting timings or output hashes:

```bash
stat -c '%y %s %n' target/release/openwepp-cli-hill
sha256sum target/release/openwepp-cli-hill
```

If a fixture runfile hardcodes output paths, verify where the CLI writes real
artifacts before comparing hashes. For H2637-class timing runs, sequence
plain/hybrid runs and hash the actual `output/` artifacts between runs.
