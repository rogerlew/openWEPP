# Oracle Re-anchoring Evidence (D10B-S2, Leg B)

Status: executed (binding recorded rev 24; tolerance ratification follows
S3/S4 evidence)
Evidence mode: Static + Ran (primary extraction commands; D10/D10B runs)

## The decision

Case-4 acceptance authority moves FROM "NS_trace vs the digitized
enhanced-WEPP Figure-4 model trace" TO "grid-refinement convergence to the
Iwagaki-primary characteristics oracle."

## Why the digitized trace cannot be acceptance

1. **Clean-room:** the trace is the output of an implementation openWEPP
   deliberately does not inspect (`docs/planning/mofe-water-balance-sequencing.md`
   §3); matching it is implementation parity with an uninspectable target.
2. **Internally inconsistent spec:** R-63's printed (11c) is a
   transcription error against its own citation chain (Davis 3.20, Mingham
   31f) — the paper text underdetermines the implementation it describes.
3. **(REQUALIFIED at rev 26, review A MAJOR-5):** the D10
   divergence-under-refinement observation (`NS_trace` 0.263 -> 0.101)
   was the signature of the DEFECTIVE pre-rev-24 SOLVER under the
   un-primary `k_o = 200` law, not evidence about the trace —
   post-correction the trace agrees with the oracle within ~2%/1.4 s.
   The demotion rests on grounds 1, 2, and 4 alone (each sufficient).
4. **Confounded operand:** the D-val ran `k_o = 200`, an operand the paper
   never specifies; the primary's own law is Manning `n = 0.009`.

Disposition: ADR-0017-class comparator flag — recorded, expected to
differ, never acceptance (rev 24 `INV-OFEROUTE-011`).

## The oracle

- **Configuration (primary, verbatim):** Iwagaki 1955 experiment (B) +
  duration case (c) `T = 10 s`: 24 m channel, three 8 m reaches,
  `sin theta = 0.020/0.015/0.010`, lateral supplies
  `0.1080/0.0638/0.0800 cm/s`, Manning `n = 0.009` (m-s units).
- **Law:** wide-channel Manning KWE — `q = (sqrt(S_o)/n) h^(5/3)`,
  equivalently the solver's `q = alpha h^1.5` with
  `f = 8 g n^2 / h^(1/3)` — so oracle and solver discretize the SAME PDE
  and the comparison isolates numerics (like-for-like, ADR-0017).
- **Method:** method of characteristics (`dh/dt = v` along
  characteristics during supply; straight-line characteristics after
  cutoff; section-boundary `h` jumps from `q`-continuity across the slope
  change) with kinematic Rankine-Hugoniot shock fitting
  (`ds/dt = [q]/[h]`, Lighthill-Whitham R-01). Reference computation at
  tight integration tolerances with its own convergence/self-consistency
  evidence (`numerics-convergence-evidence.md`).
- **Fidelity caveats (recorded, not acceptance):** the primary's own
  computed/experimental hydrographs include the sidewall correction
  `R = Bh/(B+2h)` (B = 19.6 cm) and laminar/turbulent Re-switching
  (500/1500). The oracle deliberately binds the wide-channel pure-Manning
  form to match the solver's PDE; agreement with Iwagaki's published
  hydrograph figures is therefore a supporting fidelity note, not the
  acceptance surface.

## Acceptance shape (law, not number)

Solver-vs-oracle error must DECREASE MONOTONICALLY across at least three
grid resolutions, landing within named tolerances on: peak unit
discharge, sampled `t_peak`, and 10-90% rise time; plus total-variation
non-increase across the shock and exact booked-ledger closure
(`INV-OFEROUTE-006` rev 24). Proposed tolerance basis (to be ratified from
the S3/S4 evidence, per the package's S2 rule): finest-grid agreement
within a few percent on peak and timing metrics, with the ratified numbers
recorded in `iwagaki-case4-evidence.md` and the contract's tolerance
notes.
