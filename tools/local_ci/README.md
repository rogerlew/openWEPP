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

## TESTGATE Increment Execution
Build the repository-owned planner/executor, then create one external evidence
directory for an exact base/head increment:

```bash
cargo build -p openwepp-gate-planner --bin openwepp-gate-plan
testgate_dir="$(mktemp -d)"
python tools/local_ci/testgate.py \
  --binary target/debug/openwepp-gate-plan \
  --base HEAD^ \
  --artifact-root "${testgate_dir}" \
  --intent-package docs/work-packages/<id>/package.md \
  --dirty \
  --execute
```

The helper validates base-commit package authority, writes intent and terminal
plans, runs policy-owned `LIGHT` nodes, and emits the canonical ten-check
`pre-heavy-audit.json`. `HEAVY` begins only for that exact `READY` audit. The
LIGHT receipt, HEAVY claims, executor digest, package admission, plan, and roots
remain inseparable.

Every node writes a digest-bound checkpoint and declared outputs to the durable
`/testgate-history/recovery/<run>-<attempt>` mirror. The terminal plan is copied
there before HEAVY starts. A checkpoint cannot suppress work from its self-hash:
resume requires a hosted-runner attestation over an exact archive index, verifies
the prior plan/node/binding and every indexed byte, and additionally verifies an
aggregate receipt when one exists. Pre-receipt imports record a provenance ID;
receipt-backed imports record both receipt and provenance lineage.

The always-run finalizer reconciles orphaned `STARTED` admissions, snapshots the
full ledger and every ledger-referenced recovery root, rejects symlinks and
unindexed bytes, and uploads the exact archive. A hosted verifier attests its
index even when execution failed. Every later trusted run verifies the newest
attestation and refreshes provenance; ledger and recovery bytes themselves are
restored only into empty durable history. This preserves A→B→C recovery without
treating a digest chain as authorship.

A tooling failure opens a blocking defect immediately. Infrastructure receives
at most one declared retry; an unmatched process termination is reconciled to a
typed infrastructure failure before another admission. Representable audit
failures still produce the schema-valid ten-check artifact.

Critical plans retain separate full-regression and global CRAP nodes unless a
repository-reviewed three-baseline proof is active in policy or explicitly
selected with `--combined-proof-id`. Admission binds the current host/image,
exact parity and coverage lineage, and the 120%/80% economy limits. The policy
keeps the active proof null until real protected-CI measurements are reviewed;
no synthetic proof is permitted.

The helper writes an independently verified unsigned receipt and
`observation.json`. Local output remains
`LOCAL_RECEIPT_PENDING_GITHUB_ATTESTATION` and cannot close an increment. Use a
fresh external directory for every attempt; output collision fails closed.

The stable black-box follow-up interface is
`tools/local_ci/testgate_qualification.py`. Its `validate`, `run`, and `verify`
subcommands freeze the subject, invoke the ordinary helper once per declared
case, stop on the first mismatch, and independently rehash the resulting
evidence. Qualification never converts local probe evidence into a live
provider claim. A real combined proof must be collected and pinned before the
Q12 subject freeze.

Nextest lifecycle roles are named `affected`, `checkpoint`, `campaign`, and
`release`. Selection still comes from the terminal plan; a profile name alone
never authorizes narrowing.

The production helper invokes one binary `transition` for LIGHT, audit construction, and HEAVY admission. The binary persists the LIGHT receipt and `READY` audit, carries that audit in process, and rejects standalone HEAVY use because a self-hash alone cannot authenticate audit provenance. This keeps the two independent inventory enumerations at LIGHT validation and audit construction while eliminating a forged-audit gap and any third local enumeration.
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
