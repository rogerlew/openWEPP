# FARPOINT01 F-C — Legacy-vs-openWEPP closure contrast on H2637 (19 OFEs)

Status: COMPLETE 2026-06-16
Evidence mode: **Ran** (both engines, both `wepp_ui` variants) + **Static** (duality/bound reasoning)

Satisfies the package criterion "measure legacy's own closure on the same
high-OFE substrate (the comparator-trust curve continued past 10)." Legacy is a
**flag, not a target** ([ADR-0017](../../../decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md));
no `OPENWEPP-DEFECTIVE`/`LEGACY-DEFECTIVE` verdict is drawn beyond what an
assumption-light physical bound supports.

## Dispositive metric — outlet runoff vs precipitation (assumption-light)

The exported hillslope surface runoff is `QOFE_outlet × A_outlet ≡ Q_outlet ×
A_hillslope` (the WSHED01 duality). On H2637 the legacy columns satisfy this
duality **exactly** (ratio `1.0000`), so the exported-runoff volume is computable
without per-OFE routing assumptions. Total precip is `19,837,945 m³`.

| engine / variant | outlet runoff | % of precip | runoff ≤ precip? |
|---|---|---|---|
| **openWEPP** without_ui | 14,085,670 m³ | **71.0 %** | ✅ |
| **openWEPP** with_ui | 14,085,670 m³ | **71.0 %** | ✅ |
| legacy `wepp_260606` without_ui | 11,011,150 m³ | 55.5 % | ✅ |
| legacy `wepp_260606` **with_ui** | 25,331,296 m³ | **127.7 %** | ❌ **violated** |

**Legacy with `wepp_ui` produces outlet runoff exceeding total precipitation
(127.7 %)** — physically impossible at the hillslope boundary, the documented
WB-05A OFE19 hourly q-cap non-conservation, now quantified. openWEPP is
**`wepp_ui`-invariant** (71.0 % either way — it has no q-cap path that breaks
conservation) and runoff-bounded on the same substrate.

This is the FARPOINT01 differentiating result at the output-surface level:
openWEPP's conservation closure holds at 19 OFEs where legacy's output surface
does not.

## Corroborating signature — per-OFE reported-column non-closure

A like-for-like per-OFE water balance from each engine's published per-OFE
columns:
- **openWEPP**: closes at `< 1e-11` (the hard per-element + hillslope-total
  fail-closed gate — F-B; exit 0 across 235,961 rows × 19 OFEs × 34 yr).
- **legacy**: does **not** reconcile; the whole-run per-OFE residual grows toward
  the terminal OFE and is amplified by `wepp_ui` (OFE19 residual `−1.12e6` mm
  without_ui → `−2.95e6` mm with_ui). This is the `QOFE`/`Q` (slplen vs totlen)
  normalization split in legacy `watbal.for` — `UpStrmQ`/`QOFE` are not
  mass-equivalent in the reported columns (carved-letter RC, ratio ≈ OFE
  ordinal). It is an **output-surface** reporting inconsistency; whether legacy's
  internal physics conserves is a separate question this package does not
  adjudicate.

## Honesty boundaries

- The full hillslope-total residual *including lateral* (`latqcc`) is sensitive to
  a legacy MOFE routing reconstruction (outlet-only vs per-OFE lateral egress) I
  cannot pin from the columns alone — so the **runoff ≤ precip bound** (duality-
  verified, assumption-light) is the dispositive metric here, not a precise
  legacy total residual. Reconstructing legacy's MOFE routing is out of scope
  (ADR-0017: do not build a legacy-physics authority).
- openWEPP 71.0 % vs legacy without_ui 55.5 % is a **magnitude** difference, not a
  conservation one — both are bounded. Magnitude fidelity is a Stage-2 question
  (`MOFE-MAGPARITY01`, ROADMAP), judged against the already-closed structure.

## Inputs

- Legacy: `/tmp/openwepp_farpoint01_h2637/{without_ui,with_ui}/output/H2637.wat.dat`.
- openWEPP: `.../{without_ui,with_ui}/owepp_output/H2637.wat.parquet` (F-B build).
