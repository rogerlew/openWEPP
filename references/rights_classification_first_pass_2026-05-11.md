# Rights Classification — First Pass (2026-05-11)

Scope: `/workdir/openWEPP/references/*` corpus imported from `wepp-forest`.

Purpose: record first-pass redistribution decisions and rationale used to split
artifacts between `vendorable/` (tracked) and `copyrighted/` (local cache).

## Method

Classifications in this pass were made conservatively:

- moved to `vendorable/` only when redistribution allowance was explicit in
  source text or official agency policy;
- otherwise left in `copyrighted/` pending deeper rights review.

## Moved to `vendorable/`

| File | Basis for first-pass redistributable classification |
|---|---|
| `vendorable/USGS_PP1302_1986_Kinematic_Wave_Models.pdf` | USGS report series + USGS policy states USGS-authored reports/publications are U.S. public domain unless otherwise marked. |
| `vendorable/USGS_PP1302_1986_Kinematic_Wave_Models.md` | Local extract derived from the vendored USGS source above. |
| `vendorable/HEC_RAS_Hydraulic_Reference_Manual_v6_4_1.pdf` | Document text explicitly states HEC-RAS documentation was developed with U.S. Federal resources and is public domain; may be copied/distributed. |
| `vendorable/HEC_RAS_Hydraulic_Reference_Manual_v6_4_1.md` | Local extract derived from the vendored HEC-RAS source above. |
| `vendorable/HEC_RAS_Hydraulic_Reference_Manual_v6_4_1.txt` | Local extract derived from the vendored HEC-RAS source above. |
| `vendorable/R16_2024_HEC_RAS_Hydraulic_Reference_Manual_v6_6.pdf` | Front matter includes “Approved for Public Release. Distribution Unlimited” and HEC-RAS public-domain statement in the manual front matter block. |
| `vendorable/R16_2024_HEC_RAS_Hydraulic_Reference_Manual_v6_6.md` | Local extract derived from the vendored HEC-RAS source above. |
| `vendorable/Amico2011.pdf` | Article text states `CC Attribution 3.0 License`; official Copernicus page confirms DOI `10.5194/tc-5-469-2011`. Added 2026-06-25 for snow/frost research annotation. |
| `vendorable/Devoie2022.pdf` | Article text states Creative Commons Attribution 4.0 License; official Copernicus page confirms DOI `10.5194/essd-14-3365-2022`. Added 2026-06-25 for snow/frost research annotation. |
| `vendorable/Krinner2018_ESM-SnowMIP.pdf` | Copernicus *Geoscientific Model Development*, CC-BY 4.0. DOI `10.5194/gmd-11-5027-2018`. Added 2026-06-25 for snow-melt model-complexity authority. |
| `vendorable/Lute2022_SnowClim.pdf` | Copernicus *Geoscientific Model Development*, CC-BY 4.0. DOI `10.5194/gmd-15-5045-2022`. Added 2026-06-25 for shallow-snow SNOBAL-stability discussion. |
| `vendorable/Gupta2023_HESS.pdf` | Copernicus *Hydrology and Earth System Sciences*, CC-BY 4.0. DOI `10.5194/hess-27-191-2023`. Added 2026-06-25 for shortwave radiation -> coupled melt+ET evidence. |
| `vendorable/Vionnet2012_Crocus.pdf` | Copernicus *Geoscientific Model Development*, CC-BY 4.0. DOI `10.5194/gmd-5-773-2012`. Added 2026-06-25 for snow density/albedo reference-implementation. |
| `vendorable/Anderson2006_SNOW17.pdf` | NOAA/NWS NWSRFS documentation (SNOW-17). U.S. Government work, public domain (17 U.S.C. 105). Downloaded from `weather.gov`. Added 2026-06-25 for the seasonal degree-day melt-factor lineage. |
| `vendorable/Jennings2018_NatComm.pdf` | *Nature Communications* 9:1148, CC-BY 4.0 (license statement in article). DOI `10.1038/s41467-018-03629-7`. Added 2026-06-26 for rain/snow partition — observed-phase dataset + temperature/RH threshold method. |
| `tests/fixtures/precip_phase_observed/jennings2018/jennings_et_al_2018_file2_ppt_phase_met_observations.csv` | Dryad dataset DOI `10.5061/dryad.c9h35`, version 1, published 2019-01-31, CC0. Exact 1.2 GB hourly archive is tracked by Git LFS at its real consumer fixture path; SHA-256 `0cc82fbc5211c2c24b19653c4711d63a88fc4ed7bd90fc39cce84913d071f3a1`. Dryad requests citation of original UCAR RDA `ds464.0` lineage. Added 2026-08-05. |
| `vendorable/NSIDC-0768_GlobalSeasonalSnowClassification_v1_UserGuide.pdf` | NSIDC/NASA DAAC (Liston/Sturm) Global Seasonal-Snow Classification v1 user guide. NSIDC/NASA EOSDIS open data; document states a citation-on-use requirement, no redistribution restriction. Downloaded from `nsidc.org` 2026-06-28 for the climate-class snow-density paradigm (R-60). |

