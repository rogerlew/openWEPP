# Required Reading Map

Status: `SCAFFOLDED`

Required-reading budget: approximately `135000` bytes for Core file paths
excluding the package-local map and section-scoped roadmap extraction.
Disposition: `OK` (`<=400000` bytes).

## Core

| Path | Why |
|---|---|
| `AGENTS.md` | Repository-wide kernel/work-package rules. |
| `docs/work-packages/AGENTS.md` | Work-package execution, evidence, review, verification, and hold rules. |
| `docs/standards/AGENTS.md` | Standards routing for prompt/package wording. |
| `docs/standards/prompt-wording-guidance.md` | Required kickoff-prompt wording and subagent authorization language. |
| `docs/specifications/science-contracts/AGENTS.md` | Contract-first process and no-surrogate physics rules. |
| `crates/AGENTS.md` | Rust implementation rules and closure gates. |
| `tests/AGENTS.md` | Contract-derived test and integration-test rules. |
| `docs/ROADMAP.md` `## Watershed Runtime Performance Queue` | Queue order and reframed M-T2B/M-T2/M-T3 dependencies. |
| `docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md` | Canonical groundwater/baseflow process authority. |
| `docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md` | Parser-to-process handoff and sidecar guard authority. |
| `docs/work-packages/20260708-groundwater-baseflow-srivastava-authority-001/package.md` | M-T2A scope and authority posture. |
| `docs/work-packages/20260708-groundwater-baseflow-srivastava-authority-001/artifacts/worker-handoff.md` | Exact M-T2B implementation handoff. |
| `docs/work-packages/20260708-openwepp-management-yaml-canonical-authorization-001/package.md` | Canonical YAML input surface and runtime consumer proof context. |
| `docs/work-packages/20260708-openwepp-management-yaml-canonical-authorization-001/artifacts/worker-handoff.md` | YAML follow-on constraints relevant to native route authority. |
| `docs/work-packages/20260708-landuse-migration-cli-spec-implementation-001/package.md` | Migration CLI output and closing-test state relevant to fixture readiness. |
| `docs/work-packages/20260708-landuse-migration-cli-spec-implementation-001/artifacts/worker-handoff.md` | Publish-order and no-sidecar constraints. |

## Conditional

| Path | Trigger |
|---|---|
| `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | Before active Lane D ledger, active owner, `INV-OFEROUTE-012`, or route-coefficient eligibility edits. |
| `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md` | Before touching `latqcc` lineage or subsurface export ownership. |
| `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-YAML-001.md` | Before touching YAML input-surface authority or native-management parsing. |
| `docs/contracts/openwepp-management-lanuse-authority-contract.md` | Before touching Lane D native/default route-coefficient eligibility. |
| `docs/specifications/science-contract-authoring-procedure.md` | Before authoring or amending any `SC-*` contract. |
| `docs/specifications/science-contracts/kernel-process-contract-profile.md` | Before amending kernel process obligations or BEI rows. |

## On Demand

| Path | Trigger |
|---|---|
| `/workdir/wepp-forest_260430_baseline/src/main.for` | Rechecking `gwcoeff.txt` branch selection. |
| `/workdir/wepp-forest_260430_baseline/src/contin.for` | Rechecking storage/baseflow/deep-seepage recurrence. |
| `/workdir/wepp-forest_260430_baseline/src/wshpas.for` | Rechecking pass/HBP propagation. |
| `/workdir/wepp-forest_260430_baseline/src/wshdrv.for` | Rechecking watershed-driver temporary storage. |
| `/workdir/wepp-forest_260430_baseline/src/wshchr.for` | Rechecking watershed/channel generated-baseflow consumption. |
| `/workdir/wepp-forest_260430_baseline/src/wshcqi.for` | Rechecking channel intake and threshold branches. |
| `/workdir/wepp-forest_260430_baseline/src/watbalprint.for` | Rechecking legacy water-balance publication semantics. |
| `crates/openwepp-input-contract/src/parsers/gwcoeff.rs` | Parser handoff or test changes. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**` | Direct runtime state, Lane D active, ledger, and publication changes. |
| `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/**` | Runtime input projection changes. |
| `crates/openwepp-runner/src/**` | Runner intake, CLI, manifest, or fixture command changes. |
| `crates/openwepp-watershed-orchestrator/src/**` | Generated baseflow consumer proof or hold-lift implementation. |
| `crates/openwepp-watershed-output/src/**` | Publication metadata/output changes. |
| `tests/integration/**` | Focused integration test additions. |
