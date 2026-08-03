# Owned-File Manifest

Status: reconciled

Evidence mode: **Static**

## Production and test identities

| File | SHA-256 |
|---|---|
| `erosion_continuity.rs` | `b95bb3900355b43f6f0674f2942928da0e3b79a1ca03ae4c650f3180158b5037` |
| `erosion.rs` | `869f9f33894f730d2fb16c5cdb5560e580f3c7021e9d1bbfd4d435e8343c527a` |
| `erosion_hb04.rs` | `ae76c95e23815e0fdb72485b471f395fc84d7800f2f54d28da70e2df7fca52f2` |
| EROD16 integration fixture | `c194c3d3b5fae4fd30b9b9d49cf35b07d23ccf20958392e896fa8584ac6017c3` |
| `SC-SED-001.md` | `c0d73c88858959ce481f4de579d07a495945323156169f7b916ef5d62072c2c1` |
| `storm-partition.csv` | `4f70c91e120f015c60abce12f0e8732e901c2ffff5eb6cd4dfa7fe34f7bc2d53` |

## Declared owned surfaces

- the complete W2C package tree;
- the Wave-1 continuity implementation, its immediate orchestration comment,
  and owned unit/integration tests;
- `SC-SED-001` and the science-contract index; and
- the snow campaign roadmap, main roadmap, and work-package catalog.

No file outside the package's intended write set is modified. No manifest,
lockfile, dependency, fixture forcing, observation, coefficient, assurance
report source, or snow-runtime file changed.

Revisions 58–60 change canonical documentation authority only. The four runtime/
test hashes above are unchanged from logs 31–44 and their four-file diff
identity is `ada609e061f5cc9eb91eaa249169ae0317548aeec71f0c57fc388d05bb1b64ee`.
The five-file runtime/test/contract diff identity after revision 60 is
`2089324becad4b78809ed11c72830522c99ad73c37dacf1098bfc635807e0f80`.