## Left in `copyrighted/` (pending review)

All remaining imported artifacts stay in local cache pending targeted rights
verification, including journal papers, book chapters, and web-page exports
with ambiguous footer/site-level copyright statements.

Snow-melt additions (2026-06-25), kept in `copyrighted/` as AMS journal
copyright (freely readable but not redistributable):

- `copyrighted/Ohmura2001_meltindex.pdf` — *J. Appl. Meteorol.* 40:753–761,
  DOI `10.1175/1520-0450(2001)040<0753:PBFTTB>2.0.CO;2`. The physical-basis
  authority for the temperature-index melt factor.
- `copyrighted/Menard2021_BAMS.pdf` — *BAMS* 102(1):E61–E79,
  DOI `10.1175/BAMS-D-19-0329.1`. Snow-model implementation-error intercomparison.
- `copyrighted/sturm2010_swe_climate_classes.pdf` — *J. Hydrometeorol.* 11(6):1380–1394,
  DOI `10.1175/2010JHM1202.1`. AMS journal copyright (freely readable, not
  redistributable); author open copy (morageology.com/pubs/296.pdf) cached locally
  2026-06-28 for the regime-divergent (climate-class) snow-density paradigm
  assessment (R-58).
- `copyrighted/sturm1995.pdf` — *J. Climate* 8(5):1261–1283,
  DOI `10.1175/1520-0442(1995)008<1261:ASSCCS>2.0.CO;2`. AMS journal copyright;
  operator-supplied local cache 2026-06-28 (R-59). The binding decision-threshold
  authority that unblocks SNOWDENSITY-10.3.22.
- `copyrighted/hydr-JHM-D-21-0070.1.pdf` — Sturm & Liston (2021),
  *J. Hydrometeorol.* 22(11):2917–2938, DOI `10.1175/JHM-D-21-0070.1`. AMS journal
  copyright; operator-supplied local cache 2026-06-28 (R-61); updated thresholds,
  renamed classes. The Annals of Glaciology "compaction behavior of three climate
  classes" paper remains a track-down item, not yet cached.

Note: `copyrighted/noaa_6392_DS1.pdf` (Anderson 1976, NOAA Tech. Report NWS-19)
is a U.S. Government work and is public domain; it is eligible for promotion to
`vendorable/` but is left in place to avoid breaking existing references.

Operator-supplied melt-physics papers (2026-06-25), kept in `copyrighted/` as
journal copyright (IGS/Cambridge, Wiley, Elsevier — freely cited, not
redistributable): `pellicciotti2005.pdf`, `carenzo2009.pdf`, `hock1999.pdf`,
`brock2000.pdf`, `walter2005.pdf`, `marks1999.pdf`, `magnusson2015.pdf`,
`lundquist2013.pdf`, `varhola2010.pdf`. Indexed as R-43..R-51 in
`annotated_bibliography.md`.

Operator-supplied rain/snow-partition papers (2026-06-26), kept in
`copyrighted/source_pdfs/` as journal copyright (Wiley/Elsevier): `susong1999.pdf`
(+ duplicate `susong1999-2.pdf`), `marks1998.pdf`, `kormos2014.pdf`,
`harder2013.pdf`. Indexed as R-54..R-57. (`Jennings2018_NatComm.pdf` is CC-BY and
sits in `vendorable/`.)

## Evidence references used in this pass

- USGS FAQ: “Are USGS reports/publications copyrighted?”  
  https://www.usgs.gov/index.php/faqs/are-usgs-reportspublications-copyrighted
- USGS policy context (17 U.S.C. 105 / public-domain treatment):  
  https://www.usgs.gov/data-management/data-licensing
- HEC-RAS front matter (public-domain statement) in source manuals:
  - local v6.4.1 extract (`references/vendorable/HEC_RAS_Hydraulic_Reference_Manual_v6_4_1.md`)
  - local v6.6 PDF front matter (`references/vendorable/R16_2024_HEC_RAS_Hydraulic_Reference_Manual_v6_6.pdf`)

