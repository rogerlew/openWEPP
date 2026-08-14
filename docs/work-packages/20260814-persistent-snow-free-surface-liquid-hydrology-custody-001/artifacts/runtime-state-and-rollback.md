# Runtime State And Rollback

Evidence class: `Static + Ran`

The default production `DirectRunFrame` contains no surface-liquid owner. An
explicit shadow caller may attach a strictly validated
`DirectSurfaceLiquidOwnedState`; production construction and dispatch remain
unchanged.

The state digest covers owner, run, configuration, every typed OFE/tile/surface
store, every 1800-second WB14 continuation value, and accepted transaction
lineage. Canonical-byte round-trip and field-level digest-sensitivity tests
pass; raw root serde persistence is unavailable.

Resource authorization reads one immutable beginning snapshot. Candidate
construction occurs in clones, debits finalized use only, credits signed
condensation, and emits excess as typed ingress. Timed ingress builds its
candidate from the post-resource clone. Failure in authorization, final-use
validation, condensation validation, WB14 continuation, routing, or cross-owner
validation leaves the complete beginning state byte-identical.

The unified shadow bridge installs validated soil, surface-liquid, LSE tile, and
soil-thermal ending candidates only into the returned owner envelope. Actual
infiltration uses the shared production transition on the exact cloned lane;
retained and infiltrated enthalpy credit typed receivers. It never mutates the
production frame supplied by the caller. Its beginning snapshot SHA-256 binds
the complete canonical soil-owner bytes plus surface configuration/state; a
stale digest fails before the final constitutive callback.
