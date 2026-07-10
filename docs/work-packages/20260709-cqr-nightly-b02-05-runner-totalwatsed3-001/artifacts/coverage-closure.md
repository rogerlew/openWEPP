# ADR-0021 Coverage Closure

Tier: science. This CLI publishes the watershed totalwatsed3 consumer surface,
whose pass/WAT input lineage, output identity, and units are protected by
`SC-SYSTEM-001`. The applicable floor is 90% lines and 90% regions, with no
function below 75% regions without an exclusion.

The public tests bind help/errors, exact input selection, required PASS failure,
optional explicit failure, aggregate and per-hillslope discovery, and emitted
totalwatsed3 values to real binary consumption. The detached-scaffold oracle
proved the new cases against pre-decomposition behavior. The explicit
obligation/surface-to-test binding and `SC-SYSTEM-001` applicability disposition
are recorded in `obligation-to-test-map.md`; no coverage exclusion is used.

Final production coverage is 94.416% direct lines and 93.443% deduplicated
regions. Every primary function clears 75% regions; no exclusion is needed.
