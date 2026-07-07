# Source-Authored Input Audit

Status: EXECUTED-HOLD-ROUTE-COEFFICIENT-BRIDGE-AUTHORITY. Evidence mode: Ran.

## Selected Roots

This package reuses the selected D16 roots from the predecessor and scans the
whole root trees available in this session:

- `/wc1/runs/al/algebraic-radium`
- `/wc1/runs/un/unpalatable-rind`
- `/wc1/runs/ar/arboreal-dendrite`

## Current Inventory

Ran:

```text
find /wc1/runs/al/algebraic-radium -type f -name '*.man' | wc -l
44

find /wc1/runs/un/unpalatable-rind -type f -name '*.man' | wc -l
40

find /wc1/runs/ar/arboreal-dendrite -type f -name '*.man' | wc -l
73
```

The current external-root inventory is `157` management files. This differs
from the predecessor's narrower subdirectory count (`120`) because the
arboreal-dendrite root now exposes additional `.man` files outside the earlier
`landuse` subdirectory. The zero-match route-coefficient result is unchanged.

## Native Route-Coefficient Scan

Ran:

```text
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

rg -l '^ow-lanuse-1$' \
  /wc1/runs/al/algebraic-radium \
  /wc1/runs/un/unpalatable-rind \
  /wc1/runs/ar/arboreal-dendrite -g '*.man' | wc -l
0

rg -l 'routing_coefficients' \
  /wc1/runs/al/algebraic-radium \
  /wc1/runs/un/unpalatable-rind \
  /wc1/runs/ar/arboreal-dendrite -g '*.man' | wc -l
0

find /wc1/runs/al/algebraic-radium \
  /wc1/runs/un/unpalatable-rind \
  /wc1/runs/ar/arboreal-dendrite -type f -name '*.run.toml' | wc -l
0
```

## Snapshot Digests

These digests identify the mutable external file sets inspected in this
package.

| Root | `.man` count | File-list digest | Content-list digest |
|---|---:|---|---|
| `/wc1/runs/al/algebraic-radium` | 44 | `bba16a6580497763e39b659989143396823e835f3f4f41b351d0ef954a58283d` | `31c6750577ff6c006bb4a47855baa1ea31ff1fce5f0b61101f748b3adafded56` |
| `/wc1/runs/un/unpalatable-rind` | 40 | `cfe38d70f0e97a1d744eb9e579c4e8b864b18a471f30f327be6909025bd31529` | `faa34e91fe00d9f83e678e2bf1afe319346a0f80352105316e9b14ae9a6244f9` |
| `/wc1/runs/ar/arboreal-dendrite` | 73 | `99224242b073e0eeed578bfce82b76a562de7b1063b507e9eb4754f3df9c1cfe` | `4936b76340c013340859ccf13e38561ed69ce198043033ea900a79151a3f51d9` |

## Result

The source-authored native input path is not available in the current
repo/session. There are no native `ow-lanuse-1` managements, no
`routing_coefficients` markers, and no openWEPP active runfiles in the selected
external roots.
