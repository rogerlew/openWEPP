# Provider-parent expected-red controls QA — 01

**Reviewer:** `/root/provider_qa` (secondary QA)  
**Evidence:** Static inspection only. The reviewed initial test-only file was SHA-256 `f6b289de0660efe99308d54ac01cd214b04a22ec505dba6b5be064974250ff52`; no canonical frozen source snapshot, implementation body, compilation, test execution, or physical run was then supplied.

**Subsequent custody note:** parent-controls review cut01 is now recorded as 760 entries, aggregate SHA-256 `723cd1a81e42cbd8f297b99dba63f6263187d2ac1593b8f6ad1d4ed02947a21f`. This note identifies the later cut only; it does not extend this review or alter its findings.

## Findings

### HOLD — delegated balance assertion is not an independent oracle

`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/m1_provider_parent_controls.rs:88` calls `execution.independently_reconstructed_balance()` and accepts its result. The test contains no independently reconstructed mass, energy, material, or resource operands. A method on the production execution result can share the implementation’s accounting or closure mistake, so this does not establish the required independent balance evidence.

The test must reconstruct its operands from frozen provider records, beginning/ending owners, receiver outputs, and receipt values in the control module, then compare independently computed balances to the execution result.

### HOLD — receipt/consumption checks are opaque booleans

`m1_provider_parent_controls.rs:81-82` accepts `resource_and_material_receipts_are_bound()` and `complete_parent_was_consumed_once()` as booleans. Similarly, the late-rejection test accepts producer-side booleans at lines 100-102. These assertions do not inspect receipt identities, resource/material quantities, owner/boundary identifiers, consumption ledger entries, or the late owning error path. They can pass if an implementation returns `true` without proving the relevant semantics.

The controls need concrete expected receipt/ledger reconstruction and assertions over the actual emitted identities, quantities, owner bindings, and once-only consumption state. The negative path should assert the typed rejection and prove that caller/parent state, receipts, and publication remain unchanged after the late failure.

### HOLD — thirty-record coverage is incomplete

`m1_provider_parent_controls.rs:41-73` checks only records 0 and 1, cardinality, adjacency, and generic per-support predicates. It never compares all 30 selected frozen records to the provider’s expected ordinal, interval, and five override bit values. The first rain breakpoint check is useful but cannot detect a duplicated, substituted, or otherwise wrong later record.

The positive control must enumerate all 30 records for parent zero and compare each expected ordinal, start/end interval, and all override binary64 bits against the actual support input before the solver/receiver path consumes it.

## Non-blocking follow-up

The test-only module is appropriately wired under `#[cfg(test)]` in `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs:103`. Its imports deliberately name currently absent parent-driver APIs, so compile failure is expected until the implementation body is supplied; no build or runtime disposition can be inferred now.

## QA disposition

**HOLD — do not release these expected-red controls as the parent implementation contract yet.** Fix the three concrete oracle/coverage gaps, freeze a source cut, and obtain independent re-review before authoring or launching the parent body. Provider-only evidence remains unchanged and does not clear parent, cycle, balance, restart, cost, or final gates.
