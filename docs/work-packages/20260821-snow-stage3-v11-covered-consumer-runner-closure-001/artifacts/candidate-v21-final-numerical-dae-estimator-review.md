# Candidate-v21 final numerical/DAE/estimator review

Ran: exact frozen candidate commit `8cbdc7fb9ce598fed3e138ef01823863058b84f2`.
The reviewer reran the package-local matrix and byte-compared its JSON output.

Disposition: `HOLD`.

Critical: for affine `y'=a y+b`, the candidate floor interpolation and
two-point Gauss rule reproduce the satisfied CN trapezoidal residual exactly.
Therefore `d_H=e_H=0` while the exact-minus-CN endpoint error is nonzero. The
matrix correctly reports 20 failed `gamma=2` enclosures; no finite positive
`gamma` repairs an identically zero estimate. `INV-SNOWENERGY-065` is unmet.

Additional major gaps: the scalar stiff rows are not a genuine linear DAE;
active-set-local entries are not executed vectors; complete SCC/seven-owner
generated-transfer conservation is not reconstructed; and candidate-CN
effectivity for the real 1.875-second receipt is unavailable.

Positive findings: the 1.2-second selector and 600 ms support floor are exact;
the above-selector CN Richardson sign is correct; and the four-component
coupling fail-closure is numerically sound.

Temporal-operator and Batch V2 implementation are not authorized.
