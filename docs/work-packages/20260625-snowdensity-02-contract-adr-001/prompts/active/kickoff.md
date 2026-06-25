# SNOWDENSITY-02 Kickoff

Execute `docs/work-packages/20260625-snowdensity-02-contract-adr-001/`.

Goal: codify the snow-density remediation envelope before runtime work. Amend
`SC-SNOWFREEZE-001` with opt-in `physics_bulk` candidate authority, no-site-
tuning, state variables, conservation obligations, and activation constraints;
add the deliberate-legacy-divergence ADR; and add contract-derived guard tests.

Do:

- Preserve `legacy_wepp` as default and rollback.
- Keep `physics_bulk` opt-in candidate scope only.
- Make SNOTEL/PySnobal/legacy diagnostic profile roles explicit.
- Require no per-site constants and no SSD residual tuning.
- Record review, verification, line-count governance, and handoff evidence.

Do not:

- Implement production runtime snow physics.
- Add runtime model selectors, output schemas, parser fields, or defaults.
- Tune constants to SNOTEL sites.
- Resume frost-compatibility bit-parity or edit frost physics.

Subagent authorization: none. Execute locally.
