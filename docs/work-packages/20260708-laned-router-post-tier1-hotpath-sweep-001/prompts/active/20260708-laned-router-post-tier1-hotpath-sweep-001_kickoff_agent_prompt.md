# Kickoff Prompt - Lane D Post-Tier1 Explicit Router Hotpath Sweep

You are executing
`docs/work-packages/20260708-laned-router-post-tier1-hotpath-sweep-001/`.

Read, in order:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- this package's `package.md`

Execute only the bounded explicit-router hotpath scope:

- retain and reuse the `prepare_step_alpha()` max celerity and first max-cell
  index for CFL evidence after final `dt` clipping;
- avoid additive-friction-only prework on pure-skin cells by delaying
  additive-path `slope.sqrt()`;
- add focused tests and run package gates;
- record artifacts truthfully.

Do not reopen hybrid implicit stepping, mesh/fidelity adjudication, tolerance
changes, watershed/channel routing, baseflow export, sediment process physics,
or the unratified `Re^0.45` approximation envelope.

No subagent authorization is granted by this package unless the operator
separately authorizes spawning.
