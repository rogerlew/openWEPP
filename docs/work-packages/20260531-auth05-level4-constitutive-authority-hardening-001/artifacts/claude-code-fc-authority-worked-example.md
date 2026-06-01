# Worked Example — Independent −33 kPa Field-Capacity Authority vs Model (H1)

Author: Claude Code
Date (UTC): 2026-05-31
Status: **SUPERSEDED / CORRECTED (2026-05-31).** The central conclusion below —
that the model FC is "~2× too low" against a "−33 kPa physics authority" — is
**withdrawn**. The "authority" used here (`Σ fc_measured·dg = 223 mm`) is the
wepppy SSURGO producer's *declared* `field_cap`, which is `(wthirdbar_r) /
(1−rock)` — a rock-INFLATED pipeline intermediate (`ssurgo.py` `ERIN_ADJUST_FCWP`),
NOT an independent whole-soil field capacity. That intermediate is designed to be
reduced by WEPP's `cpm` (`scon.for`); legacy WEPP consumes the same wepppy files
and applies `cpm`, so the model's `cpm`-corrected value (~107 mm, matching legacy
~114 mm) is the contract-correct value, not an error. openWEPP must apply `cpm`
to honor the wepppy-producer↔WEPP contract. See SC-SOIL-001 (FC/WP producer
contract) and the AUTH12 review (`claude-code-review-findings.md`). The numbers
below are retained for the audit trail; do not cite this artifact as evidence of
an FC defect.
Scope: single hillslope (H1 / `p1.sol`), field capacity (`ProfileFCStore`).

Evidence: **Ran** unless marked Static. Reproduction commands in §7.

---

## 1. Purpose

Provide the first constitutive check that uses an authority **independent of both
the model and legacy WEPP** — the soil-physics definition of field capacity
(volumetric water content at −33 kPa / 1/3 bar) — and compare it to the model's
published `ProfileFCStore`. This is the kind of check the AUTH03/AUTH05 Level-4
gates do not perform (those compare the runtime surface to a re-implementation of
the model's own `cpm`/`coca`/`sm20c` pipeline; see
`docs/work-packages/.../auth03.../artifacts/claude-code-review-findings.md`).

## 2. The authority

Field capacity is defined as the volumetric water content at −33 kPa matric
potential. For the H1 soil (`p1.sol`, WEPPcloud 9002, mukey 2396862, "GRV-ASHY-SIL"
gravelly ashy silt loam, forest), the per-layer θ at 1/3 bar is provided directly
by the soil inputs, and the build note records it as already rock-adjusted:
`78776563::field_cap estimated from wthirdbar_r and rock`. So the declared FC
column is the −33 kPa retention authority on a whole-soil basis, sourced from
SURGO, not from openWEPP's correction chain and not from legacy WEPP.

## 3. Data (Ran — read from `p1.sol`)

| Layer | Depth (mm) | Thickness (mm) | θ_fc (−33 kPa) | θ_wp (−1500 kPa) | bd (g/cm³) | om % | rock % (hdr) |
|---|---|---|---|---|---|---|---|
| 1 | 0–200 | 200 | 0.258 | 0.070 | 0.8 | 5.0 | 0 |
| 2 | 200–290 | 90 | 0.258 | 0.070 | 0.8 | 5.0 | 17 |
| 3 | 290–570 | 280 | 0.214 | 0.042 | 0.8 | 2.0 | 25 |
| 4 | 570–1130 | 560 | 0.082 | 0.014 | 0.8 | 1.0 | 66 |
| 5 | 1130–1600 | 470 | 0.090 | 0.026 | 1.3 | 0.25 | 44 |

(rock % from the `p1.sol` comment header `fraggt10_r + frag3to10_r`.)

## 4. Computation (Ran)

`ProfileFCStore_authority = Σ θ_fc,i · dg_i · 1000`:

```
0.258·200 + 0.258·90 + 0.214·280 + 0.082·560 + 0.090·470
= 51.6 + 23.22 + 59.92 + 45.92 + 42.30  =  223.0 mm   (over the 1600 mm soil)
```

Mean θ_fc = 223.0 / 1600 = 0.139. Extending the bottom layer to the model's
1800 mm normalized profile (+200 mm at θ_fc = 0.09) gives ≈ 241 mm.
`ProfileWPStore_authority` = 52.1 mm (mean θ_wp = 0.033).

## 5. Result (Ran — model from `H1.wat.parquet`, 0213 run; legacy derived from comparator)

| Quantity | ProfileFCStore | mean θ_fc | ProfileWPStore |
|---|---|---|---|
| **Physics authority (−33 kPa)** | **223 mm** (≈241 @1800 mm) | **0.139** | 52.1 mm |
| **Model (openWEPP)** | **107.1 mm** | **0.060** | 31.4 mm |
| **Legacy WEPP** (≈, comparator-derived) | **~114 mm** | ~0.063 | — |
| ProfilePorosityCap (model) | 1092.7 mm | — | — |

## 6. Findings

1. **The model's field capacity is ~2× too low against soil physics** — 107 mm
   vs 223 mm; retained θ ≈ 0.060 where the −33 kPa retention is 0.139.
2. **Legacy carries the same error** (~114 mm ≈ half the authority). Model and
   legacy agree to within ~2–6%, so the legacy-parity comparison reports
   `ProfileFCStore` as "nearly closed" (27/39) **while both are ~50% below the
   physics.** This is the re-anchoring thesis demonstrated on one hillslope:
   parity-against-legacy structurally cannot detect an error legacy shares;
   a physics authority detects it immediately.
3. **Mechanism (leading hypothesis):** the SURGO FC is already rock-adjusted
   (§2 build note), then `scon`/`legacy_correct_layer_moisture` applies the `cpm`
   rock-fragment reduction again (these layers are 17–66% rock) plus the
   `sm20c`/`0.83` heuristics — roughly halving an already-correct value. `cpm`
   double-counting rock is the prime contributor; `FC/porosity = 0.20` means the
   `0.83` cap is not binding.
4. **Consistency with the over-drainage defect:** free drainage relaxes soil to
   field capacity. A field capacity at half its physical value drains the profile
   to roughly half the water it should retain — the documented over-drainage
   (H1 Total-Soil ~72 mm; governance map §1).
5. **Teeth demonstration:** a gate asserting `model ProfileFCStore == θ(−33 kPa)
   store within tolerance` **fails today** (107 ≠ 223). The AUTH03/AUTH05 Level-4
   gates **pass** on this same code, because their "authority" is a
   re-implementation of the model's `cpm` pipeline (which reproduces the same
   half-value). This is the concrete difference between an authority that can
   disagree with the model and one that cannot.

## 7. Reproduction

- Soil: `p1.sol` (H1) from the 0213 parity run
  `/tmp/hphys0213_20260530T233248Z/parity/runs/p1.sol`; θ_fc/θ_wp are the layer
  columns; provenance note in the file header.
- Authority: §4 arithmetic.
- Model values:
  `duckdb "select ProfileFCStore, ProfileWPStore, ProfilePorosityCap from
  '/tmp/hphys0213_20260530T233248Z/parity/hillslope_output/H1.wat.parquet' limit 1"`.
- Legacy value derived from the semantic comparator delta (approximate).

## 8. Caveat (what is proven vs hypothesized)

Proven: the −33 kPa FC store is 223 mm; the model publishes 107 mm; they differ
~2×; legacy ≈ model (parity hides it); a gate on this authority fails the model.
Hypothesis (strongly supported, not exhaustively proven): that 223 is the
physically-correct target and 107 is over-corrected. This hinges on whether
`scon`'s post-(−33 kPa) corrections are physically justified; the rock
double-count (SURGO rock-adjusts, then `cpm` rock-adjusts again) is the leading
explanation, but not every `scon` term has been independently traced. The result
is therefore stated as: **the physics authority and the model disagree by ~2×,
the disagreement is legacy-invariant, and the rock double-count is the leading
cause.**

## 9. Scope note

This is a single-hillslope (H1) worked example. The natural extension is to run
the −33 kPa authority across all 39 hillslopes to quantify how widespread the
under-retention is and whether it scales with rock-fragment content (not yet
run).
