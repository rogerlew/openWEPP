# COLD-CANOPY-M1 complete-parent controls — correctness review 04

**Verdict: narrow HOLD.** Controls02's resource/material, direct-caller, all-30-support, independent reservoir, distinct zero-parent, and same-caller late-rollback defects are corrected sufficiently for body authoring. Two generic `is_err()` checks still leave once-only completion and positive-capability custody satisfiable by an unrelated error, so the corrected control cut is not yet released.

**Evidence class: Static.** Independent focused review of frozen canonical 760-entry source aggregate `b5184360f6e397a2b25bc1c0d05788bffdaedd99f8d04b607f71055413559ef3`. No source/configuration was edited and no command was run.

## Finding

1. **High — the two consumption discriminators accept any error and the rejected positive candidate is not concretely authenticated.** `m1_provider_parent_controls.rs:334` accepts any second-completion error, so wrong phase, corrupt state, or a generic receiver failure passes without proving that the first `complete_parent` consumed the parent exactly once. `:360-371` does require `LateOwningReceiver` and restores the actual caller snapshot, but it reduces the retained real candidate to `transaction_id > 0` and again accepts any replay error. A replay can therefore fail because the candidate belongs to the wrong transaction/support or because caller state is invalid, without proving that the fresh positive capability was consumed by the late owning rejection. Require an exact typed already-consumed variant from the second completion. On the late path, inspect the already-existing `rejected_candidate().contribution_join().contributors()` APIs: require a nonempty set, positive finite tile mass, finite enthalpy, terminal receiver kind/source parcel, and transaction equality with the retained actual candidate/current attempted support; also assert the exact late stage/reason. Require replay to return the exact consumed/replay refusal variant. These are assertions over existing receiver evidence and error taxonomy, not new physics or a new trace framework.

## Verified controls02 corrections

- `:68-112` makes the test own the actual-caller support loop, binds every provider record/GSI/forcing/slab, checks real staging/cursor/counters and authenticates each accepted slab against seven actual ending `OwnerState`s.
- `:116-230` exposes the canonical V11 debit, flux, transition and complete-owner candidate types and reconstructs owner inventory arithmetic; `:231-282` reconstructs BGC receiver carbon, nitrogen and dry-matter deltas from actual `MaterialTransfer`s. Empty domains are joined to their typed raw candidate/source operands instead of passing through a generic predicate.
- `:283-310` separates concrete positive receipt/caller-consumption evidence from the raw zero-source census. `:317-333` inspects the actual one-time parent installation, ending owner bytes, parent counters, cleared clock/staging and ordered slab IDs.
- `:337-383` advances an authentic zero-transfer prefix on the same caller and permits only a fresh `LateOwningReceiver`; absence of a positive support now fails the test. The actual pre/post-injection caller snapshot comparison fixes the prior disposable-clone loophole.

These corrections release the corresponding resource/material reconstruction, provider bridge, reservoir closure, actual caller staging/commit observation, and rollback-shape subscopes. During body review, every new raw/test-private accessor still must be traced to the actual V11/M1 producer; its name alone is not evidence.

## Residual validation

The file is intentionally expected-red because its parent body is absent. After the narrow discriminator fix, compilation and the focused nonphysical parent-control execution remain required. Complete-parent implementation, physical progression, both cycles, restart, cost, and broader gates remain outside this static test review.

## Reviewed identities

- parent controls: `m1_provider_parent_controls.rs` SHA-256 `3780aa1d0fb503ff5974c6a7cc3b042f27db673252ab1e0d201fdedf29cfe603`
- canonical source aggregate: `b5184360f6e397a2b25bc1c0d05788bffdaedd99f8d04b607f71055413559ef3`
- source recorder: `provider-parent-parent-controls-review-04-source.json` SHA-256 `79a36a818d33eaec5da6a208b5cf6af69760a6f652fb7200a4b8d0136bc1e0eb`
- observer-relative patch: `provider-parent-parent-controls-review-04-from-observer-cut02.patch` SHA-256 `27ee44984142f57ea98f6dc3bbfe61b81e050005446b518f2704de3a129bcb15`
