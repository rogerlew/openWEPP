# Line-Count Governance

Status: complete / pass

Evidence class: Ran

| File | Lines | Disposition |
| --- | ---: | --- |
| `v2.rs` | 2,953 | Below mandatory 3,000-line refactor threshold |
| `v2/amendment.rs` | 2,988 | Below mandatory threshold after receipt extraction |
| `v2/assembly.rs` | 2,133 | Existing large-module warning; bounded figure integration only |
| `v2/svg.rs` | 675 | New cohesive SVG security module |
| `v2/receipt.rs` | 120 | New cohesive receipt-root module |
| Root `AGENTS.md` | 153 | Within stated 100–160-line policy |

Receipt root serialization and validation were extracted when
`amendment.rs` crossed 3,000 lines during review. The two near-threshold
modules are recorded as decomposition debt, not closure violations.
