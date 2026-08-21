# Implementation Review A — Time And V10 Compatibility

## LSE positive-support production addendum

Verdict: **HOLD**.

### Passing surfaces

- `LseSupportAdmissibilityReceiptV1::admit` runs before candidate cloning,
  interval-envelope construction, or the LSE nonlinear path. A below-minimum
  result therefore fails closed before Newton and does not mutate parent or
  live staged owners.
- Parent, segment, slab, ordinal, absolute support, exact derived duration
  bits, model/configuration/beginning-state identity, tolerance/numerical
  policy, minimum, and domain-prefixed canonical digest are bound.
- The 600 ms minimum is a physical adopter guard; coupled-time 1 ns structural
  identity remains valid and no duration is rounded, replaced, or retried.
- No V10 vegetation source or persisted/coupled-time wire is changed.
- Focused evidence: formatting and package checks PASS; LSE support unit test,
  exact-minimum actual-stack test, and below-minimum no-mutation test PASS.
  Broad Clippy reached the orchestrator tests but is blocked by the unrelated
  pre-existing `float_cmp` assertion in
  `direct_runtime/surface_liquid_wb14.rs:406`.

### Release-blocking finding `IMPL-LSE-A-001`

The sealed receipt is not admitted into the accepted parent/slab chronology.
`execute_imported_v10_stack` stores only one ephemeral
`last_support_receipt: Option<_>` on `DirectV11RealConsumerStack`, overwriting
it on every successful segment. `V11ImportedV10SegmentOutput`, the vegetation
parent candidate/checkpoint, and restart/publication chronology contain no LSE
support receipt. Consequently a multi-slab parent cannot retain every admitted
receipt, finalization cannot authenticate exact one receipt per accepted slab,
and restart cannot prove or replay-protect the accepted support-policy prefix
required by `SC-LANDSURFACEENERGY-001@6` and the V11 transaction amendment.
The current tests inspect the executor's last diagnostic receipt immediately;
they do not prove custody after segment acceptance, parent finalization, or
restore.

Required closure: add the typed support receipt to the imported segment output
and accepted V11 segment record; require exact parent/segment/slab/support and
configuration/beginning-LSE-state joins during `accept_segment`; retain the
ordered receipts in the parent candidate/checkpoint; reject missing,
duplicate, reordered, substituted, or digest-invalid receipts atomically; and
add unequal multi-slab plus checkpoint/restore tests. The diagnostic getter may
remain but cannot be the authority carrier.

## Superseding implementation rerun

`IMPL-LSE-A-001` is closed.

The mandatory sealed `V11LseSupportReceiptEnvelope` now flows through imported
segment output, accepted segment candidate, accepted segment checkpoint,
parent candidate, and parent checkpoint/restore chronology. Its closed wire
rejects unknown fields; reserializes exact declaration-order canonical bytes;
authenticates the raw bytes and domain-prefixed blank-receipt digest; freezes
model, numerical/tolerance policy, and minimum support; and binds parent,
segment, slab, ordinal, absolute support, duration bits, LSE configuration,
beginning LSE state, and beginning soil-thermal state.

Execution and acceptance compare the receipt against the current staged LSE
and soil-thermal owners. Restore begins from the complete parent-beginning
owner set and advances the predecessor owner set from each accepted segment's
ending owners. Missing, mutated, digest-valid reframed, duplicate, substituted,
and replayed receipts reject without parent mutation.

Evidence: vegetation `272/272` PASS; vegetation Clippy with `-D warnings` PASS;
four-crate LSE/vegetation/orchestrator/restart check PASS; formatting and diff
hygiene PASS.

**Superseding verdict: PASS.** No Implementation Review A finding remains
open.
