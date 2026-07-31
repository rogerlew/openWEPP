# Pre-Implementation Contract Gate

Status: `PASS`

Evidence mode: `Ran`

Canonical `SC-SNOWENERGY-001` v3 and `SC-SNOWFREEZE-001` v119 bind the
active control volume, `G_0`, conservative projection, timestep hierarchy, and
prohibited limiter alternatives.

Ran:

```text
cargo nextest run --test snow_surface_eb03_contract
8 passed, 0 skipped
```

Production edits were not begun before this gate passed.
