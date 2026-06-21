# R6E No-Compatibility Proof

Evidence mode: Static + Ran.

Status: partial; cutover remains fail-closed.

Forbidden direct publication authority sources remain:

- `SimulationOwnedWb13Row`;
- compatibility `HillslopeWritebackSurface` publication values;
- `KernelWritebackPayload`;
- runtime publication symbols used as direct authority;
- stale logical state;
- zero skeleton direct frame publication capture;
- wrappers around any of the above.

## Static Scans

Cutover arm in `04_direct_publication.rs`:

```bash
awk 'BEGIN{capture=0}
  /HillslopeRuntimeSelection::DirectPublicationFrameCutover => \{/ {capture=1}
  capture {print}
  /HillslopeRuntimeSelection::DirectPublicationFrameShadow => \{/ {exit}' \
  crates/openwepp-runner/src/hillslope/04_direct_publication.rs |
  rg -n "SimulationOwnedWb13Row|HillslopeWritebackSurface|KernelWritebackPayload|runtime_surface|wb13|writeback|stale"
```

Result: no matches.

Retained row producer scan is obsolete because R6E removed the hand-authored
`retain_direct_publication_day_rows` producer from the compatibility loop.

Full helper-module scan:

```bash
rg -n "SimulationOwnedWb13Row|HillslopeWritebackSurface|KernelWritebackPayload|runtime_surface|wb13|writeback|stale" \
  crates/openwepp-runner/src/hillslope/04_direct_publication.rs
```

Result: one match in the fail-closed rejection message that names forbidden
sources; no authority reads.

## Counter / Output Evidence

Focused tests pass and prove:

- run-frame construction: `1`;
- executor construction: `1`;
- publication capture: `1`;
- direct compute/state/downstream/shadow counters: nonzero;
- skeleton runs: `0`;
- compatibility-edge invocations: `0`;
- public output files: none.

## Boundary

This is not a complete no-compatibility proof for successful R6 publication
cutover because cutover still fails at HBP process parity. It proves the R6E
cutover candidate no longer uses the compatibility loop to author direct rows
and refuses to write public outputs when direct process parity is not proven.
