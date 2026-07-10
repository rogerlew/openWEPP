# Characterization

Initial known behavior surface:

- public parser entry points: `parse_slope_file`, `parse_slope_str`;
- public options/types: `SlopeParserOptions`, `SlopeParserMode`,
  `DatverSource`, `DistanceMode`, `SlopeProfile`, `SlopeOfe`, `SlopePoint`;
- public typed error enum: `SlopeParserError`.

Existing tests:

- `tests/integration/infile_slope_parser_contract.rs`
- parser use in watershed/runtime integration tests.

Pre-production-edit requirement:

- Add or confirm test-first characterization for every high-CRAP display and
  top-level orchestration branch changed by the decomposition.
- Run the new/changed test against scaffold source in a detached worktree before
  production edits.
