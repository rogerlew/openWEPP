# Independent correctness review — decoded route assessment

**Static:** independently inspected the verified detached source at
`/workdir/openwepp-experiments/b01-wb14-cadence/current-context-capture-20260913`
and the parent-produced COMPLETE selective/decode evidence. **Ran:** read-only
structural comparisons over the retained decoded JSON only. I did not run the
selector, decoder, raw traversal, Rust, build, runtime, probe, or model. The
parent's extraction/decode logs are supplied execution evidence, not commands I
ran. Scope is route attribution and affected runtime correctness; no native,
science, reader, restart, or checkpoint acceptance.

## Findings

- **HIGH — the actual snow-free WB14 parent is internally day-inconsistent and
  cannot admit the requested child.** Decoded ingress is transaction **255**, day
  **4**, interval **22**, 60 s. The outer parent also says day 4/interval 22 over
  `[385200000000000,387000000000000)` ns and is positioned at
  `385920000000000` ns. Its sole `ofe-1` authority instead says day 0/interval 0;
  its beginning cursor, persistent state and candidate state are likewise
  day 0/next interval 0 with zero cumulative water. Guard and caller inputs are
  equal, guard/caller beginning states are equal, caller beginning equals caller
  working, and the parent persistent/candidate states equal those unchanged
  caller states (typed-state SHA-256
  `9058fd7dad195a9dd75475badf716e6b7c585644180c9ec59a54c9767c6d0b32`).
  The guard therefore reports `initial=true`, expected day 0/interval 0 versus
  actual day 4/interval 22, `accepted_parent_local_projection=false`, and
  `state_mutated=false`, followed by `SURFACELIQUID-E-008`. Source explains the
  split: [`begin_after_native_inactive_prefix`](/workdir/openwepp-experiments/b01-wb14-cadence/current-context-capture-20260913/crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress_coordinator.rs:938)
  labels the outer state from its day/interval arguments, while
  [`wb14_parent_authority_v1`](/workdir/openwepp-experiments/b01-wb14-cadence/current-context-capture-20260913/crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_wb14.rs:1127)
  derives each authority's day/interval from the stale persistent cursor.
  [`validate_nested`](/workdir/openwepp-experiments/b01-wb14-cadence/current-context-capture-20260913/crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress_coordinator.rs:735)
  does not cross-bind those inner indices to the outer indices, so construction
  accepts the split object and the later cadence guard rejects it. The rejection
  is typed and fail-closed, but this remains a runtime correctness blocker.

- **MEDIUM — duplicated provisional-custody construction continues to permit
  route-specific evidence drift.** Covered construction records the pre-child
  context in
  [`snow_stage3_v11_terminal_execution.rs`](/workdir/openwepp-experiments/b01-wb14-cadence/current-context-capture-20260913/crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_terminal_execution.rs:2352),
  whereas the separately implemented snow-free constraint/reduction/ledger/
  provisional-slab/binding path in
  [`snow_stage3_v11_real_parent_execution.rs`](/workdir/openwepp-experiments/b01-wb14-cadence/current-context-capture-20260913/crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_real_parent_execution.rs:65)
  has no recorder. The decoded discriminator now attributes this E008 to that
  uninstrumented copy, confirming the prior review's maintainability finding.
  Centralize the shared construction/capture seam or retain an explicit,
  tested justification for both implementations.

## Route attribution and affected verification

The corrected selective pass is terminal COMPLETE: immutable source SHA-256
`709dc4992b2fd08fd8bae755c730a5a01341a9a18cee8a0817ca76a249974e03`,
8,236,702,644 bytes, 123,093 physical rows, exact ordinals 123091/123092,
complete syntax/EOF, unchanged source stat, and selected-event SHA-256
`2525f332e1a3550ec436b3e3d4c7b03984a5abd94320aee46b1fee5565409a9e`.
The strict decoded payload receipt is COMPLETE. Guard and caller input bytes are
identical (SHA-256
`9c46a8b867a6a05c45902ae13a43be05bb7fb5d28971f3ab4dd0c89e277c3b55`);
the caller parent bytes are SHA-256
`07f50df314daf5d2be2c8019752f34cd49e20c732515c97af4aad424b58163b4`.

The sole per-OFE authority has a non-null `inactive_prefix` with:

- parent support `[385200000000000,387000000000000)` ns;
- `prefix_end_ns = 385920000000000`, exactly the caller binding's child start;
- `coupled_parent_transaction_sha256` exactly equal to the caller binding;
- `prefix_ending_owner_sha256` exactly equal to the binding's
  `parent_beginning_complete_owner_set_sha256`;
- authority working `accepted_until_ns = 385920000000000`,
  `next_child_ordinal = 0`, and zero child receipts.

This combination uniquely identifies the **first snow-free successor physical
ingress attempt after a represented-snow inactive prefix** among the relevant
current in-process routes. The only non-test prefix producer is the
`active_lanes.is_empty()` successor branch at
[`snow_stage3_v11_adaptive_execution.rs`](/workdir/openwepp-experiments/b01-wb14-cadence/current-context-capture-20260913/crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_adaptive_execution.rs:2169):
it requires no prior snow-free successor receipt, nonempty covered owner joins,
and both V3/V4 residents. It passes the prefix through
[`execute_real_v11_parent`](/workdir/openwepp-experiments/b01-wb14-cadence/current-context-capture-20260913/crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_real_parent_execution.rs:134),
whose stack stages
[`begin_after_native_inactive_prefix`](/workdir/openwepp-experiments/b01-wb14-cadence/current-context-capture-20260913/crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/frozen_litter_v3_adoption.rs:801).
That constructor leaves the represented prefix outside physical-child numbering,
so the first physical child remains ordinal zero. Covered construction initializes
the stack prefix to `None` and never calls this setter; later snow-free successors
also pass `None` because the snow-free receipt list is no longer empty. Any
successfully accepted physical child increments the ordinal, excluding a prior
accepted-child carryover. Thus the prior review's route ambiguity is resolved for
this caller. This is source plus retained-operand attribution, not an observed
stack trace.

The lineage joins above authenticate the prefix **to this child binding**. They do
not independently authenticate the complete represented-snow prefix history:
the selected pair does not contain the ordered owner-join receipts and accepted
terminal-event groups from which `coupled_receipt_sha256` and the prefix proof are
derived. No full-prefix, native, or scientific acceptance follows.

## Residual risk and missing tests

- The complete current pre-child context remains absent on the attributed route;
  parent transaction checkpoint, coupled-clock owners, reduction, ledger,
  provisional receipt, transformed successor support, terminal parcels, native
  constructor custody, and deferred soil custody are still unavailable.
- No independent reconstruction of the prefix receipt/event history exists in
  this evidence. A self-contained prefix and matching binding are insufficient
  for complete-prefix authentication.
- No Rust behavior ran here. Missing focused coverage includes rejection of the
  outer/inner day-index split at parent construction and parity of pre-child
  recording across covered and first snow-free provisional paths.
- Native reader/restoration, conservation, isolation, restart, source-quality,
  comparator and science acceptance remain unmet and outside this review.

**Verdict: HOLD / NOT ACCEPTED for the runtime checkpoint.** The limited decoded
route assessment has no remaining attribution blocker: transaction 255 reached
the first snow-free successor ingress after the represented-snow prefix, and the
retained values establish the exact fail-closed day/cursor mismatch. No hook,
runtime, native, or science correction was verified.
