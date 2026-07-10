# Required-Reading Map

| Source | Why it is required | Read result |
|---|---|---|
| `AGENTS.md` | Repository invariants, CQR protocol, typed errors, and final gates. | Read before scaffold. |
| `crates/AGENTS.md` | Rust crate conventions for the target. | Read before scaffold. |
| `docs/work-packages/AGENTS.md` | Package shape, CQR/nightly, commit, review, and verification requirements. | Read before scaffold. |
| `docs/specifications/science-contracts/AGENTS.md` | Kernel/runtime science-contract routing. | Read before scaffold. |
| `SC-WATBAL-001` invariant guard map | Direct runtime invalid domain, ordering, and closure states remain typed hard errors. | Read relevant guard map and direct-runtime sections. |
| `SC-OFEROUTE-001` unit/guard map | Invalid or missing direct routing operands remain fail-closed. | Read relevant direct-runtime unit/guard sections. |
| CQR ExecPlan and mechanical/CQR guides | Behavior-preserving CQR scope, metrics, scaffold/completion commits. | Read before scaffold. |
| ADR-0021 | Test/coverage closure threshold and real-production-consumer requirements. | Read before scaffold. |
| Target module and `src/tests/tests_mod/direct_runtime.rs` | Existing type variants, exact output strings, and the serialized audit test seam. | Read before scaffold. |

The relevant contract constraint is semantic rather than formula-specific: this
refactor cannot weaken, default, redirect, rename, or silently recover a typed
direct-runtime guard failure. Exact display strings are treated as diagnostic
API identity for characterization.
