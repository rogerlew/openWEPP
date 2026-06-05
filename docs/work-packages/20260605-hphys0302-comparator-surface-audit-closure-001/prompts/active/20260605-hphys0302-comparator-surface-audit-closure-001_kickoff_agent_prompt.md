# HPHYS0302 Kickoff Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0302-comparator-surface-audit-closure-001/package.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/target-window-lineage-schema.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0301-h39-forcing-melt-term-producer-closure-001/artifacts/worker-handoff.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0301-h39-forcing-melt-term-producer-closure-001/artifacts/claude-code-review-findings.md`

Files:

- `docs/work-packages/20260605-hphys0302-comparator-surface-audit-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `tests/integration/hphys0302_comparator_surface_audit_contract.rs`
- `Cargo.toml`

Task: execute package objective end-to-end for declared scope. Audit `RM`,
`Snow-Water`, raw/post-raw melt, and term-level melt comparator surfaces for
like-for-like physical quantity and unit pairing. If a source-line producer
defect is proven through valid paired surfaces, record implementation scope; if
not proven, record the concrete missing surface blocker and continuation scope.

Constraints: contract-first sequencing; canonical SC authority; pinned baseline
provenance from `/workdir/wepp-forest_260430_baseline`; typed guards; no silent
defaults; no production forcing, snow, WB17, WB18, WB19, or WB13 edits from aggregate deltas alone.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases,
including dual review, review disposition, dual verification, comparator-surface
ledger, decision, and worker handoff.
