# Worker Handoff

Status: complete
Evidence mode: static
Date: 2026-05-27

## Static

### What was completed
- Current watershed execution/output surfaces inventoried.
- Baseline watershed routing/orchestration routine map authored.
- Nine concrete implementation gaps classified with severity and write targets.
- Dependency-ordered queue `WSHED02..WSHED09` published.

### Immediate next action
- Start `WSHED02` to align canonical contract authority and explicit gap rows
  before any production code migration.

### Key watch-items for next worker
- Do not claim WS11/WS12 parity from current synthetic tests alone.
- Remove manual WS12 coefficient test seeding by wiring parser/runtime seam.
- Treat `OWSOUT-E-004` replacement as a late-stage task after routing/physics
  migration surfaces are executable.
- Keep `SC-SED-001` / `SC-ROUTE-001` / audit text synchronized when closures
  land, to avoid contract-doc lag.

### Baseline provenance correction carried forward
- Use `/workdir/wepp-forest_260430_baseline/src/detach.for` as the channel
  detachment authority surface (not `chndet.for`).
