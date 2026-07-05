# Zone Taxonomy Evidence

Status: executed
Evidence mode: Static + Ran

## Source Surface

Static:

- `references/copyrighted/Papanicolaou2018.md` describes Figure 9 as the
  Zone 1 / Zone 2 taxonomy surface and reports thresholds:
  bare `I*=0.16`, `Psi*=0.004`; isolated roughness `I*=0.33`,
  `Psi*=0.017`; vegetation `I*=0.68`, `Psi*=0.022`.
- The supplemental workbook stores the isolated-roughness block under the
  label `Clods`.

Ran:

- `tools/dval/zone_taxonomy.py` reads local
  `Figure_9.xlsx`, verifies sha256
  `ec198018d34414298b08419ba1b303de86ebd11b642a03731a0e68e2b04b8f28`,
  parses only derived taxonomy columns, asserts published `I*` support,
  asserts published `Psi*` support within 10% relative grid tolerance, and
  emits scalar JSON only.
- Command:
  `.venv/bin/python tools/dval/zone_taxonomy.py --fig9 references/copyrighted/Papanicolaou2018-supplemental/wrcr23071-sup-0002-2017wr021109-ds01/Figure_9.xlsx > docs/work-packages/20260705-mofefid-d9-dval-disposition-001/artifacts/zone-taxonomy-20260705-1545.json`

## Results

| Class | Grid / threshold support | Zone 2 `l` behavior | Zone 1 behavior |
|---|---|---|---|
| Bare | nearest grid `I*=0.1667` vs published `0.16`; nearest `Psi*=0.004124` vs `0.004` (`3.11%` relative error) | `l=0.9997..1.0023`, `R2 >= 0.9995` | No multi-point sub-threshold fit because the published threshold is below the second workbook intensity grid point. |
| Isolated roughness (`Clods`) | nearest grid `I*=0.3333` vs `0.33`; nearest `Psi*=0.016897` vs `0.017` (`0.61%` relative error) | `l=0.9877..1.0515`, `R2 >= 0.9979` | `l_mean=1.335`, `R2 >= 0.9945`, showing roughness-sensitive nonlinearity below threshold. |
| Vegetation | nearest grid `I*=0.6867` vs `0.68`; nearest `Psi*=0.024015` vs `0.022` (`9.16%` relative error) | `l=0.9976..1.1293`, `R2 >= 0.9873` | `l_mean=1.444`, `R2 >= 0.9954`, showing roughness-sensitive nonlinearity below threshold. |

## Disposition

Ran: Zone 1 / Zone 2 taxonomy is executed and passes the qualitative D9
acceptance surface. No hold is required.
