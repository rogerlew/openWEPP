Scope: local repository science-contract/kernel migration task; flat-file reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- docs/work-packages/20260604-hphys0283-spring-snowmelt-runoff-infiltration-partition-001/package.md
- docs/specifications/science-contract-authoring-procedure.md
- docs/specifications/science-contracts/kernel-process-contract-profile.md
- docs/specifications/science-contracts/index.md
- docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md
- docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md
- docs/specifications/science-contracts/contracts/SC-WATBAL-001.md
- docs/specifications/science-contracts/contracts/SC-PERC-001.md
- docs/specifications/science-contracts/contracts/SC-EVAP-001.md
- docs/decisions/0011-architecture-first-top-down-science-contracts.md
- docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md
- /workdir/wepp-forest_260430_baseline/src/watbal_hourly.for
- /workdir/wepp-forest_260430_baseline/src/grna.for
- /workdir/wepp-forest_260430_baseline/src/winter.for
- /workdir/wepp-forest_260430_baseline/src/snowd.for
- /workdir/wepp-forest_260430_baseline/src/melt.for
Files: the intended write set listed in package.md.
Task: execute HPHYS0283 end-to-end for the declared scope: localize and correct the spring 2014 snowmelt runoff/infiltration partition causing `Total-Soil` collapse.
Constraints: contract-first sequencing; canonical SC authority; baseline provenance from `/workdir/wepp-forest_260430_baseline` commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; typed guards; no silent defaults; no heuristic/proxy physics; preserve corrected negative-melt authority.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases, including dual review, finding disposition, dual verification, focused tests, targeted H1/H7/H39 traces, and full H1..H39 semantic metrics.