## Addendum 2026-07-06 (D10B acquisitions)

| File | First-pass classification | Basis |
|---|---|---|
| `copyrighted/19840021490.pdf` (+ `19840021490.md` extract) | `copyrighted/` (conservative) | Davis 1984, ICASE Report 84-20 / NASA CR-172373. NTRS public download (doc 19840021490), but ICASE was USRA-operated under NASA contract, so 17 U.S.C. 105 public-domain status is not automatic for contractor reports. Vendorable candidate pending an explicit NTRS rights statement. Indexed as R-102. |
| `copyrighted/Tseng2010_Hydroinformatics.pdf` | `copyrighted/` | Tseng 2010, Journal of Hydroinformatics 12(3) — IWA Publishing copyright. Indexed as R-103. |

## Follow-up

- Add per-entry rights status fields in `annotated_bibliography.md`.
- Review HEC-HMS web exports and USDA/ASCE/AGU-derived artifacts individually.
- Reclassify from `copyrighted/` to `vendorable/` only with explicit evidence.
- Check the NTRS rights statement for doc 19840021490 (R-102) for possible
  `vendorable/` reclassification.

## Addendum 2026-08-08 (vegetation constitutive-source intake)

The following files were acquired for independent review of the vegetation
radiation/interception/conductance slice. Classification follows the same
affirmative-license rule as the first pass.

| File | Classification | Basis |
|---|---|---|
| `vendorable/Best2011_JULES_Part1.pdf` | `vendorable` | Article text states Creative Commons Attribution 3.0; DOI `10.5194/gmd-4-677-2011`; SHA-256 `84a909165937108a48d566ecce6a46d4b4c1fa3a3640c7a4b3d65a41c67355a7`. |
| `vendorable/Forrester2014_LightAbsorption.pdf` | `vendorable` | Article text states Creative Commons Attribution 4.0; DOI `10.1186/s40663-014-0017-0`; SHA-256 `e37b393b6f05f9b202c3c4ac2a8c19a60cb1f84945bc51118aa13098f2d9dbb0`. |
| `vendorable/Bonan2014_StomatalConductance.pdf` | `vendorable` | Article text states Creative Commons Attribution 3.0; DOI `10.5194/gmd-7-2193-2014`; SHA-256 `f30cf69192383fd10e231f858c81e5ad9a5649e653bef419c638b428a4b32fe0`. |
| `vendorable/Martens2017_GLEAMv3.pdf` | `vendorable` | Article text states Creative Commons Attribution 3.0; DOI `10.5194/gmd-10-1903-2017`; SHA-256 `51eb4aa1a69bfea44fe06d41d5891f18ec44d0635e6370a34fd4770119ff5eb2`. |
| `vendorable/LaschBorn2020_4C_v2_2.pdf` | `vendorable` | Article text states Creative Commons Attribution 4.0; DOI `10.5194/gmd-13-5311-2020`; SHA-256 `2a82f7123cbf262c2845be2ac87c41b998a308f8f28f4b55cb376fe6c61d4e19`. |
| `copyrighted/ShuttleworthWallace1985_NERC_Report.pdf` | `copyrighted/` | NERC report scan includes the full Shuttleworth-Wallace article as Appendix IV but states no affirmative redistribution permission; SHA-256 `b761d661f007a52a5f6c7dcbf0c7d3e9a82698b6e79c6290844c6cefec278626`. |
| `copyrighted/MOD16_User_Guide_V6.pdf` | `copyrighted/` | Official NASA-hosted Version 2.2 user guide, but the PDF states no affirmative redistribution permission; SHA-256 `a43b47bc33256cad2c7f61566bcf32cd8365383a0eee4b6f02f93042fdcdb687`. |
| `copyrighted/Pereira2016_WetCanopy.pdf` | `copyrighted/` | Accepted manuscript states CC BY-NC-ND 4.0. The noncommercial/no-derivatives terms are conservatively treated as local-cache-only; DOI `10.1016/j.jhydrol.2016.01.035`; SHA-256 `634d235c0a82e0723dcc5144ecaabed18cb7b426542ee2b783aa64f53a2abca0`. |
| `copyrighted/Cain1998_ModellingEvaporationPlantCanopies.pdf` | `copyrighted/` | Institute of Hydrology Report No. 132 states Institute copyright and no affirmative redistribution permission; SHA-256 `066e8c836786963748bd39601ad3dbea5abfc6a5c8cea53a209aee0f0474538d`. |
| `copyrighted/gash1979.pdf` (+ `gash1979.md` transcription) | `copyrighted/` | Operator-supplied Wiley/Royal Meteorological Society journal article with no affirmative redistribution license; DOI `10.1002/qj.49710544304`; PDF SHA-256 `920091bea907032133bf3f56d1171ba3b59a8957acaeaecc3043a73924388f22`; Markdown SHA-256 `c90d6c3dc8f78e82de0519815bbdffc1efa53f1b3f4514c6dd09da3ee55aff2b`. |
| `copyrighted/jarvis1976.pdf` (+ `jarvis1976.md` transcription) | `copyrighted/` | Operator-supplied article whose PDF states Royal Society copyright; DOI `10.1098/rstb.1976.0035`; PDF SHA-256 `c8f683110be5b0ce033106466f237f21ff28b2fb02f4f3c9640f1838930ccb10`; Markdown SHA-256 `e4ae72367fbae040b0791c661340b1349089f948c637147e2063e93c3ecd9ae5`. |
| `copyrighted/stewart1988.pdf` (+ `stewart1988.md` transcription) | `copyrighted/` | Operator-supplied Elsevier journal article with no affirmative redistribution license; DOI `10.1016/0168-1923(88)90003-2`; PDF SHA-256 `df1719eb3c7b6f78c3d2d55509b077565bc7fef0e7744d67d80c6b907d06c598`; Markdown SHA-256 `ec0ef1e89187472a5428daa8d62c7dea77b76250fa3f2dc925ec76f5a1dc5652`. |
| `copyrighted/kelliher1995.pdf` (+ `kelliher1995.md` transcription) | `copyrighted/` | Operator-supplied Elsevier journal article with no affirmative redistribution license; DOI `10.1016/0168-1923(94)02178-M`; PDF SHA-256 `84dbc68328d6ea8686753057e95242deb7eff1f266cc6a72943f0e318c57b95b`; Markdown SHA-256 `ffe32d37153ccbd87663f6633d21635145588b4da8068b8972c64f7612020a99`. |

