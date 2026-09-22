**Static QA — HOLD for circular receipt construction.**

**High — [SC-VEGETATION-001.md](/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md:3382):** `support_receipt_sha256` is embedded in `SourceJoinV1`, therefore in `payload_content_sha256`, which derives `run_identity`. The required current-GSI receipt must then bind that same run. This creates `payload → run → support receipt → payload`, so no known answer can be minted.

Split static source descriptors from derived execution evidence:

1. Mint `run_identity` from the static canonical payload without a support-receipt hash.
2. Create and validate the current-GSI support receipt bound to that run/cycle/support/scalar.
3. Bind that derived receipt hash in `forcing_receipt` and parent admission evidence, outside the payload/run preimage.

The revised cut otherwise fixes the earlier integrity gaps: full payload hash covers run identity, full projection hash covers forcing receipt, typed adapters replace raw Rust-symbol parsing, source/adapter dependencies are hash-bound, the schedule is bounded, and `VEG-E-144` now precedes parent/staging.

**Medium — [SC-VEGETATION-001.md](/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md:3420):** define `full_projection_content_sha256` explicitly as the raw SHA-256 of canonical `ProjectionV1` bytes. It is used in the forcing frame but not defined.

The retained endpoint value `GSI=1.0` is correctly stated as insufficient correspondence evidence. A new derived provider receipt may bind the actual frozen source without inventing a physical input, but it must exist and validate before parent construction. No physics admission follows this review.
