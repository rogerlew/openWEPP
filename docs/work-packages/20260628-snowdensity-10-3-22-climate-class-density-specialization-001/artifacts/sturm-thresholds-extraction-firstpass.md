# Sturm Classification Thresholds — First-Pass Extraction (NAVIGATION AID, NOT AUTHORITY)

- Author: Claude Code, 2026-06-28
- Purpose: unblock the SNOWDENSITY-10.3.22 `HOLD-AUTHORITY-GAP` re-run by locating
  the classification thresholds and density params now that the references are in.
- **STATUS: FIRST-PASS. Every value below MUST be re-verified by Codex directly
  from the source PDFs before use (operator instruction 2026-06-28).** These are a
  navigation aid, not the authority. In particular, the 1995 values here are as
  *reported by Sturm & Liston 2021*, second-hand — they must be confirmed against
  the primary Sturm 1995 source.

## Source status

| Source | Ref | Local | Machine-readable? |
|---|---|---|---|
| Sturm 1995 (original 6-class tree) | R-59 | `copyrighted/sturm1995.pdf` | **NO — scanned, no text layer (needs OCR / visual read)** |
| Sturm 2010 (density params, Table 4) | R-58 | `copyrighted/sturm2010_swe_climate_classes.pdf` | yes (born-digital, extracted) |
| Sturm & Liston 2021 (updated thresholds + 1995 back-refs) | R-61 | `copyrighted/hydr-JHM-D-21-0070.1.pdf` | yes (born-digital, extracted) |
| NSIDC-0768 user guide (algorithm overview) | R-60 | `vendorable/NSIDC-...UserGuide.pdf` | yes |

## Classification variables (all three confirmed)

Air temperature, precipitation, wind speed — reduced to two indices plus wind:

- **CDM (Cooling Degree Month)** — an air-temperature index combining snow-season
  temperature and duration. Form (from 2021 Eq. 1): `CDM = Σ_months max(0, Tc − Ta)`
  over months with mean air temp `Ta < Tc`, where **`Tc = 10 °C`** (critical
  threshold air temp; 2021 lines 163–164). _Verify the exact summation/units in
  source._
- **SPR (Snowfall Precipitation Rate; mm day⁻¹)** — water-equivalent snowfall rate
  (2021 Eq. 2). Used with a high/low threshold.
- **Wind** — 1995 used a wind-speed climatology directly; 2021 substituted a
  land-cover surrogate (trees ⇒ lower wind). **openWEPP has wind, so it should use
  the 1995 wind-based form directly** (cleaner than the land-cover proxy).

## First-pass threshold values — VERIFY EACH FROM SOURCE

| Threshold | 1995 value (pairs with Sturm 2010 names) | 2021 value | Provenance / flag |
|---|---|---|---|
| CDM critical temp `Tc` | 10 °C | 10 °C | 2021 L163–164. Verify in 1995. |
| Ephemeral↔seasonal (very-high CDM) | **30 °C·month** | 61 °C·month | 2021 L151,159 cites 1995=30. **Differs — use 1995=30 for 1995-named classes.** |
| High↔low air-temp (CDM) | ≈ **−25 °C** (retained) | same | 2021 L184–185 ("…(−25 °C) remained the same as in 1995"); OCR-ambiguous — **verify value + meaning.** |
| High↔low precip (SPR) | **2 mm day⁻¹** | 4 mm day⁻¹ | 2021 L208–210 explicitly: "…4 mm day⁻¹; Sturm et al. (1995) used 2 mm day⁻¹". |
| Wind speed threshold | **IN SCANNED 1995 PDF — not machine-extractable** | (land-cover proxy) | **Codex must OCR / visually read the 1995 decision-tree figure for the wind cutoff and the split order.** |

**The decision-tree structure (order of splits across CDM / SPR / wind → one of
{tundra, taiga, alpine, maritime, prairie, ephemeral}) is in the scanned 1995
figure/table and was not machine-extractable here. Codex must read it from the
1995 source.**

## Density params (Sturm 2010 Table 4 — verified from born-digital PDF)

`ρ(h,DOY) = (ρmax − ρ0)·[1 − exp(−k1·h − k2·DOY)] + ρ0` (Eq. 6), ρ in g cm⁻³, h in cm:

| Class | ρmax | ρ0 | k1 | k2 |
|---|---|---|---|---|
| Alpine | 0.5975 | 0.2237 | 0.0012 | 0.0038 |
| Maritime | 0.5979 | 0.2578 | 0.0010 | 0.0038 |
| Prairie | 0.5940 | 0.2332 | 0.0016 | 0.0031 |
| Tundra | 0.3630 | 0.2425 | 0.0029 | 0.0049 |
| Taiga | 0.2170 | 0.2170 | 0.0000 | 0.0000 |

(Taiga: constant 217 kg m⁻³, no densification — the depth-hoar regime.) **Ephemeral
is excluded by Sturm 2010** ("…ephemeral snow it had to be excluded from the
analysis") ⇒ ephemeral packs use a fresh-snow / existing-Anderson fallback (no
seasonal-class densification), documented with that rationale.

## Class-name mapping (critical)

2021 renames: **Alpine → Montane Forest, Taiga → Boreal Forest.** Sturm 2010 Table 4
and the 1995 tree use the **1995 names** — so pair the 1995 tree with Sturm 2010.
If any 2021 threshold is used as a cross-check, map names back to 1995 first.

## Verification requirement (operator, 2026-06-28)

Codex must re-derive ALL thresholds and the tree structure **from the source PDFs**:
- **Primary: Sturm 1995** (OCR / visually read the scanned PDF) — the original tree
  + wind threshold, matching Sturm 2010's class names.
- **Cross-check: Sturm & Liston 2021** — confirm/extend, noting the 30→61 CDM and
  2→4 mm day⁻¹ updates and the class renames; do not silently inherit 2021 values
  for 1995-named classes.
- Treat the values in this file as a navigation aid only.
