# Review Agent A

Status: complete

Evidence mode: static-review

Static:

- Reviewed required HPHYS0298 package, contract, test, harness, and evidence
  artifacts in the openWEPP worktree at HEAD
  `2e626969f7d0789ed80b2a3b4666fb6dc7689de8`.

Ran:

- No validation gates or comparator harnesses were executed by this reviewer.

## Findings

1. High - The first-divergence classifier can assign a later cut-point before an
   earlier forcing divergence is checked.

   `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py:598`
   checks `baseline_raw_melt_minus_openwepp_raw_melt_mm` before the hourly
   forcing check at line 604, and the classifier never checks the accumulated
   snowfall forcing delta at all despite collecting it at lines 709 and 724.
   This violates the contract order in
   `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md:142`
   (`hourly-forcing` must precede `raw-hourly-melt`). The ledger shows this
   has already affected classifications: H1 first-2013 records baseline raw snow
   `68.569 mm` versus openWEPP raw snow `6.856969696969698 mm` at
   `artifacts/paired-lineage-ledger.json:55` and `:63`, but the row is classified
   as `raw-hourly-melt` at `:86`. H39 first-2013 also has both raw rain and raw
   snow forcing deltas (`:588-597`) but is classified as `raw-hourly-melt` at
   `:620`. This can send follow-on correction work to snowmelt math when the
   first divergent boundary is actually forcing/partition input.

2. High - Missing openWEPP trace fields are silently converted to closure values.

   `hphys0298_paired_lineage_partition.py:116-140` returns `0.0` or `[]` for
   missing trace rows/fields, and lines `727-734` replace `wb13_rm_mm == 0.0`
   with `routed_melt + post_rain` before computing
   `openwepp_wb13_rm_identity_abs_sum_mm`. That masks absent or legitimately
   zero WB13 `RM` publications as closed producer-consumer identity. The package
   and contracts require missing required paired trace data to hold/fail closed,
   not canonicalize-and-proceed. Any ledger claim that `Q`/WB13 identity is
   closed is therefore not reliable until missing fields are distinguished from
   real zeros and required symbols hard-fail or mark the window `UNRESOLVED`.

3. High - The published partition ledger does not satisfy the contract-required
   source-provenance payload.

   `SC-WATBAL-001.md:243` requires each valid ledger row to include canonical
   symbol values and units plus baseline and openWEPP source-line provenance.
   The generated JSON row at
   `artifacts/paired-lineage-ledger.json:51-89` contains aggregate sums and
   verdict fields, but no per-symbol `canonical_symbol`, `unit`,
   `source_path`, or `source_line_or_function` fields. The Markdown summary at
   `artifacts/paired-lineage-summary.md:28-38` is also aggregate-only. Without
   source-line provenance, the ledger cannot support the claimed
   `OPENWEPP-DEFECTIVE` verdicts or direct a contract-authorized follow-on fix.

4. Medium - Baseline observe identity evidence omits the required
   instrumented-observe-off lane.

   `package.md:297-305` requires three lanes: pinned release without observe,
   instrumented binary without `wepp_observe.on`, and instrumented binary with
   `wepp_observe.on`. The harness only runs `release` without observe and
   `observe` with observe enabled at
   `hphys0298_paired_lineage_partition.py:507-508`. The identity artifact then
   reports only release-vs-observe equality at
   `artifacts/paired-observe-identity-evidence.md:15-18`. This does not meet the
   package gate that lane 2 match lane 1 and lane 3 match lane 2, so the
   baseline trace evidence remains contract-incomplete even though release-vs-on
   WAT files are bit-identical.

5. Medium - The contract-derived test does not guard all target windows in the
   canonical contract text.

   `tests/integration/hphys0298_paired_lineage_partition_contract.rs:16-27`
   checks only broad `SC-SNOWFREEZE-001` substrings. The explicit nine-window
   loop at lines `50-62` reads `package.md`, not the canonical contract. A future
   edit could remove or corrupt target windows in
   `SC-SNOWFREEZE-001#INV-SNOWFREEZE-029` while this test still passes. This is a
   contract-first coverage gap because the package-local plan is not authority.

6. Medium - Package progress overstates gate/review/disposition completion.

   `package.md:36-39` marks optional production-fix handling, validation gates,
   dual reviews, dual verification, disposition, and worker handoff complete.
   The corresponding artifacts are still queued or not-run:
   `artifacts/gate-results.md:3-7`, `artifacts/verification_agent_a.md:3-7`,
   `artifacts/worker-handoff.md:3-9`, and
   `artifacts/review-disposition.md:3-9`. This is evidence-truthfulness drift and
   blocks any non-HOLD closure.

## Residual Risk And Missing Tests

- The review did not run `cargo test --test hphys0298_paired_lineage_partition_contract`,
  the full workspace gates, or the HPHYS0298 harness.
- Add a harness/unit regression that proves forcing deltas (`hrrain` and
  `hrsnow`) are evaluated before raw melt and that missing required trace fields
  produce `UNRESOLVED`/trace-gap rather than zero-filled closure.
- Extend the contract test to assert all nine target windows and provenance
  requirements directly against the canonical `SC-*` contracts.

## Approval

Blocking findings remain. I do not approve HPHYS0298 closure or follow-on
correction targeting from the current ledger.
