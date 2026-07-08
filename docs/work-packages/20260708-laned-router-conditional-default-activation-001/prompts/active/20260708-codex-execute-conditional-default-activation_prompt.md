# Execute Conditional Default Activation

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260708-laned-router-conditional-default-activation-001/package.md`
sequentially through disposition.

Required reading:

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260708-laned-router-conditional-default-activation-001/artifacts/required-reading-map.md`

Conditional:

- `crates/AGENTS.md` and `tests/AGENTS.md` before Rust/test edits.
- `SC-SED-001` only if erosion water-magnitude coupling is changed; it should
  not be changed in this package.

Required-reading budget: OK; map:
`artifacts/required-reading-map.md`.

Files:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `crates/openwepp-runner/src/hillslope/laned_active.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `tests/integration/laned_shadow_h2637.rs`
- `docs/work-packages/20260708-laned-router-conditional-default-activation-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

Task: execute the package objective end-to-end for the declared scope.

Constraints: contract-first sequencing; canonical SC authority; typed guards;
no silent defaults that hide mixed routing authority; no surrogate physics; no
mesh-policy, routed-shape, or annual-sediment tolerance changes.

Real consumer proof: prove the runner default/no-env path attaches
`DirectLanedActiveConfig` and the active executor consumes projected
`routing_coefficients` only when every scheduled lane is complete. Prove the
all-legacy fallback does not attach active routing and preserves protected
scientific outputs. Prove mixed authority fails closed before streaming.

Subagent requirement: REQUIRED for heavy batch/closure/comparator runs when
available. This prompt explicitly authorizes subagent spawning/delegation to
review, verification, and comparator/closure-gate subagents for package-local
review, verification, default/fallback comparator work, and heavy Rust closure
gates; outputs: compact metrics plus package artifact paths; write access:
bounded to package artifacts unless explicitly assigned implementation fixes.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
