# Type-System and Symbolic Logic Strategy

Status: Draft
Last updated: 2026-05-23
Evidence mode: Static
Scope: openWEPP contract congruence and verification strategy (no required churn in wepp-forest)

## 1. Problem Statement

openWEPP already treats `SC-*` science contracts as authority. The next step is
to make those contracts explicit in code and machine-checkable, so we can prove
more correctness properties without relying on repeated full simulation runs.

Target outcome:
- contract-to-code congruence is enforced by build/verification gates,
- invalid states become unrepresentable or explicitly rejected,
- closure and guard obligations are provable at component boundaries,
- wepp-forest remains comparator/provenance authority, not a moving dependency.

## 2. Reality Check: What "No Simulation Run" Can and Cannot Mean

What can be proven without full model runs:
- Interface completeness: required fields, phases, and producer-consumer seams.
- Domain safety: finite values, unit compatibility, bounds, and guard branches.
- Local algebraic invariants: per-step closure identities and branch obligations.
- Protocol conformance: call ordering, state transitions, and legal mutation lanes.

What still needs execution evidence:
- Long-horizon emergent behavior under realistic forcing traces.
- Cross-component calibration confidence under broad scenario distributions.
- Legacy comparator deltas for semantic parity acceptance.

Conclusion:
- We can dramatically reduce dynamic validation surface, but not eliminate it.
- The realistic goal is "simulation-minimized acceptance," not "simulation-free forever."

## 3. External Deep Dive

This section evaluates the six anchors as design patterns for openWEPP.

## 3.1 rustls

Observed pattern:
- `rustls::ConfigBuilder` uses typestate (`WantsVersions`, `WantsVerifier`, etc.)
  to force required configuration steps at compile time.
- Project posture is safety-by-default with explicit secure defaults and
  memory-safe protocol processing.

Why it matters for openWEPP:
- This is a strong template for "required contract decisions must be made exactly once."
- It maps well to kernel invocation and writeback readiness states.

Transferable to openWEPP:
- Typestate builders for contract-complete kernel context construction.
- Compile-time prevention of partial or ambiguous contract activation.
- "Dangerous" escape hatches isolated and explicit when governance permits.

Limitation:
- rustls is not a theorem-proving framework; it is type-driven API hardening.

## 3.2 Embedded Rust HALs

Observed pattern:
- Embedded Rust guidance formalizes peripherals as state machines.
- HAL APIs encode pin/peripheral state in types, with legal transitions as methods.
- HALs intentionally support type erasure (`into_dynamic`, erased pins) when runtime
  flexibility is needed, trading guarantees for controlled dynamism.

Why it matters for openWEPP:
- This is directly analogous to simulation lifecycle phases and projection modes.
- It demonstrates a practical split: compile-time strict path plus explicit runtime mode.

Transferable to openWEPP:
- Encode simulation phases and kernel preconditions as typestate tokens.
- Provide explicit downgrade paths for dynamic/legacy bridges, never implicit.
- Keep downgrade sites auditable and narrowly scoped.

Limitation:
- Strong static guarantees require disciplined API design and ownership planning.

## 3.3 Session Types (Protocols as Types)

Observed pattern:
- Libraries like `dialectic` and `session_types` encode communication protocols
  as Rust types (`send`, `recv`, `offer`, `choose`, dual protocols).
- Correct protocol order is enforced by the compiler.

Why it matters for openWEPP:
- openWEPP has many protocol-like seams: parser -> projection -> kernel ->
  writeback -> accumulator -> comparator metadata.
- These can be modeled as typed protocol traces rather than ad-hoc call sequences.

Transferable to openWEPP:
- Encode seam contracts as protocol traits and state transitions.
- Prevent illegal phase/order combinations at compile time.
- Make "who may mutate what, when" explicit in types.

Limitation:
- Session typing adds abstraction cost; apply only to high-risk seams, not every function.

## 3.4 Kani + Firecracker (Symbolic Model Checking in Production Rust)

Observed pattern:
- Kani verifies Rust harnesses with symbolic inputs (`kani::any`) and assertions.
- Firecracker reports production use of Kani harnesses in CI and bug discovery,
  including a rounding issue in rate-limiter logic.
- Kani supports contract-style specs (`requires`, `ensures`, `proof_for_contract`,
  `stub_verified`) for compositional verification.

Why it matters for openWEPP:
- This is the closest match to "prove contract obligations without full simulation."
- It is practical for finite/bounded kernels and boundary invariants.

Transferable to openWEPP:
- Verify closure, domain guards, and branch safety using symbolic harnesses.
- Use contract stubbing to scale proofs across layered kernels.
- Keep runtime code unchanged while adding proof modules under `cfg(kani)`.

