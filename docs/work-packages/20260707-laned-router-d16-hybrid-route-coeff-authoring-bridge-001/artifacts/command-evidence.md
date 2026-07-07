# Command Evidence

Status: EXECUTED-HOLD-ROUTE-COEFFICIENT-BRIDGE-AUTHORITY. Evidence mode: Ran.

## Source Scans

```text
find /wc1/runs/al/algebraic-radium -type f -name '*.man' | wc -l
44

find /wc1/runs/un/unpalatable-rind -type f -name '*.man' | wc -l
40

find /wc1/runs/ar/arboreal-dendrite -type f -name '*.man' | wc -l
73

find /wc1/runs/al/algebraic-radium \
  /wc1/runs/un/unpalatable-rind \
  /wc1/runs/ar/arboreal-dendrite \
  -type f \( -name '*ow-lanuse*' -o -name '*routing*coeff*' \
  -o -name '*route*coeff*' -o -name '*.run.toml' \) | sort | wc -l
0

rg -l '^ow-lanuse-1$|routing_coefficients' \
  /wc1/runs/al/algebraic-radium \
  /wc1/runs/un/unpalatable-rind \
  /wc1/runs/ar/arboreal-dendrite -g '*.man' | sort | wc -l
0
```

## Active Guard

```text
cargo test -q --test laned_shadow_h2637 h2637_active_fails_closed_without_routing_coefficients

running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.25s
```

## Local Package Gates

The local package gates are recorded in `gate-results.md`.
