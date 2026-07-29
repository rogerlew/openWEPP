# Caption

**CAL-05 source/decay equifinality ridge.** Five analytically constructed
annual-input and decay-rate pairs follow distinct annual stock trajectories
but converge to the same year-20 terminal surface stock. “Equifinality” means
that different parameter combinations can produce the same observed outcome.

Paired figure: `cal05-source-decay-ridge.svg`.

## Why this figure exists

The 4×4 response grid suggests that source and decay can compensate for one
another. This figure makes that ambiguity explicit by constructing five pairs
that are guaranteed to share the same terminal stock. It demonstrates why an
endpoint-only calibration can appear successful while leaving the underlying
processes unresolved.

## How to read it

- In the left panel, decay rate increases from left to right and annual input
  increases from bottom to top. Moving along the ridge requires more input to
  offset faster loss.
- In the right panel, each colored line is the annual-end surface stock from
  one of those five pairs. Color and dash pattern both identify the pair, so
  color is not the only cue.
- The lines take visibly different paths but meet at the same year-20 value.
  The endpoint alone cannot tell which path produced it.
- Compare this with the daily-recovery figure: richer time-series information
  can distinguish paths that a common endpoint cannot.

## Plain-language takeaway

Five different stories end on the same last page. If only the final residue
stock is observed, the model cannot determine whether the site received less
litter with slow decay or more litter with fast decay. Measurements through
time—or independent source information—are needed to separate those effects.

## Ancillary information

- All five endpoints reproduce 0.8522711968936513 kg/m² within
  1.12×10⁻¹⁵ kg/m².
- The ridge demonstrates finite-horizon terminal-stock nonidentifiability. It
  is not an equilibrium result, probability model, empirical fit, or preferred
  parameter set.
- The ridge is an `ASSUMED_FOR_EXECUTION_ANALYTIC_RIDGE` construction and does
  not resolve missing predictive needle/fine-woody source authority or
  empirical decomposition readiness.
- The positive source/rate relationship is confounding evidence, not a
  probability distribution or uncertainty interval.

## Source data

- [`terminal-stock-ridge-design.csv`](../terminal-stock-ridge-design.csv)
- [`ridge-producer-results.csv`](../ridge-producer-results.csv)
- [`terminal-stock-equifinality.csv`](../terminal-stock-equifinality.csv)
- [`sensitivity-and-covariance.csv`](../sensitivity-and-covariance.csv)
- Renderer: [`plot_results.py`](../../tools/plot_results.py)