Limitations (important):
- Kani has feature limits; concurrency is out of scope and analyzed as sequential.
- Bounded model checking is not full theorem proving over unbounded behavior.

## 3.5 Verus

Observed pattern:
- Verus adds specification/proof constructs to Rust syntax and statically proves
  correctness using SMT-backed reasoning.
- Goal is full functional correctness for low-level systems code.
- Supports only a Rust subset and is under active development.

Why it matters for openWEPP:
- Best candidate for proving stronger functional properties on selected kernels
  where Kani bounds become too weak or expensive.
- Especially relevant for high-risk arithmetic and alias-sensitive logic.

Transferable to openWEPP:
- Deep proofs for a small set of contract-critical kernels.
- Proof-carrying specs for invariants that must not regress.
- Candidate for "reference proof implementation" per domain (not blanket adoption).

Limitation:
- Proof authoring cost is high; use surgically, not workspace-wide by default.

## 3.6 RustBelt

Observed pattern:
- RustBelt provides machine-checked semantic foundations for Rust safety claims,
  unsafe encapsulation, and related aliasing models (e.g., Stacked Borrows).
- It is foundational research, not a drop-in engineering toolchain.

Why it matters for openWEPP:
- It clarifies what assumptions are trustworthy in unsafe boundaries and aliasing.
- It supports governance rules for unsafe code and boundary soundness claims.

Transferable to openWEPP:
- Use as policy foundation for unsafe/FFI restrictions.
- Align contract language with explicit aliasing and mutation discipline.

Limitation:
- Not an operational verifier you can run directly on openWEPP codebase.

## 4. Strategy for openWEPP

## 4.1 Core Position

Keep canonical authority in existing `SC-*` markdown contracts, but add an
executable contract layer that is:
- generated or hand-maintained in lockstep,
- enforced at compile time where possible,
- symbolically/deductively verified where types are insufficient.

## 4.2 Four Assurance Layers

Layer A: Type-level contract enforcement (compile time)
- Extend existing typed surfaces in:
  - `openwepp-unit-boundary`
  - `openwepp-sim-contract`
  - `openwepp-kernel-contract`
- Add typestate for phase progression and seam readiness.
- Make illegal contract states unconstructable.

Layer B: Runtime guard normalization (already present, expand systematically)
- Continue typed `ClosureViolation` / `SimulationStatus` model.
- Ensure every `SC-*` invariant has explicit guard mapping to code.
- Reject silent fallback behavior in production paths.

Layer C: Symbolic verification harnesses (Kani)
- Add proof harnesses for contract-critical invariants:
  - closure equations,
  - denominator/bounds safety,
  - branch completeness for degenerate states,
  - anti-regression of explicit error classes.
- Use `proof_for_contract` and `stub_verified` for scalability.

Layer D: Selective deductive proofs (Verus pilots)
- Apply only to highest-risk kernels where bounded proofs are insufficient.
- Treat as a narrow deep-assurance lane, not default implementation mode.

## 4.3 No-Churn Constraint Against wepp-forest

This strategy does not require changing wepp-forest:
- wepp-forest remains provenance/comparator anchor.
- openWEPP internal contracts become stricter and more machine-checkable.
- parity workflows remain for acceptance confidence, but fewer dynamic runs are
  needed to catch contract-shape errors and local invariant breaches.

## 5. Proposed Contract-to-Code Congruence Model

## 5.1 Congruence Unit

Define a congruence unit per invariant:
- `contract_id` (for example `SC-WATBAL-001`)
- `invariant_id` (for example `INV-WATBAL-004`)
- typed state surface(s) involved
- guard function(s)
- proof harness(es), if applicable
- evidence class (`Static`/`Ran`) and gate severity

## 5.2 Required Mapping Table

Maintain a machine-readable mapping (file or generated module) with fields:
- `invariant_id`
- `owner_crate`
- `owner_symbols`
- `guard_kind` (`compile_time`, `runtime`, `symbolic`, `governance`)
- `guard_path` (function/module path)
- `proof_path` (optional)
- `status` (`implemented`, `partial`, `missing`)

This becomes the source for "contract closure coverage" reporting.

## 5.3 Build Gates

Minimum gate sequence:
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. contract congruence checker (no orphan invariants)
5. Kani harness suite for selected invariants
6. comparator tier routing checks (unchanged policy)

## 6. Recommended Implementation Roadmap

## Phase 1: Typestate and Symbol Surface Hardening

Actions:
- Add typestate phase tokens for kernel invocation/writeback in
  `openwepp-kernel-contract`.
