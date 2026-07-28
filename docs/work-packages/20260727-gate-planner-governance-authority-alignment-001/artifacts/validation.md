# Validation

Evidence class: `Ran`.

## Current Results

| Check | Result |
| --- | --- |
| `cargo nextest run --test testgate_align_authority_contract` | PASS, 5/5 after review strengthening |
| `bash tools/release/check_authority_suite_antievasion.sh` | PASS |
| `cargo nextest run --test auth11_required_suite_obligation_guards_contract` | PASS, 3/3 |
| `cargo fmt --check` | PASS |
| `cargo nextest run --test snowdensity03_physics_bulk_offline_contract` | PASS, 2/2 |
| Scoped `markdown-doc lint` for root/work-package instructions, standards, catalogs, roadmap, and this package | PASS, 34 files, 0 findings |
| JSON parse for live impact map and historical registry | PASS |
| Historical Git blob reconstruction and SHA-256 | PASS, exact recorded digest |
| Removed test registration/path and frozen status checks | PASS |

The first alignment-target attempt exposed two Markdown line-wrap-sensitive
string assertions. Three substantive cases passed. The assertions were changed
to normalize whitespace and bind the ratified wording; the complete target then
passed 5/5. This was test robustness, not a governance-content failure.

Final diff hygiene, documentation rerun, and exact-diff reconciliation are
recorded after review findings are dispositioned.
