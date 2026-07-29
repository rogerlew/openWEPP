# CAL-09 Figure And Table Plan

Status: `FROZEN — operator-reviewed inventory selected for CAL-09 execution`

This artifact is the next planning surface. No candidate is selected merely
because a predecessor package already produced a plot. Final report assets
must answer a scientific question, resolve from retained data, remain
accessible without color, and have a Markdown or tabular alternative.

## Candidate Main-Report Figures

| ID | Scientific question | Candidate content | Likely evidence source | Status |
| --- | --- | --- | --- | --- |
| F1 | How do accepted coefficient combinations change model dynamics? | Three accepted timing exemplars with GSI21, canopy-cover, and LAI time series plus their six coefficient values | CAL-04B coefficients and CAL-06 daily model output | `MAIN REPORT` |
| F2 | How do deciduous, mixed, and evergreen structure differ through a year? | Canopy-cover and LAI time series across three sites and all available strata | CAL-06 | `SUPPLEMENT` |
| F3 | How does seasonal state propagate into litter, residue, and frost? | Daily litter, aggregate surface residue, and frost-depth time series | CAL-06 | `MAIN REPORT` |
| F4 | What did the temperate calibration identify, and did it transfer? | Observed interval chronology and accepted-ensemble modeled crossings through time at Hubbard and Harvard | CAL-04B | `MAIN REPORT` |
| F5 | Why are litter source and decay not uniquely determined? | Twenty-year source-decay ridge trajectories and source/rate pairs | CAL-05 | `MAIN REPORT` |
| F6 | Does canopy ordering produce coherent within-site snow response? | Harvard and Marcell modeled canopy/snow climatologies with observed snow-depth day-of-year medians | CAL-06 | `MAIN REPORT` |
| F7 | Are independent Southern observations consistent with modeled seasonality? | Alerce and Bezà observed GCC and accepted-ensemble GSI time series | CAL-07C | `SUPPLEMENT` |
| F8 | What is the tropical dry-forest contradiction? | Product-consistent Bezà `gcc_mean`, `gcc_90`, and model time series for both evaluation years | CAL-07F | `MAIN REPORT` |
| F9 | What was learned from the Elliot reproduction? | No figure; retain compact prose or table treatment | CAL-02 | `DROPPED BY OPERATOR` |
| F10 | What can and cannot currently be claimed? | Converted to integrated claim table T4 | CAL-09 claim matrix | `CONVERTED TO TABLE` |

## Candidate Main-Report Tables

| ID | Question | Candidate content | Status |
| --- | --- | --- | --- |
| T1 | What evidence was admitted and for what role? | Operand, observation, site, scale, unit, role, and source lineage | `MAIN REPORT / SUPPLEMENT DETAIL` |
| T2 | Which parameters were fitted or only bounded? | Units, authority, accepted ensemble, correlation, and identifiability | `MAIN REPORT / SUPPLEMENT DETAIL` |
| T3 | What did each evaluation cell show? | Process/litter/gradient/hemisphere results and verdicts | `MAIN REPORT` |
| T4 | What is the integrated claim envelope? | Supported, bounded, contradicted, and not-evaluated claims | `MAIN REPORT` |
| T5 | What happened in the Elliot comparison? | Compact reproduction scorecard | `SUPPLEMENT` |

## Freeze Decisions

1. F1 retains accepted correlated combinations; a one-at-a-time perturbation
   would obscure covariance and was not selected.
2. F3 remains coupled because its question is same-day propagation from
   litter to residue and frost.
3. F4 retains the time trend so calibration and transfer chronology remain
   inspectable; scalar residuals are reported in the table and prose.
4. F6 retains the model climatology and observed day-of-year medians with an
   explicit warning that they are not paired-date validation series.
5. F7 moves to the supplement; F8 carries the product-consistent independent
   Southern Hemisphere result in the main report.
6. Closure and churn remain in tables and supplement prose rather than
   ornamental insets.

## Built Candidate Set

The inspectable SVGs, paired Markdown sidecars, derived source rows, and source
manifest are in `figure-candidates/`. The deterministic builder is
`tools/build_candidate_figures.py`.

## Per-Figure Freeze Contract

For every selected figure, execution must record:

- stable figure and claim IDs;
- one-sentence scientific question;
- exact source objects and identities;
- quantities, units, signs, transformations, and aggregation;
- domain, forest class, site, period, sample/member count, and missing data;
- uncertainty or explanation of its absence;
- visual encodings that do not rely on color alone;
- caption stating quantity, domain, method, and scientific point;
- accessible text and source-table alternative;
- deterministic generation command and environment;
- rejected misleading alternatives; and
- manuscript and supplement placement.
