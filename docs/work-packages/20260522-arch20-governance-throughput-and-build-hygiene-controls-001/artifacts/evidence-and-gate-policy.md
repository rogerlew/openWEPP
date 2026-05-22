# Evidence and Gate Policy

Evidence mode: `Static`
Status: `complete`

## Source Finding Linkage

- [DIRECT] `docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/disposition-register.md` (`CRF-008` and `CRF-009` closure evidence requirements).
- [DIRECT] `docs/work-packages/README.md` (evidence classification conventions and correctness-over-completion policy).
- [DIRECT] `/home/workdir/openWEPP/AGENTS.md` (truthfulness posture and required Rust gates).

## Evidence Posture Requirements

1. Every artifact MUST declare evidence mode: `Static` and/or `Ran`.
2. `Static` claims MUST cite direct files, contracts, or code paths.
3. `Ran` claims MUST list executed commands and outcomes.
4. Claims MUST NOT imply execution when only static inspection occurred.

## Package Type Classification

Every package MUST declare one classification in `gate-results.md`:

- `docs-only`: write set is documentation/spec artifacts only.
- `code-touch`: any Rust/CLI/source code files touched.
- `mixed`: both documentation and code surfaces touched.

## Required Gates by Package Type

### Docs-Only

1. Artifact completeness check (all required files exist).
2. Pending-state check (no required artifact remains pending).
3. Scope consistency check (changed files stay within declared docs scope).

### Code-Touch (and Mixed)

Docs-only gates plus all required workspace Rust gates:

1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`

## Gate Recording Requirements

1. Each gate entry MUST include command and pass/fail result.
2. Any failed or skipped required gate MUST force disposition `HOLD`.
3. Non-failing warnings MAY be accepted only when explicitly documented.
4. Verification artifacts MUST independently confirm gate claims.

## Disallowed Practices

1. Recording crate-local checks as substitutes for workspace gates.
2. Marking package `complete` with unresolved required-gate failures.
3. Labeling inferred outcomes as direct execution evidence.

## Governance Adoption Rule

Starting immediately for ARCH remediation packages, disposition documents SHOULD
link to `gate-results.md` rather than restating unverified gate outcomes.
