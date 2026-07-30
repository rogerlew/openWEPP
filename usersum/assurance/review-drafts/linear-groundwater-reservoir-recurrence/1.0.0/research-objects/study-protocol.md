# ASSURE-05 Groundwater Study Protocol

Status: FROZEN BEFORE FRESH RESULT EXECUTION

Evidence class: Static

## Question

Does openWEPP realization
`01ed70550a4e371e99afe35c4bdd4d9b667e812c` correctly implement the authorized
one-day linear groundwater-reservoir recurrence and transfer its generated
baseflow/deep-seepage volumes through the declared production consumer path?

This is formulation, code-verification, integration, and realization-transfer
evidence. It is not an empirical assessment of streamflow or groundwater-level
accuracy.

## Preregistered Equations And Timing

For daily recharge `D_i`, pre-export accepted storage `S_i`, baseflow `Qb_i`,
deep seepage `Qs_i`, and fixed one-day interval:

```text
S_i  = S_(i-1) + D_i - Qb_(i-1) - Qs_(i-1)
Qb_i = kb * S_i
Qs_i = ks * S_i
```

The terminal run identities are:

```text
S_N = S_0 + sum(D_i) - [sum(Qb_i) - Qb_N] - [sum(Qs_i) - Qs_N]
S_N - Qb_N - Qs_N = S_0 + sum(D_i) - sum(Qb_i) - sum(Qs_i)
```

All state and flux operands are cubic metres. `kb` and `ks` are inverse days.
The first identity compares pre-export terminal storage; the second compares
post-export terminal storage. The signs and prior-day debit order are fixed
before execution.

## Analytical Case

- area: `1,000 m2`;
- initial storage depth: `0.010 m`, hence `S_0 = 10 m3`;
- `kb = 0.10 d^-1`; `ks = 0.05 d^-1`;
- recharge: `2 m3` on day 1 and `4 m3` on day 2;
- expected values computed by an analysis procedure independent of the Rust
  recurrence implementation; and
- absolute per-value implementation allowance: `1.0e-12 m3`.

The negative over-export vector uses `kb = 0.80 d^-1` and `ks = 0.30 d^-1`.
It must fail before accepting exports greater than storage.

## Production Case

H2637 is the production-scale recurrence case. It spans 731 daily steps and 19
OFE records in the prior accepted evidence; those counts are observations to be
reconfirmed, not prefilled results. The fixture uses `kb = 0.04 d^-1`,
`ks = 0 d^-1`, and `S_0 = 0`. The ignored nextest case executes the same native
fixture with the active owner disabled, active by default, and explicitly
active; the default and explicit-active HBP and Parquet bytes must match.

For each H2637 terminal identity, the acceptance allowance is
`1.0e-9 * max(abs(reference storage), 1.0) m3`. Acceptance is two-sided:
`abs(residual) <= allowance`. Exact producer self-consistency is supporting
evidence only; the independent procedure reads produced manifest operands.

## Required Executable Evidence

1. the focused quick-profile selector for recurrence, guards, threshold,
   authority, HBP serialization, and watershed consumption;
2. the ignored H2637 active-owner production test under nextest process
   isolation;
3. exact output hashes for the retained manifest, HBP, and pass Parquet from
   the explicit-active leg;
4. independent analytical and H2637 reconstructions; and
5. assurance validate/plan/build/check against the revised report source.

## Prior Knowledge Boundary

Srivastava et al. (2013) evaluated a coupled WEPP plus linear-reservoir routine
at Priest River Experimental Forest. The official U.S. Forest Service record
and paper identify a calibrated 2005-2009 streamflow comparison and report that
including baseflow improved the study configuration. The formulation motivates
the recurrence. Its site-conditioned NSE, runoff-volume deviation, fitted
coefficients, and baseflow fractions are not openWEPP test results and will be
reported only as prior-study findings with that boundary.

Primary record: <https://research.fs.usda.gov/treesearch/43824>.
DOI: <https://doi.org/10.13031/2013.42691>.

## Uncertainty And Exclusions

- Floating-point reconstruction error is quantified; it is not environmental
  uncertainty.
- H2637 is a deterministic integration fixture, not an observed watershed.
- The production case has `ks = 0`, so nonzero deep seepage is exercised only
  by the analytical vector and domain tests.
- No parameter uncertainty, forcing uncertainty, measurement error,
  calibration/evaluation partition, subdaily timing, or model-form error is
  estimated.
- No coefficient transferability or site fitness is inferred.

## Change Control

Results observed after this protocol was frozen are recorded without changing
the equations, acceptance allowances, or exclusions. A method change creates a
new evidence root and must be explained before rerun. Contrary and failed cases
remain visible.
