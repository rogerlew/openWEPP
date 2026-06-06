# Radiation-Bound Validation Ledger

Status: complete

Evidence mode: `Ran`

Ran:

After-state validation used release binary
`target/release/openwepp-cli-hill` with SHA-256
`6aa3a88c6acfb6b57fa409a7073c755ca7ee866f449df0111a194f0e01435628`.

Command pattern:

```text
target/release/openwepp-cli-hill \
  --run-dir /wc1/runs/in/indispensable-presenter/wepp/runs \
  --run-file /tmp/wbval01_rocky_mountain_20260606T000000Z/generated_runfiles_nodiscovery/<p>.toml \
  --output-dir /tmp/wbval02_after/<p> \
  --policy compat
```

Results:

| Hillslope | RC | WAT reachable | After error | Acceptance |
|---|---:|---|---|---|
| `p2` | 1 | no | `CLIM-RUNTIME-E-017`, `radly=486` | invalid upstream input |
| `p4` | 1 | no | `CLIM-RUNTIME-E-017`, `radly=486` | invalid upstream input |
| `p6` | 1 | no | `CLIM-RUNTIME-E-017`, `radly=486` | invalid upstream input |
| `p9` | 1 | no | `CLIM-RUNTIME-E-017`, `radly=486` | invalid upstream input |
| `p14` | 1 | no | `CLIM-RUNTIME-E-017`, `radly=486` | invalid upstream input |
| `p17` | 1 | no | `CLIM-RUNTIME-E-017`, `radly=486` | invalid upstream input |

Radiation conservation and guard posture:

- Daily radiation was not clipped, capped, scaled, or silently normalized.
- The invalid source value `486 Ly d^-1` is preserved in typed runtime evidence.
- The old hourly publication error is replaced by a source-bound error before
  hourly synthesis.
- Existing hourly unit-lineage tests and HPHYS0277 impossible-radiation tests
  pass after the change.
