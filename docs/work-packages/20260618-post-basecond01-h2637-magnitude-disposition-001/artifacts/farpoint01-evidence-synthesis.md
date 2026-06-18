# FARPOINT01 Evidence Synthesis

Evidence class: Static + Ran

Status: complete.

Package:
`20260618-post-basecond01-h2637-magnitude-disposition-001`

## Question

FARPOINT01 surfaced that H2637 routes `71.0%` of precipitation to PASS
`runvol`, while the bounded legacy without-UI comparator reports `55.5%`.
The question for this package is no longer whether another known internal term
needs investigation. It is whether the completed investigation chain supports a
defect disposition, an expected Stage-2 divergence, or an external-authority
gap.

## Evidence Chain

| Step | Evidence | Result |
|---|---|---|
| Original FARPOINT01 magnitude flag | [`fc-legacy-closure-contrast.md`](../../20260613-mofe-farpoint01-high-ofe-routing-closure-demonstration-001/artifacts/fc-legacy-closure-contrast.md) measured openWEPP H2637 without UI at `14,085,670 m3`, `71.0%` of precipitation. Legacy without UI was bounded at `55.5%`; legacy with UI was non-conserving at `127.7%`. | The comparator delta was a Stage-2 magnitude flag under ADR-0017, not a conservation failure and not a legacy parity target. |
| Inter-OFE transfer, area scaling, export, and conservation | [`magparity01-per-term-verdict.md`](../../20260618-mofe-magparity01-per-ofe-runoff-magnitude-adjudication-001/artifacts/magparity01-per-term-verdict.md) found adjacent `UpStrmQ` carry residual `2.27e-13 mm`, exact adjacent `SubRIn` carry, area duality residual `6.82e-13 mm`, transfer residual `0.0 mm`, and PASS export residual `5.46e-12 m3`. | No `INV-RUNOFFPART-028`, area-scaling, export, or conservation defect. |
| Runoff partition source | [`magparity01-runoff-decomposition.md`](../../20260618-mofe-magparity01-per-ofe-runoff-magnitude-adjudication-001/artifacts/magparity01-runoff-decomposition.md) decomposed PASS `runvol` into `97,987 m3` local surface residual plus `13,987,683 m3` routed lateral flow from OFE1-OFE18. | The 71% signal is routed lateral/subsurface magnitude, not a local-surface or aggregate-area artifact. |
| WB19 lateral equation and operands | [`latqcc-equation-correctness.md`](../../20260618-stage2-latqcc-h2637-magnitude-001/artifacts/latqcc-equation-correctness.md) recomputed selected H2637 WB19 rows. Maximum residuals were floating-point scale: WAT `latqcc_mm - q_m * 1000 = 0.0 mm`; equation potential residual `4.163336342344337e-17 m`; `Qd - (q + Qdd) = 0.0 m`. | WB19 `latqcc` is equation-correct for the traced rows; no lateral equation, withdrawal, or capacity defect was found. |
| STAGE2-LATQCC disposition | [`latqcc_disposition.md`](../../20260618-stage2-latqcc-h2637-magnitude-001/artifacts/latqcc_disposition.md) classified the remaining question as `CONTRACT-GAP` because current contracts and suites lack an external absolute-magnitude authority for H2637. | No code or contract fix was authorized by equation/operand evidence. |
| `ksatadj` source-intent defect | [`review-claude-independent.md`](../../20260618-refintent001-ksatadj-satfrac-defect-closure-001/artifacts/review-claude-independent.md) verified the `sat_frac = avsat/(avpor*avcpm)` fix is correct for `ksatadj = 1` soils, but H2637 has `ksatadj = 0` and the post-fix H2637 WAT SHA matched the pre-fix run. | `ksatadj` was a real defect and was fixed, but it is byte-inert on H2637 and not the 71% driver. |
| Base conductivity sensitivity | [`base-cond-per-step-verdict.md`](../../20260618-stage2-base-conductivity-h2637-magnitude-001/artifacts/base-cond-per-step-verdict.md) reported that `ksat_x0.9` changed WAT/PASS checksums, aggregate `latqcc`, PASS `runvol`, and peak WAT `latqcc`. | H2637 magnitude is byte-sensitive to raw base soil conductivity. |
| Base conductivity source-intent split | [`base-cond-per-step-verdict.md`](../../20260618-stage2-base-conductivity-h2637-magnitude-001/artifacts/base-cond-per-step-verdict.md) found raw H2637 `.sol` `ksat` parsing correct, H2637 `ksatadj = 0`, hourly lateral `ui_ssh` arithmetic correct for H2637, and runtime WB19 consumption of `wb19_lateral_ssh` correct. The defective term was vertical `wb18_perc_ssc` only. | The lateral conductivity lineage was verified; the vertical projection defect was routed to BASECOND01. |
| BASECOND01 correction and rerun | [`h2637-rerun-evidence.md`](../../20260618-basecond01-ssc-harmonic-normalization-defect-closure-001/artifacts/h2637-rerun-evidence.md) recorded the post-BASECOND01 H2637 no-UI rerun: `runvol_pct_precip` stayed `71.0036550031206`, aggregate deltas were `0`, and peak `latqcc` moved only by `4.2632564145606e-14 mm`. [`basecond01_disposition.md`](../../20260618-basecond01-ssc-harmonic-normalization-defect-closure-001/artifacts/basecond01_disposition.md) closed the vertical `ssc` defect. | Vertical `ssc` is fixed and is aggregate-inert on H2637. It must not be reopened to chase the comparator flag. |
| Independent post-BASECOND01 review | [`review-claude-independent.md`](../../20260618-stage2-base-conductivity-h2637-magnitude-001/artifacts/review-claude-independent.md) concluded that two conductivity defects are now ruled out as H2637 drivers, while the lateral conductivity lineage is verified end to end. | The H2637 71% result is correct-by-construction under current source-intent contracts; only absolute magnitude remains outside current authority. |

## Synthesis

The complete chain leaves no open in-envelope openWEPP defect for the H2637
71% result:

- transfer, conservation, area duality, and export close;
- WB19 equation and selected high-magnitude operands reconstruct to
  floating-point precision;
- `ksatadj` was fixed but is byte-inert because H2637 has `ksatadj = 0`;
- vertical `ssc` was fixed but is aggregate-inert on H2637;
- the remaining byte-live driver is the lateral base-conductivity path, and
  that path is source-intent correct for H2637.

The legacy `55.5%` value remains an ADR-0017 comparator flag, not a target. The
legacy with-UI `127.7%` value is non-conserving and cannot supply an authority
envelope.

## Result

FARPOINT01's H2637 71% magnitude flag is resolved as `NO DEFECT` / expected
Stage-2 divergence by a verified lateral lineage. The residual question is not
"which internal term is defective?" but "is this absolute H2637 forest lateral
flow magnitude physically correct against external authority?" Current openWEPP
contracts and suites do not provide that authority, so the remaining absolute
magnitude question is a documented `CONTRACT-GAP`.
