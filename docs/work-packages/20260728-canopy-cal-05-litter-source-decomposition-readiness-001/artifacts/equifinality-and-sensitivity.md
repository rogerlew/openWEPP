# Equifinality and Sensitivity

Evidence class: `Ran`

The direct-runtime surface-stock state responds to both synthetic annual input
and the surface-rate operand. Across the 16 frozen grid cells, year-20
terminal stock spans `0.1255480617` to `8.2000000000 kg m^-2`.

The complete daily synthetic series uniquely recovers frozen truth
`S020-K050`: daily-stock SSE is exactly zero. Every other frozen grid member
has positive SSE; the next smallest is `69.2982364078`.

Terminal stock alone is nonidentifying. All five analytically constructed
source/rate pairs reproduce `0.8522711968936513 kg m^-2` within
`1.12e-15`, well inside the frozen `1e-12` tolerance. This is a finite-horizon
source-rate ridge, not equilibrium, empirical calibration, or a preferred
parameter set.

Quantitative local sensitivities are retained in
`sensitivity-and-covariance.csv`. Terminal-stock sensitivity to source is
nonzero at every rate slice, and sensitivity to rate is nonzero at every
source slice. The five-pair ridge has positive source-rate covariance and
correlation, which is confounding evidence rather than a probability model.
The temperature modifier is interior at `0.4976215946`; flat and
standing-water factors are saturated at 1 but do not limit the selected
environmental index.

Zero direct-kernel rate with positive input accumulates without an equilibrium
claim. It is not the native-forest configured-zero projection, which uses the
contract fallback for a seasonal litter signal.
