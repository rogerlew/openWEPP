# ASSURE-02 Line-Count And Code-Quality Disposition

Evidence class: Ran

## Applicability

ASSURE-02 is documentation-only. No changed or untracked `.rs` file exists.
Rust line-count, coverage, cyclomatic-complexity, CRAP, formatting, Clippy,
workspace-nextest, and deny gates are therefore `N/A`, not silently skipped
implementation evidence.

## Principal Document Sizes

| Document | Lines | Bytes |
| --- | ---: | ---: |
| ADR-0038 | 80 | 4,535 |
| V&V strategy | 258 | 13,117 |
| V2 architecture | 186 | 9,687 |
| Lifecycle contract | 232 | 13,335 |
| Source/build contract | 198 | 9,572 |
| Report standard | 286 | 13,099 |
| Migration plan | 159 | 11,944 |
| Implementation roadmap | 161 | 7,688 |
| Groundwater manuscript prototype | 342 | 17,547 |
| Package contract | 208 | 10,326 |

Detailed implementation rules are separated from the strategy into the
architecture, lifecycle, source/build, standard, and prospective package
documents so the V&V strategy remains a bounded philosophy document rather
than a schema or compiler specification.
