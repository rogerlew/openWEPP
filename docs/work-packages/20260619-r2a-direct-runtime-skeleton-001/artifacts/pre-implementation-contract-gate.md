# R2A Pre-Implementation Contract Gate

Status: complete.
Evidence mode: Static.

Before production edits, record:

- required reading complete;
- owned-file manifest complete;
- source inventory and forbidden API list complete;
- no `SC-*`, output, publication, or process-physics authority change required;
- direct skeleton default-disabled regression plan complete;
- line-count implications understood.

Do not edit Rust before this gate is complete.

## Gate Evidence

Static:

- Required reading is complete; see `required-reading-map.md`.
- Owned file manifest is initialized; see `owned-file-manifest.md`.
- Source inventory confirms the existing compatibility path still owns
  `HillslopeKernelRequest`, `KernelWritebackPayload`,
  `HillslopeWritebackSurface`, registry, hot-table, indexed-surface,
  dense-refresh, and dirty-flush mechanisms in scheduler/runner modules.
- R2A implementation target is a separate direct-runtime namespace with a
  no-op/shadow skeleton only. No hydrology, erosion, plant, decomposition,
  frost, snow, publication, schema, unit, metadata, conservation, or canonical
  `SC-*` authority change is required.
- Default-disabled regression plan is to keep compatibility execution as the
  default `HillslopeRuntimeSelection::Compatibility`, with no direct-frame or
  direct-executor construction unless explicit skeleton selection is requested.
- Existing line-count risk: `scheduler.rs` is already above the 3000-line
  threshold and will not be touched for R2A. The runner setup file is above the
  2000-line WARN threshold and may receive a small setup-selection edit that
  must be documented in line-count governance.

Disposition: PASS. Rust edits may proceed inside the owned write set.
