# Provider-parent expected-red controls QA — 02

**Reviewer:** `/root/provider_qa` (secondary QA)  
**Evidence:** Static inspection of frozen 760-entry cut, aggregate SHA-256 `e89c3053500428381c70c825a45de024c2899d71cf276a0463f25349baf49eed`. No parent body, compilation, control execution, or physical run was supplied.

## Findings

### HOLD — resource/material receipt loop can pass with no receipts

`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/m1_provider_parent_controls.rs:71-77` iterates the resource and material receipt collections but asserts neither a nonzero/expected count nor expected owner inventory, amount sums, or expected receipt identities. Empty collections therefore satisfy every assertion, and an arbitrary finite cross-owner receipt can satisfy the local shape checks.

Use the actual receipt types to construct expected per-support receipt inventory from the admitted record, native receiver result, and parent transaction. Assert the expected count, owner/source/destination roles, support/transaction/slab binding, identity, and independently summed material/resource quantities. This requires no invented production API: the control already accesses `support.resource_receipts()`, `support.material_receipts()`, and the accepted slab.

### HOLD — post-commit and capability assertions remain driver-reported booleans

`m1_provider_parent_controls.rs:95-100` accepts `post_commit_clock_is_cleared()`, `post_commit_staging_is_cleared()`, and driver-returned `post_commit_caller_owner_bytes()`. Lines 106-116 similarly accept receiver-disposition and capability booleans. These are improvements in naming but still let the driver report its own state rather than requiring a control-side observation of the actual caller/clock and capability/receipt objects.

Capture actual caller state after the positive execution through the existing real-caller snapshot/accessor path, and compare its owner bytes, clock/staging absence, and publication/consumption state with the expected committed output. For a positive receiver, assert its concrete capability/receipt identity and consuming ledger entry rather than only `capability_was_minted()` and `capability_was_consumed()`.

### HOLD — late-owning control accepts a non-late alternative

`m1_provider_parent_controls.rs:127-140` treats `NoPositiveNativeReceiverInParent` as a passing branch in a test whose stated purpose is a late owning rejection after positive native work. That branch has no positive receipt and establishes that the required late path was not reached.

Keep this test strict: it must accept only `M1ParentExecutionError::LateOwningReceiver`, prove the positive receiver receipt and then the unchanged real caller state. If the no-positive condition is an intended case, cover it with a separately named discriminator test; it cannot satisfy the late-owning requirement.

## Verified corrections

The prior all-record gap is corrected: lines 42-60 enumerate all 30 supports and compare ordinal, provider-record interval, expected 60-second interval, all applied override bits, GSI receipt, parent forcing receipt, slab ordinal/support, and owner/revision shape. The prior delegated reservoir-balance call is replaced at lines 61-69 by control-side mass and enthalpy reconstruction using explicit operands and constants, with residual and closure bounds.

## Non-blocking follow-up

The module remains correctly test-only through `#[cfg(test)]` wiring in `land_surface_energy_shadow/mod.rs:103`. Its named body APIs are expected-red until implementation exists. No source/build/run custody or runtime result is implied by this static review.

## QA disposition

**HOLD — retain parent-body implementation and runtime release.** The two verified corrections improve test quality, but receipt/resource semantics, actual post-commit observation, and the strict late-owning discriminator remain required before these controls can serve as the parent contract.
