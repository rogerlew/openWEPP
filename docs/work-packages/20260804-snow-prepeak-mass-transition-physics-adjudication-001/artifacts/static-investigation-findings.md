# Static Investigation Findings

Status: `complete / four read-only audits`

Evidence mode: `Static`

## Authoritative Order And Consumer

The current order is CoE hourly accumulation/melt and liquid-capacity handling,
then density/layer projection, then the Stage-3 thermal solve, then Stage-3
liquid disposition. Positive applied CoE melt reduces the authoritative pack
before any Stage-3 cold-content constraint. Negative applied CoE values do not
restore SWE and are discarded by positive-parts daily redistribution.

The runner publishes the full upstream `liquid_handoff_m` to the hyetograph and
liquid-input path. It does not consume `stage3_routed_liquid_m`. Runtime SWE is
the post-CoE density outcome minus separately enabled Stage-3 sublimation; the
selected sublimation model is disabled. Stage-3 retained and refrozen amounts
therefore do not delay authoritative SWE loss or reduce current downstream
liquid publication.

Relevant sources include:

- `infiltration_reconciliation.rs:1392-1420`, `1491-1545`, `1692-1779`, and
  `1955-2053` for the CoE gate, pack mutation, capacity store, and finalization;
- `runoff_reconciliation.rs:327-374`, `655-900`, and `1242-1275` for density,
  Stage-3 ordering, cold content, and liquid routing;
- `snow_mass_transition.rs:183-186` and `230-244` for linked-ledger guards; and
- `00c_day_input_builder_impl.rs:320-351` for the real hydrologic consumer.

## Material Semantic Corrections

- `stage3_cold_content_before_j_m2` is post-CoE and post-density-projection
  state, not pre-CoE or pre-export cold content.
- `stage3_retained_liquid_delta_m` records newly retained incoming liquid and
  is nonnegative in the producer. It is not the complete day-over-day layer
  liquid-store delta; earlier layer trimming can scale or remove stored liquid.
- Positive Stage-3 retention plus refreeze is capture throughput inside a
  diagnostic arm, not persistent endpoint storage and not causal peak-SWE
  evidence.
- The two ledger boundaries are nested. Solid loss and released rain form
  Stage-3 incoming; routed, retained, refrozen, and residual values partition
  that incoming. They are not one additive water-budget composition.

## Confirmed Interactions

The CoE and Stage-3 paths use the same one-percent pore-volume capacity formula
but own separate stores. Stage 3 receives only CoE-released excess; naively
making Stage-3 routing authoritative would offer a second capacity reservoir
unless storage ownership is redesigned.

Density wet-compaction forcing is also constructed from
`snowpack_state_loss + routed_melt_m`, while routed melt already includes state
loss plus released rain. This yields `2 * state_loss + released_rain` in the
current diagnostic forcing. It changes density/layer geometry rather than
direct aggregate SWE and is not a demonstrated second SWE debit.

## Authority Limits

The Rust CoE equations faithfully transcribe the post-2007 legacy formula
family; a simple transcription defect is not the leading hypothesis. `A/B/C/D`
remain empirical melt-depth terms with mixed drivers, not identifiable heat
fluxes. Daily-local signed cancellation is closer to comparator chronology
than seasonal netting but is still feedback-free algebra, not refreeze physics.
Legacy agreement remains a provenance flag, not correctness authority.
