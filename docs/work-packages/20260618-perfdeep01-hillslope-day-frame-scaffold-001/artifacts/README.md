# Artifacts

Status: scaffolded 2026-06-18 (pending Codex execution).

Required deliverables:

- `perfdeep01-frame-schema.md` - the `HillslopeDayFrame` field schema: every current scalar symbol → typed
  field; the SoA array families (hourly/layer/frost); the borrowed climate-forcing handling.
- `perfdeep01-roundtrip-identity.md` - seed/flush `f64::to_bits()` round-trip evidence on a real H2637
  OFE-day surface (the Stage-0 load-bearing identity gate).
- `perfdeep01-publication-operand-ledger.md` - **review Finding 2 closure**: every output/publication
  runtime-surface read (WB13/WAT/PASS assembly + ~5 HBP scalars + manifest provenance) mapped to a frame
  field/projection. Must be complete, not sampled.
- `perfdeep01-guard-tier-catalogue.md` - **review Finding 1 closure**: every writeback guard site classified
  static-bound vs runtime-derived-bound; the two-tier guard policy preserving message-id + diagnostic
  attribution semantics.
- `perfdeep01-contract-transition-map.md` - **review Finding 3 closure**: how the frame coexists with the
  `HillslopeKernel`/`KernelWritebackPayload` contract across crates during migration; the cutover interface.
- `perfdeep01-endpoint-rss.md` - H2637 `.hbp`/`wat`/`pass` identity + endpoint flat vs 669.97 s (frame is
  shadow/flagged-off; no production behavior change).
- `perfdeep01_disposition.md` - Stage-0 complete → Stage-1 (hydrology island) go, or blockers found.
