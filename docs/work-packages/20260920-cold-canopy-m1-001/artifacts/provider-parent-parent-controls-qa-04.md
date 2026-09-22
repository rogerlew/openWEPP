# Provider-parent expected-red controls QA — 04

**Reviewer:** `/root/provider_qa` (secondary QA)  
**Evidence:** Static inspection of frozen 760-entry cut, aggregate SHA-256 `b5184360f6e397a2b25bc1c0d05788bffdaedd99f8d04b607f71055413559ef3`; reviewed controls SHA-256 `3780aa1d0fb503ff5974c6a7cc3b042f27db673252ab1e0d201fdedf29cfe603`. No parent body, compilation, control execution, or physical run was supplied.

## Findings

No blocking QA finding in the revised expected-red control body.

## Verified fixes

- The positive path advances the actual caller support-by-support and observes actual caller staging, cursor, parent-only counters, installed owners, cleared clock/staging, parent receipt, and second-commit refusal. It no longer relies on a driver-report snapshot for post-commit state.
- All 30 supports bind to the admitted provider record's ordinal, interval, five override bits, GSI receipt, forcing receipt, and actual accepted slab. Reservoir mass and enthalpy closure remain independently reconstructed in test code.
- Resource custody uses concrete existing V11 types: `V11ResourceDebit`, `V11AdmittedResourceFlux`, `V11SharedResourceOwnerTransition`, and `V11CompleteOwnerCandidate`. The control cross-checks raw candidate receipts against admitted receipts, transition receipt sets, actual slab/parent/segment/support bindings, and owner-level beginning/debit/flux/ending arithmetic. Empty typed sets are preserved as lawful zero cases rather than being made to pass through a generic boolean.
- Material custody uses actual `MaterialTransfer` and biogeochemistry receiver pools, reconstructing each receiver's carbon, nitrogen, and dry-matter delta from raw transfers. This removes the former opaque resource/material summary predicates.
- Positive native receiver handling now compares the concrete receipt to the caller's actual consumption receipt and rejects replay. The zero native-source path is separately asserted as an empty source-operand zero-total case.
- The late-rejection control accepts only `M1ParentExecutionError::LateOwningReceiver`. It intentionally permits earlier injected zero-transfer supports as a real prefix, then requires a fresh positive native candidate and unchanged real-caller snapshot at the late seam. Exhaustion now panics rather than accepting `NoPositiveNativeReceiverInParent`.

## Non-blocking follow-up

The module remains test-only under `#[cfg(test)]`. Its calls name the intended parent body, so these are implementation-contract controls and have no compile or runtime result yet. Exact source/build/run custody and all parent, cycle, restart, cost, Critical, and authority gates remain separate.

## QA disposition

**PASS — release this test-only expected-red control body for the bounded parent implementation.** This is not a parent-body, runtime, or physical-launch clearance.
