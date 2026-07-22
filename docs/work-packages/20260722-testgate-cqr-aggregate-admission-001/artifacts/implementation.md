# Implementation

Static: scaffold commit is `de22154d`; it predates every correction edit.

Static: the correction adds one consistent rule to the CQR standard,
work-package instructions, nightly ExecPlan, and per-module template: a
multi-package CQR batch that will close through one aggregate terminal diff
must commit an `ACTIVE` or `READY` aggregate authority package before the first
module implementation edit. The aggregate write set covers the master plan,
all module package trees, intended source/test paths, catalog, and closeout
evidence while each module retains its own one-module authority. Missing
authority blocks before implementation; retroactive widening remains forbidden.
