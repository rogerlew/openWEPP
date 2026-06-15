# CQR01 Kickoff Agent Prompt

Scope: local repository flat-file edits only in `/workdir/openWEPP`.

Autonomy: execute the package end-to-end through source refactor, validation,
artifact updates, dual review/verification, and disposition unless a declared
hard blocker is reached.

Objective: decompose
`crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs`
so `compute_active_frost_coupling` no longer needs
`#[allow(clippy::too_many_lines)]`, preserving all frost numeric behavior,
guards, units, formulas, thresholds, and public call sites.

Subagent authorization: this prompt explicitly authorizes subagent
spawning/delegation to review and verification subagents for bounded read-only
review of this package's artifacts and source diff. Expected outputs are
`artifacts/review_agent_a.md`, `artifacts/review_agent_b.md`,
`artifacts/verification_agent_a.md`, and
`artifacts/verification_agent_b.md`; write access is limited to package
artifact files. If subagents are unavailable, perform equivalent independent
local reviews and record that path.

Required reading budget:

- local bytes total: 164664
- disposition: OK (`<=400000` bytes)
- map: `artifacts/required-reading-map.md`

## Required Reading

Core:

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260615-cqr01-frost-entry-complexity-001/package.md`

Conditional:

- `/workdir/openWEPP/docs/standards/AGENTS.md`
- `/workdir/openWEPP/docs/standards/mechanical-refactor-authoring-guide.md`
- `/workdir/openWEPP/docs/standards/code-quality-refactor-authoring-guide.md`
- `/workdir/openWEPP/docs/standards/module-test-enhancement-authoring-guide.md`
- `/workdir/openWEPP/docs/standards/kernel-work-package-preparation.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`
- `/workdir/openWEPP/crates/AGENTS.md`

On-demand:

- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/work-packages/20260614-refactor023-hillslope-coupling-line-count-split-001/package.md`

## Execution Instructions

1. Record baseline line counts, lint suppression census, and available
   quality metrics before editing production code.
2. Run focused frost characterization before production edits:
   `cargo test --test clim06_frost_frozen_soil_kernel_contract`.
3. Extract cohesive internal blocks from `compute_active_frost_coupling` into
   private helpers in the same file. Preserve expression grouping, statement
   order, branch order, and thresholds. Do not change formulas or units.
4. Remove the `#[allow(clippy::too_many_lines)]` only after the function is
   below the lint threshold.
5. Run focused checks, then the required closure gates:
   `cargo fmt --check`;
   `cargo clippy --workspace --all-targets -- -D warnings`;
   `cargo test --workspace`;
   `cargo deny check`.
6. Update all package artifacts with `Static:` and `Ran:` evidence labels.
7. Complete dual reviews, finding disposition, dual verification, final
   disposition, and worker handoff.

Stop conditions:

- Focused frost characterization fails before production edits.
- A necessary change would alter physics behavior, thresholds, public API, or
  contract authority.
- Required tooling is unavailable and no package-conforming fallback can record
  evidence truthfully.
