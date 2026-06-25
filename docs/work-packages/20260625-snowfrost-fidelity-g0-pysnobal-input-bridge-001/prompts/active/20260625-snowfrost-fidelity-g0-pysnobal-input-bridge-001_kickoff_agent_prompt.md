# SNOWFROST-FIDELITY-G0 Kickoff

Execution mode: package-end-to-end.

Autonomy: execute
`docs/work-packages/20260625-snowfrost-fidelity-g0-pysnobal-input-bridge-001/package.md`
through final disposition without asking for next steps unless a named HOLD
boundary in the package is reached.

Subagent authorization: this prompt explicitly authorizes subagent
spawning/delegation to read-only science-review, harness-review, and
verification subagents for PySnobal forcing-lineage review, anti-alias review,
and final verification. Expected outputs are compact Markdown findings in the
package review/verification artifacts. Subagents may not edit files.

Required posture:

- Implement Rust input generation and Python PySnobal execution only.
- Do not change production snow/frost/hydrology physics, constants, runtime
  activation, observation tolerances, or compatibility/default status.
- Do not install PySnobal dependencies silently. Probe and record the explicit
  Python executable used for PySnobal.
- Reuse openWEPP parser/runtime input projection for WEPP inputs and
  daily-to-hourly forcing. Do not rebuild those algorithms in Python.
- Treat PySnobal `temp_ground_degC` as soil temperature at `z_g`, not as
  `frost.hourly.surface_temp_c_####` or `surtmp(hour)`.
- Convert snowfall depth to water-equivalent mass before feeding PySnobal:
  `snow_mass_mm = snowfall_depth_m * snow_density_kg_m3`.
- Label net shortwave, longwave, and constant ground temperature as diagnostic
  proxies, not production authority.
- A phase may be marked complete only when all of its current-scope gates have
  current evidence. If a gate depends on later evidence, close that phase or
  package in a named HOLD state instead.

Required reading:

Core:

- `AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260625-snowfrost-fidelity-g0-pysnobal-input-bridge-001/package.md`

Conditional:

- `crates/AGENTS.md` before Rust edits.
- `tests/AGENTS.md` before test edits.
- `docs/specifications/science-contracts/AGENTS.md` before
  contract-derived tests or SC edits.
- `docs/specifications/unit-governance.md` before schema/conversion edits.

On-demand:

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/wepp-input-files/specs/climate-file.spec.md`
- `tools/snowfreeze_observed/README.md`
- `/workdir/pysnobal/README.md`
- `/workdir/pysnobal/pysnobal/defaults.py`
- `/workdir/pysnobal/pysnobal/c_snobal/libsnobal/vars.c`
- `/workdir/pysnobal/pysnobal/c_snobal/libsnobal/g_soil.c`

Required-reading budget metrics:

- Initial core local read set is expected to remain below `OK <= 400000`
  bytes. Record actual byte count and disposition in
  `artifacts/required-reading-map.md`.

First actions:

1. Fill `artifacts/required-reading-map.md` with read status and byte metrics.
2. Probe PySnobal import using an explicit Python executable and record results
   in `artifacts/pre-implementation-evidence.md`.
3. Record PySnobal schema and `T_g`/`z_g` source evidence before code edits.
4. Implement the Rust exporter, then the Python harness, following package
   phases and updating artifacts as gates run.
