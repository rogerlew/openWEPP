# PERFDEEP08 Kickoff Agent Prompt

In `/home/workdir/openWEPP`, execute
`docs/work-packages/20260619-perfdeep08-disabled-path-hard-isolation-001/`
end to end.

Execution mode: package-end-to-end.

Autonomy: execute the declared scope through implementation, artifact updates,
review, verification, disposition, and roadmap/catalog updates without asking
the user for next steps unless a hard blocker prevents a truthful disposition.

Subagent authorization: this prompt explicitly authorizes spawning/delegating to
read-only reviewer and verifier subagents for package artifact review,
disabled-path hard-isolation proof review, regression-gate review, line-count
governance review, and gate-legitimacy verification. It also explicitly
authorizes spawning/delegating to comparator or batch-runner subagents for H2637
endpoint/identity runs if the local tooling supports it. Expected outputs are
compact findings or metrics recorded in the package artifacts. Write access is
limited to artifact files unless the package is explicitly amended.

Objective: close the PERFDEEP07 default-disabled hold by making failed
PERFDEEP dense/direct-frame compatibility plumbing zero-cost when all opt-ins
are off. Do not implement R2+ direct-frame runtime work in this package.

Required reading:

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/ROADMAP.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep08-disabled-path-hard-isolation-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/disposition.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/gate-results.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/perfdeep07-disabled-path-audit.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/perfdeep07-zero-cost-disabled-proof.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/worker-handoff.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/perfdeep07-hold-lift-disposition.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/no-compatibility-proof-plan.md`
- `/home/workdir/openWEPP/docs/architecture/array-native-runtime-specification.md`
- `/home/workdir/openWEPP/docs/decisions/0025-array-native-hillslope-day-frame.md`

Before Rust edits:

- `/home/workdir/openWEPP/crates/AGENTS.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`

Execution order:

1. Fill required-reading and owned-file artifacts.
2. Reproduce or replace the retained PERFDEEP07 disabled baseline.
3. Audit all default-disabled construction and hot paths for opt-in-only dense,
   indexed, direct-frame, writeback, hot-symbol, or symbol-resolution work.
4. Patch hard isolation without changing physics or output meaning.
5. Run focused tests and identity checks for each viable candidate.
6. Record rejected/slower candidates.
7. Run the three-run H2637 default-disabled endpoint gate. Required median:
   `<= 676.67 s`.
8. Run full closure gates only when the P0 gate passes:
   `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo test --workspace`, `cargo deny check`, docs lint, and
   `git diff --check`.
9. Complete line-count governance, dual review, finding disposition, dual
   verification, roadmap/catalog updates, and worker handoff.

Required closeout:

- populate all queued artifacts with `Static:` and `Ran:` evidence;
- close as `READY-FOR-R2`, `HOLD`, or `NO-GO`;
- do not claim R2+ readiness unless the disabled-path median gate and full
  closure gates pass;
- do not implement direct-frame hydrology or direct executor code.
