# Source And License Manifest

Status: `PASS / intake and terminal identity verified`

Evidence mode: `Static`

| Repository | Local checkout | Commit | Intake state | License SHA-256 |
|---|---|---|---|---|
| `laurencelin/RHESSysEastCoast` | `/workdir/RHESSysEastCoast` | `375c75b1cd2202217651dff43aa113d80b9c1118` | clean | `4fd4ecf2fd01cf53c99754bcac5a6dbee255a0be0539dd84ffe12e06808374be` |
| `laurencelin/GIS2RHESSys` | `/workdir/GIS2RHESSys` | `6b20883dea7c9fd92f71ec69eaca015ebf6dfe18` | clean | `4fd4ecf2fd01cf53c99754bcac5a6dbee255a0be0539dd84ffe12e06808374be` |

Recheck both identities and worktrees at audit intake and terminal disposition.

Ran at intake from `/home/workdir/openWEPP`: `git rev-parse HEAD`,
`git status --short`, and the corresponding `git -C` commands for both external
checkouts. openWEPP was clean at
`86faf6fd22421372c6d9874b7bd0b7e1cabd439f`; both pinned repositories were
clean at their declared commits. Both license files independently hashed to
the recorded digest.

Both accepted GIS generator entry scripts contain raw GitHub `master` fallback
URLs for vegetation, soil, and land-use collections. Those mutable network
paths are source-observed behavior only, are not part of the pinned evidence
identity, and are rejected for successor compatibility. The successor must
accept explicit local bytes bound to a repository commit and file digest and
must not fetch defaults.
