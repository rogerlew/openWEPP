# Committed Fixture Adoption

Status: `passed`

Evidence mode: `Static:` committed fixture inventory and parser/source checks.

## Adopted Fixtures

| Fixture | Committed path | Class | Disposition |
| --- | --- | --- | --- |
| `carnivorous-adobo` | `tests/fixtures/watershed/carnivorous-adobo/` | 32-hillslope existing committed fixture | retained for post-W5 strict committed scaling |
| `onshore-xenophobia` | `tests/fixtures/watershed/onshore-xenophobia/` | full 1305-hillslope large fixture | adopted for W6 full large-watershed scaling |

## Carnivorous-Adobo W6 Launch Surface

- W6 adds `runs/case.run`, schema `openwepp-watershed-runfile-v1`, so the
  public watershed CLI can run the committed fixture without `/wc1` or scratch
  launch files.
- W6 adds `runs/p1.source.run` through `runs/p32.source.run`, schema
  `openwepp-hillslope-runfile-v1`, so generated pass files come from committed
  hillslope inputs.
- Updated manifest entries: `208`.
- Fixture README records the W6 launch-file extension.

## Onshore-Xenophobia Adoption

- Source substrate: `/wc1/runs/on/onshore-xenophobia/wepp`.
- Source run directory: `/wc1/runs/on/onshore-xenophobia/wepp/runs`.
- Committed run directory:
  `tests/fixtures/watershed/onshore-xenophobia/runs/`.
- Checksum manifest:
  `tests/fixtures/watershed/onshore-xenophobia/input-manifest.sha256`.
- Fixture README:
  `tests/fixtures/watershed/onshore-xenophobia/README.md`.
- Manifest entries: `7847`.
- Regular files: `6541`.
- Symlinks: `1306`.
- Hillslope inputs: `p1` through `p1305`, each with `.run`, `.man`, `.slp`,
  `.cli`, `.sol`, and `.source.run`.
- Watershed core: `pw0.run`, `pw0.str`, `pw0.chn`, `pw0.imp`, `pw0.man`,
  `pw0.slp`, `pw0.cli`, and `pw0.sol`.
- Sidecars: `chan.inp`, `chntyp.txt`, `gwcoeff.txt`, `pmetpara.txt`,
  `snow.txt`, `tc.txt`, and `wepp_ui.txt`.

## Topology Summary

| Field | Value |
| --- | ---: |
| Hillslopes | `1305` |
| `pw0.str` rows | `544` |
| Channel elements | `544` |
| First channel element id | `1306` |
| Last channel element id | `1849` |
| Impoundments | `0` |
| `pw0.chn` declared channels | `544` |
| `chan.inp` selected channel ids | `1` (`1849`) |

## Representation Notes

The source stores `p1.cli` through `p1305.cli` and `pw0.cli` as hard links to
one byte-identical climate file. The committed fixture stores one canonical
`runs/shared/onshore-xenophobia.cli` file plus relative symlinks at each legacy
climate filename. This preserves the full input content without storing more
than three gigabytes of duplicate climate data in Git.

No watershed subsetting was applied.
