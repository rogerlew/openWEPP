# Line-count Governance

Evidence class: `Ran`

Exact current affected Rust counts:

| File | Lines | Disposition |
|---|---:|---|
| `tests/integration/vegetation_boundary_authority_contract.rs` | 2,681 | `WARN` |
| `crates/openwepp-vegetation/src/v9_state.rs` | 416 | `PASS` |
| `crates/openwepp-vegetation/src/config.rs` | 756 | `PASS` |
| `crates/openwepp-vegetation/src/lib.rs` | 41 | `PASS` |

The authority integration file exceeds the 2,000-line warning threshold but
remains below the 3,000-line hard stop. It is a historical multi-version
contract harness whose ordered V1--V9 sections share immutable path/digest
helpers and cross-version assertions. Splitting during this authority migration
would expand review scope and risk weakening historical coverage. Follow-on
intent: after this critical closure, extract version-local V3/V5 historical
provenance and V9 successor tests into path-attached sibling modules while
retaining common immutable helpers in the parent. No 3,000+ exception exists.
