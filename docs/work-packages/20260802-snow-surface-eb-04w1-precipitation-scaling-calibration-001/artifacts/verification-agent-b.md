# Verification Agent B — Exact-Terminal QA

Status: `PASS`

Evidence mode: **Ran + Static**.

## Ran Evidence

- Recomputed the freeze, receipt, frozen-tool, and release-binary SHA-256
  identities. They match the bound values: freeze `910e115...892`, receipt
  `47847f9...f95`, tool `5926c5e...346`, and binary `b50dd71...ec9`.
- Independently audited all 32 source/scaled fixture pairs: `490,912` daily
  rows, maximum precipitation residual `2.8421709430404007e-14 mm`, zero
  protected-token changes, zero non-daily-line changes, and 256 byte-identical
  non-climate-file comparisons.
- Verified the exact `4 x 8 = 32` receipt inventory, 32 hash-valid cell
  provenance records, all return codes zero, all bound tool/freeze/binary
  identities, and all 192 retained output hashes and sizes.
- Independently reconstructed every strict joint-improver set and the frozen
  lexicographic selection: Mica Creek `1.5 / 5`, Niwot `1.3 / 4`, Paradise
  `1.5 / 5`, and Snowbird `1.5 / 2`. Maximum reported diagnostic closure is
  `3.331e-15 m`, below the `1e-12 m` acceptance tolerance.
- Byte-compared all four `1.0` WAT outputs and four `1.0` snow traces with the
  retained EB-04W baseline; all eight comparisons pass exactly.
- Ran the transformer self-check and isolated-cache Python compilation; both
  pass. Ran package plus roadmap/catalog Markdown lint, `git diff --check`,
  and SVG XML parse; all pass with zero findings.
- Rasterized and inspected the corrected seasonal-trajectory SVG. All four
  panels render the corrected `Median SWE (m)` axis, and observed/model series
  agree with the sidecar without clipping or overlap.

## Static Evidence

- The exact tracked predecessor diff is limited to `docs/ROADMAP.md`, the snow
  campaign roadmap, and the work-package catalog. The remaining changes are
  inside the new declared package tree. No production Rust, contract,
  manifest, test, source fixture, observation, selector, default, schema,
  assurance, or historical-package path is changed.
- The execution prompt is archived and `prompts/active/` contains only its
  registry README. Figure SVGs have exact same-stem Markdown sidecars.
- The readiness matrix uses the three required ADR-0042 dimensions:
  `IMPLEMENTED`, `CALIBRATION_READY_DATA_LIMITED`, and
  `PARTIALLY_IDENTIFIABLE`. It prospectively retains the observations as
  `CALIBRATION`, records zero independent validation, states that covariance
  is not estimable on this one-coefficient surface, and admits no promotion,
  transferability, or final calibrated multiplier.
- Dual science/QA reviews exist and their numeric/provenance conclusions
  agree. Their findings are enumerated in the review-disposition artifact.
  Heavy Rust validation is legitimately not selected because the reconciled
  write set does not touch production, contracts, manifests, or tests.

## Finding Recheck

My initial terminal scan found that `scientific-disposition.md` still grouped
Mica Creek's boundary selection with Paradise and Snowbird as an unresolved
optimum. The owning agent corrected that statement before this verdict. The
terminal text now distinguishes Paradise and Snowbird's unresolved joint
optima, Mica's already-turning magnitude fit and one-day boundary chronology
gain, and Niwot's continuing magnitude response. This closes V-B1 without a
model rerun or result change and agrees with the review disposition.

## Residual Risks And Verdict

The observations are calibration-only, precipitation scaling remains
confounded with phase, representativeness, retention, and pre-peak loss, three
frozen selections are boundary-censored, and Niwot remains magnitude-low.
Those are correctly retained scientific limitations, not execution failures.

Both independent terminal verifications now pass. Package/roadmap/catalog
status reconciliation and the gate-record final label are mechanical owning-
agent closure steps after these verifier artifacts land; they do not require
new scientific or runtime evidence.

Terminal verdict: `PASS`. All execution, transformation, provenance,
conservation, baseline-replay, figure, write-set, security, schema,
documentation, review-disposition, and ADR-0042 claim-boundary checks pass.
