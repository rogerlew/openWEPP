# Terminal Gate Evidence

Evidence class: `Ran`

The package-authorized terminal runner executed the conservative sequence once
on the frozen source and stopped on no failure:

| Gate | Result |
|---|---|
| `cargo fmt --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| `cargo nextest run --workspace --profile full` | PASS: 2,118 passed, 28 slow, 5 skipped across 195 binaries in 564.225 seconds |
| `cargo deny check` | PASS: advisories, bans, licenses, and sources OK |
| `bash tools/release/run_adjudicated_crap_gate.sh --base-ref 5613bb4d63b38a5c64cca08be6f089999f03987d` | PASS: 2 raw, 2 adjudicated, 0 actionable, 4 touched production files |

Fresh CRAP acquisition ran from 2026-07-18T11:39:22Z through 12:23:39Z.
It measured 10,404 production entries from 244 sources and 461 measurement
inputs. Closure eligibility is true. Pre/post Git status was identical outside
ignored generated `target/` evidence.

Principal evidence hashes:

- source manifest: `5dbab66a4d857c97e9d0f622ad642c316a8cfc00060bf468f0e5f59c20c63ebb`;
- adjudicated report JSON: `b9b7e18dcb211eb6369c6f1bd859ca1e81f158eda9660ac924d071283a020d15`;
- workspace CRAP JSON: `07a628dd926bc8d2128cf012d8f91557eca5864c63242c3b52428827b5baeaa7`; and
- workspace LCOV: `090ed6bc432e1342960450d072ef457cedb40556501b65a8965a405c65ddaca4`.

The two remaining raw rows are the existing `CQR-LOW-L08` and `CQR-LOW-L11`
adjudications outside the touched crate. No actionable touched or untouched row
remains.

## Documentation Gates

After closure documents were written, the docs-maintainer workflow ran:

- `markdown-doc lint` against `docs/ROADMAP.md`, the work-package catalog, the
  predecessor package, and this package: PASS, initially 32 files and finally
  33 files after the terminal-verification artifact, with 0 errors and 0
  warnings in both runs.
- `uk2us` preview against all changed package/roadmap/catalog Markdown: REVIEWED.
  No rewrite was applied because the preview proposed unsafe identifier and
  acronym changes such as `CoE` to `ce`; unrelated historical spelling changes
  were outside this package.

These documentation-only closure updates do not alter the measured production
source manifest or invalidate the terminal Rust/CRAP evidence.
