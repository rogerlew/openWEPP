# PERFDEEP06 No-Hot-Loop-Map Proof

Status: queued.
Evidence mode: not-run.

## Required Content

Prove statically which mechanisms must be absent from the migrated normal
success path:

- `BoundarySymbol`;
- `BoundaryValue`;
- `SymbolRegistry::id_of`;
- `HillslopeKernelRequest`;
- `KernelWritebackPayload`;
- `HillslopeWritebackSurface`;
- logical-surface fallback reads;
- dense/logical refresh and flush inside migrated phase execution;
- hot-loop `format!`, owned symbol cloning, collection rebuilds, and equivalent
  allocation helpers.

Classify any remaining logical/indexed use as I/O, replay, diagnostic, or
non-migrated-boundary only.

## Gate

This artifact is complete only when it can be used as the review checklist for
PERFDEEP07's direct-frame implementation.
