# Function-Region Extraction Method

Static: `function-regions.tsv` is the complete 111-row production-function
record for exact head `68e9b747`. Its columns are declaration line, cargo-crap
function name, covered LLVM regions, total LLVM regions, and percentage. The
file intentionally has no header so its exact SHA-256 remains
`9dc0bf13209f6a6642905ffbfb1a86e962ad4555a0fcc84b8e0333cc45b8fcd6`.

The extraction uses the cargo-crap production inventory restricted to
`pre_heavy.rs` declaration lines 1-1,743. Each cargo-crap row is matched to
LLVM records by exact declaration line and the Rust-v0 length-prefixed leaf
symbol. Generic monomorphizations are unioned by exact source-region geometry:
start line/column, end line/column, file ID, expanded-file ID, and region kind.
A deduplicated region is covered when any matched instantiation has a positive
execution count. This produced 111 mappings with zero failures.

The production aggregate independently restricts LCOV `DA` records and LLVM
regions to lines 1-1,743. The final figures are 1,324/1,378 lines and
1,886/2,104 regions. The raw LLVM JSON and matching cargo-crap JSON hashes are
bound in `coverage-closure.md`.
