# B01-WB14-AVAILABLE-SOURCE-RED-WITNESS

DRAFT FOR OWNER ADOPTION. This document does not authorize its own execution.

## Active checkpoint, source, scope and bounds

**One deliverable:** a source/build-bound fresh-process witness of the existing
WB14 cadence refusal using the existing runner and unchanged collector. This is
NOT completion of the failed evidence-interface design, adoption of a cadence
baseline, or authorization to repair cadence.

**Governing evidence revision:**
`484b03b21fb059c4d2b99c66c4bc7cf7c1c1fe2c` in `rogerlew/openWEPP`.
Use `docs/work-packages/20260911-b01-wb14-evidence-interface-design-001/package.md`
and its two reviews as the failed predecessor. Proposed new owning record:
`docs/work-packages/20260912-b01-wb14-available-source-red-witness-001/package.md`.
Do not mark the predecessor complete or approved.

**Actual executable-source candidate:** `reconciled-available145-r1`, unmodified,
currently retained at
`/workdir/openwepp-experiments/b01-wb14-source-reconciliation/reconstructed-available145`.
Expected actual 927-entry map:
`c7aea0707d0c6a0876ec4bee5a7fcf15e751a092e086f517ced7edba8282b71e`.
Regular-file census digest:
`8b29e888b96c223f46d8c2432c49560bbf68c15871491cfdaa68b1221cd373c6`.
Symlink-target census digest:
`6c50ef172d22672ba98f0b66f3a45da1849b0c47409f771affd079430196b58f`.
These identify different inventories. Do not interchange them or claim the old
working145 map `b6fb949e...` was recovered. Preserve the two available test
substitutions and their UNKNOWN historical semantic differences. Reuse accepted
reconstruction evidence; verify the actual source and inputs consumed by this
build without repeating the exhausted historical recovery search.

**Proposed permission on adoption:** use this separately identified source ONLY
for this diagnostic build/run. Source adoption for cadence qualification stays
paused. Build products belong in a separate disposable target; freeze the selected
binary and unique evidence under a new durable directory outside `/tmp`.

**Allowed changes:** the new package record, evidence, narrowly scoped offline
extraction/checking scripts and locator updates. Reading and building the unchanged
source, compiled test listing, and the one named authentic invocation below are
allowed. No Rust edits, new facade/hook/fixture, assertion or cfg changes, formatting
of Rust, cadence patch, contract change, dependency/lockfile change, historical
receipt rewrite, injection, performance experiment, or checkpoint write/resume.

**Run allowance:** exactly ONE authentic result-bearing invocation of the retained
`gradual_warm_tail7` case, B01, one OFE, physical observations enabled, from lawful
day-zero fixture construction. Use the original seven-row forcing unchanged.
The existing runner should terminate at its natural failure; do not bypass it.
No candidate-green, capture/arm pair, repeat, or suffix is included. The collector's
per-process timeout is prospectively set to 3600 seconds; timeout is NOT an expected
WB14 failure. A second authentic invocation requires a new owner decision.

**Hard effort bound:** on adoption, 60 additional active minutes or two further
unsuccessful correction/verification cycles, whichever occurs first, for this
new limited executable-witness scope. Preserve the failed design's 2/2 cycles and
24 minutes 12 seconds as predecessor consumption, not zero or unused credit.
This is an explicit proposed new allowance, not a third design correction or an
automatic reset. Carry this witness ledger through all its continuations. Count
ordinary build, feature-selection and evidence-script corrections. Exclude only
recorded pure machine/reviewer waits. Do not launch a historical-budget search.

**Stopping conditions:** source or fixture drift outside the declared identity;
a need to edit Rust/authority/fixtures or relax a check; an unavailable necessary
local source/input; an unrelated downstream failure; the authentic invocation
returning, timing out or failing infrastructure; or the hard effort bound. Complete
safe evidence collection after the invocation, but do not start the next task.

## Why this checkpoint is not circular

