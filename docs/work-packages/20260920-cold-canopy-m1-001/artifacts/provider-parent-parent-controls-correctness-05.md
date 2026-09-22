# COLD-CANOPY-M1 complete-parent controls — correctness review 05

**Verdict: HOLD for bounded complete-parent body authoring.** The two narrow controls04 error/candidate blockers are resolved, but final inspection found one pre-existing hard contradiction in the same cut: the success test requires the installed post-finalization owner bytes to equal the last pre-finalization slab bytes while also requiring the persistent parent revision to increment. Canonical parent finalization necessarily changes the sealed vegetation owner lineage. The earlier provisional PASS from this review is withdrawn.

**Evidence class: Static.** Same-reviewer verification of frozen canonical 760-entry source aggregate `f7a99af6e8980694a183eb181b8ec13b3cc1e31a8f89d0343d8cc3db54bb88b3`. No source/configuration was edited and no command was run.

## Findings

1. **High — the asserted final owner bytes omit the required parent-finalization owner transition.** `m1_provider_parent_controls.rs:327-340` requires `caller.beginning_owner_bytes() == prior`, where `prior` is the ending owner map staged by the last 60-second support, and simultaneously requires parent revision/publication `+1`. Canonical V11 finalization increments `last_parent_transaction_id`, normalizes the physical and every stratum/occupancy lineage, recomputes hashes, and emits a distinct finalized complete owner set (`openwepp-vegetation/src/v11.rs:1163-1220`). The existing production integration then installs that finalized owner set through an ownership-transfer event and explicitly requires the mutation set to be `vegetation` or `bgc + vegetation` (`snow_stage3_v11_real_parent_execution.rs:291-310`; `snow_stage3_v11_attachment_helpers.rs:48-204`). Thus exact equality to the last staged map would force omission of the required logical finalization, or force the counter accessor to report a revision absent from serialized owner state. Have `complete_parent` consume/return the actual sealed finalized complete-owner candidate (or expose its existing receipt), compare installed caller bytes to that candidate, assert the vegetation bytes differ from the last slab only by the authorized lineage/finalization projection, and retain byte equality for inactive/unmodified owners such as snow. Independently assert the physical M/H, pools, and other non-lineage operands are unchanged across this zero-duration logical transition.

2. **High — `parent_only_counters()` can hide per-support persistent-lineage increments.** `:67,97,331-337` treats an ambiguous getter as the parent-counter oracle. It could read only committed/live counters while the staged serialized M1 owner and stratum `last_transaction_id` advance on every support, allowing the original per-support increment defect to pass. At each support, inspect the actual staged typed M1/stratum lineage or parse the canonical staged vegetation bytes and prove the persistent parent transaction stays at the admitted beginning value. At finalization, inspect the actual installed typed owner/bytes and prove that persistent parent and every required stratum/occupancy lineage advance exactly once. The existing finalization handoff and ownership-transfer event are the canonical mechanism; no new domain tag or policy is needed.

## Fix verification

- `m1_provider_parent_controls.rs:341-346` now requires the exact second-completion result `M1ParentExecutionError::NoLiveParent(CoupledTimeError::ParentNotFinalizable)`. An unrelated phase, support, receiver, or state error can no longer satisfy once-only parent consumption.
- The accepted-positive path at `:300-305` and rejected-positive path at `:421-426` now require the actual canonical M1 refusal identity `VEG-E-143 / receiver / duplicate_transfer`; generic replay failure no longer passes.
- `:359-370` establishes support 0 as an authentic accepted prefix on the actual caller and binds its slab ID and accepted-until clock coordinate. Each injected attempt at `:371-382` is therefore fresh and occurs after real staged work.
- `:384-415` requires the exact late owning variant and canonical receiver identity `VEG-E-143 / late_receiver / injected_failure`. It retains the actual native candidate, binds its transaction to the current attempted support, and inspects a nonempty concrete contributor set for terminal kind, canonical source parcel ID, matching transaction, positive finite tile mass, and finite specific enthalpy. `:416-426` then proves unchanged actual caller state and the exact duplicate-transfer replay refusal.
- Exhausting supports without that concrete positive late event still fails at `:438`; a no-positive discriminator cannot satisfy the gate.

The future body must obtain these errors and candidate/contributor records by invoking the canonical coupled-time and M1 receiver seams. Constructing an adapter-side error with the expected variant or strings would not satisfy implementation review, even if these tests passed. Raw resource/material and caller accessors likewise remain subject to source-provenance review as already recorded in correctness04.

## Residual validation

The parent body is absent, so this review ran no compile, control, or physical command. After the two finalization/counter controls above are corrected, compilation, the focused nonphysical parent controls, actual producer tracing, complete parent progression, cycles, restart, cost and broader required gates remain pending.

## Reviewed identities

- parent controls: `m1_provider_parent_controls.rs` SHA-256 `9720e73b3361cfe8fd6bd911a58aa83587b45471ff58cb73ec33dcec561c4a51`
- canonical source aggregate: `f7a99af6e8980694a183eb181b8ec13b3cc1e31a8f89d0343d8cc3db54bb88b3`
- source recorder: `provider-parent-parent-controls-review-05-source.json` SHA-256 `bc8c074971598b1511e88e88c5734bb268086047af89bd3698a464de54f59a0a`
- observer-relative patch: `provider-parent-parent-controls-review-05-from-observer-cut02.patch` SHA-256 `ead935a86cd80f4169d9c43c58a883b0c803501336391ae9ada597441779403a`
