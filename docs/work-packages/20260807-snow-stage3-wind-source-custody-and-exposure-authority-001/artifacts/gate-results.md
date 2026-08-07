# Gate Results

Status: `PASS`.

Evidence mode: `Ran`.

| Requirement | Result |
| --- | --- |
| Freeze JSON plus five CLI SHA-256 checks | PASS |
| `cargo fmt --all -- --check` | PASS |
| Four affected contract targets | PASS, `30/30` |
| `markdown-doc lint` over package, contracts, roadmaps, catalog | PASS, `92` files |
| typed assurance validation | PASS; `DRAFT`; public reports `0` |
| `git diff --check` | PASS |
| Prompt archive byte identity | PASS, SHA `a8a60064...` |

No heavy correctness run is selected because the exact diff changes authority,
DRAFT bindings, and contract tests only; comparator delegation is not applicable.