- Introduce explicit capability tokens for mutation lanes (read/compute/apply).
- Add compile-time checks that required symbols for each seam are present.

Exit criteria:
- key seam mis-ordering cannot compile.
- boundary payloads cannot be partially constructed for required contracts.

## Phase 2: Invariant-to-Guard Coverage Closure

Actions:
- For each active `SC-*`, enumerate invariants into congruence map.
- Link each invariant to existing guard code or open gap.
- Expand closure/domain primitives only where needed by active invariants.

Exit criteria:
- every invariant in active `SC-*` has explicit guard classification.
- no "unmapped invariant" remains for promotable surfaces.

## Phase 3: Kani Pilot on Water-Balance and Writeback Surfaces

Actions:
- Start with `SC-WATBAL-001` and kernel writeback boundaries.
- Add `cfg(kani)` proof modules for:
  - closure residual relation,
  - stress-factor bounds and zero-demand branch semantics,
  - domain error typing guarantees.
- Use `stub_verified` to compose proofs across helper layers.

Exit criteria:
- Kani harness suite runs in CI at acceptable cost.
- at least one previously test-only invariant is proven symbolically.

## Phase 4: Verus Feasibility Slice

Actions:
- Choose one high-value kernel routine with bounded interface and heavy
  arithmetic/alias risk.
- Build a standalone Verus pilot artifact proving core functional properties.
- Evaluate annotation burden and maintenance cost.

Exit criteria:
- go/no-go decision for selective Verus adoption based on evidence, not opinion.

## 7. Risks and Controls

Risk: Verification tooling complexity outpaces team bandwidth.
- Control: keep Kani default; gate Verus to narrow pilots.

Risk: Contract prose and code drift apart.
- Control: enforce congruence map in CI as a hard gate for contract-affecting PRs.

Risk: False confidence from bounded proofs.
- Control: keep comparator-tier policy and clearly label proof scope assumptions.

Risk: Over-modeling by type system harms iteration speed.
- Control: use explicit downgrade points (embedded-HAL style), never hidden fallbacks.

## 8. Decision Summary

Recommended direction:
- Adopt a layered approach: typestate + runtime typed guards + Kani symbolic proofs.
- Use Verus selectively for deep proofs where justified.
- Treat RustBelt as foundational policy input, not runtime tooling.

This is the shortest path to "contract closure in code" with strong guarantees
and minimal disruption to existing provenance and comparator workflows.

## 9. References

rustls:
- https://docs.rs/rustls/latest/rustls/struct.ConfigBuilder.html
- https://rustls.dev/
- https://github.com/rustls/rustls

Embedded Rust HAL patterns:
- https://docs.rust-embedded.org/book/design-patterns/hal/gpio.html
- https://docs.rust-embedded.org/book/static-guarantees/state-machines.html
- https://docs.rust-embedded.org/book/static-guarantees/typestate-programming.html
- https://docs.rs/stm32f4xx-hal/latest/stm32f4xx_hal/gpio/index.html
- https://docs.rs/atsamd-hal/latest/atsamd_hal/sercom/spi/index.html

Session types:
- https://docs.rs/dialectic/latest/dialectic/
- https://docs.rs/dialectic/latest/dialectic/tutorial/index.html
- https://docs.rs/dialectic/latest/dialectic/types/index.html
- https://docs.rs/session_types/latest/session_types/
- https://arxiv.org/abs/2009.13619

Kani and Firecracker:
- https://model-checking.github.io/kani/
- https://model-checking.github.io/kani/usage.html
- https://model-checking.github.io/kani/rust-feature-support.html
- https://model-checking.github.io/kani/reference/experimental/contracts.html
- https://model-checking.github.io/kani-verifier-blog/2023/08/31/using-kani-to-validate-security-boundaries-in-aws-firecracker.html
- https://model-checking.github.io/kani-verifier-blog/2022/07/13/using-the-kani-rust-verifier-on-a-firecracker-example.html
- https://github.com/firecracker-microvm/firecracker/blob/main/docs/design.md

Verus:
- https://github.com/verus-lang/verus
- https://verus-lang.github.io/verus/guide/
- https://arxiv.org/abs/2303.05491
- https://www.microsoft.com/en-us/research/publication/verus-a-practical-foundation-for-systems-verification/

RustBelt:
- https://plv.mpi-sws.org/rustbelt/
- https://plv.mpi-sws.org/rustbelt/popl18/
- https://plv.mpi-sws.org/rustbelt/stacked-borrows/
- https://plv.mpi-sws.org/rustbelt/rbrlx/
