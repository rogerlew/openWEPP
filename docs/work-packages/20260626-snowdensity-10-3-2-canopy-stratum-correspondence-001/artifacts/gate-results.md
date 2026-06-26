# Gate Results

Status: complete.

Evidence class: Static + Ran.

| Gate | Result | Evidence |
|---|---|---|
| Harvard observed strata enumerated | PASS | `stratum-correspondence-evidence.md` lists `HF237` hemlock/hardwood/open and `HF155` site-level SWE context. |
| Marcell observed strata enumerated | PASS | `stratum-correspondence-evidence.md` lists conifer/deciduous/open by cover type. |
| Current modeled surfaces enumerated | PASS | Harvard `p8` and Marcell `p10` are single mixed-forest surfaces with runtime `cancov = 0.55`. |
| Explicit stratum binding disposition | PASS | Every Harvard/Marcell advertised stratum is marked unbound or provisional context. |
| One-hillslope defensibility decision | PASS | `binding-decision.md` rejects current one-hillslope use for canopy-stratum verdicts. |
| No fixture inputs changed | PASS | `git diff -- tests/fixtures/cancov_forest crates docs/specifications/science-contracts \| wc -c` returned `0`. |
| No production physics changed | PASS | Same no-diff check returned `0`; no `crates/` files modified. |
| `git diff --check` | PASS | Ran after artifact creation. |
| Package evidence presence | PASS | Required closeout artifacts present. |
| Scoped doc lint | PASS | `wctl doc-lint --path docs/work-packages/README.md` and package path completed with no errors. |

Commands run:

```bash
git status --short --branch
find tests/fixtures/cancov_forest/harvard_mixed_ma tests/fixtures/cancov_forest/marcell_mixed_mn -maxdepth 3 -type f
rg -n "HF155|HF237|Harvard|Marcell|hemlock|hardwood|conifer|deciduous|open cover|cover type|10\\.2737/RDS-2021-0016|snow depth|SWE" tests docs tools crates -g '!target'
nl -ba tests/fixtures/cancov_forest/harvard_mixed_ma/p8.man | sed -n '1,45p'
nl -ba tests/fixtures/cancov_forest/marcell_mixed_mn/p10.man | sed -n '1,45p'
git diff --check
git diff -- tests/fixtures/cancov_forest crates docs/specifications/science-contracts | wc -c
wctl doc-lint --path docs/work-packages/README.md
wctl doc-lint --path docs/work-packages/20260626-snowdensity-10-3-2-canopy-stratum-correspondence-001
```

Not run:

- Full Rust workspace gates. This package is evidence/docs-only and changed no
  production Rust code.
- Observation comparators. The Harvard/Marcell stratum observation tables are
  not installed in the repo and this package's job is correspondence, not
  comparison.
