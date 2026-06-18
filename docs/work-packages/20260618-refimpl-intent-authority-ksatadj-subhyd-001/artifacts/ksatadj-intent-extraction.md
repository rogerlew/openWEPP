# `ksatadj` Intent Extraction

Package:
`20260618-refimpl-intent-authority-ksatadj-subhyd-001`

## Evidence Class

Static:

- Pinned legacy source:
  `/workdir/wepp-forest_260430_baseline`
  at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Source files inspected:
  `/workdir/wepp-forest_260430_baseline/src/input.for`
  and `/workdir/wepp-forest_260430_baseline/src/infpar.for`.

Ran:

- No legacy binary execution. This artifact extracts source intent only.

## Source Anchors

Static:

- `input.for:467-473` reads disturbed-soil policy:
  `ksatadj`, `ksatfac`, `ksatrec` for `solwpv > 7778` and `solwpv <= 9002`;
  `ksatadj` and `lkeff` for `solwpv >= 9003`.
- `input.for:592-623` applies minimum saturated-conductivity handling and
  converts input `ssc2` from `mm h^-1` to `m s^-1`.
- `input.for:748-928` normalizes input soil layers to WEPP 200 mm runtime
  layers and preserves the modern `solwpv = 9002` VG/theta lineage.
- `infpar.for:237-260` computes top-two tillage-layer weighted averages:
  `avpor`, `avcpm`, `avsm15`, `avthetafc`, and `avthetadr`.
- `infpar.for:286-296` computes total tillage-layer water:
  `avsat = (st(1)+st(2))/tillay(2) + avsm15`, capped by `avpor * 0.98`.
- `infpar.for:606-648` applies the second cap against `avpor * avcpm`, computes
  `sat_frac`, applies the 9001/9002+/9003 `ksatadj` formulas, and writes
  `ks = keff / 3.6e6`.

## Extracted Intent

For `ksatadj = 1`, source intent is:

1. Work on the current effective `ks` in `m s^-1`.
2. Convert `ks` to `mm h^-1` before branch formulas by multiplying by `3.6e6`.
3. Compute top-two tillage-layer weighted terms with
   `weight_i = dg_i / tillay(2)`.
4. Use total water content for saturation:
   `avsat = (st_1 + st_2) / tillay(2) + sum(thetdr_i * weight_i)`.
5. Use `avpor * avcpm` as the saturation denominator.
6. Clamp source-intent saturation through the two caps:
   `avsat <= avpor * 0.98`, then
   `avsat < avpor * avcpm` with the second cap set to `0.99` of that product.
7. Compute `sat_frac = min(avsat / (avpor * avcpm), 1.0)`.
8. For `solwpv = 9001`, use the exponential recovery branch:
   `keffu = ks * 3.6e6`, `keffl = keffu / ksatfac`, then
   `keff = ((keffu-keffl)/(exp(1/ksatrec)-1)) *
   (exp(sat_frac/ksatrec)-1) + keffl`.
9. For `solwpv >= 9002`, use the Saxton-Rawls exponent branch:
   `psi = (ln(1500)-ln(33))/(ln(avthetafc)-ln(avthetadr))`,
   `lambda = 1/psi`, and
   `keff = (ks * 3.6e6) * sat_frac^(2*lambda + 3)`.
10. For `solwpv = 9003`, if `lkeff > 0`, apply `keff = max(keff, lkeff)`.
11. Convert `keff` back to `m s^-1` as `keff / 3.6e6`.

## Non-Authoritative Legacy Behavior

Static:

- Legacy binary output magnitude is not part of the authority. ADR-0017 still
  demotes comparator agreement/divergence to investigation evidence.
- Known legacy non-conservation or disabled branch behavior is not encoded as
  source intent.
- The authority is the explicit algorithm above, not a demand to match a WAT,
  PASS, or outlet runoff number.
