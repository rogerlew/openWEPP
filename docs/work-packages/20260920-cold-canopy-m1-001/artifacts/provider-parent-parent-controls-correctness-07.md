# COLD-CANOPY-M1 complete-parent controls — correctness review 07

**Verdict: PASS for bounded complete-parent body authoring.** The sole correctness06 BGC finalization gap is resolved. This releases the expected-red parent controls for implementation; it is not runtime or physical acceptance.

**Evidence class: Static.** Same-reviewer delta-only verification of frozen canonical 760-entry source aggregate `9ad8430be0c4a0763a0d2945bcbed3cac4747d8969bf4391516e7e15f769f076`. No source/configuration was edited and no command was run.

## Findings

No blocking finding in the assigned fix scope.

## Fix verification

- `m1_provider_parent_controls.rs:93-99` adds a test-side canonical BGC projection that removes only `last_transaction_id`.
- `:429-434` parses the actual prior and sealed-finalized BGC owner bytes and requires exact equality after removing only that lineage field. Mineral-layer inventories, material receiver pools, map membership, numerical encodings, and every other serialized BGC field must therefore remain identical through the zero-duration logical finalization.
- `:435-438` separately requires the finalized BGC transaction ID to equal the finalized M1 parent revision. Thus the allowed `bgc + vegetation` mutation can only express the existing canonical lineage advance.

The staged M1/stratum lineage checks, sealed seven-owner installation, exact parent `+1`, vegetation non-lineage equality, canonical typed refusals, resource/material controls, and positive late rejection accepted in correctness05/06 remain unchanged and are reused.

The future body must drive the existing canonical V11 finalization handoff and ownership-transfer event. Test-private accessors and wrapped failures remain subject to implementation source tracing; adapter-manufactured owner sets or expected errors are not accepted.

## Residual validation

The parent body is absent, so this review ran no compile, control, or physical command. Compilation, focused nonphysical parent controls, actual producer tracing, complete parent progression, cycles, restart, cost and broader gates remain pending.

## Reviewed identities

- parent controls: `m1_provider_parent_controls.rs` SHA-256 `7e5c9fbd587a5a1a1945c1933ea9098bdc83d7f0c4743447abcb3e9a670cb676`
- canonical source aggregate: `9ad8430be0c4a0763a0d2945bcbed3cac4747d8969bf4391516e7e15f769f076`
- source recorder: `provider-parent-parent-controls-review-07-source.json` SHA-256 `b94af370f588bd2e8ffcb5a8cf38ae0261876da72d4844c7d352426adccd8498`
- observer-relative patch: `provider-parent-parent-controls-review-07-from-observer-cut02.patch` SHA-256 `6428f1c8406f8c012d447245ac6213074f8cda1b09227f5f3635aee48d1e9872`
