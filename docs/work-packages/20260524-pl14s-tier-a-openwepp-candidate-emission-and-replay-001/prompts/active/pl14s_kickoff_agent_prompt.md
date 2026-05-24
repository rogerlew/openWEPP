# PL14S Kickoff Agent Prompt

Scope: local repository engineering task for Tier-A legacy comparison; flat-file
reads/edits and local command execution only; no external connectivity.
Phase: A only.
Files:
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `tools/legacy_comparison_suite/README.md`
- `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`
- `docs/work-packages/20260524-pl14s-tier-a-openwepp-candidate-emission-and-replay-001/artifacts/pl14s-contract-implementation-evidence.md`
- `docs/work-packages/20260524-pl14s-tier-a-openwepp-candidate-emission-and-replay-001/artifacts/pl14s-replay-lane-configuration-and-guard-map.md`
- `docs/work-packages/20260524-pl14s-tier-a-openwepp-candidate-emission-and-replay-001/artifacts/pl14s-legacy-comparison-suite-design.md`
Task: ratify PL14S semantic-parity replay authority and reusable legacy
comparison suite design for hillslope water-balance surfaces.
Constraints: contract-first sequencing; semantic parity (not bit parity);
pinned baseline provenance (`wepp_260430`); no silent defaults; explicit
investigation-grade diagnostics; erosion excluded from scope.
Autonomy: execute this phase end-to-end and update phase artifacts without
requesting additional user direction unless hard-blocked.
Outputs: update listed PL14S artifacts for this phase only.

Required reading:
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/decisions/0003-parity-semantic-not-bit.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
