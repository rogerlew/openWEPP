# Rust Scientific Coding Standard

- **Status:** Active
- **Last updated:** 2026-05-11
- **Applies to:** all Rust crates in openWEPP

## 1) Purpose

This standard defines how openWEPP Rust code is written, documented, and
validated for scientific-model correctness and maintainability.

It implements a deliberate blend:

- Rust ecosystem conventions for formatting, documentation, testing, and linting
- scientific software reproducibility and review practices
- legacy WEPP kernel continuity (short Fortran-era symbol names)

## 2) Non-negotiable rules

1. Kernel math must preserve upstream science intent from `wepp-palimpsest`; no
   physics invention and no silent retuning.
2. Fortran-style short variable names are allowed (and often preferred) inside
   kernel translation code, but each module must define those symbols clearly.
3. Every kernel module must include a variable glossary with symbol meaning and
   units.
4. Every public API item must carry rustdoc that includes contract references.
5. Large monolithic files are disallowed; split by domain responsibility.
6. Tests should be in separate test files by default; keep production files
   focused on implementation.
7. Any module ported from legacy WEPP source must include the required
   attribution/governance header fields defined in
   `docs/governance/legacy-source-attribution-and-contributors-policy.md`.

## 3) Naming and symbol policy

### 3.1 Rust naming baseline

Follow Rust naming conventions for item kinds (`snake_case`, `UpperCamelCase`,
etc.) unless a kernel-compatibility exception applies.

### 3.2 Kernel symbol compatibility exception

Inside translated kernel calculations, preserve short canonical symbols where
they improve traceability to upstream equations/contracts (examples: `q`, `r`,
`ks`, `tau`, `tc`).

Do not create ad hoc synonyms for the same physical quantity inside a module.
Pick one canonical symbol and keep it stable.

### 3.3 Required module-level symbol glossary

Each kernel module must contain a module-level documentation table with:

- `symbol`
- `description`
- `units`
- `contract/source` (contract ID, ADR, or upstream file+section)

Recommended format:

```rust
//! # Symbol Glossary
//! | Symbol | Meaning | Units | Contract/Source |
//! |---|---|---|---|
//! | `q` | Unit discharge | m^2 s^-1 | SC-HYD-012 §3.2 |
//! | `tc` | Time of concentration | s | SC-HYD-012 §4.1 |
```

If a value is dimensionless, label units as `1`.

### 3.4 Boundary naming

At external boundaries (CLI args, JSON fields, public structs), prefer explicit
descriptive names over short kernel symbols. Keep short symbols in kernel math
paths and map at boundaries.

## 4) Comment and documentation standard

### 4.1 Comment classes

Use three explicit layers:

1. `//!` module docs for scope, symbol glossary, and contract links.
2. `///` item docs for public APIs and any non-trivial internal kernels.
3. `//` inline comments for equation blocks, invariants, and numerical choices.

### 4.2 Contract-linked comments (required)

Non-trivial scientific logic must include references to its governing contract,
ADR, or upstream source section. Comments should answer:

- what physical/algorithmic rule is being implemented
- why this formulation is used
- where the normative contract is defined

### 4.3 Function doc sections (required where applicable)

For kernel and public boundary functions, include rustdoc sections as needed:

- `# Contract`
- `# Units`
- `# Numerics`
- `# Errors`
- `# Panics`
- `# Safety` (for `unsafe`)

`# Errors`, `# Panics`, and `# Safety` follow Rust API guidance when relevant.

### 4.4 Legacy attribution header for migrated modules (required)

For modules that port legacy WEPP units, include a top-of-file `//!` attribution
block with source class, migration method, legacy replacement path, and
authorship/contributor fields per:

`docs/governance/legacy-source-attribution-and-contributors-policy.md`

At minimum for non-clean-room ports:

```rust
//! SPDX-License-Identifier: CC0-1.0
//! Origin-Class: WEPP-Core-Public-Domain
//! Migration-Method: direct-port-fixed-to-rust
//! Replaces: src/<unit>.for
//! Contract-Spec: docs/specs/source/<unit>_spec.md
//! Original-Author(s): <name or not-stated-in-source>
//! Contributors: <deduplicated semicolon-separated names>
```

