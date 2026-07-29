# Caption

**CAL-05 terminal-stock response.** Twenty-year terminal surface-residue
stock across the frozen 4×4 synthetic annual-input and nominal decay-rate
grid. Each row represents a hypothetical amount of litter added each year,
each column represents how quickly that material decays, and each cell reports
the modeled stock remaining after 20 years in kg/m².

Paired figure: `cal05-terminal-stock-response.svg`.

## Why this figure exists

CAL-05 asks whether litter input and decomposition can be estimated separately
from residue observations. Both processes affect the amount remaining on the
ground: input adds material while decay removes it. Before attempting an
empirical fit, this synthetic grid checks that the model responds to both
controls and shows how easily they can compensate for one another.

## How to read it

- Move upward through the rows to increase annual litter input.
- Move right through the columns to increase the nominal decay rate.
- Read the number inside a cell as the surface-residue stock at the end of the
  20-year simulation.
- Compare cells across a row to see the effect of decay at fixed input, and
  down a column to see the effect of input at fixed decay.
- The zero-decay column accumulates repeated inputs; it is a boundary case, not
  an equilibrium prediction.

## Plain-language takeaway

Adding more litter generally leaves more residue, while faster decay leaves
less. However, similar final stocks can arise from different combinations. A
single end-of-study residue measurement is therefore like seeing only the
balance in an account: it does not reveal how much entered and how quickly it
left.

## Ancillary information

- Synthetic annual surface-litter input spans 0.10–0.40 kg/m²/year.
- Nominal decay rate spans 0–2 year⁻¹.
- Terminal stock spans 0.1255480617–8.2000000000 kg/m² across the 16 cells.
- Every axis value is `ASSUMED_FOR_EXECUTION`. The surface is a deterministic
  readiness experiment, not empirical calibration, natural litter-source
  authority, probability, or a physiological bound.
- The figure shows the modeled aggregate surface pool. It does not partition
  leaf, needle, or fine-woody inputs or establish that their predictive source
  terms are available.

## Source data

- [`deterministic-design.csv`](../deterministic-design.csv)
- [`reconstruction-results.csv`](../reconstruction-results.csv)
- [`sensitivity-and-covariance.csv`](../sensitivity-and-covariance.csv)
- Renderer: [`plot_results.py`](../../tools/plot_results.py)
