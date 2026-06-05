# HPHYS0301 Kickoff Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0301-h39-forcing-melt-term-producer-closure-001/package.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001/artifacts/worker-handoff.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001/artifacts/raw-post-raw-lineage-ledger.json`
- `/workdir/wepp-forest_260430_baseline/src/brkpt.for`
- `/workdir/wepp-forest_260430_baseline/src/stmtim.for`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/melt.for`

Files:

- `docs/work-packages/20260605-hphys0301-h39-forcing-melt-term-producer-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `tests/integration/hphys0301_h39_forcing_melt_term_producer_contract.rs`
- `Cargo.toml`
- Production files only if evidence identifies a source-line producer defect.

Task: execute package objective end-to-end for declared scope. Reconcile H39
first-2013 baseline residual rain/snow evidence against openWEPP raw,
retained, released, post-winter rain, raw melt, and routed melt traces. If a source-line openWEPP producer defect is proven, implement the baseline-authoritative correction. If not proven, record the concrete blocker and continuation scope.

Constraints: contract-first sequencing; canonical SC authority; pinned baseline
provenance from `/workdir/wepp-forest_260430_baseline`; typed guards; no silent
defaults; no production forcing, snow, WB17, WB18, WB19, or WB13 edits from
raw-rain aggregate deltas alone; preserve corrected `/workdir/wepp-forest`
negative-melt authority.

Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases,
including dual review, review disposition, dual verification, implementation
decision, and worker handoff.
