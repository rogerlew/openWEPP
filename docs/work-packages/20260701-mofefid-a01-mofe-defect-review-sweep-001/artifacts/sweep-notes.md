# A01 Sweep Notes

Working notes per surface (S1..S6). Findings graduate to `findings.md`
with dispositions. Evidence classes marked per claim.

## S3 — publication geometry (seeded from B01 evidence)

- `Q` and `QOFE` publication (`01_publication.rs:361-379`): `q = q_runoff ×
  efflen/cumulative_length`, `qofe = q_ofe × efflen/ofe_length` (pre-260516
  legacy convention — dispositioned as `MOFEFID-B02` contract decision, not
  an A01 defect). `runvol = QOFE × per-OFE area` — the legacy cancellation;
  self-consistent with the QOFE convention. Runner geometry gate blocks
  `efflen > cumulative_length + 1e-9` (`05_runner_execution_and_outputs.rs:204`).
- **Seed finding F-A3:** dormant `QcapSoftLimit` clamp-status taxonomy value
  (`sim-contract status.rs:113`) — defined, tested in the taxonomy test,
  never emitted by any producer. Dead taxonomy surface. [Static]
- Peak operands (`compute_r7d6_peak_runoff`, `runoff.rs:690-789`): floors +
  duration clamp; `efflen` used as kinematic timing length. No geometry
  duality smell found in the peak path itself (QOFE-basis pairing was
  verified in the T-B2 arc). [Static]

## S4 — per-OFE closure reconstructability (seeded from B01 evidence)

- **Seed finding F-A4 (gap, not defect):** no external tool can reconstruct
  per-OFE conservation from openWEPP's published WAT/PASS the way the
  wepp-forest external audit reads `H.wat`/`H.pass`. `owcmp` is per-column
  diff; snow tools read internal traces. `INV-WATBAL-096` itself warns that
  WB13 row aliases are structural checks, not conservation identities.
  The exported column set (P, RM, Q, Ep/Es/Er, Dp, UpStrmQ, SubRIn, latqcc,
  Total-Soil, frozwt, Snow-Water, …) appears *nearly* sufficient; whether
  it fully closes (interception flux column nullable; snow/frost storage
  deltas derivable from state columns) needs a worked example. The
  **independence property** — an audit computing a residual the model never
  sees — is the wepp-forest program's one unambiguous methodological win
  and openWEPP currently lacks it. Disposition direction: follow-up
  (external per-OFE closure audit tool, shaped by the B11 latqcc-day
  design constraint), candidate Lane C2 sibling. [Static]
- **Seed finding F-A5 (hardening):** R4B `closure_residual_m`
  (`storage.rs:823-843`) re-evaluates the assignment RHS minus itself —
  algebraically zero; catches only non-finite arithmetic. The substantive
  guards are nonnegativity + the projection ledger-vs-state pair
  (`projection.rs:179-204`). The naming invites false confidence in
  contract citations. [Ran — verified this package]

## Cross-references

- B10's `INV-SNOWFREEZE-015` net-algebra vs SNOWSCI-S1 positive-parts
  tension is dispositioned in B01 (contract-decision follow-up); not
  re-opened here.
- **Seed question F-A1 (S1):** `INV-RUNOFFPART-029` mandates the lane
  transfer state carry a "runoff-continuation/case-classifier outcome";
  the four-case classifier appears only in `erosion.rs:749-796` as a
  *validator* of an externally supplied `case_value`. Producer trace in
  progress — if no runoff-path computation exists, this is a spec-vs-code
  consistency finding. [Static, open]
