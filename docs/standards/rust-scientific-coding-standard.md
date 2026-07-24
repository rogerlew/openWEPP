# Rust Scientific Coding Standard

- **Status:** Active
- **Last updated:** 2026-06-07
- **Applies to:** all Rust crates in openWEPP

## 1) Purpose

This standard defines how openWEPP Rust code is written, documented, and
validated for scientific-model correctness and maintainability.

It implements a deliberate blend:

- Rust ecosystem conventions for formatting, documentation, testing, and linting
- scientific software reproducibility and review practices
- legacy WEPP kernel continuity (short Fortran-era symbol names)

## 2) Non-negotiable rules

1. Kernel math must preserve science intent from governing openWEPP contracts;
   no physics invention and no silent retuning.
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
8. Unsafe code and FFI/interoperability boundaries must follow
   `docs/governance/unsafe-and-interop-restrictions-policy.md`.
9. Production-path code must not ship with unresolved stubs (`todo!`,
   `unimplemented!`, or placeholder `panic!`/`unreachable!` messages such as
   `TODO`, `stub`, or `not implemented`).
10. If behavior is intentionally unavailable, return a typed error and wire
    runtime diagnostics through structured observability events rather than
    placeholder panics.

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
//! SPDX-License-Identifier: Apache-2.0
//! Origin-Class: WEPP-Core-Public-Domain
//! Migration-Method: direct-port-fixed-to-rust
//! Replaces: src/<unit>.for
//! Contract-Spec: docs/specs/source/<unit>_spec.md
//! Original-Author(s): <name or not-stated-in-source>
//! Contributors: <deduplicated semicolon-separated names>
```

### 4.5 Unsafe code and interop restrictions (required)

All unsafe/FFI work must satisfy:

1. Safe Rust by default (`#![deny(unsafe_code)]` unless boundary exception is
   documented).
2. `unsafe` only in minimal boundary modules (no broad propagation into core
   orchestration paths).
3. Every `unsafe` block has a `// SAFETY:` invariant comment.
4. Every public `unsafe fn` has rustdoc `# Safety` requirements.
5. `extern` declarations must use explicit ABI and `unsafe extern`.
6. Durable cross-language boundaries use `extern "C"` / `extern "system"`;
   avoid `extern "Rust"` for stable interop contracts.
7. FFI types require explicit layout contracts (`#[repr(C)]` /
   `#[repr(transparent)]`) and C-compatible signatures.
8. Panics/unwinds must not cross non-unwind foreign ABI boundaries.

See normative policy:
`docs/governance/unsafe-and-interop-restrictions-policy.md`.

## 5) File and module decomposition

### 5.1 Decompose by domain behavior

Prefer many focused files over monoliths. A practical target is one primary
kernel responsibility per file/module (for example infiltration, runoff,
erosion detachment, channel routing adapter).

### 5.1.1 `.rs` file line-count governance (required)

Line count is governed through coding standards and code review (not CI signal
quality alone):

- `WARN threshold`: 2000 lines in a single `.rs` file.
- `Required refactor threshold`: 3000 lines in a single `.rs` file.

Review policy requirements:

1. Files at or above 2000 lines must include an explicit decomposition note in
   review/disposition artifacts describing current boundary rationale and
   follow-on split intent.
2. Files at or above 3000 lines must be split before closure/disposition unless
   the file is generated data/code or fixture content explicitly marked as an
   exception with rationale.
3. Exception approvals for 3000+ non-generated files must be documented in
   package artifacts with owner and sunset plan.

### 5.2 Keep orchestration thin

Top-level module files should mostly wire domain functions/types together.
Numerical kernels should live in dedicated modules.

### 5.3 Avoid mixed concerns

Do not combine in one file:

- core physics kernels
- CLI parsing
- serialization/deserialization adapters
- long integration tests

## 6) Ownership, borrowing, and mutation patterns (opinionated)

### 6.1 State ownership contract

1. Each mutable state surface has one owner at a time in a timestep.
2. Orchestrators own mutable run state; kernels do not own global/shared state.
3. Kernels read state through immutable borrows and emit typed deltas/fluxes.
4. State mutation happens at explicit apply/writeback points, not ad hoc in math
   helpers.

### 6.2 Preferred function signature shapes

