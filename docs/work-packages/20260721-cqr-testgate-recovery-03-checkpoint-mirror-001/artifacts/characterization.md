# Characterization

Ran: direct module characterization landed before production decomposition.
Two focused tests pass under Nextest run
`3788975e-47c3-4545-9980-dc618ef504f9`:

- isolated child processes exercise absent, ephemeral, relative, successful,
  artifact-alias, and symlinked configured roots without leaking the process
  environment into parallel tests;
- direct directory cases exercise parent components, regular-file components,
  symlink components, and incremental directory creation;
- the success case proves exact bytes for two outputs and exact canonical JSON
  bytes at `.checkpoints/<node_id>.json`.

The first focused run correctly failed because the test scratch retained lexical
`../..` components. Canonicalizing only the fixture base corrected the fixture;
production code was unchanged. No scratch directory remains after the passing
run.
