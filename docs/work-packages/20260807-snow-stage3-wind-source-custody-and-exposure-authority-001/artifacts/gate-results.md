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
| Provider/fixture CLI SHA-256 equality | PASS, `4/4` |
| Provider parquet `vs` to CLI `w-vl` serialization | PASS, `61,364/61,364` daily rows; zero NaNs or mismatches |
| Follow-on four affected contract targets | PASS, `30/30` |
| Follow-on Markdown lint | PASS, package `32`, contracts `58`, roadmaps/catalog `3` |
| Follow-on typed assurance validation | PASS; generation `b756fd...`; `DRAFT`; public reports `0` |
| Authority-claim amendment affected targets | PASS, `30/30` |
| Authority-claim amendment Markdown lint | PASS, package `32`, contracts `58`, roadmaps/catalog `3` |
| Authority-claim amendment fmt/diff hygiene | PASS |

No heavy correctness run is selected because the exact diff changes authority,
DRAFT bindings, and contract tests only; comparator delegation is not applicable.
