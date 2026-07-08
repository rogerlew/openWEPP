# Execute Laned Router Tier 1 Local Numerics

Execution mode: package-end-to-end.
Autonomy: execute all phases through disposition without additional user
intervention unless a hard blocker is proven.
Connectivity: local repository flat-file work only. Do not assume external
network access or external service connectivity.

Package:
`docs/work-packages/20260708-laned-router-tier1-local-numerics-001/`

Objective: execute the backlog's `Tier 1 - local numerics` optimizations for
the Lane D active overland-flow router as one contract-first package:
analytic celerity, bounded Newton alpha solve, hot-path `h * h.sqrt()` for
`h^1.5`, and a bounded `Re^0.45` optimization only if its approximation
envelope is contract-authorized.

Core required reading:

- `AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/standards/kernel-work-package-preparation.md`
- `docs/backlog/20260706-laned-router-numerics-performance-tiers.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- package-local `package.md`

Implementation-local required reading before code edits:

- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/standards/local-ci-gate-selection.md`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/d10b_reconciliation_tests.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/iwagaki_oracle.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/profile.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs`
- `crates/openwepp-runner/src/hillslope/laned_active.rs`
- `tests/integration/laned_shadow_h2637.rs`

Before the first `SC-*` edit, also read:

- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`

Timing context:

- `docs/work-packages/20260706-mofefid-d15-active-owner-optimization-001/artifacts/baseline-profile.md`
- `docs/work-packages/20260706-mofefid-d15-active-owner-optimization-001/artifacts/slot-profile.md`
- `docs/work-packages/20260706-mofefid-d15-active-owner-optimization-001/artifacts/optimization-plan.md`

Constraints:

- Amend `SC-OFEROUTE-001` before production code.
- Do not claim bit identity. This package deliberately changes numerical
  method and must close through contract authority, oracle agreement,
  conservation, fidelity deltas, and timing evidence.
- Do not promote Tier 2 mesh policy, default activation, hybrid stepping, or
  sediment/process physics.
- Do not relax closure/CFL tolerances or add silent shadow/DC01 fallbacks.
- Do not implement unbounded fast-math or unproven `f32` substitutions.
- Prove the real active H2637 consumer reads the new numerics; producer-only,
  shadow-only, or counter-only evidence is insufficient.

Subagent authorization: this package explicitly authorizes
spawning/delegating to review, verification, comparator/timing,
numerics-review, and contract-authority subagents. Heavy comparator, timing,
and final full-closure gates must be delegated to a `comparator_suite_runner`
subagent when available. Expected outputs are package-local artifacts. Write
access is bounded to package-local artifacts unless a subagent is explicitly
assigned an implementation fix.

Required final outputs:

- `artifacts/required-reading-map.md`
- `artifacts/method-authority-and-design.md`
- `artifacts/contract-implementation.md`
- `artifacts/contract-test-implementation.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/operand-lineage.md`
- `artifacts/implementation.md`
- `artifacts/timing-protocol.md`
- `artifacts/timing-evidence.md`
- `artifacts/fidelity-delta.md`
- `artifacts/consumer-path-proof.md`
- `artifacts/gate-results.md`
- `artifacts/line-count-governance.md`
- `artifacts/owned-file-manifest.md`
- dual review artifacts
- dual verification artifacts
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`
