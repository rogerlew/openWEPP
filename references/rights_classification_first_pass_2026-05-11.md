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

## Left in `copyrighted/` (pending review)

All remaining imported artifacts stay in local cache pending targeted rights
verification, including journal papers, book chapters, and web-page exports
with ambiguous footer/site-level copyright statements.

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
