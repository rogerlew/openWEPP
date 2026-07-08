# Disposition

Evidence mode: Static + Ran.

## Review Findings

### A-H1 Runtime `max_dt_s` Metadata Missing

Disposition: Accepted and fixed.

Changes:

- `DirectLanedActiveRunSummary` now carries `max_dt_s`.
- `DirectLanedActiveTraceRecord` now carries `max_dt_s`.
- `DirectLanedActiveTraceDetail` now carries `max_dt_s`.
- The run manifest `laned_active` block serializes `max_dt_s`.
- The trace JSON row and selected trace-detail payload serialize `max_dt_s`.
- The final ladder was rerun after the fix; every rung's manifest-owned
  `laned_active.max_dt_s` matches its material
  `OPENWEPP_LANED_ACTIVE_MAX_DT_S`.

Regression:

- Focused orchestrator test routes with `max_dt_s = 150` and asserts both
  trace-detail and trace-row metadata retain `150`.

### A-M1 Duplicate 300 s Cap

Disposition: Accepted and fixed.

The runner no longer defines an independent `300.0` parser bound. It uses the
orchestrator-exported `LANED_ACTIVE_MAX_DT_S`, which is the runtime policy
constant consumed by active routing.

### B-H1 Closure Artifacts And Gates Incomplete

Disposition: Accepted; closure artifacts are being completed in this package.

This disposition, gate-results, line-count, verification, final-disposition,
and handoff artifacts close the missing-artifact class once gates are recorded.

### B-H2 Kernel Profile Compliance Artifact Missing

Disposition: Accepted and fixed.

`artifacts/kernel-profile-compliance.md` records the profile checklist for the
rev-43 contract/runtime projection change.

### B-M1 Analyzer Replay Required Ignored Raw Traces

Disposition: Accepted and fixed.

`artifacts/timestep-policy-analysis-inputs.json` is now a committed compact
extract of the six target trace rows and step traces. The analyzer falls back
to that extract when ignored raw traces are absent. Replay was verified by
temporarily moving `artifacts/timestep-policy-runs/` aside and rerunning the
analyzer successfully.

### B-M2 Focused Selector Test Evidence Not Recorded

Disposition: Accepted and fixed in gates.

Gate results record focused runner selector tests and orchestrator active
router tests, including the new provenance regression.

### B-L1 Analyzer Zip Comparisons Could Truncate

Disposition: Accepted and fixed.

The analyzer now fails explicitly on pair length or outlet-bin-span mismatch
before computing L1/Linf/top-diff metrics.

## Final Technical Disposition

The package verdict remains `TIMESTEP-POLICY-ARTIFACT-CLOSED`. The fixed-300
`mn_corn_h4` day-792 miss is not a routed-shape tolerance failure and does not
justify widening tolerance. It requires future target-`dx` promotion evidence
to be coupled space-time evidence.
