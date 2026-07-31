# Consumer-Path Evidence

Status: `HOLD`

Evidence mode: `Ran`

Ran: `tools/run_consumer_cells.py` built six isolated direct-production runs
from the unmodified H.J. Andrews conifer fixture and the real
`openwepp-cli-hill --direct-production-executor`. Full machine-readable
results are in [consumer-cells.json](consumer-cells.json).

| Cell | Result | Trace rows | Cumulative sublimation | Final traced SWE | WAT SHA-256 |
|---|---|---:|---:|---:|---|
| B absent | PASS | 16,437 | 0 m | 0 m | `6b19319c...e7a43c3` |
| B empty | PASS | 16,437 | 0 m | 0 m | `6b19319c...e7a43c3` |
| B disabled | PASS | 16,437 | 0 m | 0 m | `6b19319c...e7a43c3` |
| L | PASS | 16,437 | 0 m | 0 m | `35333632...124fc37` |
| S | FAIL CLOSED | 18 | 0.0013524 m | 0.0360654 m | partial |
| LS | FAIL CLOSED | 14 | 0.0009066 m | 0.0488007 m | partial |

The absent, empty, and explicit-disabled B controls have identical WAT and
trace hashes. This proves same-binary selector equivalence; it is not a
pre-EB-03 binary comparison. L completes and produces a different WAT hash
while preserving the protected snow trace.

S and LS both fail with the typed state error
`snow_sublimation.surface_temperature_k=0` against a strictly positive Kelvin
domain. Material snow remains, so
the failure is not pack exhaustion. Independent verification reproduced S
with both multilayer and bulk density providers, excluding density geometry as
the cause. The selected latent-only Stage 3 thermal composition is therefore
not a viable shared provider, and package exit criterion 4 does not pass.

Disposition: `HOLD / CLOSE_AS_MODEL_LIMITATION`. EB-04 is not admitted.
