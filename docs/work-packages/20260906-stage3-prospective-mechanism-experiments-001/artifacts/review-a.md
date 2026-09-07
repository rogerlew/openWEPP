Static: independent review A of prospective authority cut 1, before treatment
implementation. No review-B findings were read before first submission.

## Findings

No blocking findings in the four-path authority/test cut identified by
`authority-proposal.md`. This is not an implementation, execution, or terminal
package approval. Common observation-only source edits were still mutable and
are outside this cut.

## Reviewed cut and reasoning

Base: `e89befa4678eadec039b3e7f7fe0a176af8e9dc5`.

| Exact path | SHA-256 |
| --- | --- |
| `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md` | `94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d` |
| `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` | `109be57ca5e5cac79262b9b22c5de18ec1d85eaac3a645ce85015bd4a2af78cf` |
| `tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs` | `f31b98b1d6bad044db880746be413b488478b2fab3aa4b7e5429f3c3b4a83f87` |
| `tests/integration/land_surface_energy_balance_authority_contract.rs` | `aea74e997574cb620196ace0da204bfc29f8de3b0b276d4429d85c6448663e80` |

The 204-line additive cut preserves every predecessor assertion. The new F
section at SC-SNOWENERGY-001:3509 binds the actual INV-088/C-056 authority,
limits removal to the same-invocation redundant carrier call, and retains
independent role/discovery/exact/batch/final work, typed ownership, first-error,
rollback, forced-two-call and closure proofs. The new R section at
SC-LANDSURFACEENERGY-001:3168 retains INV-164/C-020's complete direct-edge graph,
fallibility/crossability matrix, shared canonical physics, exact probe custody,
forced-complete parity and no post-start fallback. Both explicitly separate
prospective engineering decisions from historical rules and production HOLD.

R's expected probes must derive from actual signed stencils and independently
classified coordinates, with distinct authentic map/solve/iteration/sweep
identities and reconciliation of starts, completions, errors and dropped records.
Producer counters/labels cannot furnish expected values. Nonzero *completed*
replay is required on the real primary consumer; no hardcoded replacement
histogram or manufactured all-centered fixture is authorized.

The two added Rust tests bind these textual requirements and exposure rows;
they explicitly claim no mechanism execution. Their small shared assertion
shape is ordinary contract-test scaffolding, not duplicated process arithmetic.
No equations, constants, domains, tolerance/guard precedence, runtime error
taxonomy or serialization behavior change in this cut.

## Residual risk and execution still required

The complete existing graph/crossability/custody obligations are substantial;
mere declaration, counter presence or a successful one-day run cannot close
them. Treatment review must check real consumer consumption and one shared
canonical evaluator, independently reconstructed closure, and exact parity.
Every applicable current-scope gate remains required; historical expected-red
seams cannot be suppressed, relabeled PASS, or used as a reason to defer a new
scientific obligation. No validation non-deferral waiver is made here.

Ran by reviewer: path instruction discovery, four-file SHA-256 verification,
scoped `git diff --check` (PASS), diff numstat, and Rust line counts. Rust tests,
builds, Clippy and comparative measurements were NOT RUN by this reviewer.
The authority author's check reports are supporting evidence, not reviewer-run
execution. Parent must execute the named new/existing contract-binding tests
and record preimplementation admission before behavior edits.

Line counts: snow integration test 2436 (WARN), LSE integration test 1833 (OK).
Accept the authority proposal's bounded additive-test rationale and maintainers'
follow-on split intent for the existing snow test host; neither reaches 3000.

GO for prospective authority/test design on this immutable cut. Runtime
admission and terminal verification remain pending. Protocol findings are in
`protocol-review-a.md`.
