# Gate Results

Status: complete.

Evidence class: Ran + Static.

| Gate | Result | Evidence |
|---|---|---|
| Raw `.man` values archived | PASS | `canopy-provenance-evidence.md` per-fixture table. |
| wepppy projected values cited | PASS | `winter-cancov-validation.md` cited with evergreen/mixed/deciduous winter means. |
| openWEPP runtime evidence generated | PASS | Eight `coe_melt_summary.json` files retained under `artifacts/coe_melt_runtime_surface/`. |
| Sleepers endpoint resolved | PASS | `mismatch-disposition.md` classifies Sleepers as moderate static pasture/ag, not proven lowest-cancov endpoint. |
| Mismatches dispositioned | PASS | `mismatch-disposition.md`. |
| No fixture inputs changed | PASS | `git diff -- tests/fixtures/cancov_forest crates docs/specifications/science-contracts \| wc -c` returned `0`. |
| No production physics changed | PASS | Same no-diff check returned `0`; no `crates/` files modified. |
| `git diff --check` | PASS | Ran after artifact creation. |
| Package evidence presence | PASS | Required closeout artifacts present. |
| Scoped doc lint | PASS | `wctl doc-lint --path docs/work-packages/README.md` reported 1 file validated, 0 errors/warnings; package path reported 0 files validated, 0 errors/warnings. |

Commands run:

```bash
cargo run -q -p openwepp-runner --bin openwepp-snowbench -- coe-melt ...
git diff --check
git diff -- tests/fixtures/cancov_forest crates docs/specifications/science-contracts | wc -c
wctl doc-lint --path docs/work-packages/README.md
wctl doc-lint --path docs/work-packages/20260626-snowdensity-10-3-1-canopy-projection-provenance-001
find docs/work-packages/20260626-snowdensity-10-3-1-canopy-projection-provenance-001/artifacts -maxdepth 2 -type f
```

Not run:

- Full Rust workspace gates. This package is evidence/docs-only and changed no
  production Rust code.
