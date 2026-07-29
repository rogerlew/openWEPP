# Execution Incident 001 — Negative Reconstructed VPD

Evidence class: `Ran`

Status: `OPEN BLOCKER`

## What ran

On 2026-07-28, `tools/execute.py` regenerated the frozen inputs and invoked
the package-local Rust executor over the two sites and 37 accepted members.
The executor failed closed before publishing any daily result file:

```text
Error: "invalid VPD for SH-EN-ALERCE 2022-07-22"
```

The focused producer-phase and consumer-ordering tests did not run because
the executor failure terminated the fixed sequence first.

## Independent diagnosis

An independent calculation over all 3,332 frozen forcing rows found three
negative values under OBL-PLANT-P-013:

| Site | Date | VPD (Pa) | Tmax (C) | Tmin (C) | Tdew (C) |
|---|---|---:|---:|---:|---:|
| SH-EN-ALERCE | 2022-07-22 | -58.8605 | 11.10 | 5.86 | 9.44 |
| SH-EN-ALERCE | 2022-09-15 | -70.4924 | 10.97 | 5.32 | 9.30 |
| SH-EN-ALERCE | 2025-09-09 | -1.00224 | 12.29 | 9.71 | 11.06 |

The 2025 occurrence lies inside an observational scoring year. This is not
only a disposable warm-up issue.

## Disposition

No clipping, dew-point adjustment, day deletion, interpolation, or
member/site omission was applied. Each would violate the frozen protocol and
the contract's fail-closed negative-VPD rule without explicit canonicalization
authority. No partial daily output was published.

CAL-07 therefore moves to `HOLD / FORCING AUTHORITY INCOMPATIBLE`. Diagnostic
tables and figures may be produced from the frozen source inputs, but no
canopy result, observational agreement verdict, or Order 7 advancement may be
claimed.
