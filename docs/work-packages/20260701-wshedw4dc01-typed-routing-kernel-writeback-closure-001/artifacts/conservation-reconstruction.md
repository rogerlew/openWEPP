# Conservation Reconstruction

Status: `PASS-LIMITED`

Evidence class: `Static` and `Ran`.

Operand lineage:

- Direct channel state publishes:
  `runoff_volume_m3`, `peak_discharge_m3_s`, `duration_seconds`, and routed
  sediment state from WS10/WS11/WS18/WS20 helpers.
- Direct impoundment state publishes:
  `outflow_volume_m3`, `outflow_rate_m3_s`, `duration_seconds`, and `hnext_m`
  from WS12 helpers.
- Typed publication consumes only routed frame state and hillslope contribution
  metadata; it no longer harvests compatibility writeback fields.
- Typed publication fails closed when a dispatched channel or impoundment lacks
  routed state.

Rejected aliases and wrong formulas:

- Rejected shortcut formulas remain absent from Rust sources:
  `incoming_peak + control.qinf` and
  `incoming_peak * self.routing_globals.dtchr_seconds`.
- Public CLI and direct kernel do not use old symbol-map request/writeback
  surfaces.

Magnitude and closure evidence:

- Public generated-mode test decodes `ebe_pw0.parquet` and requires positive
  `peak_runoff` and `runoff_volume`, proving generated HBP pass payloads reach
  typed watershed publication.
- Worker-pool row-equivalence test decodes all watershed output Parquet rows
  and proves jobs=1 and jobs=N output row order/content identity.
- Focused WS10, WS11/WS20, and WS12 integration contracts passed after the
  direct implementation and shared WS20 refactor.

Scoped limitation:

- This artifact records public-output magnitude/identity sanity evidence and
  physics contract regression evidence.
- It does not claim a carnivorous-adobo output conservation audit because that
  committed fixture is parser/input substrate only for the current CLI surface.
