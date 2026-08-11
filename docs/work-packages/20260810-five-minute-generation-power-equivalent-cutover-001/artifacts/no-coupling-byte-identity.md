# No Coupling Byte Identity

Status: `PASS — post-review diagnostics-off/on protected identities`

Evidence mode: `Ran`

The exact rebuilt release binary ran the same two-day source-complete warm-rain
p61 case with WAT5 enabled and disabled under
`/home/workdir/openwepp-wat5-terminal/{on,off}`. Protected outputs were
byte-identical:

| Surface | Enabled SHA-256 | Disabled SHA-256 |
|---|---|---|
| HBP | `fd01aeadc9716923ff12e78e1c589ec4e15f8b855a395a1f390486296f81de3d` | same |
| PASS Parquet | `e5b8d5ace2f82b97e4d15d1ab2e02c2e415fde3e6076f411389180ca9c064b7f` | same |
| WAT Parquet | `707bdc15fc442a56fd5c47cfa8b531463911f5f3363f2a89d667181eebfea943` | same |
| loss JSON | `92c88db6f19b49c062b3e74fb357926e9ac5ed4b15984025d528523c7ed45bf8` | same |

Only WAT5 and the manifest entry were additive. The source exclusion guard
also scans HBP assembly, OFE routing, watershed orchestration/output, and
runner watershed paths for WAT5 consumers.