Use one of these patterns by default:

1. Pure compute (read-only):

```rust
pub fn compute(input: &KernelInput, state: &KernelState) -> Result<KernelFlux, KernelError>
```

2. Explicit apply/writeback:

```rust
pub fn apply(state: &mut KernelState, delta: &KernelDelta) -> Result<(), KernelError>
```

3. Thin orchestrator step:

```rust
pub fn step(ctx: &StepContext, state: &mut RunState) -> Result<StepReport, StepError>
```

Avoid multi-`&mut` signatures over related state surfaces when a single owned
state struct plus explicit sub-borrows can express the same operation more
clearly.

### 6.3 Mutation and borrowing rules

1. Prefer read -> compute -> apply phases in order.
2. Keep mutable borrow lifetimes short; do not hold `&mut` across unrelated
   work.
3. No hidden mutation in getters/converters.
4. No `RefCell`/`Cell`/`Mutex`/`RwLock` inside core physics kernels.
5. Interior mutability is allowed only in bounded infrastructure surfaces
   (for example metrics/cache shims) with rationale comments.
6. Do not clone state to bypass borrow checker constraints in hot paths.
   If cloning is intentional, document rationale and expected cost.

### 6.4 Struct templates for scientific kernels

Use explicit input/state/delta/diagnostic surfaces with unit-bearing fields.

```rust
#[derive(Debug, Clone)]
pub struct InfiltrationInput {
    /// mm
    pub rain_mm: f64,
}

#[derive(Debug, Clone)]
pub struct InfiltrationState {
    /// mm
    pub storage_mm: f64,
}

#[derive(Debug, Clone)]
pub struct InfiltrationDelta {
    /// mm
    pub d_storage_mm: f64,
}

#[derive(Debug, Clone)]
pub struct InfiltrationDiagnostics {
    /// mm
    pub closure_residual_mm: f64,
}

#[derive(Debug, Clone)]
pub struct InfiltrationOutcome {
    pub delta: InfiltrationDelta,
    pub diagnostics: InfiltrationDiagnostics,
}
```

For orchestration boundaries, prefer one owned aggregate state plus focused
sub-structs (for example `HydrologyState`, `ErosionState`, `RoutingState`)
instead of passing many unrelated mutable scalars through long call chains.

### 6.5 Module organization templates

Kernel-domain crate/module template:

```text
src/
  lib.rs            # public exports only
  types.rs          # input/state/delta/diagnostic structs
  kernel.rs         # compute (read-only) logic
  apply.rs          # writeback/apply logic
  invariants.rs     # closure/physical checks
  errors.rs         # typed error enums
```

Orchestrator crate/module template:

```text
src/
  lib.rs
  state.rs          # owned run-state aggregates
  schedule.rs       # ordering/timestep policy
  dispatch.rs       # kernel call wiring
  io_boundary.rs    # HBP/parquet/CLI boundary mapping
  errors.rs
```

`lib.rs` should re-export stable API surfaces and avoid embedding substantial
kernel math.

### 6.6 Borrow-checker conflict resolution order

When borrow conflicts appear, resolve in this order:

1. Narrow borrow scope and move last-use points earlier.
2. Extract immutable scalars before mutable writeback.
3. Split state structs by ownership domain.
4. Introduce IDs/indices instead of long-lived references.
5. Use interior mutability only at approved infrastructure boundaries.

## 7) Test organization and QA standard

### 7.1 Test placement

Prefer separate files for tests:

- unit tests in dedicated `*_tests.rs` modules or `tests/` helpers
- integration tests under `tests/`
- doc tests in rustdoc examples for public behavior

Use inline `#[cfg(test)]` blocks only for tightly local unit behavior that
would be harder to test externally.

### 7.2 Required quality gates

Before disposition, execute the exact terminal plan selected under
`testing-and-gate-strategy.md`. For affected Rust surfaces the plan includes
formatting, warnings-denied Clippy, placeholder/stub scanning, affected tests
and doctests, and applicable A0/A1/A3 and specialized gates. TESTGATE records
coverage/CRAP as `DEFERRED_TO_QUALITY_CI`; explicit CQR/module-test-enhancement
packages retain their owned metrics. Manifest, lock, dependency-policy, or
toolchain dependency changes select cargo-deny. Critical changes, campaign
closure, and release select full workspace correctness regression, full
doctests, and cargo-deny.

