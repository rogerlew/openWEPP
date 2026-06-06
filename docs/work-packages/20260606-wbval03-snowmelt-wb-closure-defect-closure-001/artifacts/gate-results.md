# Gate Results

Status: executed-hold

Evidence mode: `Ran`

Ran:

| Gate | Command | Result |
|---|---|---|
| Release build | `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | Pass. |
| Current J-95 target reproduction | Loop over `p7`, `p11`, `p18`, `p20` with WBVAL01 generated TOML wrappers and `--policy compat` | All four fail before J-95 with `CLIM-RUNTIME-E-017`, `radly=486`. |
| Current prior-WAT-emitter reproduction | Loop over `p1`, `p3`, `p5`, `p8`, `p10`, `p12`, `p13`, `p15`, `p16`, `p19`, `p21`, `p22` with WBVAL01 generated TOML wrappers and `--policy compat` | All twelve fail before WAT publication with `CLIM-RUNTIME-E-017`, `radly=486`. |
| Complete identity audit | Read saved WBVAL01 WAT parquet files for the 12 prior emitters | Completed; omitted terms do not explain the residual. |

Current blocker text:

```text
CLIM-RUNTIME-E-017: runtime context symbol radly=486 is out of domain
(allowed 0 <= radly <= baseline sunmap horizontal daily potential (rpoth/r3))
```

Documentation gates:

- `markdown-doc lint --path docs/work-packages/20260606-wbval03-snowmelt-wb-closure-defect-closure-001 --no-ignore`
  passed, `23` files validated.
- `markdown-doc lint --path docs/work-packages/README.md --no-ignore` passed.
- `git diff --check` passed.
