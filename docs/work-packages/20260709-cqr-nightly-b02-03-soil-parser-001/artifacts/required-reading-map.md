# Required-Reading Map

| Source | Why it is required | Read result |
|---|---|---|
| `AGENTS.md` | Repository invariants, CQR protocol, typed errors, and final gates. | Read before scaffold. |
| `crates/AGENTS.md` | Rust crate conventions for input-contract parsing. | Read before scaffold. |
| `docs/work-packages/AGENTS.md` | Package shape, CQR/nightly, commit, review, and verification requirements. | Read before scaffold. |
| `docs/specifications/science-contracts/AGENTS.md` | Science-contract routing for parser-derived typed inputs. | Read before scaffold. |
| `SC-INFILE-SOIL-001` | Datver grammar, exact parser error taxonomy, normalization, and no-silent-fallback rules. | Read sections 1–8 before scaffold. |
| `SC-SOIL-001` | Downstream soil-domain/fail-closed invariants protected by typed parsed fields. | Read relevant invariant guard map. |
| CQR ExecPlan and mechanical/CQR guides | Behavior-preserving CQR scope, metrics, scaffold/completion commits. | Read before scaffold. |
| ADR-0021 | Test/coverage closure threshold and real-consumer requirements. | Read before scaffold. |
| Target module and embedded tests | Existing parse branches, datver forms, and characterization seams. | Read before scaffold. |

No production parser extraction may alter datver acceptance, quoted-header
normalization, policy row arity, layer-field association, error code/message,
or parser-to-runtime field meaning.