The conservative full command set is reserved for critical, campaign, release,
or explicit rollback plans. It is not an implementation-package default.

### 7.3 Scientific-model verification requirements

For kernel-impacting changes:

1. Run comparator/parity tests with confidence-tier interpretation per
   ADR-0011 (single OFE + daily as high confidence; hourly/watershed as
   investigation signals).
2. Add/update regression tests for the exact changed behavior.
3. Validate units and domain constraints for new variables at module level.
4. Document any tolerance changes with rationale in the change artifact.

### 7.4 Review checklist for scientific PRs

Every PR touching kernel math should confirm:

1. Contract references exist and are specific.
2. Symbol glossary is present and complete (meaning + units).
3. Variable naming is consistent with upstream equations or mapped explicitly.
4. Tests isolate the changed behavior and include negative/edge cases.
5. Parity evidence is attached when numerical output can change.
6. Every cited contract Test-Vector Obligation maps to a named, implemented
   test (§7.6).

### 7.5 Test case-family taxonomy (Normative)

Contract-derived and kernel tests are designed against a fixed family set, not
re-derived per contract. Every test set covers the applicable families; a
contract whose "Test-Vector Obligations" omit an applicable family is
non-compliant. Obligation sections reference these letters instead of restating
the taxonomy.

- **A. Nominal in-domain** — representative valid input; assert the primary
  equation/output is finite and within sign/bounds.
- **B. Boundary / near-threshold** — at and just across each guard threshold
  (e.g. `tau_f == taucn`, `theta == FC`).
- **C. Branch** — each distinct algorithmic branch/case is reached (e.g.
  detachment vs deposition; multi-OFE case 1–4).
- **D. Domain-reject** — each out-of-domain input fails closed with the exact
  typed status/error code named in the contract.
- **E. Missing-symbol** — each required symbol absent ⇒ typed hard failure
  (named code); no default.
- **F. Non-finite** — `NaN`/`±Inf` in each required input ⇒ typed hard failure.
- **G. Conservation / continuity residual** — the closure identity holds within
  the contract tolerance for valid input, and a forced residual violation fails
  with the named status.
- **H. Fail-closed posture** — assert *no* silent clamp/default/pass-through on
  any A–G failure path.

### 7.6 Obligation-to-test binding (Normative)

This obligation-to-test binding is universal correctness authority. ADR-0021's
module coverage thresholds and CRAP ≤ 30 bound are binding for explicit
CQR/module-test-enhancement objectives and observational elsewhere under
ADR-0041.

- Every entry in a contract's "Test-Vector Obligations" / "Contract-Derived Test
  Vectors" maps to a named, implemented test (one obligation ⇒ one or more
  `#[test]` functions).
- The mapping is recorded in an obligation→test map (a work-package evidence
  artifact; see
  [module-test-enhancement-authoring-guide.md](module-test-enhancement-authoring-guide.md)).
