# REFACTOR014-KERNEL-KERNELDECOMP Kickoff Agent Prompt

Scope: local repository engineering task; flat-file reads/edits only; no
external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without
a request for additional user direction unless hard-blocked.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/standards/mechanical-refactor-authoring-guide.md`
- `/workdir/openWEPP/docs/prompt_templates/mechanical-refactor-kickoff-template.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib_mod/kernel.rs`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/kernel_core.rs`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/constants.rs`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/types.rs`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing.rs`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/diagnostics.rs`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/validation.rs`
- `/workdir/openWEPP/docs/work-packages/20260608-refactor014-openwepp-watershed-orchestrator-lib-mod-kernel-completion-001/package.md`

Files:
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/kernel_core.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/constants.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/types.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/diagnostics.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/validation.rs`
- `docs/work-packages/20260608-refactor014-openwepp-watershed-orchestrator-lib-mod-kernel-completion-001/artifacts/*.md`

Task: complete the `lib_mod/kernel` mechanical decomposition objective end-to-end for
this declared scope.

Constraints:
- preserve exported behavior and API intent.
- no bounded-surface migration of domain logic before this package closes.
- no process-physics formula, constant-value, threshold, or guard rewrite.
- no canonicalize-and-proceed behavior for domain violations.
- no broad fallbacks that mask required typed error semantics.

Execution:
1. capture pre-refactor inventories (symbols, visibility, and line counts).
2. complete bounded moves from `kernel_core.rs` into named submodules
   (`constants`, `types`, `helpers`, `routing`, `diagnostics`, `validation`).
3. update `kernel.rs` module wiring and re-exports as needed for internal callers.
4. capture post-refactor inventories and line counts.
5. update required artifacts with truth-labeled evidence.
6. complete dual review and dual verification placeholders with finding
   disposition.

Required commands (record result + exit code):
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test -p openwepp-watershed-orchestrator --tests`
- `cargo test --workspace`
- `cargo deny check`

Note: if required gates are already known to be blocked by an external scope issue,
record that blocker and keep artifacts blocker-ready instead of claiming closure.

Outputs: updated kernel modularization artifacts plus package artifacts with explicit
`Static:`/`Ran:` sections and completed disposition readiness.
