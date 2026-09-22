# COLD-CANOPY-M1 complete-parent controls — correctness review 06

**Verdict: narrow HOLD.** Test06 correctly binds staged persistent lineage and installs the sealed finalized owner set, but it permits a BGC finalization mutation without proving that only BGC logical lineage changed. One test-side comparison remains before bounded body authoring is released.

**Evidence class: Static.** Same-reviewer focused verification of frozen canonical 760-entry source aggregate `075a936eb7b5d963d23709e509af648a97d96459b3b8bb345ac3f3b629407b18`. No source/configuration was edited and no command was run.

## Finding

1. **High — allowed BGC finalization is not constrained to its logical lineage field.** `m1_provider_parent_controls.rs:385-397` accepts mutation set `bgc + vegetation`, but `:398-420` checks physical equality only for the M1 vegetation owner and owners that must remain byte-identical. It never compares the pre/post-finalization BGC payloads. A body could alter BGC mineral layers or material receiver pools during the zero-duration logical parent transition and still pass. The canonical BGC state consists of `layers`, `receivers`, and `last_transaction_id` (`openwepp-biogeochemistry/src/lib.rs:69-74`), and the existing parent finalizer clones the prior BGC state and changes only `last_transaction_id` (`v9_real_consumer_shadow.rs:3491-3496`). Decode `prior["bgc"]` and `finalized_owners["bgc"]`, remove only `last_transaction_id`, and require the remaining JSON to be exactly equal. When the BGC owner changed, also require its finalized `last_transaction_id` to equal the finalized parent/M1 revision. This is the requested physical-pool conservation assertion and introduces no new policy.

## Verified correction

- `:143-148` now decodes the actual staged canonical M1 owner on every support and proves its persistent revision and all stratum transaction lineages remain equal to the admitted beginning. The separate counter getter can no longer hide per-support lineage mutation.
- `:374-405` compares actual installed bytes to the sealed finalized seven-owner set, requires vegetation to change, restricts the mutation set to the canonical `vegetation` or `bgc + vegetation` alternatives, and retains exact bytes for snow, hydrology, LSE, soil thermal, and surface liquid.
- `:407-420` requires exact parent/stratum `+1` and proves all non-lineage fields in the M1 vegetation JSON are identical to the final staged physical ending.

All earlier controls05 error, positive receiver, and late-rejection fixes remain accepted and were not reopened. After adding the single BGC comparison, the expected-red controls are adequate for bounded body authoring, subject to later source tracing of every private accessor and canonical failure.

## Reviewed identities

- parent controls: `m1_provider_parent_controls.rs` SHA-256 `a7ed819e5ca6d24f216b85a0572a4b41466e1ff10bc9bcaf297b2e7806ff3f74`
- canonical source aggregate: `075a936eb7b5d963d23709e509af648a97d96459b3b8bb345ac3f3b629407b18`
- source recorder: `provider-parent-parent-controls-review-06-source.json` SHA-256 `d1e070cd5e0350cf98138a4cdacadc7bc29dc859813e1895d396ce499dac2619`
- observer-relative patch: `provider-parent-parent-controls-review-06-from-observer-cut02.patch` SHA-256 `ebd6532ef2ab931bd360d8bee8527715e62fd50f4a7d26df3d6bb029c6ebee25`
