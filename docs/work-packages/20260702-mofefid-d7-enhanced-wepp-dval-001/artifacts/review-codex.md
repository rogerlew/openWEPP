# Codex Review — MOFEFID-D7 Scaffold

Date: 2026-07-02
Reviewer: Codex
Scope: plan/scaffold review only; no production code or tests modified.
Disposition: **READY TO EXECUTE after scaffold revision**.

## Evidence

Static:
- Read `package.md` and `artifacts/feasibility-findings.md`.
- Read `SC-OFEROUTE-001` (`INV-OFEROUTE-002`, `INV-OFEROUTE-011`, gaps,
  and D-val obligations).
- Read D01 `validation-cases.json` and `source-manifest.md`.
- Read ADR-0011 and ADR-0017 for contract-first / comparator-as-flag posture.
- Read local gitignored `references/copyrighted/Papanicolaou2018.md` §3.1 /
  Table 1. No copyrighted source was vendored.

Ran:
- Inspected local `Figure_4.xlsx` / `Figure_5.xlsx` workbook headers with
  pandas to verify sheet roles, unit heterogeneity, and cut-point risk.
- Ran `sha256sum` on `Figure_4.xlsx`, `Figure_5.xlsx`, and
  `3.1_Validation_Input.docx`; hashes matched the package/manifest values.

No Rust gates were run; this was a documentation scaffold review.

## Findings And Disposition

### CX-D7-001 — Authority Split Must Be Explicit

Severity: High
Evidence: Static

The method-fidelity framing is correct under ADR-0011/0017: D7 should reproduce
the enhanced-WEPP method traces, not claim direct validation against nature.
However, the scaffold mixed the paper's observed-data `Ef` with D7's trace
reproduction metric. That could let a passing openWEPP-vs-enhanced-WEPP trace
be reported as an openWEPP observed-data validation.

Disposition: **Accepted, fixed in package.md.** The scaffold now separates
`NS_trace` (openWEPP vs enhanced-WEPP trace) from paper `Ef_obs`
(enhanced-WEPP vs observed/measured citation) and forbids converting one into
the other.

### CX-D7-002 — NS Alone Can Pass A Wrong Hydrograph

Severity: High
Evidence: Static

`NS >= 0.85` plus peak/time-to-peak is necessary but not sufficient. A case
could pass NS while missing runoff initiation, event volume, rising-limb shape,
shock steepness, modality, or recession behavior.

Disposition: **Accepted, fixed in package.md.** The acceptance model now
requires case-specific shape co-conditions in addition to `NS_trace`, peak, and
timing.

### CX-D7-003 — Case 3 And Zone Taxonomy Were Under-Scoped

Severity: High
Evidence: Static

The original S3 text named bare/isolated rise-to-steady cases but did not
explicitly carry Case 3 vegetation patchiness through D-val. `INV-OFEROUTE-011`
also names Zone 1/Zone 2 stream-power taxonomy, while the scaffold focused on
the four hydrograph cases.

Disposition: **Accepted, fixed in package.md.** S3 now explicitly includes
Cases 1-3 and calls out Case 3's D5/D6 vegetation-strip / infiltration-cascade
composition. S5 now requires either taxonomy reproduction or a contract
amendment that truthfully defers the taxonomy obligation.

### CX-D7-004 — S0 Needed Executable Like-For-Like Criteria

Severity: Medium
Evidence: Static + Ran

S0 was directionally right, but it needed enough required fields to prevent a
repeat of the HPHYS comparator-surface false-result class. The workbook header
inspection confirmed that Figure 4/5 carry different roles, units, and likely
geometry bases.

Disposition: **Accepted, fixed in package.md.** S0 now requires source file,
sheet, columns, role, time origin/unit, discharge unit, geometry basis,
conversion, comparison window, magnitude sanity, and pass/fail disposition.

### CX-D7-005 — Offline NS Needed Reproducibility Control

Severity: Medium
Evidence: Static + Ran

The cited-scalars plus offline-NS approach is copyright-safe, but not
reviewable enough if the extraction code remains a discarded spike. Copyright
policy does not prevent checking in a script that reads local gitignored inputs
and emits only derived scalar metrics.

Disposition: **Accepted, fixed in package.md.** The package now requires a
checked-in copyright-safe extraction/comparison harness that verifies sha256 and
does not commit workbook rows or full hydrograph series.

### CX-D7-006 — Iwagaki Shock Lag Should Be A Known Provisional Gap

Severity: Medium
Evidence: Static

The feasibility spike already observed a material Iwagaki timing lag. Waiting
until S4 to name the gap would understate known risk, while fixing it inside D7
would overrun D-val analysis if it proves to be a solver/cascade defect.

Disposition: **Accepted, fixed in package.md.** S4 now starts with a
package-local provisional shock-lag gap and resolves it by closure, narrowing
it to operands or promoting it to `SC-OFEROUTE-001` if needed. Solver/cascade
code correction splits only after attribution evidence exists.

### CX-D7-007 — Operand-Limited Verdict Must Stay Separate

Severity: Low
Evidence: Static

Unknown or texture-derived operands, especially Iwagaki `k_o`, must not be
tuned into a pass or mislabeled as solver defects.

Disposition: **Accepted, fixed in package.md.** S1 now requires load-bearing
operands to be sourced, frozen, or bounded tightly enough that the verdict is
invariant; otherwise the verdict class is `operand-limited`.

## Final Judgment

The scaffold is ready to execute after the recorded revisions. The next
executor should not start NS comparisons until S0/S1 are closed, and should not
report any result as validation against observations.
