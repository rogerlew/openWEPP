# Independent Reconstruction Plan

Status: partial.
Evidence mode: Static + Ran.

Independent reconstruction must not call the production direct projection
builder under test.

Required reconstruction surfaces:

- WAT water-balance fields reconstructed from direct frame operands and area
  basis;
- PASS `runvol` and `sbrunv` reconstructed from direct runoff/lateral depths
  and accepted outlet area;
- HBP scalar event fields reconstructed from direct event operands;
- loss JSON reconstructed from parsed run/climate/static inputs plus direct
  execution counters;
- manifest checksums/provenance independently recomputed from input/output
  paths and direct runtime counters.

Reconstruction tests are required acceptance evidence, not follow-on cleanup.

R6A evidence:

- `r6a_direct_projection_consumers_read_publication_frame_operands` constructs a
  `DirectRunPublicationFrame` directly in the test and asserts WAT/PASS/loss and
  manifest projections from independently supplied expected operands. It does
  not call the runner shadow builder under test.

Remaining R6 cutover reconstruction:

- independent byte/Arrow reconstruction from real fixture files;
- HBP byte identity reconstruction from direct erosion/event producers;
- manifest checksum recomputation from actual output paths;
- loss JSON parity against production writer payload.
