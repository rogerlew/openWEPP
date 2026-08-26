# Real terminal DAE numerical review

Status: `GO` for the corrected defect-shaped `HOLD`; no real-candidate PASS.

Static: independent reviewer Jason inspected the exact terminal diff, Stage-3
transition, E04/LSE residuals, owner framing, receipt tool, numerical scaffold,
and `real-terminal-dae-defect-hold.md`.

Ran: the reviewer ran the receipt tool test 1/1, the focused numerical module
4/4, and `git diff --check` successfully.

Initial findings were one high, two medium, and one low:

1. The original 19-coordinate table omitted open LSE and hydrology custody and
   overclaimed complete-owner status.
2. Absolute `ModelTimeNs` was converted to `f64` seconds at callback boundaries.
3. Explicit scaling reached only the private Newton primitive, not CN/reference
   APIs.
4. The SCC prose mixed coarse orchestration nodes and internal nonlinear blocks.

Static / correction verification: all four are closed. The artifact now limits
the table to a 21-coordinate candidate-active census, adds open LSE and WB14
state, records the separate covered/open algebraic blocks, and explicitly
declines a complete `x` cardinality. `EvaluationTime` retains exact outer
`TimeSupport` plus a normalized abscissa without an absolute-clock cast. CN,
Gauss, and Radau accept explicit unknown/residual scales. Coarse and internal
SCCs are named separately. The focused correction rerun passed 4/4 and diff
hygiene passed; no new finding was reported.

Static / stop condition: the reviewer independently confirmed that production
exposes support-integrated Stage-3 fluxes, whole-support E04 storage, and
endpoint hydrology/WB14 transactions, not a complete arbitrary-time owner-local
`f/g`. Supplying the absent snow/canopy/hydrology/routing rates would select new
temporal physics. `CHILD1-REAL-DAE-001 / HOLD` is therefore the correct
numerical disposition.
