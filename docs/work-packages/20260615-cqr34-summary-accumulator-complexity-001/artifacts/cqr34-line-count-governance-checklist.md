# CQR34 Line-Count Governance Checklist

Evidence mode: **Static** and **Ran**

## Line Counts

| File | Before | After | Threshold |
| --- | ---: | ---: | ---: |
| `crates/openwepp-summary-accumulator/src/lib.rs` | 1222 | 1373 | 3000 |
| `docs/work-packages/README.md` | 696 | 699 | n/a |
| `docs/work-packages/cqr-burndown-execplan.md` | 772 | 772 | n/a |

## Suppression Census

- [DIRECT] Existing target-file suppressions:
  `#![allow(clippy::missing_errors_doc)]` and
  `#![allow(clippy::module_name_repetitions)]`.
- [DIRECT] No new `allow`, `expect`, `unwrap`, or `unsafe` usage was added in
  production code.
- [DIRECT] `expect` usage in the target file remains test-only.

## Governance

- [DIRECT] No touched Rust file is at or above `3000` lines.
- [DIRECT] No new dependency, unsafe block, fallback wrapper, public API, or
  serialization behavior was introduced.
