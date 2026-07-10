# Characterization Plan

Cover every topology parse error display, fixture grammar/reference failure,
pre-execution validation result, and cycle/reference closure through public APIs
before structural extraction.

Planned focused test path:

- `tests/integration/topology_graph_validation_gate.rs`

Initial gap from static read:

- existing tests cover the canonical valid path, several validation violations,
  and the missing-file `ReadError` variant;
- they do not yet cover all parser error variants, display strings, duplicate
  headers/nodes, marker failures, invalid numeric fields, or all reference
  existence/domain branches needed to safely decompose the high-CRAP parser,
  display implementation, and validation gate.
