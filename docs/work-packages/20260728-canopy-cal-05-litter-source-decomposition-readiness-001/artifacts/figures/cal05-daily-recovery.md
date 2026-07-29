# Caption

**CAL-05 complete daily-trace recovery.** Daily surface-stock sum of squared
errors for every member of the frozen 16-candidate source/rate grid. The
experiment generated a synthetic daily record with `S020-K050`, then tested
whether each candidate could reproduce that record. Nonzero errors use a
logarithmic vertical scale.

Paired figure: `cal05-daily-recovery.svg`.

## Why this figure exists

The terminal-stock response shows that one endpoint can be ambiguous. This
figure asks the complementary question: if the complete day-by-day residue
history were available, would the frozen source/rate combinations still be
indistinguishable? It is a controlled identifiability check using known
synthetic truth, not field calibration.

## How to read it

- Candidate names encode the synthetic annual input and nominal decay rate.
  `S020-K050` means 0.20 kg/m²/year of input and 0.50 year⁻¹ nominal decay.
- Bar height is the sum of squared differences between a candidate's daily
  stock and the synthetic truth. Smaller is better.
- Because incorrect candidates differ by orders of magnitude, their bars use
  a logarithmic scale.
- The orange `exact 0` marker is not a missing bar. It means `S020-K050`
  reproduces every retained daily stock value exactly.

## Plain-language takeaway

The full trajectory acts like a movie rather than a final photograph. Within
this frozen synthetic grid, the movie contains enough timing and magnitude
information to recover the generating source/rate pair uniquely—even though
one final stock value may not.

## Ancillary information

- `S020-K050` is the sole recovered truth and has exactly zero daily-stock
  SSE.
- Every other frozen candidate has positive error; the next-smallest SSE is
  69.2982364078.
- The experiment remains `ASSUMED_FOR_EXECUTION`; no parameter was selected
  for production use.
- Recoverability is conditional on complete, exact synthetic daily stock and
  the 16 candidates tested. Field observations may be sparse, noisy, or on a
  different material basis.
- The result does not supply empirical source composition, validate predictive
  needle/fine-woody inputs, or calibrate decomposition.

## Source data

- [`synthetic-recovery.csv`](../synthetic-recovery.csv)
- [`deterministic-design.csv`](../deterministic-design.csv)
- [`reconstruction-results.csv`](../reconstruction-results.csv)
- Renderer: [`plot_results.py`](../../tools/plot_results.py)
