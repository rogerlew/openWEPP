# crates/AGENTS.md
> Agent playbook for openWEPP Rust crates.

## Authorship
**This document and all AGENTS.md documents are maintained by GitHub Copilot / Codex / Claude Code, which retain full authorship rights for all AGENTS.md content revisions. Agents may author and revise AGENTS.md documents when and where they see fit.** Revisions must preserve applicable user direction, package scope, review expectations, and higher-precedence governance.

## Mission Snapshot
- Own Rust implementation of the openWEPP simulation engine, typed state, CLIs, output, orchestration, and contracts.
- Preserve kernel contract authority and typed fail-closed behavior.
- Keep runtime behavior grounded in canonical `SC-*` contracts and pinned baseline provenance.
- Maintain explicit subprocess, unit, error, and numeric boundaries.

## Primary Assets / Key Files
- `crates/openwepp-*` — engine crates, orchestrators, runtime adapters, outputs, unit boundaries, and contract helpers.
- `docs/specifications/science-contracts/contracts/SC-*.md` — canonical authority for kernel-affecting behavior.
- `docs/specifications/unit-governance.md` — unit wrapper and conversion authority.
- `docs/decisions/0004-subprocess-hillslope-orchestration.md` — subprocess orchestration rules.
- `tests/AGENTS.md` — integration-test conventions.

## Standard Workflow
1. Read root `AGENTS.md` and this file before editing Rust crates.
2. For kernel-affecting behavior, read `docs/specifications/science-contracts/AGENTS.md` and the relevant `SC-*` contract first.
3. Confirm work is authorized by an active package when substantive implementation is involved.
4. Apply contract-first sequencing: contracts, contract-derived tests, pre-implementation evidence, then production code.
5. Preserve public APIs, aliases, units, and error contracts unless the package explicitly authorizes a change.
6. Record validation evidence in the owning package artifacts when package scope applies.

## Rust Authoring Rules
- Use typed error enums in production paths; avoid broad `Result<_, Box<dyn Error>>` swallowing.
- No `.unwrap()` or `.expect()` in production paths. Tests may use them with intent.
- Surface NaN, divide-by-zero, overflow, invalid domains, and missing state as typed errors; do not silently default.
- `unsafe` blocks require a `// SAFETY: ...` comment explaining the invariant.
- Use `std::process::Command` with explicit arg arrays for subprocesses; no shell interpolation.
- Preserve variable naming continuity with legacy WEPP symbols where contracts require it.

## Kernel Behavior Rules
- No provisional, surrogate, or heuristic process-physics math in production kernel/runtime publication paths.
- Do not canonicalize-and-proceed on domain violations unless a canonical `SC-*` contract authorizes bounded normalization.
- Do not remove, loosen, or convert fail-closed guards without contract-first amendment, regression tests, before/after evidence, and accepted dual-review disposition.
- Comparator agreement is an investigation signal, not standalone acceptance or rejection authority.

## Validation Checklist
- Fast iteration when useful: focused `cargo nextest run -p <crate>` or `cargo check -p <crate>`.
- Required closure before implementation package disposition: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo nextest run --workspace --profile full`, `cargo deny check`.
- Use `cargo nextest run --workspace --profile quick` for ordinary local loops and `cargo nextest run --workspace --profile frost` for snow/frost-focused work; fall back to `cargo test` only for libtest-specific behavior or explicitly required legacy harness checks.
- Contract-derived tests and closure checks for touched state surfaces.
- Legacy comparator delta review using confidence tiers when migration/parity is in scope.
- For release CLI timing/comparator evidence, build the exact runner binary
  target before execution. Use
  `cargo build --release -p openwepp-runner --bins` for broad runner evidence
  or explicit `--bin` names for a narrower package, then record binary path,
  mtime/size or hash, and run command in the package artifact. Do not rely on
  generic workspace `cargo build --release` to refresh non-default runner bins.

## Line-Count Governance
- `.rs` files at or above 2000 lines are `WARN` and need decomposition rationale plus follow-on split intent in review/checklist artifacts.
- `.rs` files at or above 3000 lines require refactor before closure unless an approved generated/fixture exception documents owner and sunset plan.
- Package closure is blocked while any 3000+ non-exempt file remains undispositioned.

## Common Pitfalls
- Do not hide behavior changes inside mechanical refactors.
- Do not add fallback wrappers for missing required dependencies or invalid state.
- Do not duplicate wepppy orchestration, GIS, climate, or run-state concerns in openWEPP.
- Do not treat hourly/watershed comparator deltas as standalone rejection signals.

## References
- Science contracts: `docs/specifications/science-contracts/AGENTS.md`.
- Work packages: `docs/work-packages/AGENTS.md`.
- Tests: `tests/AGENTS.md`.
