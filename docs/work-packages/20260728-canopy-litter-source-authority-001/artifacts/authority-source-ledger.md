# Authority-source Ledger

Status: `prospective review candidate`

Evidence class: `Static`

Access date: `2026-07-28`

Checksums are SHA-256 over the downloaded PDF bytes. The PDFs were inspected
outside the repository and are not redistributed here. Page anchors identify
printed pages where available and PDF pages otherwise.

| ID | Authenticated object | SHA-256 | Exact claim anchor | Admission role |
| --- | --- | --- | --- | --- |
| `SRC-KLOEPPEL-2007` | Kloeppel, Harmon, and Fahey, “Estimating Aboveground Net Primary Productivity in Forest-Dominated Ecosystems,” Chapter 5, pp. 63–81, DOI `10.1093/acprof:oso/9780195168662.003.0005`; Forest Service object `https://research.fs.usda.gov/treesearch/28767` | `dd0c06ab5bd0fa6d3f9e061a7d99a9a4cf450859872e6408e39e751b01caa1bc` | Printed pp. 69–71, especially PDF p. 8: mature-evergreen annual approximation, standing-foliage/median-longevity method, and resorption qualification | Conditional gross foliage-production meaning and rejection of identity conversion to litter dry mass |
| `SRC-KEANE-JOURNAL-2008` | Keane, “Biophysical controls on surface fuel litterfall and decomposition in the northern Rocky Mountains, USA,” *Canadian Journal of Forest Research* 38:1431–1445, DOI `10.1139/X08-003`; `https://research.fs.usda.gov/treesearch/30831` | `db136aa6e13ec1476833f85a4d3826f976326899e13bfcb5c863817c0e35e710` | Printed p. 1431 abstract/PDF p. 1; printed pp. 1434–1435/PDF pp. 4–5: six collected components, `1 m2` traps, collection cadence, drying, sorting, and diameter classes | Tissue-separated, dry-mass interval measurement and observed variability; not exact-day timing |
| `SRC-KEANE-RP70-2008` | Keane, *Surface Fuel Litterfall and Decomposition in the Northern Rocky Mountains, USA*, RMRS-RP-70, DOI `10.2737/RMRS-RP-70`; `https://research.fs.usda.gov/treesearch/29449` | `b41fe5ef26186eeae5abbf42d631ae2cb8ce381775049a823ed9061f41eaa7ef` | Printed pp. 5 and 9 / PDF pp. 5 and 9: woody diameter classes and collection cadence; printed Table 2 / PDF p. 12: component dry-mass interval rates | Material definitions, interval support, and range/heterogeneity; not a transferable default |
| `SRC-CLM5-TECHNOTE` | CLM5 Technical Note, `https://files.cesm.ucar.edu/models/clm/5.0/CLM50_Tech_Note.pdf` | `9ca0f0e5b7aff712a0ef7f5198f111c4b250cac4417a4f000e36c6c143f2e363` | §20.1.4 and §20.2, printed pp. 180 and 183: background litterfall and inverse-longevity rate; §20.2 expressly identifies continuous shedding as an improvement area | Diagnostic model-design comparison only |
| `SRC-LIM-MAIN-2024` | Lim et al., “Overlooked branch turnover creates a widespread bias in forest carbon accounting,” *PNAS* 121, DOI `10.1073/pnas.2401035121`; author SafeDeposit object `745e4382-a3c0-4e3c-ba4a-488759f81c10` from `https://www.safedeposit.se/projects/484` | `94fcc7fc95a007834d26bfb7025d84431f57fe76c0f057b3b0e6ff8662d6c6a5` | PDF p. 1: branch litterfall proxy omits attached-dead loss and understates turnover; PDF p. 4/Figs. 3–5: height increment, density, crown turnover, branchfall, attached-dead storage, and in-canopy loss | Binding distinction between turnover and ground deposition; state requirements |
| `SRC-LIM-SUPP-2024` | Lim et al. supplement, SafeDeposit object `7f43887e-c4e7-4264-a34f-0cb6ff34d7de` from `https://www.safedeposit.se/projects/484` | `c4ddcd4b566036fd9a9e806c279fe663a402b4d806d4a72be659e1bd4a858c1e` | Table S4, printed p. 5/PDF p. 16: turnover models and operands; Fig. S9, PDF p. 11: branchfall, attached-dead change, and in-canopy mass loss | Rejects aggregate-structural-biomass shortcut and immediate-deposition interpretation |
| `SRC-WHITE-2000` | White et al., “Parameterization and Sensitivity Analysis of the BIOME–BGC Terrestrial Ecosystem Model,” *Earth Interactions* 4, DOI `10.1175/1087-3562(2000)004<0003:PASAOT>2.0.CO;2`; publisher full-text record | Not locally downloaded; publisher object is access-controlled | Parameter tables and discussion of leaf longevity/turnover and parameter uncertainty | `DIAGNOSTIC_ONLY`; no production operand depends on it |
| `SRC-BERNIER-2008` | Bernier, Hanson, and Curtis, “Measuring Litterfall and Branchfall,” pp. 91–101, DOI `10.1007/978-1-4020-8506-2_7`; ORNL bibliographic record `https://www.ornl.gov/publication/chapter-7-measuring-litterfall-and-branchfall` | No authenticated downloadable object retained | Chapter scope and bibliographic identity only | `SUPPORTING_ONLY`; no production operand depends on an unverified passage |
| `SRC-BASELINE-DECOMP` | Pinned baseline commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`, `src/decomp.for` | `934064c3bb52e76acabb40df1a77ae51ccf8d244f3e4a7b7d406766e834f4dd1` | Lines 579–606: decay and interrill/rill area-weighted ground mass; lines 648–661: parallel per-unit-area explanation; lines 693–707: cut projection | Static topology provenance, not a recurring canopy-source law |
| `SRC-BASELINE-RESDEP` | Same pinned baseline, `src/res_dp.for` | `60d08e2b2e7cd93b1f0412eea3759efbf7c7a0f328d1eecfa8c4d63a1874a2b6` | Lines 81–126: residue mass-to-depth conversion | Downstream depth-consumer provenance |

## Non-authoritative retained observations

The Hubbard Brook and Harvard objects retain their hashes and field-level
provenance in CAL-04B/CAL-05. They remain `DIAGNOSTIC_ONLY`: Hubbard does not
consistently expose separated needle/twig dry mass, while Harvard pools
nonfoliar carbon and does not supply fine-wood dry mass.

## Audit result

No executable magnitude, conversion, or timing constant is admitted from an
unauthenticated article. `SRC-WHITE-2000` and `SRC-BERNIER-2008` may inform
the rejection rationale but cannot determine production values. No operator
article acquisition is required for the present boundary-interface decision.
