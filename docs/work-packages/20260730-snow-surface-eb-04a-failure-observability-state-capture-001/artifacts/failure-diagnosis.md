# EB-04A Failure Diagnosis

Ran:

All 24 frozen EB-04 failures reproduced on the same model day and remained
fail-closed:

| Exact family | Count | What the snapshot establishes |
| --- | ---: | --- |
| Below-absolute-zero thermal projection | 17 | The active control volume retains positive cold content while its remaining ice mass becomes so small that `T = -CC/(m c_ice)` crosses `0 K`; the conductivity primitive correctly rejects that temperature. |
| Saturation-vapor-pressure underflow | 5 | The projected temperature is still above `0 K` but close enough to it that the SNOBAL saturation-vapor-pressure calculation underflows to zero and its positive-pressure type rejects the result before conductivity evaluation. |
| Prior-layer thickness aggregate mismatch | 2 | Reconstructed layer depths differ from the scalar prior depth by `1.008e-9 m` at Harvard open/S and `1.088e-9 m` at Marcell open/LS, just beyond the existing `1e-9 m` closure tolerance. |

The original “effective conductivity” label was therefore not a constitutive
conductivity diagnosis. Twenty-two failures are upstream extreme-cold
state/projection signatures; the remaining two are layer-geometry
reconciliation signatures.

Failure days span model day 13 through 12,517. S and LS are both represented;
two L failures are also retained. This breadth supports EB-04B’s planned
chronology analysis and rules out a startup-only explanation.

No correction is selected here. EB-04B must determine how sublimation mass
export, latent cooling, retained cold content, active-volume mass, and layer
geometry approach these boundaries.
