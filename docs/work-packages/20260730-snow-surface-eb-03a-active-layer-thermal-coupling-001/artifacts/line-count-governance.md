# Line-Count Governance

Status: `complete / pass`

Evidence mode: `Ran`

Terminal line counts:

| File | Lines |
| --- | ---: |
| meteorology surface-energy module | 1,472 |
| hillslope Stage 3 support types | 798 |
| runoff-reconciliation implementation | 2,142 |
| direct-runner day/trace builder | 2,297 |
| EB-03 contract test | 180 |
| EB-03 runtime test | 382 |

The production modules are established split-module surfaces rather than new
standalone files. EB-03A extracted the conduction-diagnostic accumulation
helper when strict Clippy identified a function-length regression. Final
`cargo clippy --workspace --all-targets -- -D warnings` passes, including
function length and argument-count governance. No package-specific hard file
line ceiling was exceeded.
