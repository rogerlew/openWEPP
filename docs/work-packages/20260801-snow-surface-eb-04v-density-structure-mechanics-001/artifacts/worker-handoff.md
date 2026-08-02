# Worker Handoff

Status: `terminal handoff complete`.

Evidence mode: `[Static] + [Ran]`.

EB-04V adds a diagnostic ledger, not new snow-density physics. Downstream work
may rely on these real-consumer fields for the active nonlegacy snow-density
models:

- direct initial/load/liquid/temperature/snowfall operands;
- fresh-snow density before mixing;
- uncapped fresh-mixing, wet, destructive-metamorphism, and overburden bulk
  increments;
- separate internal/runtime cap, structural, fallback, and Stage-3 terms; and
- final density plus additive closure.

The exact release cohort is bound by `execution-receipt.json` and binary
`fb670d086937a7785a2549339832f71b96fc98f3c8992ec8d24961123b33826f`.
Use only the canonical artifact set and
`target/snow_surface_eb04v_density_diagnostics/`. The two named invalidated
artifact/target trees are chronology only and are not decision eligible.

Scientific handoff:

- wet compaction is active and the largest positive compaction term in both
  retained over- and under-density groups;
- fresh mixing, structural projection, and caps can materially oppose it;
- no single coefficient or missing process is uniquely identified;
- existing observations remain consumed `DIAGNOSTIC_ONLY` evidence; and
- no calibration, efficacy, promotion, or default change is authorized.

EB-04W is next and should investigate the five open-control mountain
under-persistence failures using event/day snowfall, phase, SWE increment,
sublimation, melt/runoff, energy, peak-timing, and ablation-onset ledgers. It
must preserve the forcing-ownership boundary and seal any result-bearing rules
before execution. EB-04X follows for Harvard forest/open geometry and
interception.

Maintenance debt is bounded but near-term: `SNOW-DENSITY-MAINT-01` should split
the 1,990-line density module at its next semantic edit/2,000-line threshold;
`RUNNER-TRACE-MAINT-01` should split the runner trace formatter at its next
semantic edit/2,500-line threshold. A separate mechanical governance package
may centralize the 33 repeated literal contract-version assertions.

Terminal quick, frost, and full-workspace profiles passed. The package closes
`DIAGNOSTIC_COMPLETE / EFFICACY_HOLD / NO_PROMOTION`; the retained ledger is
available to EB-04W, but it does not authorize density calibration or a runtime
default change.
