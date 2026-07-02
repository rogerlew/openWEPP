# Fixture Candidate Inventory

Status: `passed`

Evidence mode: `Static:` local source inspection plus committed-fixture checks.

## Candidate Scan

`/wc1/runs/**/watershed` was scanned for complete larger watershed source
substrates. Complete candidate class required numbered `pN.{run,man,slp,cli,sol}`
sets and a usable watershed source layout. Several `>=1000` candidates were
present, but the user selected `onshore-xenophobia` as the W6 full-watershed
candidate.

## Adopted Candidate

| Field | Value |
| --- | --- |
| Source substrate | `/wc1/runs/on/onshore-xenophobia/wepp` |
| Source run directory | `/wc1/runs/on/onshore-xenophobia/wepp/runs` |
| Hillslope input sets | `p1` through `p1305`, complete |
| Watershed structure | `pw0.str`, datver `99.1`, `544` structure rows |
| Channels | `544` channel elements, `pw0.chn` declares `544`, `ipeak=4` |
| Impoundments | `0`, from `pw0.imp` |
| Run horizon | `100` years from `p1.run` and `p1305.run` |
| Sidecars | `chan.inp`, `chntyp.txt`, `gwcoeff.txt`, `pmetpara.txt`, `snow.txt`, `tc.txt`, `wepp_ui.txt` |
| Source climate representation | `p1.cli` through `p1305.cli` and `pw0.cli` are one hard-linked inode |
| Committed representation | one canonical `runs/shared/onshore-xenophobia.cli` plus relative `pN.cli`/`pw0.cli` symlinks |
| Adoption disposition | accepted as full `1305`-hillslope W6 fixture |

## Source-Quality Notes

- `tests/fixtures/watershed/onshore-xenophobia/runs/case.run` binds all `1305`
  generated hillslope jobs; no topology subset is used.
- Soil-file comments were sanitized to remove embedded absolute `/wc1` source
  paths and helper implementation names. WEPP data records were not
  intentionally changed.
- Source `.err` files and generated outputs were excluded.
- The shared climate symlink representation is a repository-size control only;
  it preserves the source's byte-identical hard-link semantics and does not
  reduce the watershed.
