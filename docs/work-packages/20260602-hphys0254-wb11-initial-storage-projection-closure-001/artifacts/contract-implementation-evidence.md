# Contract Implementation Evidence

Status: complete

Evidence mode: static

Static:

- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` now records `INV-WATBAL-041`, requiring WB11 initial hydrology seed storage to use normalized corrected-layer hydrology aliases rather than parser-layer depth aliases.
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md` now records `INV-SOIL-015`, separating generic parser/external-authority soil symbols from hydrology-owned seed aliases.
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md` now records the HPHYS0254 lower-layer saturation-ratio `stu` cap for baseline WB18 percolation continuity.
- `docs/specifications/science-contracts/index.md` now records HPHYS0254 authority scope: `wb11_nsl` and `wb19_*`/WB18 percolation aliases are hydrology seed surfaces; generic `nsl`, `dg_####`, `thetfc_####`, and `thetdr_####` remain parser/external-authority surfaces.

Decision:

- Use alias separation instead of redefining generic symbols. This preserves AUTH05/AUTH07 constitutive/parser authority while allowing WB11/WB18/WB19 to consume baseline-normalized hydrology seed geometry.
