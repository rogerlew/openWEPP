# ASSURE-06 Terminal Verification B

Date: 2026-07-16 UTC
Role: package-authorized independent terminal verifier B
Evidence class: Static + Ran

## Verdict

**PASS — no actionable findings.**

This is coding-agent verification, not human scientific review, approval, or
publication authorization.

## Non-Deferral Audit

- The actual source and rendered reports contain the scored/read/excluded phase
  populations, eligibility rules, both confusion matrices, accuracy, threshold
  error, and humidity analysis.
- The actual reports lead snow results with ten site-resolved paired counts,
  rubric counts, density KGE, timing offsets, and depth-to-SWE ratios. The pooled
  67.8% is secondary and explicitly non-independent.
- The actual reports separate frost-tube and isotherm evidence, disclose
  site-specific adverse extrema and Mandan/Reynolds heterogeneity, and retain
  failed or unavailable snow controls.
- The conservation section identifies four selected rows, renders the storage,
  input, sink, and residual operands, and disclaims all-row inference.
- Both internal reviewers returned PASS after remediation. All 11 findings are
  accepted and closed; none is waived, rejected, deferred, or undispositioned.
- ASSURE-05 report and package paths are unchanged. Both ASSURE-05 and ASSURE-06
  remain held behind human authority, and the protected public report count is
  zero.

## Independent Runs

- Exact reconstruction against the retained 188-value result: PASS.
- Named and all-source validation: PASS; two `DRAFT` reports, zero public.
- Named planning and public `check --all`: PASS.
- Both designated disposable staging roots: PASS and byte-identical.
- `cargo nextest run --workspace --profile assurance-editorial`: 65/65 PASS,
  run `107502b5-4671-4d6a-93fa-bdff94160e17`.
- `cargo nextest run --test assurance_dossier_build_contract`: 13/13 PASS, run
  `30ae2c38-59f0-4ff9-be24-e2d1714cfad4`.
- `git diff --check`: PASS.
- No production Rust changed; the four touched test files remain below the
  line-count thresholds.

Terminal report-source anchor: `report.yaml` SHA-256
`feb093721686875ddf1ef59e1f0c1f8a6981608a8dad2e9e602d17733afe9d3a`.

The package may close only as `HOLD-HUMAN-APPROVAL`. Anurag or Erin is named
only conditionally if they accept an accountable role; no person is assigned by
this verification.
