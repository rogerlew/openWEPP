# ASSURE-04A Line-Count Governance

Status: PASS with dispositioned 2,000-line warning

Evidence class: Ran

| Touched Rust file | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-assurance/src/cli.rs` | 221 | PASS |
| `crates/openwepp-assurance/src/lib.rs` | 17 | PASS |
| `crates/openwepp-assurance/src/v2.rs` | 2,042 | WARN-DISPOSITIONED |
| `tests/integration/assurance_dossier_build_contract.rs` | 686 | PASS |
| `tests/integration/assurance_v2_source_contract.rs` | 709 | PASS |

`v2.rs` crosses the 2,000-line warning by 42 lines after the terminal verifier
required an explicit missing/null/value representation for every schema-
required nullable field. This is dispositioned rather than split solely to
hide the warning: the representation and its semantic checks are part of the
same cohesive source-admission contract, the file remains well below the
3,000-line exception threshold, and fresh CRAP assessed the amended structure
with zero actionable rows. No 3,000-line exception exists or is required.

Future planning, assembly, and publication logic must use their own
ASSURE-04B/C/D modules rather than extending this file without a fresh
line-count disposition.