- This binding must be machine-checked. The external-authority pattern
  (`docs/specifications/external-authority/required-suite-obligations.json` +
  `auth11_required_suite_obligation_guards_contract`) is the model; an
  equivalent guard binding ordinary `SC-*` obligations to tests is required.
  Until that guard ships, the binding is a hard review-checklist item (§7.4#6)
  and a work-package exit gate.
- A contract obligation with no bound test is a governance failure, not a
  backlog item — the same posture as an ownerless `HOLD`.

### 7.7 Test-authoring mechanics

- **Naming.** Descriptive snake_case stating the asserted law/guard, not the
  mechanics. Negative cases end `_fails_with_<status>` or `_rejects_<condition>`.
- **Fixtures.** Prefer table-driven/parametrized vectors over copy-pasted bodies
  — one row per case family (§7.5). Constitutive fixtures live under
  `tests/fixtures/constitutive/<suite_id>/` with the integrity metadata required
  by the external-authority suite schema.
- **Property-based testing.** For range-invariants — conservation, monotonicity
  (e.g. monotone inter-storm recession), boundedness (`0 <= x <= 1`), sign —
  assert the law over generated inputs in addition to example vectors. Example
  vectors pin named guard arms; property tests defend the law across the domain.
- **Float comparison.** Never `==` on floats. Compare against the contract's
  named tolerance (abs/rel/mixed with units); record the tolerance's contract
  source in the assertion message.
- **Determinism.** Tests are deterministic and local-only: no wall-clock, no
  network, no filesystem outside the test tempdir, fixed seeds for any
  generator (reinforces `tests/AGENTS.md`).

### 7.8 Non-kernel layer case families

The §7.5 taxonomy extends to layers without kernel math:

- **Parsers (`openwepp-input-contract`).** Per field/record: valid nominal;
  malformed token; truncated/short record; out-of-range value; cardinality
  underflow and overflow vs declared count; encoding/whitespace edge;
  duplicate/missing required key. All invalid inputs fail closed with a typed
  parse error; no silent default.
- **Scheduler / runner (orchestration).** Canonical phase ordering preserved;
  required ingress symbol missing ⇒ typed hard fail before the consuming phase;
  activation-flag on *and* off paths (e.g. `*_wave2_enabled = 0|1`); multi-OFE
  (`nelem > 1`) vs single-OFE; producer writeback observable by the consuming
  phase.
- **Output / aggregation.** Schema conformance; closure identity (the WB/mass
  identities) holds on the emitted surface; empty/degenerate input handled.

## 8) Recommended tooling profile

- Prefer workspace-wide checks (`--workspace`) for consistent quality.
- Keep `clippy` strict (`-D warnings`) and use targeted `#[allow(...)]` only
  with justification comments.
- Use rustdoc examples for behavioral documentation and executable examples.

### 8.1 Stub detection and runtime observability requirements

Treat stub detection as a two-layer gate:

1. Clippy catches explicit placeholder macros (`todo!`, `unimplemented!`).
2. Pattern scan catches placeholder panic/unreachable messages that can evade
   macro-only linting.

If a production-path branch is intentionally unavailable:

1. Return a typed error variant (no placeholder panic).
2. Emit a structured observability event aligned with
   `docs/specifications/subsystems/observability/trace-event-schema.md`
   (`guard_violation` and/or `intent_rejected` as applicable).
3. Include contract/invariant identity in the emitted payload so failures are
   attributable in replay and diagnostics workflows.

## 9) Relationship to other openWEPP governance docs

- Architecture-first + provenance policy: `AGENTS.md` and
  `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- Parity target semantics: `docs/decisions/0003-parity-semantic-not-bit.md`
- Runner/release governance: `docs/decisions/0007-openwepp-runner-and-release-governance.md`
- Legacy attribution/contributor governance:
  `docs/governance/legacy-source-attribution-and-contributors-policy.md`
- Unsafe/interop governance:
  `docs/governance/unsafe-and-interop-restrictions-policy.md`

This standard is implementation-focused and does not supersede those decisions.

## 10) Sources used

- Rust Style Guide: https://doc.rust-lang.org/style-guide/
- Rust Book (ownership and borrowing): https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
- Rust Book (comments): https://doc.rust-lang.org/book/ch03-04-comments.html
- Rust Book (test organization): https://doc.rust-lang.org/book/ch11-03-test-organization.html
- Cargo Book (`cargo test`): https://doc.rust-lang.org/cargo/commands/cargo-test.html
- Rustdoc book (how to write documentation): https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html
- Rust API Guidelines (documentation, naming, dependability):
  - https://rust-lang.github.io/api-guidelines/documentation.html
  - https://rust-lang.github.io/api-guidelines/flexibility.html
  - https://rust-lang.github.io/api-guidelines/naming.html
  - https://rust-lang.github.io/api-guidelines/dependability.html
- Clippy docs:
  - https://doc.rust-lang.org/clippy/
  - https://doc.rust-lang.org/clippy/lints.html
- Rust Reference (unsafe/FFI specifics):
  - https://doc.rust-lang.org/reference/items/external-blocks.html
  - https://doc.rust-lang.org/reference/unsafety.html
  - https://doc.rust-lang.org/reference/type-layout.html
  - https://doc.rust-lang.org/reference/panic.html
- Rustonomicon (unsafe/FFI guidance):
  - https://doc.rust-lang.org/nomicon/ffi.html
  - https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html
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
