# Grid40 continuation QA review — preparation and launch boundary

Reviewer: `/root/grid40_qa` (independent QA/evidence scope).  This review is
independent of the original authors and reuses the unchanged prospective QA,
recorder-fix review, and immutable correctness preparation review only where
their reviewed bytes remain unchanged.

Evidence class: **Static** source/recovery/write-boundary inspection, plus
**Ran** inspection of retained source-bound preparation receipts and the new
isolated recorder-control receipt.  I did not run a Rust evaluator, original
arm, or substitute control.

## Findings

No blocking QA finding in the reviewed continuation increment.

The lifecycle amendment is narrow and prospective.  It preserves the prior
incomplete disposition and records the resumption anchor
`2026-09-20T01:24:30Z`, the carried `135809.983578 / 140479.464696` seconds,
the required 1200-second reserve, and the derived hard deadline
`2026-09-20T02:42:19.481118Z`.  The only recorder source change is the expired
deadline replacement.  `continuation-recorder-process-checks.json` records all
five existing synthetic cases passing; its two nonzero child outcomes are
explicitly synthetic and preserved in the carried failure accounting.

The retained detached treatment source is still
`fae897fe90f6bbadcaf298607c9d23fd986cd778f9dd56195f7a5510cbd5c4be` (749
entries).  Recovery evidence reconstructs it from frozen `8cd4866f...` by
both direct T patch and C then narrow C-to-T patch.  The retained binary hashes
to `e26cc780e7065211714a37653e55f9038cee3a5a32a6964de7122cd80cf0f272`, is
55,908,800 bytes and mode `0444`; it remains read-only and is not itself a
launch binary.  `continuation-custody.json` also rechecks the immutable input,
all nineteen external build inputs, five support links, and recovery patch
hashes.  Its absent canonical run directory and its listed synthetic receipts
support `baseline 0/1` and `treatment 0/1`; they are not arm claims.

The retained final preparation receipts establish `cargo fmt --check`, default
and `test-support` component checks, and both source-bound test executions
(195 passed, 7 ignored) on T.  Strict warnings-denied Clippy remains an
inherited-debt FAIL; the retained diagnostic-multiset comparison identifies no
introduced T diagnostic.  This is bounded quality attribution, not a lint
PASS or a production qualification claim.

The actual ignored replay entry is
`grid40_experimental_tests::grid40_original_start_replay_is_explicit_and_single_arm_only`.
The retained executable is a default-feature `openwepp-land-surface-energy`
libtest binary.  Each real arm must therefore invoke only the byte-identical
executable disposable copy with this selector and `--ignored --exact
--test-threads=1`, in a separate process.

The final T write-boundary inspection identifies these explicit child-writer
environment bindings:

- `OPENWEPP_GRID40_TRACE` reads the immutable input.
- `OPENWEPP_GRID40_ARM` selects `baseline` or `treatment`.
- `OPENWEPP_GRID40_SIDECAR` writes `sidecar.json`.
- `OPENWEPP_GRID40_EVENTS` writes `events.jsonl`.
- `OPENWEPP_LSE_FIRST_TRIAL_TRACE` writes `trace.json` and, through the
  reviewed atomic replacement path, `trace.partial` and `trace.partial.next`.

Static inspection of these reachable Grid40 source files found no child-process
creation.  The required fixed eight child slots are `stdout`, `stderr`,
`trace.json`, `trace.partial`, `trace.partial.next`, `sidecar.json`,
`events.jsonl`, and `reserved-2`.  The reviewed recorder enforces eight unique
local slots, child output inventory, per-file and aggregate limits, cleared
injection/profiling environment, and durable launcher refusals.

## Required binding before baseline

This preparation QA PASS does not make an absent `pair-freeze.json` an arm
authorization.  Before baseline, create and inspect the concrete freeze using
the existing schema.  It must bind an executable disposable copy whose SHA-256
is the retained binary hash; the T source/input identities; default-feature
build/selector/arguments above; all nineteen build inputs and five links; the
updated recorder and snapshot-tool hashes; the exact environment map and eight
slots above; and this review's content hash in `write_boundary.review_sha256`.
It must set `no_subprocess=true` and `no_other_destinations=true`, with
`output_environment` mapping each actual writer to its listed arm-local file.
The freeze must remain byte-identical through baseline admission and treatment.

The root must present that concrete freeze and executable-copy custody for a
bounded confirmation before running baseline.  Baseline comparison/admission,
treatment, and numerical/result evidence require the separately assigned
`continuation-qa-result-review.md`; this review supplies no result verdict.

## Non-blocking debt and follow-up

- Strict Clippy remains FAIL on inherited debt.  The reviewed multiset
  comparison supports no introduced diagnostic, but it does not close the
  repository lint gate.

## QA preparation PASS

QA approves the unchanged final T preparation, raw-evidence/recovery custody,
narrow lifecycle/recorder correction, and the specified source-bound write
boundary.  Launch remains gated on the concrete-freeze confirmation above and
the independent correctness continuity verdict.
