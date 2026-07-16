# Line-Count Governance

Evidence class: Static

`crates/openwepp-assurance/src/v2.rs` exceeds the package's 2,000-line warning
threshold but remains below the 3,000-line closure block. This package adds
only the thin repository delegation for normalization; the transaction itself
is isolated in `v2/normalization.rs`. Moving the delegation alone would obscure
the repository API without materially decomposing the existing model.

The owned split intent is to separate the schema/model declarations and source
validation responsibilities now co-located in `v2.rs` before that file reaches
3,000 lines. That decomposition is outside this editorial fast-path objective
and must preserve the current repository API and validation contracts.

Final touched-file counts before heavy closure:

| Rust file | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-assurance/src/v2.rs` | 2,841 | WARN; split before 3,000 lines |
| `crates/openwepp-assurance/src/v2/assembly.rs` | 1,948 | Below threshold |
| `crates/openwepp-assurance/src/v2/normalization.rs` | 1,551 | Below threshold |
| `crates/openwepp-assurance/src/v2/confined.rs` | 1,380 | Below threshold |
| `crates/openwepp-assurance/src/cli.rs` | 798 | Below threshold |
| `crates/openwepp-assurance/src/error.rs` | 91 | Below threshold |
| `crates/openwepp-assurance/src/lib.rs` | 25 | Below threshold |
| `tests/integration/assurance_v2_assembly_contract.rs` | 759 | Below threshold |
| `tests/integration/assurance_v2_normalization_contract.rs` | 417 | Below threshold |

The scientific-assurance build maintainer owns the split, with 3,000 lines as
the blocking sunset rather than a license for further unrelated growth.