The old proposal required a successful observation after a point the unfixed
baseline cannot reach. Existing correction145 code instead records
`surface_liquid_wb14_cadence_failure` and
`surface_liquid_wb14_cadence_caller_failure` at the rejecting boundary. The existing
`collect.py` launches an independent process and retains failure streams and
observations. Use that route; do not build an injection/session/restart framework
to rediscover this refusal.

The runner test may fail, and its scientific execution MUST remain FAIL. An
independent offline assertion that it failed at the specified boundary may pass.
Keep these two verdicts separate. Do not change `execution_valid=false` to true,
change the runner's success assertion, or hide its nonzero exit. Diagnostic files
written by the existing harness are expected evidence, not a claim about absence
of production publication or complete output-transaction rollback.

## Build and select the existing executable

Read applicable root/work-package instructions and the existing runner/collector
instructions. Pin the collector, imported predecessor collector, case file, Nix
inputs, compiler, actual source and build flags in the record. The source of the
build recipe is the retained `build145-execution-identity.json` under
`20260911-b01-wb14-verified-cadence-repair-001/artifacts/`.

Use release, LTO=false, orchestrator and runner opt-level=1, and ALL THREE existing
fixture/restart features below. Do not use default features only or --all-features.
Derive any embedded source/build stamps from this actual source and actual build
inputs; never reuse the historical working145 source/build-input stamp. Preserve
commands and Cargo JSON output, including failed attempts.

The command shape, after declaring REPO, SRC, TARGET and LOG in the record, is:

```bash
cd "$SRC"
nix develop "$REPO" --command env \
  CARGO_TARGET_DIR="$TARGET" CARGO_PROFILE_RELEASE_LTO=false \
  RUST_MIN_STACK=67108864 \
  cargo test --manifest-path "$SRC/Cargo.toml" \
  -p openwepp-runner --lib --locked --release --no-run \
  --features openwepp-runner/test-fixture-authority,openwepp-hillslope-orchestrator/persisted-restart-v1,openwepp-hillslope-orchestrator/restart-authority-evidence \
  --config profile.release.package.openwepp-hillslope-orchestrator.opt-level=1 \
  --config profile.release.package.openwepp-runner.opt-level=1 \
  --message-format=json-render-diagnostics >"$LOG" 2>"$LOG.stderr"
```

Select the `compiler-artifact` executable whose target is `openwepp_runner` and
whose profile is a test profile; do not choose the newest target filename or use
a historical executable. Copy it to a create-new durable path, hash it, and record
the exact selected Cargo row. With BINARY set to that frozen executable:

```bash
"$BINARY" hillslope::tests::stage3_snow_accuracy_case --list --ignored --exact
```

Require exactly the existing named test to be listed. This is a compiled listing,
not test execution. An unexpected/missing selection blocks the authentic run.

Run syntax/behavior checks for the offline witness checker before the invocation.
It must reject synthetic missing-record, wrong-boundary, wrong-binary, observer-loss,
truncated-input and generic-panic/timeout inputs. These are checker tests only,
not Rust physics evidence.

## The one authentic invocation

Use the unchanged collector in the pinned B01 integration package. Let OUT be a
new durable output directory, CPU be an actually available recorded affinity ID,
and CASES be the original `artifacts/gradual-warm-tail7-cases.json`. The expected
CASES SHA-256 is
`c48cc560dff31102085ebcf852225b2063a1629ad96cfa9f7283364df44fbf3f`.

```bash
"$REPO/.venv/bin/python" \
  "$REPO/docs/work-packages/20260910-stage3-b01-snow-cycle-integration-001/collect.py" \
  "$BINARY" "$OUT" --policy B01 --case gradual_warm_tail7 --ofes 1 \
  --cases "$CASES" --cpu "$CPU" --timeout 3600
```

No --minimal, --checkpoint, --checkpoint-after-day, --resume, --reuse-run-dir,
--parent-poison-check or --archive-ack-poison-check. Check resources and durable
scratch availability before launch; use the retained archive inventory to size
space, not the compressed archive size alone. Preserve argv/cwd/environment,
fixture inputs, source/build/binary identities, stdout/stderr, run/observation/
process receipts, exit and timeout. Collector exit alone cannot establish the
runner result; read its recorded child exit and execution result.

