# PERFDEEP07 Kickoff Agent Prompt

In `/home/workdir/openWEPP`, execute
`docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/`
end to end.

Execution mode: package-end-to-end.

Autonomy: execute the declared scope through implementation, artifact updates,
review, verification, disposition, and roadmap/catalog updates without asking
the user for next steps unless a hard blocker prevents a truthful disposition.

Subagent authorization: this prompt explicitly authorizes spawning/delegating to
read-only reviewer and verifier subagents for package artifact review,
no-hot-loop-map proof review, disabled-path regression-gate review,
publication-shadow review, line-count governance review, and gate-legitimacy
verification. It also explicitly authorizes spawning/delegating to comparator or
batch-runner subagents for H2637 endpoint/identity runs if the local tooling
supports it. Expected outputs are compact findings or metrics recorded in the
package artifacts. Write access is limited to artifact files unless the package
is explicitly amended.

Objective: first make PERFDEEP02/03/05/07 dense/direct-frame compatibility
plumbing zero-cost when all opt-ins are disabled, then implement a bounded
opt-in direct-frame hydrology fast path over typed frame/view APIs. Do not
activate any opt-in by default.

Required reading:

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/ROADMAP.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/worker-handoff.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-working-set-inventory.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-publication-operand-ledger.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-direct-frame-api-plan.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-layout-allocation-ledger.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-no-hot-loop-map-proof.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-follow-on-package-sequence.md`
- `/home/workdir/openWEPP/docs/architecture/array-native-runtime-specification.md`
- `/home/workdir/openWEPP/docs/decisions/0025-array-native-hillslope-day-frame.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/artifacts/perfdeep05_disposition.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/artifacts/perfdeep05-profile.md`

Before Rust edits:

- `/home/workdir/openWEPP/crates/AGENTS.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`

Source inventory:

- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/phase.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/**`
- `crates/openwepp-hillslope-orchestrator/src/tests/**`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/**`
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
- `crates/openwepp-hillslope-output/src/**`

Execution order:

1. Fill required-reading and owned-file artifacts.
2. Audit and patch the default-disabled path before direct-frame implementation.
3. Run the P0 disabled-path gate. If it fails, stop with `HOLD` or `NO-GO`.
4. Implement the bounded direct-frame hydrology opt-in path only after the P0
   disabled-path gate passes.
5. Run focused tests, H2637 identity/timing gates, full Rust closure gates,
   docs lint, line-count governance, review, disposition, verification, and
   roadmap/catalog updates.

Required closeout:

- populate all queued artifacts with `Static:` and `Ran:` evidence;
- run `cargo fmt --check`;
- run `cargo clippy --workspace --all-targets -- -D warnings`;
- run `cargo test --workspace`;
- run `cargo deny check`;
- run package-specific H2637 identity and endpoint/RSS gates;
- run markdown lint for the package and touched docs;
- complete dual independent reviews and dual verifications;
- disposition every finding as `accepted`, `rejected`, `deferred`, or
  `follow-up`;
- close as `READY-FOR-PERFDEEP08`, `HOLD`, or `NO-GO`.
