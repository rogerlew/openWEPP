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

The helper validates base-commit package authority, invokes typed CLI argument
vectors without a shell, and writes intent and terminal plans. Execution is a
mandatory state machine: it runs only policy-owned `LIGHT` nodes, freezes their
stage receipt, obtains the ten-check `pre-heavy-audit.json`, and starts a
`HEAVY` node only when that exact audit is `READY`. The heavy receipt embeds the
audit and imports the light prefix; a monolithic heavy path fails closed.

Every node writes a digest-bound checkpoint before aggregate receipt creation.
Attempt, timing, cost, failure, and tooling-defect records are appended to
`target/local-ci-history/testgate-attempts.jsonl` by default. Trusted execution
instead places that ledger under the uploaded evidence root. The attempt index
covers pre-receipt failures and per-node checkpoints so a hosted verifier can
re-ingest and verify them after runner loss.

The helper also writes an independently verified unsigned receipt and
`observation.json`. Local output remains
`LOCAL_RECEIPT_PENDING_GITHUB_ATTESTATION` and cannot close an increment. The
trusted workflow attests the exact receipt and custom predicate before its job
can pass. Use a fresh external directory for every attempt; output collision
fails closed. Verified FAIL/BLOCKED receipts remain in that directory while the
helper exits nonzero, and executor-injected Cargo, Nextest, coverage, CRAP, and
temporary work paths remain beneath the external execution root.

The stable black-box follow-up interface is
`tools/local_ci/testgate_qualification.py`. Its `validate`, `run`, and `verify`
subcommands freeze the subject, invoke the ordinary helper once per declared
case, stop on the first mismatch, and independently rehash the resulting
evidence. Qualification never converts local probe evidence into a live
provider claim.

Nextest lifecycle roles are named `affected`, `checkpoint`, `campaign`, and
`release`. Selection still comes from the terminal plan; a profile name alone
never authorizes narrowing.

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
