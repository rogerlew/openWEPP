# R6E Anti-Alias Fixtures

Evidence mode: Static + Ran.

Status: blocked by HBP process parity.

Required before closure:

- HBP runoff/erosion fixture distinguishing peak, duration, detachment,
  deposition, sediment class concentrations, and zero aliases.
- WAT fixture distinguishing precipitation, liquid input, runoff, OFE runoff,
  ET components, deep percolation, lateral, tile, storage, snow/frost, profile,
  and interception fields.
- PASS fixture distinguishing `runvol`, `sbrunv`, erosion fields, row area,
  outlet area, and publication area aliases.
- loss fixture distinguishing climate span, executed day count, static sidecar
  fields, and optional-output payload aliases.
- manifest fixture distinguishing input checksums, output checksums, direct
  runtime counters, runtime selection, warning IDs, and output policy.

R6E did not add anti-alias fixtures because no public output family was
accepted. The existing CLI fixture also lacks a PASS Parquet output target, so
it is insufficient for final R6 PASS parity.

Next package must add or select fixtures as direct process parity work accepts
public output operands. Fixture design must include PASS Parquet and values
that make compatibility-row, zero/default, stale-state, and wrong-area aliases
observably different.
