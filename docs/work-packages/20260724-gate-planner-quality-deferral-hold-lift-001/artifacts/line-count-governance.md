# Line-Count Governance

Evidence class: Ran / Static.

Measured after implementation:

| File | Lines | Disposition |
|---|---:|---|
| `src/executor.rs` | 2,941 | WARN |
| `src/planner.rs` | 2,463 | WARN |
| `src/verifier.rs` | 2,794 | WARN |

All three files remain below the 3,000-line blocking ceiling. This correction
adds only one 12-line exactness helper to `planner.rs`; the executor and
verifier production modules do not grow. Splitting these trust-boundary modules
inside this defect closure would expand the write set and mix a structural
refactor with a four-symptom policy correction.

Split intent: move planner reconciliation checks into a dedicated
`planner/reconciliation.rs` module in the next package that materially extends
terminal reconciliation. Decompose executor and verifier along their existing
audit/receipt boundaries before either reaches 3,000 lines. No exemption from
the blocking ceiling is claimed.
