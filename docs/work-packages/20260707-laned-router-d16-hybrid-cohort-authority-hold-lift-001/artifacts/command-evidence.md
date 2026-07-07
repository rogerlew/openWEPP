# Command Evidence

Status: EXECUTED-HOLD-ROUTE-COEFFICIENT-AUTHORITY. Evidence mode: Ran.

## Commands Executed

| Command | Exit | Result |
|---|---:|---|
| `tools/owcmp/owcmp env --manifest tools/owcmp/suites/minnesota-corn-ksflag1.json` | 0 | PASS |
| `tools/owcmp/owcmp env --manifest tools/owcmp/suites/n-idaho-single-ofe-ksflag0.json` | 0 | PASS |
| `tools/owcmp/owcmp env --manifest tools/owcmp/suites/wa-cascades-mofe-ksflag0.json` | 0 | PASS |
| `tools/owcmp/owcmp manifest run --manifest tools/owcmp/suites/minnesota-corn-ksflag1.json` | 1 | BLOCKED: inventory-only |
| `tools/owcmp/owcmp manifest run --manifest tools/owcmp/suites/n-idaho-single-ofe-ksflag0.json` | 1 | BLOCKED: inventory-only |
| `tools/owcmp/owcmp manifest run --manifest tools/owcmp/suites/wa-cascades-mofe-ksflag0.json` | 1 | BLOCKED: inventory-only |
| `find /wc1/runs/al/algebraic-radium/wepp/runs -maxdepth 1 -name '*.man' \| wc -l` | 0 | 44 |
| `find /wc1/runs/un/unpalatable-rind/wepp/runs -maxdepth 1 -name '*.man' \| wc -l` | 0 | 40 |
| `find /wc1/runs/ar/arboreal-dendrite/landuse -maxdepth 1 -name '*.man' \| wc -l` | 0 | 36 |
| `rg -l "routing_coefficients" ... -g '*.man' \| wc -l` | 0 | 0 |
| `rg -l "^ow-lanuse-1$" ... -g '*.man' \| wc -l` | 0 | 0 |
| `find selected external roots -name '*.run.toml' \| wc -l` | 0 | 0 |
| `rg -n "routing_coefficients" tests/fixtures -g '*.man'` | 1 | no matches |
| `cargo test -q --test laned_shadow_h2637 h2637_active_fails_closed_without_routing_coefficients` | 0 | 1 passed |

## Interpretation

The executable evidence supports the route-coefficient authority hold. The
selected roots exist and pass inventory checks, but they cannot yet run the
active plain-vs-hybrid promotion comparison because they lack native active
input authority and executable suite shape.
