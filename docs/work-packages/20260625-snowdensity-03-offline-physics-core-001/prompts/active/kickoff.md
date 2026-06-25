# SNOWDENSITY-03 Kickoff

Execute `docs/work-packages/20260625-snowdensity-03-offline-physics-core-001/`.

Goal: add the first offline Rust `physics_bulk` candidate under snowbench only,
with typed state, candidate fresh-snow density, densification, liquid water, and
cold-content accounting. Emit five-site SNOTEL rubric output without per-site
constants.

Do:

- Keep implementation offline in `openwepp-snowbench`.
- Record candidate equations/constants and candidate-only provenance.
- Add tests for state bounds, closure, monotonicity, and no production coupling.
- Generate SNOTEL rubric JSON/Markdown as package evidence.

Do not:

- Couple `physics_bulk` into production runtime.
- Change default behavior, output schema, parser options, or rollback policy.
- Tune constants per SNOTEL site.
- Treat PySnobal or legacy WEPP as acceptance targets.

Subagent authorization: none. Execute locally.
