# Terminal Diff Reconciliation

Status: reconciled for executed cross-domain hold

Evidence mode: **Ran**

Immediately before adding this self-referential reconciliation artifact, the
working tree had these identities:

- tracked binary diff from `HEAD` SHA-256:
  `aaed6158bdfe6acbae1c59450e681ba1e9e92e6a5be7b670274208cf6b21c817`;
- complete porcelain status inventory SHA-256:
  `1af5016f14458a6847904a5fe2f15b8681a11ce4f49c619f520790eca63da2eb`;
- package-tree file/content manifest SHA-256:
  `49cbb294bbd0289f8c171cff292cd078b9d4b1b2345f346b3f6e63cc9e6cffe9`.

Result-affecting untracked source identities were independently pinned:

| File | SHA-256 |
|---|---|
| `crates/openwepp-runner/src/hillslope/tests03/eb04w2b_warm_snow.rs` | `9bb2a4fda9a7393d1c4dcc67273ed1d8975e540d33dc749603b68b8714cf76b5` |
| package frozen-rerun tool | `cab90f8c9e19f9f2c5281a064989a55db4c853bb4b39abd4a24a699283715a3d` |
| first assurance transaction | `1bf6f8b1cfec03c8ed2018ce4d1de9af0ed97cde0e3a1dda7656281dedd89f54` |
| second assurance transaction | `c5eb76f7307ca2a82124833118bcc95097bab1b5504f7d0cfc35cbce0ea4cd20` |
| terminal assurance transaction | `d78547f8d049a42b1f3b7747e7ca903043e59f544378444e0e92ac8801eecc01` |

The tracked diff contains 48 files, 405 insertions, and 103 deletions. Thirty-
three registered snow integration targets contain only the mechanical v122
contract-version adoption; substantive code/test changes remain confined to
the declared snow/runoff consumer, forcing-availability, closure, error,
assurance, roadmap/catalog, and package surfaces. `git diff --check` and scoped
Markdown lint both pass. No dependency, manifest, lockfile, fixture forcing,
observation, coefficient, selector-default, or erosion-authority file changed.

The terminal disposition is intentionally not `complete`: the exact retained
EROD16 gate fails at `61/231`, so the full profile and frozen W2A result rerun
remain unexecuted and EB-04X remains blocked.
