# Scientific And Calibration Disposition

Status: `PASS`

Evidence mode: **Ran + Inference**.

## Decision

`SITE_SPECIFIC_CALIBRATION_COMPLETE / FORCING_BRANCH_CLOSED / NO_PROMOTION`

## Empirical Results

Mica Creek selects `1.4`: peak-SWE ratio `0.968` and chronology error 21 days,
versus baseline `0.619` and 35 days. Peak magnitude is bracketed between
`1.4-1.5`, but chronology continues improving to 8 days at `2.0` while peak SWE
overshoots to `1.529`. Uniform precipitation cannot optimize both outcomes;
Mica is a real magnitude-timing tradeoff.

Niwot selects `1.7`: peak-SWE ratio `1.029` and worst chronology error 13.5
days, versus baseline `0.495` and 46.5 days. The magnitude bracket is
`1.6-1.7`. Chronology reaches 9.5 days at `1.9`, but peak magnitude then
overshoots to `1.187`. The selected cell supplies `1.767` times observed peak
SWE as effective input and retains `0.808`; it narrowly avoids the frozen
compensation flag but still exposes a large input/storage gap.

Paradise selects `1.8`: peak-SWE ratio `0.989` and chronology error 0 days,
versus baseline `0.473` and 37 days. Magnitude and chronology optima coincide.
Effective input is `1.322` and observed-date storage is `0.962`. This is the
strongest joint site-specific calibration in the experiment.

Snowbird selects `2.0`: peak-SWE ratio `0.977` and chronology error 23 days,
versus baseline `0.390` and 44.5 days. Effective input reaches `1.405` and
retained storage `0.898`, so precipitation corrects most of the magnitude
deficit. The selected cell is nevertheless the final experiment-budget
boundary and chronology remains materially early; Snowbird is not assigned a
final calibrated multiplier.

## What We Learned

Precipitation deficiency or representativeness is capable of explaining much
more of the mountain magnitude deficit than EB-04W alone could establish. It
also materially improves persistence. Yet forcing is not a complete timing
solution: Mica and Niwot trade better chronology for magnitude overshoot, and
Snowbird remains 23 days early even when peak mass is nearly correct.

No lane triggers the exact frozen compensation flag. That is not proof of no
compensation. Niwot lies close to its storage threshold, and all inferred
factors can compensate for phase, liquid retention, or modeled loss error.

## Claim And Stop-Loss Boundary

Mica Creek `1.4`, Niwot `1.7`, and Paradise `1.8` are empirical calibration
candidates only for their exact climate fixtures and SNOTEL records. Snowbird
`2.0` is a boundary sensitivity candidate. None is independently validated,
transferable, regional, a production default, or proof of gauge undercatch.

The prospectively frozen stop-loss is now reached. Do not scaffold EB-04W3 or
continue upward precipitation search. Close the forcing branch and proceed to
EB-04X, where the paired Harvard design addresses geometry and interception.
