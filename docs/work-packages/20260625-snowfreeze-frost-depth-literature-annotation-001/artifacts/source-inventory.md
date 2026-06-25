# Source Inventory

Status: complete

Static evidence:

- `git status --short` showed only two untracked reference files:
  `references/vendorable/Amico2011.pdf` and
  `references/vendorable/Devoie2022.pdf`.
- `git check-ignore -v` confirmed `references/copyrighted/**` PDFs are ignored
  by `.gitignore`.
- `pdftotext` was run to `/tmp` for all local source PDFs listed here.

| Source | Local path | Rights posture | Source role for openWEPP |
|---|---|---|---|
| Dun et al. (2010) | `references/copyrighted/Dun2008_10.13031@2013.34896.pdf` | local-only copyrighted cache | WEPP-specific frost lineage; validates the v2010.1 frost rewrite on Pullman/Morris and exposes the published-vs-pinned `Qwet` conflict. |
| Watanabe and Flury (2008) | `references/copyrighted/watanabe2008.pdf` | local-only copyrighted cache | Mechanistic frozen hydraulic-conductivity candidate; useful for `K(theta_liq, T)` and capillary-bundle reasoning. |
| Kurylyk and Watanabe (2013) | `references/copyrighted/kurylyk2013.pdf` | local-only copyrighted cache | Theory review for Clapeyron/SFCC/frozen-K formulation decisions. |
| Dall'Amico et al. (2011) | `references/vendorable/Amico2011.pdf` | redistributable, CC-BY 3.0 declared in PDF | Energy-conserving coupled heat-water numerics reference; not a WEPP drop-in model. |
| Kurylyk et al. (2014) | `references/copyrighted/kurylyk2014.pdf` | local-only copyrighted cache | Analytical benchmark authority for one-dimensional thaw with conduction/advection. |
| Azmatch et al. (2012) | `references/copyrighted/azmatch2012.pdf` | local-only copyrighted cache | SFCC-derived partially frozen hydraulic-conductivity candidate. |
| Ming et al. (2020) | `references/copyrighted/ming2020.pdf` | local-only copyrighted cache | Saturated frozen hydraulic-conductivity estimation from SFCC and capillary bundle. |
| Amankwah et al. (2021) | `references/copyrighted/Amankwah2021.pdf` | local-only copyrighted cache | Salt-exclusion SFCC context; important for salinity-sensitive future scope. |
| Cheng et al. (2023) | `references/copyrighted/Cheng2023.pdf` | local-only copyrighted cache | Impedance-factor candidate; distinguishes closed unsaturated systems from open saturated/ice-segregating systems. |
| Devoie et al. (2022) | `references/vendorable/Devoie2022.pdf` | redistributable, CC-BY 4.0 declared in PDF | SFCC dataset/repository authority for future parameter priors and uncertainty. |

Disposition:

- Vendored binaries: keep `Amico2011.pdf` and `Devoie2022.pdf` in
  `references/vendorable/` and record their CC-BY basis.
- Copyrighted binaries: do not add ignored PDFs to git; track only citation and
  source-role annotations.
