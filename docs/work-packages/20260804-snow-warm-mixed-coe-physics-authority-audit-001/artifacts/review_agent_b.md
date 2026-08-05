# Independent Science Review B

Status: `PASS_WITH_FINDINGS`

Evidence mode: Static + Ran

Disposition agreement: conditionally agrees with
`BASELINE_FIDELITY_WITH_AUTHORITY_GAP` after narrowing its rationale.

## Findings

1. High: the draft treated full energy-balance state requirements as universal
   empirical-model requirements. Ohmura 2001 supports bounded temperature-index
   utility, Walter et al. 2005 recognizes contexts where added resolution may
   not be justified, and current contracts intentionally preserve CoE
   ownership. The supportable gap is lack of independent validation or bounded
   transferability authority for the material 2007/2008 changes.
2. Medium: density exposure is interval-start state, not the density after
   same-hour snowfall mixing or at retention/routing evaluation.
3. Low: “post-2008” should be “2008,” and signed all-hour `C` subcomponent sums
   cannot be described as causal or as a positive-only `C` decomposition.

## Independent Checks

Ran: without importing the package analyzer, Paradise reproduced at `136554`
hours and `5876` days. Aggregate `A/B/C/D` were
`5.201017180308592 / 8.329885304000204 / 14.260784580060195 /
0.775319143105237 m`; `C_open/C_canopy` were
`-0.16240155090939912 / 14.423186130969457 m`; all exposure counts and the
`306.36894506885386 W m^-2` maximum applied equivalent reproduced. Maximum
term residual was `5.48e-18 m`. A single-hour formula spot check also matched.

Required remediation: accept and correct all findings before terminal
verification.
