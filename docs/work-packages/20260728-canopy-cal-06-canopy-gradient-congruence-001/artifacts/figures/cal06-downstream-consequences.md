# Caption

**CAL-06 downstream model responses.** Daily interception, evapotranspiration,
and runoff for the Harvard canopy strata. Lines show ensemble medians and
shaded bands span the frozen timing ensemble.

Paired figure: `cal06-downstream-consequences.svg`.

## Ancillary information

- All plotted quantities are expressed in millimeters per day.
- Harvard supplies open, deciduous, and mixed lanes; no Harvard conifer lane is
  available, so conifer is intentionally absent from both plots and legend.
- The curves were produced by real downstream consumers, but remain
  model-response evidence.
- Interception/ET and runoff are `NOT_ADVANCED` because upstream canopy,
  snow, litter, residue, and frost evidence is bounded or incomplete.
- Erosion-facing canopy inputs were present, but no erosion consequence output
  was emitted. `NULL_NOT_EMITTED` is not a measured zero.
- No downstream residual selected or retuned a canopy or litter operand.

## Source data

- [`daily-climatology.csv`](../daily-climatology.csv)
- [`verdict-matrix.csv`](../verdict-matrix.csv)
- [`consumer-lineage.md`](../consumer-lineage.md)
- Renderer: [`plot_results.py`](../../tools/plot_results.py)
