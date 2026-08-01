# Determinism Comparison

Status: `PASS`

Evidence class: `Ran`

Two consecutive `--analysis-only` reductions of the unchanged retained
population produced identical console output and identical hashes for every
generated result, table, report, SVG, and Markdown sidecar. The console-output
hash for both reductions was
`d06c117796b5183989a76396e5a98b104d984e4ddda2baeaa27a292da69e1bc2`.

| Generated artifact | Raw reduction A | Raw reduction B |
| --- | --- | --- |
| `qualification-results.json` | `1686bc66d705e313fe0000289e870ada7aad0435f83a8696057ffd497969772b` | same |
| `cell-qualification.csv` | `ebda902a0cb8c064d372e3c8659ed7e70663b6fb65bb0e81c32f0c6ce0fb4de9` | same |
| `runtime-qualification.md` | `e06791db46209802987b42a5549053eec48b240e6375a17e92dae13dc177751e` | same |
| correction-boundary sidecar / SVG | `94e5abdd…` / `035623e6…` | same |
| ledger-closure sidecar / SVG | `83adae19…` / `c2dc90f6…` | same |
| runtime-completion sidecar / SVG | `d2e7036b…` / `8001fd25…` | same |
| thermal-minima sidecar / SVG | `31eab047…` / `97b9f5f6…` | same |

The full raw hashes above bind the two actual reductions; abbreviated figure
hashes are presentation-only and were compared in full. Before staging, the
CSV line endings and Matplotlib SVG trailing spaces were mechanically
normalized to the repository's text-file convention. This does not alter any
value or rendered geometry. The committed canonical hashes are:

- `cell-qualification.csv`:
  `8d5cbb890a7ae52c1d80976ac9245f79a16338178282015a6b6899eba112e252`;
- correction-boundary SVG:
  `830da0e399765fd10738fb68cbf317f471006465233d85de4c4ab5eba282060e`;
- ledger-closure SVG:
  `60905f96315e33e3ad22bda3d5ffccafcd316394d4bea768c16a369463465029`;
- runtime-completion SVG:
  `df06873e307e4b8208b255fd3de43e5f63f1910dfe2bc4738d3923e2fffa703f`;
- thermal-minima SVG:
  `8fa11184b2761bfd5cd5a3f351120fe8dc72e9d77c1bc440dc3e627c3b4145e4`.
