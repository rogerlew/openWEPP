# Required-reading and prospective write map

Static: intake completed before common harness implementation, 2026-09-06 PDT.
Exact checkout e89befa4678eadec039b3e7f7fe0a176af8e9dc5; main; no tracked
differences from accepted checkpoint. Existing untracked tmp/ is excluded.

## 2026-09-08 continuation bootstrap

Ran: read the owner continuation prompt, root/package instructions,
role-implementation.md, package.md, worker-handoff.md, pause-checkpoint.md,
active kickoff, complete experiment protocol, standards routing,
prompt-wording guidance, testing-and-gate strategy, tools/agents README, and
role-runner.md before resumed execution. Triggered expansion was limited to
source/build manifests, current gate/parity/results artifacts, collector scripts,
and exact retained receipts needed for admission reconciliation. Workflow-total
exposure remains UNOBSERVED. No kernel/runtime edit is presently made.

## Core and conditional readings

| Path | Lines | Actual bytes | Purpose |
|---|---:|---:|---|
| AGENTS.md | 1-160 | 12599 | root instructions |
| docs/work-packages/AGENTS.md | 1-388 | 26781 | package governance |
| docs/codex_exec_plans.md | 1-244 | 20921 | execution plan |
| docs/standards/AGENTS.md | 1-58 | 4052 | standards routing |
| docs/standards/prompt-wording-guidance.md | 1-180 | 10508 | delegation and reading |
| docs/standards/testing-and-gate-strategy.md | 1-459 | 22200 | gate selection |
| docs/specifications/science-contracts/AGENTS.md | 1-77 | 5942 | contract-first |
| docs/standards/numerical-solver-architecture.md | 1-152 | 6742 | protected solver |
| docs/standards/kernel-work-package-preparation.md | 1-255 | 15309 | preparation |
| docs/specifications/science-contract-authoring-procedure.md | 1-280 | 13715 | amendments |
| docs/specifications/science-contracts/kernel-process-contract-profile.md | 1-146 | 5792 | contract profile |
| docs/specifications/science-contracts/index.md | 1-80 | 14926 | actual bindings |
| crates/AGENTS.md | 1-75 | 5436 | Rust |
| tests/AGENTS.md | 1-63 | 4723 | tests |

Prior evidence root P4 = docs/work-packages/20260904-stage3-authentic-coverage-memory-attribution-001/artifacts/.
P1 = docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/artifacts/.

| Evidence | Lines | Bytes read | Purpose |
|---|---:|---:|---|
| P4 final-disposition.md | all 111 | 7124 | historical conclusion |
| P4 coverage-findings.md | all 115 | 7906 | actual nonzero replay |
| P4 memory-attribution.md | all 89 | 5325 | endpoint semantics |
| P4 raw/historical_v31_gdb_take_return_summary.txt | all 40 | 2500 | accepted distribution |
| P1 performance-budget.md | 1-345,773-808 | 23009 | budgets and two mechanisms |
| P1 tolerance-authority.md | all 197 | 11902 | protected boundaries |
| P1 science-contracts/component-temperature-dependency-replay/contract_ref.md | all 245 | 13669 | existing obligations/parity |
| SC-SNOWENERGY-001.md under canonical contracts/ | 3396-3506 | 9022 | INV088/C056 |
| SC-LANDSURFACEENERGY-001.md under canonical contracts/ | 2858-3165 | 27824 | INV164/C020 |

Unique mandatory ranges above total 277927 bytes: OK, below the
400000-byte threshold. Repeated
reads to recover truncated tool output are not counted twice. Source call-site
reads are on demand, not a new mandatory pre-read of entire subsystems.

## Exact write paths and progressive supplements

Parent common harness: crates/openwepp-runner/src/hillslope/03_tests.rs;
Prospective compile correction: crates/openwepp-runner/Cargo.toml adds the existing
LSE test-support feature as a dev-dependency only. No version/resolution change;
normal production builds do not expose observer session activation. Root+crates
instructions found before edit; manifest read in full (62 lines at intake).
Clock correction prospective paths: same runner Cargo.toml and root Cargo.lock.
Reuse already locked rustix 1.1.4 as dev-only dependency with time feature; its
safe clock_gettime API avoids the runner's forbid(unsafe_code) boundary. Read
local primary rustix src/time/clock.rs lines1-105; no network or historical source
recovery. Cargo lock adjustment only records runner's dependency edge; do not
update package versions. Dependency gate applies to this actual manifest cut.
crates/openwepp-runner/src/hillslope/tests03/stage3_runner_qualification.rs;
new crates/openwepp-runner/src/hillslope/tests03/controlled_mechanism_experiments.rs.
Source read: 03_tests.rs lines 1-90; qualification.rs 830-1125,1883-1958,
function inventory and scale fixture/closure helpers as needed; long_run_qualification.rs
670-755 (CPU/status helpers). These implement no process behavior changes.

Package documents/scripts/evidence and reproduction snapshots remain under this
package. Catalog README and snow-surface-energy-balance roadmap are declared in
package.md. tools/agents/find-agents returned root+work-packages for package,
root for roadmap, root+crates for runner, root+science-contracts for contracts,
root+tests for integration tests.

Delegated exact prospective maps (required before their edits):
authority-required-reading.md; carrier-observation-required-reading.md;
lse-observation-required-reading.md. Each records its actual ranges/bytes and
exact paths, including any proposed extraction. Treatment paths are not blanket
directory authority and must be declared/reviewed before behavior edits.

## Common extraction guard correction (prospective)

Static: reviewer B found the old four-read provenance guard reads only the
pre-extraction qualification file. Exact additional write path in main/F/R:
`tests/integration/stage3_native_vegetation_laned_throughput_recovery.rs`.
Discovery returned root AGENTS.md and tests/AGENTS.md (4,723 bytes, fully read).
Read test lines895–1025 and actual helper/include/baseline call sites. Preserve
the four canonical manifest reads across both real source files, reject stale
reads in either, and bind the original baseline call and actual include. All
WAT/WAT5 assertions remain. This is source-location reconciliation, not new
science, test filtering or threshold reduction. Apply after active A full run
to keep its source cut fixed; retain old kits and freeze new common test-input
identities before measurement. Runner source and physical behavior unchanged.
