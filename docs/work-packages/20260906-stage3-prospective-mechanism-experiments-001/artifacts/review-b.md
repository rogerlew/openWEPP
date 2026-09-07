# Independent review B — prospective authority cut 1

Static: independent scientific-authority and QA review, 2026-09-06.
Ran: read-only SHA-256, diff and line-count inspection. No builds, Rust tests,
comparators or measurements ran in this review. Review A was not consulted
before the first finding submission. Common observation edits are still mutable
and are outside this reviewed authority cut.

## Findings

No blocking finding in the immutable four-path authority/test cut.

| Surface | Reviewed result |
| --- | --- |
| `SC-SNOWENERGY-001.md:3509` | Separately names F; retains INV-088/C-056 science, typed negative capability, all independent role/path executions, forced-two-call complete-result oracle, custody, error precedence and rollback. New A multiplicity replaces historical counts only for this prospective experiment. |
| `SC-LANDSURFACEENERGY-001.md:3168` | Separately names R; retains INV-164/C-020 normative direct graph, source-order/crossability matrix, same-sweep custody and shared arithmetic. Actual signed stencils and independently derived classifications replace the old existential coverage condition only for the new experiment. |
| Both canonical additions | Preserve historical FAIL/HOLD and old retention criteria; authorize neither production promotion nor solver/physics/tolerance/ownership changes. No ambiguous package-local replacement of scientific authority. |
| Both new integration tests, line 4 | Bind the new authority sections and exposure rows. They accurately claim textual binding only; they cannot satisfy runtime connectivity, scientific parity or admission. |

## Exact reviewed identity

Base `e89befa4678eadec039b3e7f7fe0a176af8e9dc5`. Independent hash inspection
matches `authority-proposal.md`; this cut adds 204 lines and deletes zero.

| Path | SHA-256 |
| --- | --- |
| `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md` | `94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d` |
| `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` | `109be57ca5e5cac79262b9b22c5de18ec1d85eaac3a645ce85015bd4a2af78cf` |
| `tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs` | `f31b98b1d6bad044db880746be413b488478b2fab3aa4b7e5429f3c3b4a83f87` |
| `tests/integration/land_surface_energy_balance_authority_contract.rs` | `aea74e997574cb620196ace0da204bfc29f8de3b0b276d4429d85c6448663e80` |

## Non-blocking debt and execution obligations

Snow test host is 2436 lines (WARN); the proposal supplies a decomposition
rationale and follow-on split intent. LSE test host is 1833 (OK). Neither
triggers the 3000-line refactor requirement. Exact-newline phrase assertions
follow the existing textual-test style but are formatting-sensitive; future
test organization may normalize whitespace while preserving semantic phrases.

Before behavior changes, execute the focused authority tests and retain the
contract-derived expected-red evidence, exact treatment paths, and
preimplementation-gate result. Before measurement, run the complete applicable
critical-cut correctness, typed guard/error/rollback/restart, forced-reference,
real-consumer and independently reconstructed scientific/output checks. Planned
commands or textual tests are not substitutes. `cargo deny check` is required
if the exact cut affects dependencies/toolchain/workspace resolution; no such
change exists in this four-path cut. Format, warnings-denied Clippy, applicable
anti-evasion and source-size checks remain required at their owning boundary.

Gate legitimacy checked: this early review makes no package-completion claim
and permits no deferred current-scope gate to be labeled PASS. A subsequent
implementation review and dual terminal verification remain required.

QA decision: GO for prospective authority/test design on the hashes above.
Runtime implementation and comparative measurement are not admitted by this
static review alone. Protocol decision is in `protocol-review-b.md`.