## 5) File and module decomposition

### 5.1 Decompose by domain behavior

Prefer many focused files over monoliths. A practical target is one primary
kernel responsibility per file/module (for example infiltration, runoff,
erosion detachment, channel routing adapter).

### 5.2 Keep orchestration thin

Top-level module files should mostly wire domain functions/types together.
Numerical kernels should live in dedicated modules.

### 5.3 Avoid mixed concerns

Do not combine in one file:

- core physics kernels
- CLI parsing
- serialization/deserialization adapters
- long integration tests

## 6) Test organization and QA standard

### 6.1 Test placement

Prefer separate files for tests:

- unit tests in dedicated `*_tests.rs` modules or `tests/` helpers
- integration tests under `tests/`
- doc tests in rustdoc examples for public behavior

Use inline `#[cfg(test)]` blocks only for tightly local unit behavior that
would be harder to test externally.

### 6.2 Required quality gates

Before merge:

1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo test --doc`
5. `cargo deny check`

### 6.3 Scientific-model verification requirements

For kernel-impacting changes:

1. Run oracle/parity tests against `wepp-palimpsest` vectors.
2. Add/update regression tests for the exact changed behavior.
3. Validate units and domain constraints for new variables at module level.
4. Document any tolerance changes with rationale in the change artifact.

### 6.4 Review checklist for scientific PRs

Every PR touching kernel math should confirm:

1. Contract references exist and are specific.
2. Symbol glossary is present and complete (meaning + units).
3. Variable naming is consistent with upstream equations or mapped explicitly.
4. Tests isolate the changed behavior and include negative/edge cases.
5. Parity evidence is attached when numerical output can change.

## 7) Recommended tooling profile

- Prefer workspace-wide checks (`--workspace`) for consistent quality.
- Keep `clippy` strict (`-D warnings`) and use targeted `#[allow(...)]` only
  with justification comments.
- Use rustdoc examples for behavioral documentation and executable examples.

## 8) Relationship to other openWEPP governance docs

- Clean-room kernel mirror policy: `AGENTS.md` and
  `docs/decisions/0002-clean-room-model.md`
- Parity target semantics: `docs/decisions/0003-parity-semantic-not-bit.md`
- Runner/release governance: `docs/decisions/0007-openwepp-runner-and-release-governance.md`
- Legacy attribution/contributor governance:
  `docs/governance/legacy-source-attribution-and-contributors-policy.md`

This standard is implementation-focused and does not supersede those decisions.

## 9) Sources used

- Rust Style Guide: https://doc.rust-lang.org/style-guide/
- Rust Book (comments): https://doc.rust-lang.org/book/ch03-04-comments.html
- Rust Book (test organization): https://doc.rust-lang.org/book/ch11-03-test-organization.html
- Cargo Book (`cargo test`): https://doc.rust-lang.org/cargo/commands/cargo-test.html
- Rustdoc book (how to write documentation): https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html
- Rust API Guidelines (documentation, naming, dependability):
  - https://rust-lang.github.io/api-guidelines/documentation.html
  - https://rust-lang.github.io/api-guidelines/naming.html
  - https://rust-lang.github.io/api-guidelines/dependability.html
- Clippy docs:
  - https://doc.rust-lang.org/clippy/
  - https://doc.rust-lang.org/clippy/lints.html
- cargo-deny docs: https://embarkstudios.github.io/cargo-deny/
- Scientific software practice references:
  - Best Practices for Scientific Computing (PLOS Biology, 2014):
    https://journals.plos.org/plosbiology/article?id=10.1371/journal.pbio.1001745
  - Good enough practices in scientific computing (PLOS Comp Bio, 2017):
    https://journals.plos.org/ploscompbiol/article?id=10.1371/journal.pcbi.1005510
  - Ten simple rules for documenting scientific software (PLOS Comp Bio, 2018):
    https://journals.plos.org/ploscompbiol/article?id=10.1371/journal.pcbi.1006561
  - Ten simple rules for making research software more robust (PLOS Comp Bio, 2017):
    https://journals.plos.org/ploscompbiol/article?id=10.1371/journal.pcbi.1005412
