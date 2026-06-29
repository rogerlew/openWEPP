# GAP-SNOWFREEZE-002 Thaw-Residual Disposition

Evidence mode: Ran.

Source artifacts:

- `thaw_residual_buckets.json`
- `thaw_residual_buckets.csv`
- `thaw_residual_daily_windows.csv`
- `thaw_residual_diagnostic.md`
- `claude-review.md`
- `claude-review-disposition.md`

## Result

After residue-cover coupling, `13` Sleepers candidate-defect timing cells remain:
`11` thaw-late cells and `2` early-onset cells. The thaw-late cells are not tiny
detector tails under the diagnostic threshold sweep through `0.05 m`.

The original H1/H2/H1b bucket is still useful but not sufficient for routing:

| Mechanism | Count | Disposition |
| --- | ---: | --- |
| `H1a` missing wet/advective thaw energy | 9 | Over-broad route unless snow depth is controlled |
| `H1b` state-machine thaw asymmetry | 2 | Minority follow-up route; one cell is snow-buried |
| `H2` tiny-tail / detection artifact | 0 | Not supported through `0.05 m` |
| Early-onset material freeze | 2 | Separate onset mechanism |

The post-review snow-depth-controlled split is now the binding diagnostic route:

| Snow route at `0.10 m` | Count | Disposition |
| --- | ---: | --- |
| `SNOW-BURIED-UNDER-MELT` | 5 | Primary snow-persistence route: snowpack loses SWE but remains insulating |
| `SNOW-BURIED-ACCUMULATION` | 2 | Forcing-limited over-accumulation / near-balance route |
| `SNOW-FREE-PERSISTENT` | 2 | Genuine `Qwet` / wet-heat candidate subset |
| `MIXED-SNOW-CONTROL` | 2 | Secondary mixed snow/frost route |

## Threshold Sensitivity

The diagnostic primary material floor is `0.02 m`, not an adopted detector
threshold. The H2 count remains zero at `0.001`, `0.0025`, `0.005`, `0.01`,
`0.02`, and `0.05 m`. At `0.10 m`, four W9 thaw-late cells would become H2, but
that threshold is too large to adopt without frost-tube observation-protocol
authority and is not used for disposition.

The snow-control split is stable for the dominant route: `7` cells are
snow-buried at each diagnostic snow-depth threshold (`0.05`, `0.10`, and
`0.20 m`). The snow-free subset varies from `1` to `3` cells across that sweep;
the `0.10 m` split is reported as the primary diagnostic lens, not adopted as a
production threshold.

## Routing

`GAP-SNOWFREEZE-002` remains open and is narrowed to:

1. Primary: snow-persistence decomposition, separating forcing-limited
   over-accumulation / near-balance from fixable spring under-melt. This route
   must happen before a broad `Qwet` package because the dominant cells are
   snow-buried and the current heat flux is blocked above the soil.
2. Secondary: `Qwet` / wet-advective thaw energy only for the snow-free
   persistent subset (`2` cells at the `0.10 m` diagnostic split).
3. Secondary: freeze/thaw state-machine top-retreat work for the remaining
   `H1b` or mixed cells if they survive snow-persistence and snow-free wet-heat
   routing.
4. Separate: early-onset material freeze cells need an onset-specific forcing or
   freeze-initiation diagnostic; they should not be used to justify a thaw
   persistence fix.

No solver, detector, fixture, contract, default, or output-schema change was made.
