# FQ3-DC RUNOFFPART Kickoff — Q/QOFE underproduction closure

Execution mode: package-end-to-end

Autonomy: execute end-to-end — Milestone-1 localization + ownership, contract
amendment, contract-derived red/green tests, pre-implementation gate, production
correction, validation, dual review/verification, disposition, defect-shaped
handoff — without asking for direction on intermediate steps. Ask only if
hard-blocked.

## Item 1 — close defect `FQ3-DC-RUNOFFPART-QQOFE-001`

openWEPP produces `Q`=0 / `QOFE`=0 on 35/42 post-FQ1 algebraic-radium single-OFE
hillslopes where legacy `wepp_260606` runs off (p8 corn Q=760 mm, p1 tah Q=278 mm;
7 corn nonzero-but-below). Make the infiltration-excess runoff partition engage so
runoff is produced when rainfall intensity exceeds infiltration capacity, per
`SC-RUNOFFPART-001`. Q=0 is universal (corn AND tah), so this is a separate root
cause from the corn-ET DC (`FQ3-DC-ET-CORN-ENGAGEMENT-001`, dispatched separately —
do not touch ET here).

Primary surfaces: `03_kernel_support_01_kernel_phases.rs` (rainfall-excess /
hyetograph-liquid-input / saturation-runoff), `03_kernel_support_00_support_helpers.rs`
(`q_runoff`, ~:4651), `runner/hillslope/mod.rs` (Q/QOFE publication).

## Milestone 1 first (localize + ownership)

Reproduce Q=0 on p8; pick a day legacy runs off; compare openWEPP vs legacy
hyetograph/rainfall-intensity, infiltration capacity, depression storage, and
infiltration-excess on that day. Name the mechanism: gentle storm-intensity
disaggregation (daily climate) / Green-Ampt over-infiltration / depression storage
/ runoff onset / computed-but-not-published. Ownership: legacy runs off on the same
daily gridmet climate → openWEPP defect. **If the cause is upstream storm-intensity
disaggregation, that may be SC-CLIMATE territory — amend in-package if it's the
partition's intensity input, else branch a defect-shaped climate target.**

## Acceptance authority + constraints

- Conversion rule: root cause in-envelope + `SC-RUNOFFPART-001`/`SC-CLIMATE`/physical
  excess-law authority ⇒ MUST land the contract-first fix.
- `wepp_260606` is a FLAG that runoff should be nonzero (ADR-0017), NOT a match
  target. Acceptance = contract-correct infiltration-excess generation.
- **Conservation must still close** (rung-1 identity + totalwatsed3 audit) after
  runoff is produced — do not make runoff by breaking the water balance.
- No comparator-match tuning, silent defaults, or downstream compensation.

## Hard constraints (protected boundaries)

- Do NOT touch annual-crop ET (FQ3-DC-ET-CORN-ENGAGEMENT-001) or p11 percolation
  (FQ1-P11). Snow magnitude → Stage-2. 17-OFE MOFE out of scope.

## Required reading

- `docs/work-packages/20260608-fq3dc-runoffpart-q-qofe-closure-001/package.md`
- FQ-3 `artifacts/fq3-defect-handoff.md` + my FQ-3 review
- `docs/decisions/0011/0017/0018`, `docs/defect_closure_execplans.md`, `AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`,
  `SC-CLIMATE-001.md`, `SC-WATBAL-001.md`
- Comparator `/home/workdir/wepppy/wepp_runner/bin/wepp_260606_hill`; corpora
  `/tmp/fq1_after/outputs`, `/tmp/fq3_exec/legacy/outputs`.