## Exact witness acceptance and claim limits

The offline checker must preserve complete raw records, decode integer identities
without narrowing, and correlate the paired guard/caller records to this actual
invocation. In a separate diagnostic result, require:

- Actual selection of one named test and a non-timeout, non-infrastructure runner
  failure with the specific SURFACELIQUID-E-008 day/interval continuation reason;
  generic nonzero status is insufficient.
- Actual input day=4, interval=22, TransactionId=255 (u128 domain), interval=60 s;
  parent [385200000000000,387000000000000) ns and attempted child beginning
  385920000000000 ns. Keep the transaction sequence separate from the coupled
  parent's full digest and accepted-slab identity. A future accepted ordinal
  is not available merely because an attempt occurred; do not manufacture it.
- Parent-child mode, nonfinal posture, supplied parent accepted-until/child-start
  equality, initial persistent day/interval and cumulative bytes, physical
  ordinal zero/no prior physical receipts, and the supplied inactive-prefix
  payload. Check pairwise consistency; preserve all actual identity/receipt
  bytes, not only summaries or self-reported digests.
- A complete parseable observation file under the existing schema, with no
  overflow/count poisoning, and unchanged frozen binary. Retain full raw-file
  hashes and extracted-record locations. Missing context is missing evidence,
  not permission to reconstruct it from expected numbers.

Success means ONLY: this named available-source executable reproduces the specified
refusal and supplies a usable boundary packet. Producer-supplied proof bytes and
hash consistency are not independent validation of the full prefix. The literal
`state_mutated=false` is not an observed complete rollback. No zero-total-WB14,
full-parent, conservation, source-equivalence, fresh-restart, output-absence,
performance, or production-readiness claim is included.

All previous cadence acceptance remains outstanding: the 18-child private vector,
first/subsequent/final/replay proof, actual adaptive runner evidence, total physical
counters, complete rollback, split restart, independent budgets, 18/4 protected
available test modules, applicable A0/A1/A3, lint and full regression. No failed
current requirement is reclassified as passed; the interface design remains
incomplete. This limited prerequisite does not replace those obligations.

## Review, preservation and owner return

Astra/medium orchestrates; Terra/medium handles build/extraction/checker details.
Reuse the two distinct independent reviewers for the changed witness scope:
correctness checks the claimed failure/source/control-flow interpretation; QA
checks actual argv, selection, identities, complete observations and negative
checker behavior. Do not reopen accepted reconciliation or fixed design portions.
This adviser conversation supplies neither independent review. Missing independent
review remains unmet; do not manufacture a signoff or launch a generic extra pipeline.

Keep one maintained record and necessary raw artifacts. Carry scoped evidence
commit/push permission; verify the published files actually retrieved. Do not claim
complete remote source/LFS recovery from verification of new evidence files.
Preserve existing source copies, failed drafts, candidate patches and checkpoints.

Return actual source/build/binary IDs, command/selection, runner FAIL versus witness
verdict, compact boundary operands with raw locations, gaps, reviewer findings and
remaining allowance. Even a successful witness returns to the owner; it does not
resume cadence repair, adopt the rejected envelope, or grant a second run.

## Immutable evidence anchors

All paths below are at repository revision
`484b03b21fb059c4d2b99c66c4bc7cf7c1c1fe2c`:

- `docs/work-packages/20260911-b01-wb14-evidence-interface-design-001/package.md`
  and `artifacts/{correctness-review,qa-review}.txt`.
- `docs/work-packages/20260911-b01-wb14-source-reconciliation-001/artifacts/revised-baseline-proposal.json`.
- `docs/work-packages/20260910-stage3-b01-snow-cycle-integration-001/collect.py`
  and `artifacts/correction145.patch` (two failure-observation records).
- `docs/work-packages/20260911-b01-wb14-verified-cadence-repair-001/artifacts/build145-execution-identity.json`.
