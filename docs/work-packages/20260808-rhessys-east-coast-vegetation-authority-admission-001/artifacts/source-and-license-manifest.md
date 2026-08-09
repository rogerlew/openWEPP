# Source And License Manifest

Status: `PASS / pinned read-only identities preserved`

Evidence mode: `Ran + primary-source inspection`

| Source | Commit | License SHA-256 | Worktree |
| --- | --- | --- | --- |
| `/workdir/RHESSysEastCoast` | `375c75b1cd2202217651dff43aa113d80b9c1118` | `4fd4ecf2fd01cf53c99754bcac5a6dbee255a0be0539dd84ffe12e06808374be` | clean |
| `/workdir/GIS2RHESSys` | `6b20883dea7c9fd92f71ec69eaca015ebf6dfe18` | `4fd4ecf2fd01cf53c99754bcac5a6dbee255a0be0539dd84ffe12e06808374be` | clean |

Selected file digests:

- `vegCollection.csv`:
  `ae6d69fb60d2c9c2dc17b9fe550f68f4eea23b52c5cdd6f8392ba913cdea8051`.
- `libraries/g2w_cf_RHESSysEC.R`:
  `47a5d2df178ac16c3e588d95cca69b3d3bab5d3193afa67c2e934433242b1999`.
- `libraries/g2w_cf_RHESSysEC_soil_fullextraction.R`:
  `088d471343b163170235465d2e47a3e925eedcfb2dcbc5d4df09253e38439ba0`.

MIT permission authorizes inspection/adaptation, not scientific promotion.

Primary sources inspected without adding source bytes:

| Source | Exact locator | Custody/rights disposition |
| --- | --- | --- |
| White et al. parameter dataset | DOI `10.3334/ORNLDAAC/652`; public Data.gov/ORNL DAAC record and public documentation PDF | federal catalog marks public access/use; no dataset bytes tracked |
| White et al. (2000) | DOI `10.1175/1087-3562(2000)004<0003:PASAOT>2.0.CO;2`, Appendix A | publisher full-text inspection; citation/locator only |
| Reich et al. (1999) | USFS-hosted PDF, Table 2 | citation/locator and short numeric fact only; no full text tracked |
| Hwang et al. (2009) | DOI `10.1029/2009WR007775`, Tables 2-3 | USFS-hosted author copy inspected; citation/locator and derived comparison only |
| Ford et al. (2010) | USFS-hosted author copy, Tables I-V/Figure 4 | citation/locator and derived comparison only |

The ORNL archive is openly available but its bundle endpoint requires an
Earthdata login. The public documentation and publisher article were sufficient
to establish parameter-family scope; this package does not claim that login as
the scientific blocker.
