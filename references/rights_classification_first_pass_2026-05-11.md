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

## Follow-up

- Add per-entry rights status fields in `annotated_bibliography.md`.
- Review HEC-HMS web exports and USDA/ASCE/AGU-derived artifacts individually.
- Reclassify from `copyrighted/` to `vendorable/` only with explicit evidence.
