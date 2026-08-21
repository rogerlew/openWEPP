Status: complete
Evidence mode: Static

Static:

- `direct_runtime/snow_stage3_shadow.rs` owns the persistent default-off
  attachment, derives carrier/event inputs from sealed receipts plus live
  `DirectDayFrame` operands, and stages one complete owner candidate.
- The exact terminal parcel is assembled from retained liquid, post-winter
  rain, and released melt in kg/m²; the handoff runtime enforces the INV-010
  debit/credit identity.
- The remainder is exactly `parent_duration - event_elapsed`, bounded to the
  admitted WB14 interval, and is passed through the real surface-liquid
  authorization/resource/ingress path.
- `03_executor.rs` advances the attachment after ordinary day spans and
  commits it at the day-frame boundary. `05_runner_execution_and_outputs.rs`
  consumes only the immutable post-transaction batch.
- `hydrology_restart.rs` projects/restores the attachment additively with the
  persisted surface-liquid owner.
