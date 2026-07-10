# CQR Nightly Batch 02 Target 04 Kickoff

Scope: local behavior-preserving CQR work only inside `/home/workdir/openWEPP`;
no external connectivity.

Execution mode: package-end-to-end.

Required reading: `AGENTS.md`, `crates/AGENTS.md`,
`docs/work-packages/AGENTS.md`, science-contract guidance, `SC-SYSTEM-001`,
`SC-ROUTE-001`, `SC-GWBASEFLOW-001`, the CQR ExecPlan and authoring guides,
ADR-0021, this package, the target CLI, and its focused contract tests. Required
reading map: `artifacts/required-reading-map.md`; budget: `~150 KiB`, `OK`.

Task: close CRAP above `30` only through behavior-preserving target-local
decomposition. Do not change CLI arguments, runfile grammar, path resolution,
topology, HBP/manifest consumer rules, contract guards, units, output semantics,
typed errors, numeric operation order, or fallbacks.

Subagent requirement: REQUIRED: use a comparator suite runner for heavy target
coverage/CRAP, workspace clippy/full-nextest/deny, comparator, and publication
evidence. This prompt authorizes bounded read-only review and verification
subagents; implementation write access must be explicitly limited to this
package's target/test/artifact paths.

Autonomy: execute through disposition unless a declared hard blocker occurs.
