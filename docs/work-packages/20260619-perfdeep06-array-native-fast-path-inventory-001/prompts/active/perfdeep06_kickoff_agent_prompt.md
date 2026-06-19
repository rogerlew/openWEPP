# PERFDEEP06 Kickoff Agent Prompt

In `/home/workdir/openWEPP`, execute
`docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/`
end to end.

Execution mode: package-end-to-end.

Autonomy: execute the declared scope through artifact updates, review,
verification, disposition, and roadmap/catalog updates without asking the user
for next steps unless a hard blocker prevents a truthful disposition.

Subagent authorization: this prompt explicitly authorizes
spawning/delegating to read-only reviewer and verifier subagents for package
artifact review, no-hot-loop-map proof review, publication operand ledger
review, and gate-legitimacy verification. Expected outputs are compact findings
recorded in the package review/verification artifacts. Write access is limited
to those artifacts unless the package is explicitly amended.

Objective: produce the ADR-0025 Stage-3 array-native fast-path inventory and
execution plan. Start from PERFDEEP05's no-go profile and the ratified runtime
specification. Do not continue the older lane-dense seam-shaving path. Do not
edit production Rust code or activate an opt-in by default unless this package is
amended before implementation with a bounded write set and gates.

Required reading:

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/ROADMAP.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/package.md`
- `/home/workdir/openWEPP/docs/architecture/array-native-runtime-specification.md`
- `/home/workdir/openWEPP/docs/decisions/0025-array-native-hillslope-day-frame.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/artifacts/perfdeep05_disposition.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/artifacts/perfdeep05-profile.md`

Conditional:

- `/home/workdir/openWEPP/crates/AGENTS.md` before any Rust crate edit.
- `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`
  before any runtime-projection edit that controls kernel branches.
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`,
  `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`,
  and `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
  if canonical contract authority must change.
- `/home/workdir/openWEPP/tests/AGENTS.md` before any test edit.

On-demand:

- PERFDEEP03 and PERFDEEP04 package/disposition/profile artifacts.
- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/phase.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/**`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/00_symbol_registry_and_indexed_surfaces.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/02_boundary_values_and_kernel_requests.rs`
- `crates/openwepp-runner/src/hillslope/**`
- `crates/openwepp-hillslope-output/src/**`

Required-reading budget: scaffolded core local byte total is `209369` bytes,
which is `OK` under the `<=400000 bytes` threshold. Recompute and update
`artifacts/required-reading-map.md` if the core list changes before execution.

Conservation/output acceptance: because this package maps publication operands,
author an operand-lineage ledger before recommending future output-path edits.
The ledger must cover units, normalization/denominator, area or volume basis,
source authority, authoritative-vs-diagnostic status, plausible wrong aliases,
anti-tautology fixtures, independent reconstruction needs, and metadata/schema
alignment.

Contract-first rule: no physics or canonical `SC-*` change is intended. If a
contract or guard-authority change becomes necessary, stop and amend the package
before implementation; then follow canonical contract, contract-derived tests,
pre-implementation gate, and production edits.

Required closeout:

- populate all queued artifact placeholders with `Static:` and `Ran:` evidence;
- run markdown lint for the package and touched docs;
- complete dual independent reviews and dual verifications;
- disposition every finding as `accepted`, `rejected`, `deferred`, or
  `follow-up`;
- update `docs/ROADMAP.md` and `docs/work-packages/README.md`;
- close as `READY-FOR-PERFDEEP07`, `HOLD`, or `NO-GO`.
