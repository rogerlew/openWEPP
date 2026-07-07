# Review Agent A — SC-OFEROUTE-002 provenance completeness/fidelity

Status: **EXECUTED** (2026-07-07)

Lane: **Agent A only** — provenance completeness/fidelity for
`SC-OFEROUTE-002` rev 1 draft and the `SC-OFEROUTE-001` rev-32 pointer
transfer. This review does **not** adjudicate code-vs-contract fidelity; that is
Agent B scope.

Evidence class:

- Static: required docs, `SC-OFEROUTE-002`, current `SC-OFEROUTE-001` rev 28-32
  rows, base `48129fac` hybrid rows, and cited T3/T3AGG/solve-cost artifacts.
- Ran: scoped Markdown lint, BEI checks, SC unit-compliance checks, and pointer
  grep/readback commands listed below.

Lane verdict: **NO-GO** for lifting `status: draft` to `approved` until the
findings below are amended and dispositioned.

## Findings

### High

- **A-H1 — Rev-31 warm-seed validity was weakened by dropping the finite and
  positive acceptance requirements.**
  `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:179`
  says a branch-local warm seed is accepted only on the evaluated branch's side
  of `Q_c`, then falls back otherwise, but it does not preserve the rev-31
  provenance condition that the candidate must also be **finite** and
  **positive** before acceptance. The Branch/Guard row has the same weakening at
  `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:270`, and
  `INV-OFEHYB-003` compresses the proof to "branch-side acceptance" at
  `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:283`.
  Provenance requires the stronger rule: the base `48129fac`
  `SC-OFEROUTE-001` hybrid row required warm seeds to be "finite, positive, and
  on the branch side being evaluated"; the solve-cost package recorded the same
  implementation condition in
  `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/implementation.md:25`.
  This is a real weakening: side-of-`Q_c` alone does not exclude non-finite,
  zero, or negative same-side candidates. Amend Algorithm §3, the Branch/Guard
  row, and the determinism invariant/guard text to bind finite + positive +
  branch-side acceptance with cold fallback.

### Medium

- **A-M1 — GAP-OFEHYB-001 promotes assessment-class predicate ideas as
  "recorded design levers" without provenance.**
  `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:397`
  describes two "recorded design levers" for the Case-4 hold lift, including a
  "spatial wave-quiet predicate" and the assertion that a q-vs-equilibrium
  departure test cannot discriminate. The cited I0 design record only records
  the explicit cool-down fallback:
  `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/i0-scheme-design.md:107`.
  Grep over the cited T3, T3AGG, and solve-cost artifacts found no provenance
  for `wave-quiet` or `q-departure` outside the new draft. The revision history
  then repeats the overclaim by saying the gap carries "two recorded design
  levers" at
  `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:404`.
  For a consolidation draft whose package rule is no new normative content, this
  is smuggled design assessment. Either trim GAP-OFEHYB-001 to the
  I0-recorded explicit cool-down lever only, or relabel the spatial predicate
  and q-departure note as non-binding assessment candidates that require a
  contract-first design increment before they become authority.

## Checked With No Findings

- Rev-29 double-collapse theorem and fail-closed consequence are preserved at
  `SC-OFEROUTE-002.md:154`; the draft also preserves the no-filled-jump commit
  prohibition at `SC-OFEROUTE-002.md:160`.
- Rev-30 aggressive mask, C-M1 hour-partition fail-closed guard, cross-span
  deficit carry, material terminal-deficit failure, and C-L1 bounded all-dry
  sub-noise drop are present in the algorithm/guard/invariant surfaces.
- I0 §2.2 seam semantics are preserved: depth carries, explicit receives
  implicit converged discharges, and implicit re-derives discharge in-solve.
- The implicit `dt` policy is present as bin cadence/no-CFL, with the sample-bin
  width recorded in constants.
- `SC-OFEROUTE-001` rev 32 is a thin pointer transfer: current Branch/Guard,
  Test-Vector, and BEI rows point to `SC-OFEROUTE-002`, while revs 28-31 remain
  historical provenance.
- Structural profile scan found the required contract sections, front matter,
  index row, BEI, gap register, and revision history in the expected places.

## Gate Notes

- Ran `markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md --path docs/specifications/science-contracts/index.md --path docs/work-packages/20260707-laned-router-hybrid-contract-authority-001`:
  **PASS**, 7 files, 0 errors, 0 warnings.
- Ran `python tools/check_sc_binding_exposure.py` on both contracts:
  `SC-OFEROUTE-001` **PASS-DEFERRED** (7 rows, 6 follow-on rows);
  `SC-OFEROUTE-002` **PASS-DEFERRED** (4 rows, 4 follow-on rows).
- Ran `bash tools/release/check_sc_unit_compliance.sh --path ...` on both
  contracts: **PASS** for each.
- Ran pointer greps for `SC-OFEROUTE-002`,
  `OFEROUTE-HYBRID-IMPLICIT-STEPPING`, and
  `OPENWEPP_LANED_ACTIVE_IMPLICIT`; no Agent-A dangling-pointer finding.

## Residual Risks

- Agent B still needs to verify current implementation fidelity, including real
  counter names, guard-map test names, and runtime seed validation behavior.
- The two findings above are documentation/authority amendments only; no
  production-code conclusion is implied by this lane.