The four operator-supplied papers and their Markdown transcriptions are locally
available for primary-source review but are not Git-vendored or distributed.
The ignored cache follows the affirmative-license rule for both originals and
full-text derivatives.

## Addendum 2026-08-08 (licensed RHESSys East Coast source intake)

| Source | Classification | Basis |
|---|---|---|
| `laurencelin/RHESSysEastCoast` at `375c75b1cd2202217651dff43aa113d80b9c1118` | `vendorable-source` | Repository `LICENSE` is MIT, copyright 2021 Laurence Lin; SHA-256 `4fd4ecf2fd01cf53c99754bcac5a6dbee255a0be0539dd84ffe12e06808374be`. Local checkout: `/workdir/RHESSysEastCoast`. |
| `laurencelin/GIS2RHESSys` at `6b20883dea7c9fd92f71ec69eaca015ebf6dfe18` | `vendorable-source-and-data` | Repository `LICENSE` is the same MIT text and digest. Local checkout: `/workdir/GIS2RHESSys`; `vegCollection.csv` and definition-generation logic are covered by the repository license. |

No source files are vendored by this docs-only increment. A later implementation
increment may vendor bounded source-derived fixtures or compatibility data only
with the MIT copyright and permission notice, exact commit/file provenance, and
contract-adjudicated scientific role. This classification does not apply to the
separate official RHESSys repository lacking a license.

## Addendum 2026-07-10 (WSHED-W11A channel-sediment authority acquisitions)

| File | First-pass classification | Basis |
|---|---|---|
| `vendorable/creams/312.pdf` | `vendorable/` | Knisel (ed.) 1980, USDA Conservation Research Report No. 26 — US-government work (17 U.S.C. 105). Indexed as R-104. |
| `vendorable/creams/312-ch3.pdf` (+ `312-ch3.md` conversion) | `vendorable/` | Foster et al. 1980, chapter of the same USDA report. Indexed as R-105. |
| `vendorable/kineros/703.pdf` (+ `703.md` conversion) | `vendorable/` | Woolhiser, Smith & Goodrich 1990, USDA-ARS ARS-77 — US-government work. Indexed as R-106. |
| `vendorable/HEC_RAS_1D_Sediment_Transport_UserManual_20260710.pdf` | `vendorable/` | USACE HEC-RAS public documentation capture, consistent with existing HEC-RAS classifications (R-16). Indexed as R-107. |
| `copyrighted/Gilley,Woolhiser,McWhorter_1985.pdf` (+ `.md` conversion) | `copyrighted/` | Transactions of the ASAE 28(1) — ASAE journal copyright; local-only cache, metadata tracked. Indexed as R-108. |
