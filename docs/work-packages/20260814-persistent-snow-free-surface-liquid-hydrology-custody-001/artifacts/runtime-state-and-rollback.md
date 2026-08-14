# Runtime State And Rollback

Evidence class: `Static + Ran`

The default production `DirectRunFrame` contains no surface-liquid owner. An
explicit shadow caller may attach a strictly validated
`DirectSurfaceLiquidOwnedState`; production construction and dispatch remain
unchanged.

The state digest covers owner, run, configuration, every typed OFE/tile/surface
store, every 1800-second WB14 continuation value, and accepted transaction
lineage. Strict JSON round-trip and digest-sensitivity tests pass.

Resource authorization reads one immutable beginning snapshot. Candidate
construction occurs in clones, debits finalized use only, credits signed
condensation, and emits excess as typed ingress. Timed ingress builds its
candidate from the post-resource clone. Failure in authorization, final-use
validation, condensation validation, WB14 continuation, routing, or cross-owner
validation leaves the complete beginning state byte-identical.

The unified shadow bridge installs the validated soil and surface-liquid ending
states only into the returned cloned frame. It never mutates the production
frame supplied by the caller. Its beginning snapshot digest binds the real
soil-owner fingerprint and surface configuration/state digests; a stale digest
fails before the final constitutive callback.
