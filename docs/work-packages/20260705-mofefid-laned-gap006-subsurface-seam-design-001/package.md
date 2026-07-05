# MOFEFID LANE D — GAP-OFEROUTE-006 SUBSURFACE-COUPLING SEAM DESIGN

Status: `COMPLETE — MERGED TO MAIN `bce8da7a` 2026-07-05` (Codex 2-round: the structural-map carry-through in rev 4; docs-only) (Claude-executed; operator: "scaffold and execute
Lane D Papanicolaou contract-first", 2026-07-05). Branch:
`laned-gap006-subsurface-seam-design`. CONTRACT-FIRST: this package
resolves the DESIGN gap and amends the contract; implementation remains
the activation work behind `INV-OFEROUTE-012`.

## Lane D state correction (entry recon)

The campaign doc's "contract/fixture stages can start immediately"
framing is STALE: Lane D's contract-first phase largely executed in the
D01–D8 arc (SC-OFEROUTE-001 ratified rev 2; solver/cascade/infiltration
landed shadow-first; the D-val harness exists). The A/B disposition gate
is satisfied by the 2026-07-04 audit
(`docs/audits/20260704_mofe_effective_length_transport_capacity_audit.md`:
the MOFE blowup is MODEL-CLASS — equivalent-plane, no relief valve —
with the two porting cut-points checked). What remains contract-shaped:

1. **`GAP-OFEROUTE-006` (this package):** the subsurface-coupling seam
   design — the one design-open item in the activation gate
   (`INV-OFEROUTE-012`).
2. `INV-OFEROUTE-011` D-val: ZERO of four validation cases cleanly
   reproduce (D8 record) — dispositions pending, NOT this package.
3. Zone 1/2 taxonomy: deferred, NOT this package.

## The seam design (see `artifacts/seam-design.md`)

Grounded in the EXISTING subsurface surfaces (SC-SUBHYD-001): the
exfiltration source term is `ui_SCrunf` (the hourly top-layer
saturation-excess clip — WEPP's representation of return flow); the
inter-OFE subsurface carry `ui_LfCrf` STAYS subsurface; the outlet
`latqcc` baseflow-class export bypasses the router unchanged; hourly
lane required for activation; one closure identity over both gate
fixtures.

## Deliverables

- `artifacts/seam-design.md` — the D1–D5 design decisions + the two
  gate-fixture specifications.
- SC-OFEROUTE-001 amendment: `GAP-OFEROUTE-006` → design-RESOLVED;
  `INV-OFEROUTE-012` rewritten with the concrete seam bindings.
- No implementation, no fixture data, no runtime change (docs-only).
