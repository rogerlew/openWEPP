# HPHYS0276 Unit Conversion Helper and Raw Literal Guard Kickoff Agent Prompt

    Scope: local repository science-contract/kernel migration task; flat-file
    reads/edits only; no external connectivity.

    Execution mode: package-end-to-end (default).

    Phase plan: execute all phases in package.md sequentially through
    disposition.

    Required reading (read before edits):

    - /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- docs/specifications/science-contract-authoring-procedure.md
- docs/specifications/science-contracts/kernel-process-contract-profile.md
- docs/specifications/science-contracts/index.md
- docs/decisions/0011-architecture-first-top-down-science-contracts.md
- docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md
- docs/work-packages/20260603-hphys0272-hourly-radiation-unit-closure-001/artifacts/disposition.md
- docs/work-packages/20260603-hphys0272-hourly-radiation-unit-closure-001/artifacts/worker-handoff.md
- docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md
- docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md
- docs/specifications/science-contracts/contracts/SC-WATBAL-001.md
- docs/specifications/science-contracts/contracts/SC-SOIL-001.md
- docs/specifications/science-contracts/contracts/SC-PERC-001.md
- docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md
- docs/specifications/science-contracts/contracts/SC-EVAP-001.md
- docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md
- docs/work-packages/20260603-hphys0276-unit-conversion-helper-and-raw-literal-guard-001/package.md

    Files:

    - crates/openwepp-unit-boundary/src/lib.rs
- tools/release/
- crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs
- crates/openwepp-unit-boundary/src/lib.rs
- crates/openwepp-hillslope-orchestrator/**
- crates/openwepp-runner/**
- tools/release/**
- tests/integration/**
- docs/work-packages/20260603-hphys0276-unit-conversion-helper-and-raw-literal-guard-001/**

    Task: execute package objective end-to-end for declared scope.

    Constraints: contract-first sequencing; canonical SC authority; baseline
    provenance where legacy units are involved; typed guards; no silent
    defaults; no heuristic unit conversions; no raw dimensional conversion
    literals unless the package explicitly allowlists them with provenance.

    Autonomy: execute package phases end-to-end and update required artifacts
    without requesting additional user direction unless hard-blocked.

    Dual review requirement: before final disposition, complete
    `review_agent_a.md` and `review_agent_b.md`; disposition every finding as
    `accepted`, `rejected`, `deferred`, or `follow-up` with rationale; apply
    and verify accepted fixes; link deferred/follow-up findings in disposition
    and worker handoff. Do not mark the package complete while any review
    finding is undispositioned.

    Outputs: update package artifacts/disposition for all completed phases,
    including gate results, owned-file manifest, worker handoff, dual review
    artifacts with finding dispositions, and dual verification artifacts with
    `Static:` vs `Ran:` labels.
