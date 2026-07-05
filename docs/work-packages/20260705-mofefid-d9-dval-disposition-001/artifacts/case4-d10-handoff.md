# Case 4 D10 Handoff

Status: executed
Evidence mode: Static + Ran

## D9 Closed

Static + Ran:

- Cases 1-3 were re-run after D8 and have non-numerics dispositions recorded
  in `artifacts/dval-case-disposition.md`.
- Zone 1 / Zone 2 stream-power taxonomy was executed from Figure 9 and passes
  the D9 qualitative acceptance surface.
- D9 made no production/default activation claim and did not touch
  `OPENWEPP_LANED_SHADOW` activation.

## D10 Owns

First actionable item: close `GAP-OFEROUTE-005`.

D10 must produce:

- TVD/shock numerical-method authority for the Case 4 class, including
  primary/source-backed limiter/convergence criteria or a documented authority
  hold.
- Iwagaki Case-4 acceptance evidence with named tolerances for `NS_trace`,
  peak ratio, sampled `t_peak`, rise, and resolution sweep behavior.
- Convergence criteria over cell count, sample interval, and max sub-step; the
  D8 evidence shows the current peak/timing are resolution-sensitive.
- The real-H2637 resolution-sensitivity reproduction from the Lane D shadow
  increment carried into the same numerical-method verdict.

## D10 Does Not Own

Static + Ran: D10 does not own Case 1, Case 2, Case 3, or Zone taxonomy
adjudication. Those non-numerics surfaces are closed by D9 artifacts and
`SC-OFEROUTE-001` rev 17.
