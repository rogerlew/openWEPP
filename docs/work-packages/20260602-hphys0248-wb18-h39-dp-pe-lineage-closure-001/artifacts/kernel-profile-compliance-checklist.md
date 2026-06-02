# Kernel-Profile Compliance Checklist

Status: completed

Evidence mode: Static + Ran

Static:
- Contract-first sequencing: satisfied, with a second contract/test amendment
  added before the second production correction after H39 exposed the baseline
  `fx=1` branch.
- Canonical authority updated: `SC-PERC-001#INV-PERC-014` and
  `SC-WATBAL-001#INV-WATBAL-036`.
- Baseline provenance cited: pinned
  `/workdir/wepp-forest_260430_baseline` commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No heuristic/proxy production physics added.
- Typed guard posture preserved for missing/non-finite/domain-invalid inputs.
- Disposition cannot claim complete H39 water-balance semantic parity because
  WB17, snow/runoff, lateral, and storage residual families remain open.

Ran:
- Targeted and workspace Rust gates passed.
- Full `H1..H39` runtime and semantic report-generation gates passed.
