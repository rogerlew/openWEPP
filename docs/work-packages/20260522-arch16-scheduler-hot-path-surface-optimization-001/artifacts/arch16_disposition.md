# ARCH16 Disposition

Static: implementation and artifact review complete.
Ran: required ARCH16 gates pass.
Status: `GO-WITH-AMENDMENTS`.

## Disposition Summary

- `CRF-003` (scheduler hot-path clone/allocation pressure): closed in ARCH16 scope.
- Full-map state/flux clone calls were removed from hillslope and watershed kernel request construction paths.
- Deterministic scheduling and typed writeback/status routing remained intact.
- No unresolved fmt/clippy/test/deny failures remain.

## Amendments / Follow-On

- Request trait signatures now use borrowed request lifetimes; downstream
  out-of-tree kernel implementers must migrate signatures accordingly.
- Watershed dependency-node label formatting (`TopologyNodeKey -> String`) is
  still performed per dispatch call and can be optimized further in a
  follow-on if needed, but is not a correctness blocker.
