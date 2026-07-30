# Protected Surface Baseline

Status: frozen before production edits

Evidence class: Ran

Base: `0f87d897745571d329dca423d0903c54742e853a`

| Surface | Git-index aggregate SHA-256 |
| --- | --- |
| tracked `usersum/**` | `80626c229e1514106e017a761598228c95039f6c112f58463094eb5f6ba47dae` |
| existing groundwater and snow/frost V2 report sources | `04d005d9fcf02c3faf982c79d060410b8638a5286ec348ca36f122063e6f2180` |
| `crates/openwepp-kernel/**` and `crates/openwepp-runner/**` | `a58993cfa06e1b6437fc1d7e3dc4afe2fdafa99644929fcd66ef214244c7292a` |

The pre-edit generated assurance generation was
`94df966626df18d8231227f83dacb9c617198553c0676d7ba21eacb931fc4160`.
The catalog admitted exactly:

- `linear-groundwater-reservoir-recurrence`;
- `snow-and-frozen-soil-process-evaluation`.

The worktree was clean after the scaffold commit. Terminal reconciliation must
repeat these index-aggregate commands and additionally inspect all changed
paths. Generated identity and catalog changes are intended; the three surfaces
above must remain exact.
