# Active Prompt

Scaffold and execute a work-package to implement per-day `cancov` in the direct
runtime to resolve the 10.3.1 blocker.

Implementation constraints:

- Contract-first under `SC-SNOWFREEZE-001`.
- Use direct production growth-state canopy, not a separate canopy model.
- Keep melt/density/radiation/albedo/frost constants unchanged.
- Preserve diagnostic schemas used by PySnobal forcing and CoE-bound density
  replay unless a contract and consumer migration explicitly authorize a change.
- Close with evidence and gates, or hold with a named blocker.
